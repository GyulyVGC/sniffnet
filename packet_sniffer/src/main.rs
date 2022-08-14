mod address_port;
mod report_info;

use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::collections::HashSet;
use etherparse::{IpHeader, PacketHeaders, TransportHeader};
use pcap::{Device, Capture, Active};
use std::fs::File;
use std::io::Write;
use crate::address_port::{AddressPort};
use crate::report_info::{AppProtocol, ReportInfo, TransProtocol};
use chrono::prelude::*;
use clap::Parser;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the network adapter to be inspected, if omitted the default adapter is chosen.
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "default")]
    adapter: String,

    /// Prints the list of available devices. Immediately terminates the program.
    #[clap(short, long)]
    device_list: bool,

    /// Sets the maximum port value to be considered, if omitted there is not ports higher bound.
    #[clap(short, long, value_parser, default_value_t = u16::MAX)]
    highest_port: u16,

    /// Sets the interval of time between report updates (value in seconds).
    #[clap(short, long, value_parser, default_value_t = 5)]
    interval: u64,

    /// Sets the minimum port value to be considered, if omitted there is not ports lower bound.
    #[clap(short, long, value_parser, default_value_t = u16::MIN)]
    lowest_port: u16,

    /// Sets the minimum value of transited packets for an address:port to be printed in the report.
    #[clap(short, long, value_parser, default_value_t = u32::MIN)]
    minimum_packets: u32,

    /// Filters packets on the basis of the IP version address (IPv4 or IPv6).
    #[clap(short, long, value_parser, default_value = "no filter")]
    network_layer_filter: String,

    /// Name of output file to contain the textual report, if omitted a default file is chosen.
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "report.txt")]
    output_file: String,

    /// Filters packets on the basis of the transport layer protocol (TCP or UDP).
    #[clap(short, long, value_parser, default_value = "no filter")]
    transport_layer_filter: String,
}

fn main() {

    let args = Args::parse();
    let adapter: String = args.adapter;
    let output_file: String = args.output_file;
    let lowest_port = args.lowest_port;
    let highest_port = args.highest_port;
    let min_packets = args.minimum_packets;
    let interval = args.interval;
    let network_layer: String = args.network_layer_filter.to_ascii_lowercase();
    let network_layer_2: String = network_layer.clone();
    let transport_layer: String = args.transport_layer_filter.to_ascii_lowercase();
    let transport_layer_2: String = transport_layer.clone();

    if args.device_list == true {
        print_device_list();
        return;
    }

    if network_layer.cmp(&"ipv6".to_string()) != Equal
        && network_layer.cmp(&"ipv4".to_string()) != Equal
        && network_layer.cmp(&"no filter".to_string()) != Equal {
        eprint!("\n\tERROR: Specified network layer filter must be equal to 'IPv4' or 'IPv6' (not case sensitive).\n\n");
        return;
    }

    if transport_layer.cmp(&"tcp".to_string()) != Equal
        && transport_layer.cmp(&"udp".to_string()) != Equal
        && transport_layer.cmp(&"no filter".to_string()) != Equal {
        eprint!("\n\tERROR: Specified transport layer filter must be equal to 'TCP' or 'UDP' (not case sensitive).\n\n");
        return;
    }

    if lowest_port > highest_port {
        eprint!("\n\tERROR: Specified lowest port is greater than specified highest port.\n\n");
        return;
    }

    if interval == 0 {
        eprint!("\n\tERROR: Specified time interval is null.\n\n");
        return;
    }

    let found_device = retrieve_device(adapter);

    if found_device.name.len() == 0 {
        eprint!("\n\tERROR: Specified network adapter does not exist. Use option '-d' to list all the available devices.\n\n");
        return;
    }

    let cap = Capture::from_device(found_device.clone())
        .expect("Capture initialization error\n")
        .promisc(true)
        .buffer_size(10_000_000)
        .open()
        .expect("Capture initialization error\n");

    let mutex_map1 = Arc::new(Mutex::new(HashMap::new()));
    let mutex_map2 = mutex_map1.clone();

    println!("\n\tParsing packets...");
    println!("\tUpdating the file '{}' every {} seconds\n", output_file, interval);

    thread::spawn(move || {
        sleep_and_write_report_loop(lowest_port, highest_port, interval, min_packets,
                                    found_device.name, network_layer,
                                    transport_layer.clone(), output_file,
                                    mutex_map2);
    });

    parse_packets_loop(cap, lowest_port, highest_port, network_layer_2,
                       transport_layer_2, mutex_map1);
}



