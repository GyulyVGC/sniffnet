mod address_port;
mod report_info;

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Add;
use etherparse::{IpHeader, PacketHeaders, TransportHeader};
use pcap::{Device, Capture};
use crate::address_port::AddressPortPair;
use crate::report_info::ReportInfo;

fn main() {

    let device = Device::lookup().unwrap();
    println!("{:?}", device);

    println!("Waiting for packets........");

    let mut cap = Capture::from_device(device).unwrap()
        .promisc(true)
        .open().unwrap();
    
    let mut map:HashMap<AddressPortPair,ReportInfo> = HashMap::new();

    while let Ok(packet) = cap.next() {
        match PacketHeaders::from_ethernet_slice(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {

                let mut address1 = String::new();
                let mut address2 = String::new();
                let mut port1= 0;
                let mut port2= 0;
                let mut transmitted_bytes;

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
                        transmitted_bytes = ipv4header.payload_len;
                    }
                    IpHeader::Version6(ipv6header, _) => {
                        address1 = format!("{:?}", ipv6header.source)
                            .replace("[","")
                            .replace("]","")
                            .replace(",",".")
                            .replace(" ","");
                        address2 = format!("{:?}", ipv6header.destination)
                            .replace("[","")
                            .replace("]","")
                            .replace(",",".")
                            .replace(" ","");
                        transmitted_bytes = ipv6header.payload_length;
                    }
                }

                match value.transport.unwrap() {
                    TransportHeader::Udp(udpheader) => {
                        port1 = udpheader.source_port;
                        port2 = udpheader.destination_port
                    }
                    TransportHeader::Tcp(tcpheader) => {
                        port1 = tcpheader.source_port;
                        port2 = tcpheader.destination_port
                    }
                    TransportHeader::Icmpv4(_) => {}
                    TransportHeader::Icmpv6(_) => {}
                }

                println!("----------------------------------");
                println!("addresses: {:?} {:?}", address1, address2);
                println!("ports: {:?} {:?}", port1, port2);
                println!("ip payload length: {:?}", transmitted_bytes);
                
                let key1: AddressPortPair = AddressPortPair::new(address1,port1);
                let key2: AddressPortPair = AddressPortPair::new(address2,port2);
                
                map.insert(key1,ReportInfo::new());
                map.insert(key2,ReportInfo::new());
                println!("map: {:?}",map);
                println!("----------------------------------");

            }
        }
    }

}