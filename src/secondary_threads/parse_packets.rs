//! Module containing functions executed by the thread in charge of parsing sniffed packets and
//! inserting them in the shared map.

use std::sync::{Arc, Mutex};
use std::thread;

use etherparse::PacketHeaders;
use pcap::{Active, Capture, Device};

use crate::networking::manage_packets::{
    analyze_link_header, analyze_network_header, analyze_transport_header, modify_or_insert_in_map,
    reverse_dns_lookup,
};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::filters::Filters;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::asn::ASN_MMDB;
use crate::utils::countries::COUNTRY_MMDB;
use crate::{AppProtocol, InfoTraffic, IpVersion, TransProtocol};

/// The calling thread enters in a loop in which it waits for network packets, parses them according
/// to the user specified filters, and inserts them into the shared map variable.
pub fn parse_packets(
    current_capture_id: &Arc<Mutex<u16>>,
    device: &Device,
    mut cap: Capture<Active>,
    filters: &Filters,
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
) {
    let capture_id = *current_capture_id.lock().unwrap();

    let network_layer_filter = filters.ip;
    let transport_layer_filter = filters.transport;
    let app_layer_filter = filters.application;

    let mut port1 = 0;
    let mut port2 = 0;
    let mut exchanged_bytes: u128 = 0;
    let mut network_protocol;
    let mut transport_protocol;
    let mut application_protocol;
    let mut skip_packet;
    let mut reported_packet;

    let country_db_reader = maxminddb::Reader::from_source(COUNTRY_MMDB).unwrap();
    let asn_db_reader = maxminddb::Reader::from_source(ASN_MMDB).unwrap();

    loop {
        match cap.next_packet() {
            Err(_) => {
                if *current_capture_id.lock().unwrap() != capture_id {
                    return;
                }
                continue;
            }
            Ok(packet) => {
                if *current_capture_id.lock().unwrap() != capture_id {
                    return;
                }
                match PacketHeaders::from_ethernet_slice(&packet) {
                    Err(_) => {
                        continue;
                    }
                    Ok(value) => {
                        let mut mac_address1 = String::new();
                        let mut mac_address2 = String::new();
                        let mut address1 = String::new();
                        let mut address2 = String::new();
                        network_protocol = IpVersion::Other;
                        transport_protocol = TransProtocol::Other;
                        application_protocol = AppProtocol::Other;
                        skip_packet = false;
                        reported_packet = false;

                        analyze_link_header(
                            value.link,
                            &mut mac_address1,
                            &mut mac_address2,
                            &mut skip_packet,
                        );
                        if skip_packet {
                            continue;
                        }

                        analyze_network_header(
                            value.ip,
                            &mut exchanged_bytes,
                            &mut network_protocol,
                            &mut address1,
                            &mut address2,
                            &mut skip_packet,
                        );
                        if skip_packet {
                            continue;
                        }

                        analyze_transport_header(
                            value.transport,
                            &mut port1,
                            &mut port2,
                            &mut application_protocol,
                            &mut transport_protocol,
                            &mut skip_packet,
                        );
                        if skip_packet {
                            continue;
                        }

                        let key: AddressPortPair = AddressPortPair::new(
                            address1.clone(),
                            port1,
                            address2.clone(),
                            port2,
                            transport_protocol,
                        );

                        let mut new_info = InfoAddressPortPair::default();
                        if (network_layer_filter.eq(&IpVersion::Other)
                            || network_layer_filter.eq(&network_protocol))
                            && (transport_layer_filter.eq(&TransProtocol::Other)
                                || transport_layer_filter.eq(&transport_protocol))
                            && (app_layer_filter.eq(&AppProtocol::Other)
                                || app_layer_filter.eq(&application_protocol))
                        {
                            new_info = modify_or_insert_in_map(
                                info_traffic_mutex,
                                key.clone(),
                                &device.addresses,
                                (mac_address1, mac_address2),
                                exchanged_bytes,
                                application_protocol,
                                &country_db_reader,
                                &asn_db_reader,
                            );
                            reported_packet = true;
                        }

                        let mut info_traffic = info_traffic_mutex
                            .lock()
                            .expect("Error acquiring mutex\n\r");
                        //increment number of sniffed packets and bytes
                        info_traffic.all_packets += 1;
                        info_traffic.all_bytes += exchanged_bytes;
                        // update dropped packets number
                        if let Ok(stats) = cap.stats() {
                            info_traffic.dropped_packets = stats.dropped;
                        }

                        if reported_packet {
                            if new_info.traffic_direction == TrafficDirection::Outgoing {
                                //increment number of sent packets and bytes
                                info_traffic.tot_sent_packets += 1;
                                info_traffic.tot_sent_bytes += exchanged_bytes;
                            } else {
                                //increment number of received packets and bytes
                                info_traffic.tot_received_packets += 1;
                                info_traffic.tot_received_bytes += exchanged_bytes;
                            }

                            // check the rDNS status and act consequently
                            match (
                                new_info.r_dns_already_requested(),
                                new_info.r_dns_already_resolved(),
                            ) {
                                (false, _) => {
                                    // rDNS not requested yet (first occurrence of this key)

                                    // Assign the r_dns field of this entry to an empty string (requested but not resolved yet).
                                    // Useful to NOT perform again a rDNS lookup for this entry.
                                    info_traffic.map.entry(key.clone()).and_modify(|info| {
                                        info.r_dns = Some(String::new());
                                    });

                                    // launch new thread to resolve host name
                                    let key2 = key.clone();
                                    let info_traffic2 = info_traffic_mutex.clone();
                                    thread::Builder::new()
                                        .name("thread_reverse_dns_lookup".to_string())
                                        .spawn(move || {
                                            reverse_dns_lookup(
                                                &info_traffic2,
                                                key2,
                                                new_info.traffic_direction,
                                            );
                                        })
                                        .unwrap();
                                }
                                (true, false) => {
                                    // waiting for a previously requested rDNS resolution
                                    // do nothing
                                }
                                (true, true) => {
                                    // rDNS already resolved
                                    // update the corresponding host's data info
                                    info_traffic
                                        .hosts
                                        .entry(new_info.get_host())
                                        .and_modify(|data_info_host| {
                                            if new_info.traffic_direction
                                                == TrafficDirection::Outgoing
                                            {
                                                data_info_host.data_info.outgoing_packets += 1;
                                                data_info_host.data_info.outgoing_bytes +=
                                                    exchanged_bytes;
                                            } else {
                                                data_info_host.data_info.incoming_packets += 1;
                                                data_info_host.data_info.incoming_bytes +=
                                                    exchanged_bytes;
                                            }
                                        })
                                        .or_insert(
                                            if new_info.traffic_direction
                                                == TrafficDirection::Outgoing
                                            {
                                                DataInfoHost {
                                                    data_info: DataInfo {
                                                        incoming_packets: 0,
                                                        outgoing_packets: new_info
                                                            .transmitted_packets,
                                                        incoming_bytes: 0,
                                                        outgoing_bytes: new_info.transmitted_bytes,
                                                    },
                                                    is_favorite: false,
                                                    is_local: new_info.is_local,
                                                    traffic_type: new_info.traffic_type,
                                                }
                                            } else {
                                                DataInfoHost {
                                                    data_info: DataInfo {
                                                        incoming_packets: new_info
                                                            .transmitted_packets,
                                                        outgoing_packets: 0,
                                                        incoming_bytes: new_info.transmitted_bytes,
                                                        outgoing_bytes: 0,
                                                    },
                                                    is_favorite: false,
                                                    is_local: new_info.is_local,
                                                    traffic_type: new_info.traffic_type,
                                                }
                                            },
                                        );
                                }
                            }

                            //increment the packet count for the sniffed app protocol
                            info_traffic
                                .app_protocols
                                .entry(application_protocol)
                                .and_modify(|data_info| {
                                    if new_info.traffic_direction == TrafficDirection::Outgoing {
                                        data_info.outgoing_packets += 1;
                                        data_info.outgoing_bytes += exchanged_bytes;
                                    } else {
                                        data_info.incoming_packets += 1;
                                        data_info.incoming_bytes += exchanged_bytes;
                                    }
                                })
                                .or_insert(
                                    if new_info.traffic_direction == TrafficDirection::Outgoing {
                                        DataInfo {
                                            incoming_packets: 0,
                                            outgoing_packets: 1,
                                            incoming_bytes: 0,
                                            outgoing_bytes: exchanged_bytes,
                                        }
                                    } else {
                                        DataInfo {
                                            incoming_packets: 1,
                                            outgoing_packets: 0,
                                            incoming_bytes: exchanged_bytes,
                                            outgoing_bytes: 0,
                                        }
                                    },
                                );
                        }
                    }
                }
            }
        }
    }
}