fn sleep_and_write_report_loop(lowest_port: u16, highest_port: u16, interval: u64, min_packets: u32,
                               device_name: String, network_layer: String, transport_layer: String,
                               output_file: String, mutex_map: Arc<Mutex<HashMap<AddressPort,ReportInfo>>>) {

    let mut times_report_updated = 0;
    let first_timestamp = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();

    loop {
        thread::sleep(Duration::from_secs(interval));
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
                write!(output, "Address: {}:{}\n{}\n\n", key.address1, key.port1, val).expect("Error writing output file\n");
            }
        }
        println!("\tReport updated ({})",times_report_updated);
    }
}



fn parse_packets_loop(mut cap: Capture<Active>, lowest_port: u16, highest_port: u16,
                      network_layer_filter: String, transport_layer_filter: String,
                      mutex_map: Arc<Mutex<HashMap<AddressPort,ReportInfo>>>) {
    loop {
        match cap.next() {
            Err(_) => {
                continue;
            }
            Ok(packet) => {

                match PacketHeaders::from_ethernet_slice(&packet) {
                    Err(_) => {
                        continue;
                    }
                    Ok(value) => {

                        let mut address1 = "".to_string();
                        let mut address2 = "".to_string();
                        let mut network_layer = "".to_string();
                        let mut port1 = 0;
                        let mut port2 = 0;
                        let mut transport_layer = "".to_string();
                        let mut exchanged_bytes: u32 = 0;
                        let mut transport_protocol = TransProtocol::Other;
                        let mut application_protocol_1: Option<AppProtocol> = None;
                        let mut application_protocol_2: Option<AppProtocol> = None;
                        let mut application_protocols: HashSet<AppProtocol> = HashSet::new();
                        let mut skip_packet = false;

                        analyze_network_header(value.ip, &mut exchanged_bytes,
                                               &mut network_layer, &mut address1, &mut address2,
                                                &mut skip_packet);
                        if skip_packet {
                            continue;
                        }

                        analyze_transport_header(value.transport,
                                                 &mut transport_layer, &mut port1, &mut port2,
                                                 &mut application_protocol_1,
                                                 &mut application_protocol_2,
                                                 &mut application_protocols,
                                                 &mut transport_protocol, &mut skip_packet);
                        if skip_packet {
                            continue;
                        }

                        let key1: AddressPort = AddressPort::new(address1,port1);
                        let key2: AddressPort = AddressPort::new(address2,port2);

                        if network_layer_filter.cmp(&network_layer) == Equal || network_layer_filter.cmp(&"no filter".to_string()) == Equal {
                            if transport_layer_filter.cmp(&transport_layer) == Equal || transport_layer_filter.cmp(&"no filter".to_string()) == Equal {

                                if port1 >= lowest_port && port1 <= highest_port {
                                    modify_or_insert_source_in_map(mutex_map.clone(), key1,
                                                         exchanged_bytes, transport_protocol,
                                                         application_protocol_1, application_protocol_2,
                                                         application_protocols.clone());
                                }

                                if port2 >= lowest_port && port2 <= highest_port {
                                    modify_or_insert_destination_in_map(mutex_map.clone(), key2,
                                                         exchanged_bytes, transport_protocol,
                                                         application_protocol_1, application_protocol_2,
                                                         application_protocols);
                                }

                            }
                        }
                    }
                }
            }
        }
    }
}



