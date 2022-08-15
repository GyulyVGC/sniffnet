use std::cmp::Ordering::Equal;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};
use etherparse::{IpHeader, PacketHeaders, TransportHeader};
use pcap::{Active, Capture};
use crate::{AddressPort, AppProtocol, ReportInfo, TransProtocol};

pub fn parse_packets_loop(mut cap: Capture<Active>, lowest_port: u16, highest_port: u16,
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