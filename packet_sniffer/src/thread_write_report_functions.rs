use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Local;
use std::io::Write;
use crate::{AddressPort, ReportInfo, Status};

pub fn sleep_and_write_report_loop(lowest_port: u16, highest_port: u16, interval: u64, min_packets: u32,
                                   device_name: String, network_layer: String, transport_layer: String,
                                   output_file: String, mutex_map: Arc<Mutex<HashMap<AddressPort,ReportInfo>>>, status_pair: Arc<(Mutex<Status>, Condvar)>) {

    let mut times_report_updated = 0;
    let cvar = &status_pair.1;
    let first_timestamp = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();

    loop {
        thread::sleep(Duration::from_secs(interval));
        let mut status = status_pair.0.lock().unwrap();
        status = cvar.wait_while(status, |s| *s == Status::Pause).unwrap();
        if *status == Status::Running {
            times_report_updated += 1;
            let mut output = File::create(output_file.clone()).expect("Error creating output file\n");

            write_report_file_header(output.try_clone().expect("Error cloning file handler\n"),
                                     device_name.clone(), first_timestamp.clone(),
                                     times_report_updated, interval, lowest_port, highest_port, min_packets,
                                     network_layer.clone(), transport_layer.clone());

            let map = mutex_map.lock().expect("Error acquiring mutex\n");

            let mut sorted_vec: Vec<(&AddressPort, &ReportInfo)> = map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)|
                (b.received_packets + b.transmitted_packets).cmp(&(a.received_packets + a.transmitted_packets)));

            for (key, val) in sorted_vec.iter() {
                if val.transmitted_packets + val.received_packets >= min_packets {
                    write!(output, "Address: {}:{}\n{}\n\n", key.address, key.port, val).expect("Error writing output file\n");
                }
            }
            println!("\tReport updated ({})",times_report_updated);
        }
        else if *status == Status::Stop {break;}
    }
}



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



fn get_min_packets_string(min_packets: u32) -> String {
    if min_packets > 1 {
        format!("<><>\t\t\tConsidering only address:port pairs featured by more than {} packets\n", min_packets)
    }
    else {
        format!("<><>\t\t\tConsidering address:port pairs featured by any number of packets\n")
    }
}



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



fn write_report_file_header(mut output: File, device_name: String, first_timestamp: String,
                            times_report_updated: i32, interval: u64, lowest_port: u16, highest_port: u16,
                            min_packets: u32, network_layer: String, transport_layer: String) {
    let cornice_string = "<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>\n".to_string();
    let adapter_string = format!("<><>\t\tPackets are sniffed from adapter '{}'\n", device_name);
    let first_timestamp_string = format!("<><>\t\t\tReport start time: {}\n", first_timestamp);
    let last_timestamp_string = format!("<><>\t\t\tReport last update: {}\n", Local::now().format("%d/%m/%Y %H:%M:%S").to_string());
    let number_updates_string = format!("<><>\t\t\tNumber of times report was updated: {}\n", times_report_updated);
    let frequency_string = format!("<><>\t\t\tReport update frequency: every {} seconds\n", interval);
    let ports_string = get_ports_string(lowest_port,highest_port);
    let min_packets_string = get_min_packets_string(min_packets);
    let network_layer_string = get_network_layer_string(network_layer);
    let transport_layer_string = get_transport_layer_string(transport_layer);
    write!(output, "{}", cornice_string).expect("Error writing output file\n");
    write!(output, "{}", cornice_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output, "{}", adapter_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output, "<><>\t\tReport updates info\n").expect("Error writing output file\n");
    write!(output, "{}", first_timestamp_string).expect("Error writing output file\n");
    write!(output, "{}", last_timestamp_string).expect("Error writing output file\n");
    write!(output, "{}", frequency_string).expect("Error writing output file\n");
    write!(output, "{}", number_updates_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output, "<><>\t\tFilters\n").expect("Error writing output file\n");
    write!(output, "{}", min_packets_string).expect("Error writing output file\n");
    write!(output, "{}", network_layer_string).expect("Error writing output file\n");
    write!(output, "{}", transport_layer_string).expect("Error writing output file\n");
    write!(output, "{}", ports_string).expect("Error writing output file\n");
    write!(output, "<><>\n").expect("Error writing output file\n");
    write!(output,"{}", cornice_string).expect("Error writing output file\n");
    write!(output,"{}\n\n\n", cornice_string).expect("Error writing output file\n");
}