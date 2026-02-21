use async_channel::{Receiver, Sender};
use listeners::{Process, Protocol};
use std::collections::HashMap;
use std::time::Instant;

const RETRY_TIMEOUT: u128 = 1500; // milliseconds
pub const VALID_PROGRAM_TIMEOUT: u128 = 60_000; // milliseconds

pub struct ProgramLookup {
    map: HashMap<(u16, Protocol), LookedUpProgram>,
    port_tx: Sender<(u16, Protocol)>,
}

impl ProgramLookup {
    pub fn new(port_tx: Sender<(u16, Protocol)>) -> Self {
        Self {
            map: HashMap::new(),
            port_tx,
        }
    }

    pub fn lookup(&mut self, key: (u16, Protocol), is_new_connection: bool) -> Option<Process> {
        if let Some(looked_up_program) = self.map.get_mut(&key) {
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
            self.map.insert(key, looked_up_program);
            // send this to the listeners routine
            let _ = self.port_tx.send_blocking(key);
            None
        }
    }

    pub fn update(&mut self, lookup_res: (u16, Protocol, Option<Process>)) {
        let key = (lookup_res.0, lookup_res.1);
        let program = lookup_res.2;
        self.map.entry(key).and_modify(|looked_up_program| {
            looked_up_program.program = program;
            looked_up_program.instant = Instant::now();
        });
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
