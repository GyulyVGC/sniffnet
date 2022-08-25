//! Module containing functions executed by the thread in charge of updating the output report
//! every ```interval``` seconds, with ```interval``` specified by the user through the
//! ```-i``` command line option.
//!
//! If the ```-i``` option is not specified, the report is updated every 5 seconds.

use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;
use std::io::Write;
use colored::Colorize;
use crate::{AddressPort, AppProtocol, ReportInfo, Status};


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
/// * `min_packets` - Minimum number of packets for an address:port pair to be considered in the report.
/// Specified by the user through the ```-m``` option.
///
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// * `output_file` - A String representing the output report file name. Specified by the user through the
/// ```-o``` option.
///
/// * `mutex_map` - Mutex to permit exclusive access to the shared variable containing the parsed packets.
///
/// * `status_pair` - Shared variable to check the application current status.
pub fn sleep_and_write_report_loop(lowest_port: u16, highest_port: u16, interval: u64, min_packets: u32,
                                   device_name: String, network_layer: String, transport_layer: String, app_layer: AppProtocol,
                                   output_file: String, mutex_map: Arc<Mutex<HashMap<AddressPort,ReportInfo>>>,
                                   status_pair: Arc<(Mutex<Status>, Condvar)>) {

    let mut times_report_updated = 0;
    let cvar = &status_pair.1;
    let first_timestamp = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();

    loop {
        thread::sleep(Duration::from_secs(interval));

            times_report_updated += 1;
            let mut output = File::create(output_file.clone()).expect("Error creating output file\n\r");

            write_report_file_header(output.try_clone().expect("Error cloning file handler\n\r"),
                                     device_name.clone(), first_timestamp.clone(),
                                     times_report_updated, lowest_port, highest_port, min_packets,
                                     network_layer.clone(), transport_layer.clone(), app_layer.clone());

            let map = mutex_map.lock().expect("Error acquiring mutex\n\r");

            let mut sorted_vec: Vec<(&AddressPort, &ReportInfo)> = map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)|
                (b.received_packets + b.transmitted_packets).cmp(&(a.received_packets + a.transmitted_packets)));

            for (key, val) in sorted_vec.iter() {
                if val.transmitted_packets + val.received_packets >= min_packets
                    && (val.app_protocols.contains(&app_layer) || app_layer.eq(&AppProtocol::Other)){
                    write!(output, "{}\n{}\n\n", key, val).expect("Error writing output file\n\r");
                }
            }
            println!("{}{}{}\r", "\tReport updated (".cyan().italic(),
                     times_report_updated.to_string().cyan().italic(), ")".cyan().italic());

            let mut _status = status_pair.0.lock().expect("Error acquiring mutex\n\r");
            _status = cvar.wait_while(_status, |s| *s == Status::Pause).expect("Error acquiring mutex\n\r");
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
        format!("<><>\t\t\tConsidering only port number {}\n", lowest_port)
    }
    else if lowest_port != u16::MIN || highest_port != u16::MAX {
        format!("<><>\t\t\tConsidering only port numbers from {} to {}\n", lowest_port, highest_port)
    }
    else {
        format!("<><>\t\t\tConsidering all port numbers (from {} to {})\n", lowest_port, highest_port)
    }
}


/// Given the minimum packets number, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `min_packets` - Minimum number of packets for an address:port pair to be considered in the report.
/// Specified by the user through the ```-m``` option.
fn get_min_packets_string(min_packets: u32) -> String {
    if min_packets > 1 {
        format!("<><>\t\t\tConsidering only address:port pairs featured by more than {} packets\n", min_packets)
    }
    else {
        format!("<><>\t\t\tConsidering address:port pairs featured by any number of packets\n")
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
        format!("<><>\t\t\tConsidering only IPv4 packets\n")
    }
    else if network_layer.cmp(&"ipv6".to_string()) == Equal {
        format!("<><>\t\t\tConsidering only IPv6 packets\n")
    }
    else {
        format!("<><>\t\t\tConsidering both IPv4 and IPv6 packets\n")
    }
}


/// Given the transport layer textual filter, the function generates the corresponding String
/// to be used in the output report file header.
///
/// # Arguments
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
fn get_transport_layer_string(transport_layer: String) -> String {
    if transport_layer.cmp(&"tcp".to_string()) == Equal {
        format!("<><>\t\t\tConsidering only packets exchanged with TCP\n")
    }
    else if transport_layer.cmp(&"udp".to_string()) == Equal {
        format!("<><>\t\t\tConsidering only packets exchanged with UDP\n")
    }
    else {
        format!("<><>\t\t\tConsidering packets exchanged both with TCP and/or UDP\n")
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
        format!("<><>\t\t\tConsidering all application layer protocols\n")
    }
    else {
        format!("<><>\t\t\tConsidering only {:?} packets\n", app_layer)
    }
}


