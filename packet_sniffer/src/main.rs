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

fn main() {

    let device = Device::lookup().unwrap();
    let mut output = File::create("report.txt").unwrap();
    println!("{:?}", device);

    println!("Waiting for packets........");
    println!("Writing report.txt file........");

    let mut cap = Capture::from_device(device).unwrap()
        .promisc(true)
        .open().unwrap();
    
    let mut map:HashMap<AddressPort,ReportInfo> = HashMap::new();

    while let Ok(packet) = cap.next() {

        let utc: DateTime<Local> = Local::now();
        let now = utc.format("%d/%m/%Y %H:%M:%S").to_string();

        match PacketHeaders::from_ethernet_slice(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {

                let address1;
                let address2;
                let mut port1= 0;
                let mut port2= 0;
                let exchanged_bytes: u32;
                let mut protocol = TransProtocol::Other;

                match value.ip.unwrap() {
                    IpHeader::Version4(ipv4header, _) => {
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
                    IpHeader::Version6(ipv6header, _) => {
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
                    TransportHeader::Icmpv4(_) => {}
                    TransportHeader::Icmpv6(_) => {}
                }
                
                let key1: AddressPort = AddressPort::new(address1,port1);
                let key2: AddressPort = AddressPort::new(address2,port2);
                
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

                for (key, val) in map.iter() {
                    write!(output, "Address: {}:{}\n{}\n", key.address1, key.port1, val).expect("File output error");
                }
                write!(output, "----------------------------------\n").expect("File output error");

            }
        }
    }
}