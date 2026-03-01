use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::TOOLTIP_DELAY;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::networking::manage_packets::get_local_port;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::program::Program;
use iced::Length;
use iced::widget::image::Handle;
use iced::widget::tooltip::Position;
use iced::widget::{Text, Tooltip, image};
use listeners::{Process, Protocol};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

const RETRY_TIMEOUT: u128 = 1500; // milliseconds
const VALID_PROGRAM_TIMEOUT: u128 = 60_000; // milliseconds

pub struct ProgramLookup {
    port_tx: Sender<(u16, Protocol)>,
    program_rx: Receiver<(u16, Protocol, Option<Process>)>,
    path_tx: Sender<String>,
    picon_rx: Receiver<(String, Handle)>,
    state: HashMap<(u16, Protocol), LookedUpProgram>,
    programs: HashMap<Program, DataInfo>,
    picons: HashMap<String, Handle>,
}

impl ProgramLookup {
    pub fn new(
        port_tx: Sender<(u16, Protocol)>,
        program_rx: Receiver<(u16, Protocol, Option<Process>)>,
        path_tx: Sender<String>,
        picon_rx: Receiver<(String, Handle)>,
    ) -> Self {
        Self {
            port_tx,
            program_rx,
            path_tx,
            picon_rx,
            state: HashMap::new(),
            programs: HashMap::new(),
            picons: HashMap::new(),
        }
    }

    pub fn lookup_and_add_data(
        &mut self,
        key: (u16, Protocol),
        is_new_connection: bool,
        new_data: DataInfo,
    ) -> Program {
        let proc = self.lookup(key, is_new_connection);

        let res = Program::from_proc(proc.as_ref());
        self.programs
            .entry(res.clone())
            .and_modify(|d| d.refresh(new_data))
            .or_insert(new_data);

        res
    }

    pub fn pending_results(&mut self) -> Vec<(u16, Protocol, Option<Process>)> {
        let mut res = Vec::new();
        while let Ok(lookup_res) = self.program_rx.try_recv() {
            res.push(lookup_res);
        }
        res
    }

    fn lookup(&mut self, key: (u16, Protocol), is_new_connection: bool) -> Option<Process> {
        if let Some(looked_up_program) = self.state.get_mut(&key) {
            let program = &looked_up_program.program;
            let was_recently_tried =
                looked_up_program.instant.elapsed().as_millis() < RETRY_TIMEOUT;
            let program_still_valid =
                looked_up_program.instant.elapsed().as_millis() < VALID_PROGRAM_TIMEOUT;
            let already_retried = looked_up_program.retried;

            if program.is_some() && is_new_connection && !program_still_valid {
                looked_up_program.retried = false;
                looked_up_program.instant = Instant::now();
                // send this to the listeners routine
                let _ = self.port_tx.send(key);
                return None;
            }

            if program.is_none() && is_new_connection && !was_recently_tried {
                looked_up_program.retried = program_still_valid;
                looked_up_program.instant = Instant::now();
                // send this to the listeners routine
                let _ = self.port_tx.send(key);
                return None;
            }

            if program.is_none() && !is_new_connection && !was_recently_tried && !already_retried {
                looked_up_program.retried = true;
                looked_up_program.instant = Instant::now();
                // send this to the listeners routine
                let _ = self.port_tx.send(key);
                return None;
            }

            program.clone()
        } else {
            let looked_up_program = LookedUpProgram {
                program: None,
                instant: Instant::now(),
                retried: false,
            };
            self.state.insert(key, looked_up_program);
            // send this to the listeners routine
            let _ = self.port_tx.send(key);
            None
        }
    }

    pub fn update(
        &mut self,
        lookup_res: (u16, Protocol, Option<Process>),
        connections: &mut HashMap<AddressPortPair, InfoAddressPortPair>,
    ) {
        let key = (lookup_res.0, lookup_res.1);
        let proc = lookup_res.2;

        // associate unassigned recent connections on port with the program
        if proc.is_some() {
            let mut reassigned_data = DataInfo::default();
            connections
                .iter_mut()
                .filter(|(k, v)| {
                    v.program.is_unknown()
                        && v.final_instant.elapsed().as_millis() < VALID_PROGRAM_TIMEOUT
                        && get_local_port(k, v.traffic_direction) == Some(key)
                })
                .for_each(|(_, v)| {
                    v.program = Program::from_proc(proc.as_ref());
                    reassigned_data.refresh(v.data_info());
                });

            if reassigned_data.tot_data(DataRepr::Packets) > 0 {
                // assign to known
                let program = Program::from_proc(proc.as_ref());
                self.programs
                    .entry(program)
                    .and_modify(|d| d.refresh(reassigned_data))
                    .or_insert(reassigned_data);
                // remove from Unknown
                // NOTE: subtracting reassigned_data from Unknown wouldn't correctly reassign final_instant,
                // so let's just reiterate through all the Unknown connections
                let mut unknown_data = DataInfo::default();
                connections
                    .iter()
                    .filter(|(_, v)| v.program.is_unknown())
                    .for_each(|(_, v)| {
                        unknown_data.refresh(v.data_info());
                    });
                self.programs
                    .entry(Program::Unknown)
                    .and_modify(|d| *d = unknown_data);
            }
        }

        // icon retrieval
        if let Some(proc) = proc.as_ref() {
            let path = &proc.path;
            if !self.picons.contains_key(path) {
                self.picons.insert(path.clone(), DEFAULT_PICON.clone());
                let _ = self.path_tx.send(path.clone());
            }
        }

        self.state.entry(key).and_modify(|looked_up_program| {
            looked_up_program.program = proc;
            looked_up_program.instant = Instant::now();
        });
    }

    pub fn handle_pending_icons(&mut self) {
        while let Ok((path, picon)) = self.picon_rx.try_recv() {
            self.picons.insert(path, picon);
        }
    }

    pub fn programs(&self) -> &HashMap<Program, DataInfo> {
        &self.programs
    }

    pub fn picon_tooltip<'a>(&self, program_path: String) -> Tooltip<'a, Message, StyleType> {
        let tooltip_class = if program_path.is_empty() {
            ContainerType::Standard
        } else {
            ContainerType::Tooltip
        };

        let handle = self.picons.get(&program_path).unwrap_or(&DEFAULT_PICON);
        let content = image(handle).height(Length::Fill);

        Tooltip::new(content, Text::new(program_path), Position::FollowCursor)
            .snap_within_viewport(true)
            .class(tooltip_class)
            .delay(TOOLTIP_DELAY)
    }
}

struct LookedUpProgram {
    program: Option<Process>,
    instant: Instant,
    retried: bool,
}

pub fn lookup_program(
    port_rx: &Receiver<(u16, Protocol)>,
    program_tx: &Sender<(u16, Protocol, Option<Process>)>,
) {
    while let Ok((port, protocol)) = port_rx.recv() {
        let program = listeners::get_process_by_port(port, protocol).ok();
        let _ = program_tx.send((port, protocol, program));
    }
}

pub fn get_picon(path_rx: &Receiver<String>, picon_tx: &Sender<(String, Handle)>) {
    while let Ok(path) = path_rx.recv() {
        if let Some(handle) = picon::get_icon_by_path(&path) {
            let _ = picon_tx.send((path, handle));
        }
    }
}

const DEFAULT_PICON_BYTES: &[u8] =
    include_bytes!("../../../resources/countries_flags/default_picon.png");

static DEFAULT_PICON: std::sync::LazyLock<Handle> =
    std::sync::LazyLock::new(|| Handle::from_bytes(DEFAULT_PICON_BYTES));
