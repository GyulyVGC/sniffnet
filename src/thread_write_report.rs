//! Module containing functions executed by the thread in charge of updating the output report
//! every ```interval``` seconds, with ```interval``` specified by the user through the
//! ```-i``` command line option.
//!
//! If the ```-i``` option is not specified, the report is updated every 5 seconds.

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration};
use std::{fs, thread};
use chrono::{Local};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use thousands::Separable;
use crate::{AppProtocol, ChartsData, Filters, InfoTraffic, Status, TransProtocol};
use pcap::Device;


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
pub fn sleep_and_write_report_loop(current_capture_id: Arc<Mutex<u16>>, lowest_port: u16, highest_port: u16, interval: u64,
                                   device: Arc<Mutex<Device>>, filters: Arc<Mutex<Filters>>,
                                   output_folder: String, info_traffic_mutex: Arc<Mutex<InfoTraffic>>,
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

    if fs::create_dir(output_folder.clone()).is_err() {
        fs::remove_dir_all(output_folder.clone()).unwrap();
        fs::create_dir(output_folder.clone()).unwrap();
    }

    let path_report = format!("{}/report.txt", output_folder);
    let path_statistics = format!("{}/statistics.txt", output_folder);

    let time_origin = Local::now();
    let first_timestamp = time_origin.format("%d/%m/%Y %H:%M:%S").to_string();

    let mut network_layer = "no filter".to_string();
    let mut transport_layer= TransProtocol::Other;
    let mut app_layer= AppProtocol::Other;
    let mut update_filters = true;

    let mut _time_header = 0;
    let mut _time_header_sort = 0;
    let mut _time_header_sort_print = 0;

    // let mut min_sent_bits_second: i128 = 0;

    // let mut max_received_bits_second: i128 = 0;

    // let mut min_sent_packets_second: i128 = 0;

    // let mut max_received_packets_second: i128 = 0;

    let mut capture_id = *current_capture_id.lock().unwrap();

    let mut output = BufWriter::new(File::create(path_report.clone()).expect("Error creating output file\n\r"));
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");
    writeln!(output, "|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |  Initial timestamp  |   Final timestamp   |").expect("Error writing output file\n\r");
    writeln!(output, "---------------------------------------------------------------------------------------------------------------------------------------------------------------------").expect("Error writing output file\n\r");

    loop {
        // sleep interval seconds
        thread::sleep(Duration::from_secs(interval));

        let current_capture_id_lock = current_capture_id.lock().unwrap();
        if *current_capture_id_lock != capture_id {
            update_filters = true;
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

            let tot_sent_packets = info_traffic.tot_sent_packets;
            let tot_received_packets = info_traffic.tot_received_packets;
            let all_packets = info_traffic.all_packets;
            let tot_received_bytes = info_traffic.tot_received_bytes;
            let tot_sent_bytes = info_traffic.tot_sent_bytes;


            let mut output2 = BufWriter::new(File::create(path_statistics.clone()).expect("Error creating output file\n\r"));

            if update_filters {
                let filtri = filters.lock().unwrap();
                network_layer = filtri.ip.clone();
                transport_layer = filtri.transport;
                app_layer = filtri.application;
                drop(filtri);
                update_filters = false;
            }

            write_statistics(output2.get_mut().try_clone().expect("Error cloning file handler\n\r"),
                                     device.lock().unwrap().name.to_string(), first_timestamp.clone(),
                                     lowest_port, highest_port, network_layer.clone(),
                                     transport_layer, app_layer,
                                     info_traffic.map.len(), all_packets,
                                     tot_received_packets+tot_sent_packets,
                                     info_traffic.app_protocols.clone());
            output2.flush().expect("Error writing output file\n\r");

            for key in info_traffic.addresses_last_interval.iter() {
                let val = info_traffic.map.get(key).unwrap();
                let index = info_traffic.map.get_index_of(key).unwrap();
                let seek_pos = 166*3 + 206*index as u64;
                output.seek(SeekFrom::Start(seek_pos)).unwrap();
                writeln!(output, "{}{}", key, val).expect("Error writing output file\n\r");
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


/// Given the lowest and highest port numbers, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
fn get_ports_string(lowest_port: u16, highest_port: u16) -> String {
    if lowest_port == highest_port {
        format!("\t[x] Considering only port number {}\n", lowest_port)
    }
    else if lowest_port != u16::MIN || highest_port != u16::MAX {
        format!("\t[x] Considering only port numbers from {} to {}\n", lowest_port, highest_port)
    }
    else {
        format!("\t[ ] Considering all port numbers (from {} to {})\n", lowest_port, highest_port)
    }
}


/// Given the network layer textual filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
fn get_network_layer_string (network_layer: String) -> String {
    if network_layer.cmp(&"ipv4".to_string()) == Equal {
        "\t[x] Considering only IPv4 packets\n".to_string()
    }
    else if network_layer.cmp(&"ipv6".to_string()) == Equal {
        "\t[x] Considering only IPv6 packets\n".to_string()
    }
    else {
        "\t[ ] Considering both IPv4 and IPv6 packets\n".to_string()
    }
}


/// Given the transport layer textual filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `transport_layer` - A TransProtocol representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
fn get_transport_layer_string(transport_layer: TransProtocol) -> String {
    if transport_layer.eq(&TransProtocol::TCP) {
        "\t[x] Considering only packets exchanged with TCP\n".to_string()
    }
    else if transport_layer.eq(&TransProtocol::UDP) {
        "\t[x] Considering only packets exchanged with UDP\n".to_string()
    }
    else {
        "\t[ ] Considering packets exchanged both with TCP and/or UDP\n".to_string()
    }
}


/// Given the application layer filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `app_layer` - A String representing the application layer protocol to be filtered. Specified by the user through the
/// ```--app``` option.
fn get_app_layer_string(app_layer: AppProtocol) -> String {
    if app_layer.eq(&AppProtocol::Other) {
        "\t[ ] Considering all application layer protocols\n".to_string()
    }
    else {
        format!("\t[x] Considering only {:?} packets\n", app_layer)
    }
}


/// Given the numbers of sniffed packets and filtered packets, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `sniffed` - Number of sniffed packets.
///
/// * `filtered` - Number of filtered packets
fn get_filtered_packets_string(sniffed: u128, filtered: u128) -> String {
    if sniffed != 0 && filtered != 0 {
        let percentage_string =
            if format!("{:.2}", 100.0*filtered as f32/sniffed as f32).eq("0.00") {
                "(<0.01%)".to_string()
            }
            else {
                format!("({:.2}%)", 100.0*filtered as f32/sniffed as f32)
            };
        format!("\tConsidered packets: {} {}\n",
                filtered.separate_with_spaces(), percentage_string)
    }
    else {
        format!("\tConsidered packets: {}\n",
                filtered.separate_with_spaces())
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
            if format!("{:.2}", 100.0*(*entry.1) as f32/tot_packets as f32).eq("0.00") {
                "(<0.01%)".to_string()
            }
            else {
                format!("({:.2}%)", 100.0*(*entry.1) as f32/tot_packets as f32)
            };

        //to align digits
        let spaces_string_1 = " ".to_string()
            .repeat(9+longest_num-num_string.len()-app_proto_string.len());
        let spaces_string_2 = " ".to_string()
            .repeat(11-percentage_string.len());

        ret_val.push_str(&format!("  \t-{}:{}{}{}{}\n",
                                  app_proto_string,
                                  spaces_string_1,
                                  num_string,
                                  spaces_string_2,
                                  percentage_string));

    }

    ret_val
}


/// Writes the output report file header, which contains useful info about the sniffing process.
///
/// # Arguments
///
/// * `output` - A String representing the output report file name. Specified by the user through the
/// ```-o``` option.
///
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `first_timestamp` - A not formatted String representing the initial timestamp of the sniffing process.
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
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
/// * `num_pairs` - Total numbers of address:port pairs considered in the report.
///
/// * `num_sniffed_packets` - Total numbers of sniffed packets.
/// ```
fn write_statistics(mut output: File, device_name: String, first_timestamp: String,
                            lowest_port: u16, highest_port: u16,
                            network_layer: String, transport_layer: TransProtocol, app_layer: AppProtocol,
                            num_pairs: usize, num_sniffed_packets: u128, num_filtered_packets: u128,
                            app_count: HashMap<AppProtocol, u128>) {

    let adapter_string = format!("Packets are sniffed from adapter '{}'\n", device_name);
    let first_timestamp_string = format!("\tReport start time: {}\n", first_timestamp);
    let last_timestamp_string = format!("\tReport last update: {}\n", Local::now().format("%d/%m/%Y %H:%M:%S"));
    let ports_string = get_ports_string(lowest_port,highest_port);
    let network_layer_string = get_network_layer_string(network_layer);
    let transport_layer_string = get_transport_layer_string(transport_layer);
    let app_layer_string = get_app_layer_string(app_layer);
    let filtered_packets_string = get_filtered_packets_string(num_sniffed_packets, num_filtered_packets);

    writeln!(output, "{}", adapter_string).expect("Error writing output file\n");

    writeln!(output, "Report updates info").expect("Error writing output file\n");
    write!(output, "{}", first_timestamp_string).expect("Error writing output file\n");
    writeln!(output, "{}", last_timestamp_string).expect("Error writing output file\n");

    writeln!(output, "Filters").expect("Error writing output file\n");
    write!(output, "{}", network_layer_string).expect("Error writing output file\n");
    write!(output, "{}", transport_layer_string).expect("Error writing output file\n");
    write!(output, "{}", ports_string).expect("Error writing output file\n");
    writeln!(output, "{}", app_layer_string).expect("Error writing output file\n");

    writeln!(output, "Overall statistics").expect("Error writing output file\n");
    writeln!(output, "\tConsidered [address:port] pairs: {}", num_pairs.separate_with_spaces()).expect("Error writing output file\n");
    writeln!(output, "\tTotal packets: {}", num_sniffed_packets.separate_with_spaces()).expect("Error writing output file\n");
    writeln!(output, "{}", filtered_packets_string).expect("Error writing output file\n");

    if num_filtered_packets > 0 {
        let app_count_string = get_app_count_string(app_count, num_sniffed_packets);
        writeln!(output, "Total packets divided by app layer protocol").expect("Error writing output file\n");
        writeln!(output, "{}", app_count_string).expect("Error writing output file\n");
    }

}