/// Writes the output report file header, which contains useful info about the sniffing process.
///
/// # Arguments
///
/// * `output_file` - A String representing the output report file name. Specified by the user through the
/// ```-o``` option.
///
/// * `device_name` - A String representing the name of th network adapter to be sniffed. Specified by the user through the
/// ```-a``` option.
///
/// * `first_timestamp` - A not formatted String representing the initial timestamp of the sniffing process.
///
/// * `times_report_updated` - An integer representing the amount of times the report has been updated.
///
/// * `lowest_port` - The lowest port number to be considered in the report. Specified by the user
/// through the ```-l``` option.
///
/// * `highest_port` - The highest port number to be considered in the report. Specified by the user
/// through the ```-h``` option.
///
/// * `min_packets` - Minimum number of packets for an address:port pair to be considered in the report.
/// Specified by the user through the ```-m``` option.
///
/// * `network_layer` - A String representing the IP version to be filtered. Specified by the user through the
/// ```-n``` option.
///
/// * `transport_layer` - A String representing the transport protocol to be filtered. Specified by the user through the
/// ```-t``` option.
///
/// * `app_layer` - An AppProtocol representing the application protocol to be filtered. Specified by the user through the
/// ```--app``` option.
///
/// # Examples
/// An example of output report file header generated by this function is reported below.
///
/// ```<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// <><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// <><>
/// <><>		Packets are sniffed from adapter 'en0'
/// <><>
/// <><>		Report updates info
/// <><>			Report start time: 15/08/2022 15:39:07
/// <><>			Report last update: 15/08/2022 15:39:52
/// <><>			Report update frequency: every 5 seconds
/// <><>			Number of times report was updated: 9
/// <><>
/// <><>		Filters
/// <><>			Considering only address:port pairs featured by more than 500 packets
/// <><>			Considering both IPv4 and IPv6 packets
/// <><>			Considering only packets exchanged with TCP
/// <><>			Considering only port number 443
/// <><>
/// <><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// <><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/// ```
fn write_report_file_header(mut output: File, device_name: String, first_timestamp: String,
                            times_report_updated: i32, lowest_port: u16, highest_port: u16,
                            min_packets: u32, network_layer: String, transport_layer: String, app_layer: AppProtocol) {
    let cornice_string = "<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>\n".to_string();
    let adapter_string = format!("<><>\t\tPackets are sniffed from adapter '{}'\n", device_name);
    let first_timestamp_string = format!("<><>\t\t\tReport start time: {}\n", first_timestamp);
    let last_timestamp_string = format!("<><>\t\t\tReport last update: {}\n", Local::now().format("%d/%m/%Y %H:%M:%S").to_string());
    let number_updates_string = format!("<><>\t\t\tNumber of times report was updated: {}\n", times_report_updated);
    let ports_string = get_ports_string(lowest_port,highest_port);
    let min_packets_string = get_min_packets_string(min_packets);
    let network_layer_string = get_network_layer_string(network_layer);
    let transport_layer_string = get_transport_layer_string(transport_layer);
    let app_layer_string = get_app_layer_string(app_layer);
    write!(output, "{}", cornice_string).expect("Error writing output file\n");
    write!(output, "{}", cornice_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output, "{}", adapter_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output, "<><>\t\tReport updates info\n").expect("Error writing output file\n");
    write!(output, "{}", first_timestamp_string).expect("Error writing output file\n");
    write!(output, "{}", last_timestamp_string).expect("Error writing output file\n");
    write!(output, "{}", number_updates_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output, "<><>\t\tFilters\n").expect("Error writing output file\n");
    write!(output, "{}", min_packets_string).expect("Error writing output file\n");
    write!(output, "{}", network_layer_string).expect("Error writing output file\n");
    write!(output, "{}", transport_layer_string).expect("Error writing output file\n");
    write!(output, "{}", ports_string).expect("Error writing output file\n");
    write!(output, "{}", app_layer_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output,"{}", cornice_string).expect("Error writing output file\n");
    write!(output,"{}\n\n\n", cornice_string).expect("Error writing output file\n");
}