fn print_device_list() {
    println!();
    for dev in Device::list().expect("Error retrieving device list\n") {
        print!("\tDevice: {}\n\t\tAddresses: ", dev.name);
        if dev.addresses.len() == 0 {
            println!();
        }
        for addr in dev.addresses {
            print!("{:?}\n\t\t\t   ", addr.addr);
        }
        println!();
    }
    println!();
}



fn retrieve_device(adapter: String) -> Device {
    let mut found_device = Device {
        name: "".to_string(),
        desc: None,
        addresses: vec![]
    };
    if adapter.eq("default") {
        found_device = Device::lookup().expect("Error retrieving default network adapter\n");
    } else {
        let dev_list = Device::list().expect("Unable to retrieve network adapters list\n");
        for device in dev_list {
            if device.name == adapter {
                found_device = device;
                break;
            }
        }
    }
    return found_device;
}



fn analyze_network_header(network_header: Option<IpHeader>, exchanged_bytes: &mut u32,
                          network_layer: &mut String, address1: &mut String,
                          address2: &mut String, skip_packet: &mut bool) {
    match network_header {
        Some(IpHeader::Version4(ipv4header, _)) => {
            *network_layer = "ipv4".to_string();
            *address1 = format!("{:?}", ipv4header.source)
                .replace("[","")
                .replace("]","")
                .replace(",",".")
                .replace(" ","");
            *address2 = format!("{:?}", ipv4header.destination)
                .replace("[","")
                .replace("]","")
                .replace(",",".")
                .replace(" ","");
            *exchanged_bytes = ipv4header.payload_len as u32;
        }
        Some(IpHeader::Version6(ipv6header, _)) => {
            *network_layer = "ipv6".to_string();
            *address1 = format!("{:?}", ipv6header.source)
                .replace("[", "")
                .replace("]", "")
                .replace(",", ".")
                .replace(" ", "");
            *address2 = format!("{:?}", ipv6header.destination)
                .replace("[", "")
                .replace("]", "")
                .replace(",", ".")
                .replace(" ", "");
            *exchanged_bytes = ipv6header.payload_length as u32;
        }
        _ => {
            *skip_packet = true;
        }
    }
}



fn analyze_transport_header(transport_header: Option<TransportHeader>,
                            transport_layer: &mut String, port1: &mut u16, port2: &mut u16,
                            application_protocol_1: &mut Option<AppProtocol>,
                            application_protocol_2: &mut Option<AppProtocol>,
                            application_protocols: &mut HashSet<AppProtocol>,
                            transport_protocol: &mut TransProtocol, skip_packet: &mut bool) {
    match transport_header {
        Some(TransportHeader::Udp(udp_header)) => {
            *transport_layer = "udp".to_string();
            *port1 = udp_header.source_port;
            *port2 = udp_header.destination_port;
            *transport_protocol = TransProtocol::UDP;
            *application_protocol_1 = from_port_to_application_protocol(*port1);
            if application_protocol_1.is_some() {
                application_protocols.insert(application_protocol_1.unwrap());
            }
            *application_protocol_2 = from_port_to_application_protocol(*port2);
            if application_protocol_2.is_some() {
                application_protocols.insert(application_protocol_2.unwrap());
            }
        }
        Some(TransportHeader::Tcp(tcp_header)) => {
            *transport_layer = "tcp".to_string();
            *port1 = tcp_header.source_port;
            *port2 = tcp_header.destination_port;
            *transport_protocol = TransProtocol::TCP;
            *application_protocol_1 = from_port_to_application_protocol(*port1);
            if application_protocol_1.is_some() {
                application_protocols.insert(application_protocol_1.unwrap());
            }
            *application_protocol_2 = from_port_to_application_protocol(*port2);
            if application_protocol_2.is_some() {
                application_protocols.insert(application_protocol_2.unwrap());
            }
        }
        _ => {
            *skip_packet = true;
        }
    }
}



