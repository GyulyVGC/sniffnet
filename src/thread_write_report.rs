//! Module containing functions executed by the thread in charge of updating the output report every 1 second

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crate::enums::status::Status;
use crate::utility::get_formatted_strings::get_report_path;
use crate::InfoTraffic;

/// The calling thread enters in a loop in which it sleeps for 1 second and then
/// updates the output report containing detailed traffic information
pub fn sleep_and_write_report_loop(
    current_capture_id: &Arc<Mutex<u16>>,
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
    status_pair: &Arc<(Mutex<Status>, Condvar)>,
) {
    let cvar = &status_pair.1;

    let path_report = get_report_path();

    let mut capture_id = *current_capture_id.lock().unwrap();

    let mut output =
        BufWriter::new(File::create(path_report.clone()).expect("Error creating output file\n\r"));
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
    writeln!(output, "|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |  Initial timestamp  |   Final timestamp   |").expect("Error writing output file\n\r");
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");

    loop {
        // sleep 1 second
        thread::sleep(Duration::from_secs(1));

        let current_capture_id_lock = current_capture_id.lock().unwrap();
        if *current_capture_id_lock != capture_id {
            capture_id = *current_capture_id_lock;
            output = BufWriter::new(
                File::create(path_report.clone()).expect("Error creating output file\n\r"),
            );
            writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
            writeln!(output, "|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |  Initial timestamp  |   Final timestamp   |").expect("Error writing output file\n\r");
            writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
        }
        drop(current_capture_id_lock);

        let mut status = status_pair.0.lock().expect("Error acquiring mutex\n\r");

        if *status == Status::Running {
            drop(status);

            let mut info_traffic = info_traffic_mutex
                .lock()
                .expect("Error acquiring mutex\n\r");

            for index in &info_traffic.addresses_last_interval {
                let key_val = info_traffic.map.get_index(*index).unwrap();
                let seek_pos = 166 * 3 + 206 * (*index) as u64;
                output.seek(SeekFrom::Start(seek_pos)).unwrap();
                writeln!(output, "{}{}", key_val.0, key_val.1)
                    .expect("Error writing output file\n\r");
            }
            info_traffic.addresses_last_interval = HashSet::new(); // empty set

            output.flush().expect("Error writing output file\n\r");

            drop(info_traffic);
        } else {
            //status is Init
            while *status == Status::Init {
                status = cvar.wait(status).expect("Error acquiring mutex\n\r");
            }
        }
    }
}
