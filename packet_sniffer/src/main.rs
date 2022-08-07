mod address_port;
mod report_info;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use etherparse::{IpHeader, PacketHeaders, TransportHeader};
use pcap::{Device, Capture};
use std::fs::File;
use crate::address_port::{AddressPort};
use crate::report_info::{ReportInfo, TransProtocol};

fn main() {

    let device = Device::lookup().unwrap();
    let output = File::create("report.txt")?;
    println!("{:?}", device);

    println!("Waiting for packets........");

    let mut cap = Capture::from_device(device).unwrap()
        .promisc(true)
        .open().unwrap();
    
    let mut map:HashMap<AddressPort,ReportInfo> = HashMap::new();

    while let Ok(packet) = cap.next() {
        match PacketHeaders::from_ethernet_slice(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {

                let mut address1 = String::new();
                let mut address2 = String::new();
                let mut port1= 0;
                let mut port2= 0;
                let mut transmitted_bytes: u32;
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
                        transmitted_bytes = ipv4header.payload_len as u32;
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
                        transmitted_bytes = ipv6header.payload_length as u32;
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

                println!("----------------------------------");
                //println!("addresses: {:?} {:?}", address1, address2);
                //println!("ports: {:?} {:?}", port1, port2);
                //println!("ip payload length: {:?}", transmitted_bytes);
                
                let key1: AddressPort = AddressPort::new(address1,port1);
                //let key2: AddressPort = AddressPort::new(address2,port2);
                
                map.entry(key1).and_modify(|info| { info.transmitted_bytes += transmitted_bytes; // TODO: Timestamp
                                                                    info.trans_protocols.insert(protocol); }
                    ).or_insert(ReportInfo {transmitted_bytes, initial_timestamp: "".to_string(), final_timestamp: "".to_string(), trans_protocols: HashSet::from([protocol])});
                map.entry(key2).and_modify(|info| { info.trans_protocols.insert(protocol);
                                                                        // TODO: Timestamp
                                                                    }
                    ).or_insert(ReportInfo {transmitted_bytes: 0, initial_timestamp: "".to_string(), final_timestamp: "".to_string(), trans_protocols: HashSet::from([protocol])});

                for (key, val) in map.iter() {
                    write!(output, "Address: {}:{}\n{}", key.get_ip(), key.get_port(), val).expect("File output error");
                }
                println!("----------------------------------");

            }
        }
    }

}