//! Module containing functions executed by the thread in charge of updating the output report
//! every ```interval``` seconds, with ```interval``` specified by the user through the
//! ```-i``` command line option.
//!
//! If the ```-i``` option is not specified, the report is updated every 5 seconds.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration};
use std::{fs, thread};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use thousands::Separable;
use crate::{AppProtocol, InfoTraffic, Status};


/// The calling thread enters in a loop in which it waits for ```interval``` seconds and then re-write
/// from scratch the output report file, with updated values.
///
/// # Arguments
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
///
/// * `interval` - Frequency of report updates (value in seconds). Specified by the user through the
/// ```-i``` option.
///
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A TransProtocol representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// * `output_folder` - A String representing the folder to contain the reports. Specified by the user through the
/// ```-o``` option.
///
/// * `info_traffic_mutex` - Struct with all the relevant info on the network traffic analyzed.
///
/// * `status_pair` - Shared variable to check the application current status.
pub fn sleep_and_write_report_loop(current_capture_id: Arc<Mutex<u16>>,
                                   info_traffic_mutex: Arc<Mutex<InfoTraffic>>,
                                   status_pair: Arc<(Mutex<Status>, Condvar)>) {

    let cvar = &status_pair.1;

    // #[cfg(target_os = "windows")]
    // std::process::Command::new("explorer")
    //     .arg("./sniffnet_report/report.txt")
    //     .spawn()
    //     .unwrap();
    // #[cfg(target_os = "macos")]
    // std::process::Command::new("cd")
    //     .args('~`)
    //     .spawn()
    //     .unwrap();
    // #[cfg(target_os = "linux")]
    // std::process::Command::new("explorer")
    //     .arg("./sniffnet_report/report.txt")
    //     .spawn()
    //     .unwrap();

    if fs::create_dir("./sniffnet_report").is_err() {
        fs::remove_dir_all("./sniffnet_report").unwrap();
        fs::create_dir("./sniffnet_report").unwrap();
    }

    let path_report = "./sniffnet_report/report.txt";

    // let time_origin = Local::now();
    // let first_timestamp = time_origin.format("%d/%m/%Y %H:%M:%S").to_string();

    let mut _time_header = 0;
    let mut _time_header_sort = 0;
    let mut _time_header_sort_print = 0;

    let mut capture_id = *current_capture_id.lock().unwrap();

    let mut output = BufWriter::new(File::create(path_report.clone()).expect("Error creating output file\n\r"));
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
    writeln!(output, "|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |  Initial timestamp  |   Final timestamp   |").expect("Error writing output file\n\r");
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");

    loop {
        // sleep 1 second
        thread::sleep(Duration::from_secs(1));

        let current_capture_id_lock = current_capture_id.lock().unwrap();
        if *current_capture_id_lock != capture_id {
            capture_id = *current_capture_id_lock;
            output = BufWriter::new(File::create(path_report.clone()).expect("Error creating output file\n\r"));
            writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
            writeln!(output, "|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |  Initial timestamp  |   Final timestamp   |").expect("Error writing output file\n\r");
            writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
        }
        drop(current_capture_id_lock);

        let mut status = status_pair.0.lock().expect("Error acquiring mutex\n\r");

        if *status == Status::Running {

            drop(status);

            let mut info_traffic = info_traffic_mutex.lock().expect("Error acquiring mutex\n\r");

            for index in info_traffic.addresses_last_interval.iter() {
                let key_val = info_traffic.map.get_index(*index).unwrap();
                let seek_pos = 166*3 + 206*(*index) as u64;
                output.seek(SeekFrom::Start(seek_pos)).unwrap();
                writeln!(output, "{}{}", key_val.0, key_val.1).expect("Error writing output file\n\r");
            }
            info_traffic.addresses_last_interval = HashSet::new(); // empty set

            output.flush().expect("Error writing output file\n\r");

            drop(info_traffic);

        }
        else if *status == Status::Stop {
            return;
        }
        else { //status is Init
            while *status == Status::Init {
                status = cvar.wait(status).expect("Error acquiring mutex\n\r");
            }
        }
    }
}


/// Given the map of app layer protocols with the relative sniffed packets count,
/// the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `app_count` - Map of app layer protocols with the relative sniffed packets count
///
/// * `tot_packets` - Total number of sniffed packets
pub fn get_app_count_string(app_count: HashMap<AppProtocol, u128>, tot_packets: u128) -> String {

    let mut ret_val = "".to_string();

    let mut sorted_app_count: Vec<(&AppProtocol, &u128)> = app_count.iter().collect();
    sorted_app_count.sort_by(|&(p1, a), &(p2, b)| {
        if p1.eq(&AppProtocol::Other) {
            Ordering::Greater
        }
        else if p2.eq(&AppProtocol::Other) {
            Ordering::Less
        }
        else {
            b.cmp(a)
        }
    });

    //compute the length of the longest packet count string, used to align text
    let mut longest_num;
    longest_num = sorted_app_count.get(0).unwrap().1.separate_with_spaces().len();
    match app_count.get(&AppProtocol::Other) {
        None => {}
        Some(x) => {
            if x.separate_with_spaces().len() > longest_num {
                longest_num = x.separate_with_spaces().len();
            }
        }
    }

    for entry in sorted_app_count {

        let app_proto_string = format!("{:?}", entry.0);

        let num_string = entry.1.separate_with_spaces().to_string();

        let percentage_string =
            if format!("{:.1}", 100.0*(*entry.1) as f32/tot_packets as f32).eq("0.0") {
                "(<0.1%)".to_string()
            }
            else {
                format!("({:.1}%)", 100.0*(*entry.1) as f32/tot_packets as f32)
            };

        //to align digits
        let spaces_string_1 = " ".to_string()
            .repeat(9+longest_num-num_string.len()-app_proto_string.len());
        let spaces_string_2 = " ".to_string()
            .repeat(10-percentage_string.len());

        ret_val.push_str(&format!("   {}:{}{}{}{}\n",
                                  app_proto_string,
                                  spaces_string_1,
                                  num_string,
                                  spaces_string_2,
                                  percentage_string));

    }
    ret_val
}