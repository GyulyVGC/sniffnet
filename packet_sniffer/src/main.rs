mod address_port;
mod report_info;

use std::collections::HashMap;
use std::collections::HashSet;
use etherparse::{IpHeader, PacketHeaders, TransportHeader};
use pcap::{Device, Capture};
use std::fs::File;
use std::io::Write;
use crate::address_port::{AddressPort};
use crate::report_info::{ReportInfo, TransProtocol};
use chrono::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the network adapter to be inspected, if omitted a default adapter is chosen
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "default")]
    adapter: String,

    /// Name of output file to contain the textual report, if omitted a default file is chosen
    #[clap(short, long, value_parser, forbid_empty_values = true, default_value = "report.txt")]
    output_file: String,

    /// Set the minimum port value to be considered, if omitted there is not ports lower bound
    #[clap(short, long, value_parser, default_value_t = u16::MIN)]
    lowest_port: u16,

    /// Set the maximum port value to be considered, if omitted there is not ports higher bound
    #[clap(short, long, value_parser, default_value_t = u16::MAX)]
    highest_port: u16,

    /// Set the minimum value of transited packets for an address:port to be printed in the report
    #[clap(short, long, value_parser, default_value_t = u32::MIN)]
    minimum_packets: u32,

    /// Print list of the available devices
    #[clap(short, long)]
    device_list: bool,
}

fn main() {

    let args = Args::parse();
    let adapter: String = args.adapter;
    let output_file: String = args.output_file;
    let lowest_port = args.lowest_port;
    let highest_port = args.highest_port;
    let min_packets = args.minimum_packets;

    if args.device_list == true {
        for dev in Device::list().unwrap() {
            print!("Device: {}\n\tAddresses: ", dev.name);
            for addr in dev.addresses {
                print!("{:?}\n\t\t   ", addr.addr);
            }
            println!("\n");
        }
        return;
    }

    let mut found_device = Device {
        name: "".to_string(),
        desc: None,
        addresses: vec![]
    };
    if adapter.eq("default") {
        found_device = Device::lookup().expect("Error retrieving default network adapter\n");
    }
    else {
        let dev_list = Device::list().expect("Unable to retrieve network adapters list\n");
        for device in dev_list {
            if device.name == adapter {
                found_device = device;
                break;
            }
        }
        if found_device.name.len() == 0 {
            eprint!("ERROR: Specified network adapter does not exist. Use option '-d' to list all the available devices.\n");
            return;
        }
    }

    let mut output = File::create(output_file.clone()).unwrap();

    println!("Writing {} file........", output_file.clone());

    write!(output,"-----------------------------------------------\n\n").expect("Error writing output error\n");
    write!(output, "Report start time: '{}'\n\n", Local::now().format("%d/%m/%Y %H:%M:%S").to_string()).expect("Error writing output error\n");
    write!(output, "Packets sniffed from adapter '{}'\n\n", found_device.name).expect("Error writing output error\n");
    write!(output, "Considering port numbers from {} to {}\n\n", lowest_port, highest_port).expect("Error writing output error\n");
    if min_packets > 1 {
        write!(output, "Considering only addresses featured by more than {} packets\n\n", min_packets).expect("Error writing output error\n");
    }
    write!(output,"-----------------------------------------------\n\n\n").expect("Error writing output error\n");

    let mut cap = Capture::from_device(found_device).unwrap()
        .promisc(true)
        .open().unwrap();

    let mut map:HashMap<AddressPort,ReportInfo> = HashMap::new();

    let mut num_packets = 0; //dopo 300 pacchetti interrompo la cattura e stampo
    while let Ok(packet) = cap.next() {

        let utc: DateTime<Local> = Local::now();
        let now = utc.format("%d/%m/%Y %H:%M:%S").to_string();

        match PacketHeaders::from_ethernet_slice(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {

                let address1;
                let address2;
                let port1;
                let port2;
                let exchanged_bytes: u32;
                let protocol;

                match value.ip {
                    Some(IpHeader::Version4(ipv4header, _)) => {
                        address1 = format!("{:?}", ipv4header.source)
                            .replace("[","")
                            .replace("]","")
                            .replace(",",".")
                            .replace(" ","");
                        address2 = format!("{:?}", ipv4header.destination)
                            .replace("[","")
                            .replace("]","")
                            .replace(",",".")
                            .replace(" ","");
                        exchanged_bytes = ipv4header.payload_len as u32;
                    }
                    Some(IpHeader::Version6(ipv6header, _)) => {
                        address1 = format!("{:?}", ipv6header.source)
                            .replace("[", "")
                            .replace("]", "")
                            .replace(",", ".")
                            .replace(" ", "");
                        address2 = format!("{:?}", ipv6header.destination)
                            .replace("[", "")
                            .replace("]", "")
                            .replace(",", ".")
                            .replace(" ", "");
                        exchanged_bytes = ipv6header.payload_length as u32;
                    }
                    None => {continue;}
                }

                match value.transport.unwrap() {
                    TransportHeader::Udp(udpheader) => {
                        port1 = udpheader.source_port;
                        protocol = TransProtocol::UDP;
                        port2 = udpheader.destination_port
                    }
                    TransportHeader::Tcp(tcpheader) => {
                        port1 = tcpheader.source_port;
                        protocol = TransProtocol::TCP;
                        port2 = tcpheader.destination_port
                    }
                    TransportHeader::Icmpv4(_) => {continue;}
                    TransportHeader::Icmpv6(_) => {continue;}
                }

                let key1: AddressPort = AddressPort::new(address1,port1);
                let key2: AddressPort = AddressPort::new(address2,port2);

                if port1 >= lowest_port && port1 <= highest_port {
                    map.entry(key1).and_modify(|info| {
                        info.transmitted_bytes += exchanged_bytes;
                        info.transmitted_packets += 1;
                        info.final_timestamp = now.clone();
                        info.trans_protocols.insert(protocol);})
                        .or_insert(ReportInfo {
                            transmitted_bytes: exchanged_bytes,
                            transmitted_packets: 1,
                            received_bytes: 0,
                            received_packets: 0,
                            initial_timestamp: now.clone(),
                            final_timestamp: now.clone(),
                            trans_protocols: HashSet::from([protocol])
                        });
                }

                if port2 >= lowest_port && port2 <= highest_port {
                    map.entry(key2).and_modify(|info| {
                        info.received_bytes += exchanged_bytes;
                        info.received_packets += 1;
                        info.final_timestamp = now.clone();
                        info.trans_protocols.insert(protocol); })
                        .or_insert(ReportInfo {
                            transmitted_bytes: 0,
                            transmitted_packets: 0,
                            received_bytes: exchanged_bytes,
                            received_packets: 1,
                            initial_timestamp: now.clone(),
                            final_timestamp: now.clone(),
                            trans_protocols: HashSet::from([protocol])
                        });
                }

            }
        }

        num_packets+=1;
        if num_packets >= 300 {
            break;
        }
    }

    let mut sorted_vec: Vec<(&AddressPort, &ReportInfo)> = map.iter().collect();
    sorted_vec.sort_by(|&(_, a), &(_, b)|
        (b.received_packets + b.transmitted_packets).cmp(&(a.received_packets + a.transmitted_packets)));
    for (key, val) in sorted_vec.iter() {
        if val.transmitted_packets + val.received_packets >= min_packets {
            write!(output, "Address: {}:{}\n{}\n\n", key.address1, key.port1, val).expect("Error writing output error\n");
        }
    }

}