fn modify_or_insert_source_in_map(mutex_map: Arc<Mutex<HashMap<AddressPort,ReportInfo>>>, key: AddressPort,
                        exchanged_bytes: u32, transport_protocol: TransProtocol,
                        application_protocol_1: Option<AppProtocol>,
                        application_protocol_2: Option<AppProtocol>,
                        application_protocols: HashSet<AppProtocol>) {
    let now_ugly: DateTime<Local> = Local::now();
    let now = now_ugly.format("%d/%m/%Y %H:%M:%S").to_string();
    mutex_map.lock().expect("Error acquiring mutex\n").entry(key).and_modify(|info| {
        info.transmitted_bytes += exchanged_bytes;
        info.transmitted_packets += 1;
        info.final_timestamp = now.clone();
        info.trans_protocols.insert(transport_protocol);
        if application_protocol_1.is_some() {
            info.app_protocols.insert(application_protocol_1.unwrap());
        }
        if application_protocol_2.is_some() {
            info.app_protocols.insert(application_protocol_2.unwrap());
        }
    })
        .or_insert(ReportInfo {
            transmitted_bytes: exchanged_bytes,
            transmitted_packets: 1,
            received_bytes: 0,
            received_packets: 0,
            initial_timestamp: now.clone(),
            final_timestamp: now.clone(),
            trans_protocols: HashSet::from([transport_protocol]),
            app_protocols: application_protocols.clone()
        });
}



fn modify_or_insert_destination_in_map(mutex_map: Arc<Mutex<HashMap<AddressPort,ReportInfo>>>, key: AddressPort,
                                       exchanged_bytes: u32, transport_protocol: TransProtocol,
                                       application_protocol_1: Option<AppProtocol>,
                                       application_protocol_2: Option<AppProtocol>,
                                       application_protocols: HashSet<AppProtocol>) {
    let now_ugly: DateTime<Local> = Local::now();
    let now = now_ugly.format("%d/%m/%Y %H:%M:%S").to_string();
    mutex_map.lock().expect("Error acquiring mutex\n").entry(key).and_modify(|info| {
        info.received_bytes += exchanged_bytes;
        info.received_packets += 1;
        info.final_timestamp = now.clone();
        info.trans_protocols.insert(transport_protocol);
        if application_protocol_1.is_some() {
            info.app_protocols.insert(application_protocol_1.unwrap());
        }
        if application_protocol_2.is_some() {
            info.app_protocols.insert(application_protocol_2.unwrap());
        }
    })
        .or_insert(ReportInfo {
            transmitted_bytes: 0,
            transmitted_packets: 0,
            received_bytes: exchanged_bytes,
            received_packets: 1,
            initial_timestamp: now.clone(),
            final_timestamp: now.clone(),
            trans_protocols: HashSet::from([transport_protocol]),
            app_protocols: application_protocols.clone()
        });
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



fn from_port_to_application_protocol(port: u16) -> Option<AppProtocol> {
    match port {
        20..=21 => {Option::Some(AppProtocol::FTP)},
        22 => {Option::Some(AppProtocol::SSH)},
        23 => {Option::Some(AppProtocol::Telnet)},
        25 => {Option::Some(AppProtocol::SMTP)},
        53 => {Option::Some(AppProtocol::DNS)},
        67..=68 => {Option::Some(AppProtocol::DHCP)},
        69 => {Option::Some(AppProtocol::TFTP)},
        80 => {Option::Some(AppProtocol::HTTP)},
        110 => {Option::Some(AppProtocol::POP)},
        123 => {Option::Some(AppProtocol::NTP)},
        137..=139 => {Option::Some(AppProtocol::NetBIOS)},
        143 => {Option::Some(AppProtocol::IMAP)},
        161..=162 => {Option::Some(AppProtocol::SNMP)},
        179 => {Option::Some(AppProtocol::BGP)},
        389 => {Option::Some(AppProtocol::LDAP)},
        443 => {Option::Some(AppProtocol::HTTPS)},
        636 => {Option::Some(AppProtocol::LDAPS)},
        989..=990 => {Option::Some(AppProtocol::FTPS)},
        _ => {None}
    }
}