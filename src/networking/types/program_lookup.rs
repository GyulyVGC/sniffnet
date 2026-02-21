use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::program::Program;
use async_channel::{Receiver, Sender};
use listeners::{Process, Protocol};
use std::collections::HashMap;
use std::time::Instant;

const RETRY_TIMEOUT: u128 = 1500; // milliseconds
pub const VALID_PROGRAM_TIMEOUT: u128 = 60_000; // milliseconds

pub struct ProgramLookup {
    state: HashMap<(u16, Protocol), LookedUpProgram>,
    port_tx: Sender<(u16, Protocol)>,
    programs: HashMap<Program, DataInfo>,
}

impl ProgramLookup {
    pub fn new(port_tx: Sender<(u16, Protocol)>) -> Self {
        Self {
            state: HashMap::new(),
            port_tx,
            programs: HashMap::new(),
        }
    }

    /// Called on new connection, or on existing connection with an already associated program
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

    /// Called on existing connection with an unknown program
    pub fn lookup_and_replace_data(
        &mut self,
        key: (u16, Protocol),
        so_far_data: DataInfo,
        new_data: DataInfo,
    ) -> Program {
        let proc = self.lookup(key, false);

        let res = Program::from_proc(proc.as_ref());

        if res.is_unknown() {
            // program is still unknown => just add data for Unknown
            self.programs
                .entry(Program::Unknown)
                .and_modify(|d| d.refresh(new_data));
        } else {
            // program just became known => subtract so_far_data from Unknown && add total for known
            self.programs
                .entry(Program::Unknown)
                .and_modify(|d| d.subtract(so_far_data));

            let mut total = so_far_data;
            total.refresh(new_data);
            self.programs
                .entry(res.clone())
                .and_modify(|d| d.refresh(total))
                .or_insert(total);
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
                let _ = self.port_tx.send_blocking(key);
                return None;
            }

            if program.is_none() && is_new_connection && !was_recently_tried {
                looked_up_program.retried = program_still_valid;
                looked_up_program.instant = Instant::now();
                // send this to the listeners routine
                let _ = self.port_tx.send_blocking(key);
                return None;
            }

            if program.is_none() && !is_new_connection && !was_recently_tried && !already_retried {
                looked_up_program.retried = true;
                looked_up_program.instant = Instant::now();
                // send this to the listeners routine
                let _ = self.port_tx.send_blocking(key);
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
            let _ = self.port_tx.send_blocking(key);
            None
        }
    }

    pub fn update(
        &mut self,
        lookup_res: (u16, Protocol, Option<Process>),
        reassigned_data: DataInfo,
    ) {
        let key = (lookup_res.0, lookup_res.1);
        let proc = lookup_res.2;

        if reassigned_data.tot_data(DataRepr::Packets) > 0 {
            // remove from Unknown
            self.programs
                .entry(Program::Unknown)
                .and_modify(|d| d.subtract(reassigned_data));
            // assign to known
            let program = Program::from_proc(proc.as_ref());
            self.programs
                .entry(program)
                .and_modify(|d| d.refresh(reassigned_data))
                .or_insert(reassigned_data);
        }

        self.state.entry(key).and_modify(|looked_up_program| {
            looked_up_program.program = proc;
            looked_up_program.instant = Instant::now();
        });
    }

    pub fn programs(&self) -> &HashMap<Program, DataInfo> {
        &self.programs
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
    while let Ok((port, protocol)) = port_rx.recv_blocking() {
        let program = listeners::get_process_by_port(port, protocol).ok();
        let _ = program_tx.send_blocking((port, protocol, program));
    }
}
