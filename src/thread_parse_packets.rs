//! Module containing functions executed by the thread in charge of parsing sniffed packets and
//! inserting them in the shared map.

use std::sync::{Arc, Mutex};

use etherparse::PacketHeaders;
use pcap::{Active, Capture, Device};

use crate::enums::traffic_type::TrafficType;
use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::filters::Filters;
use crate::utility::countries::COUNTRY_MMDB;
use crate::utility::manage_packets::{
    analyze_network_header, analyze_transport_header, is_broadcast_address, is_multicast_address,
    modify_or_insert_in_map,
};
use crate::{AppProtocol, InfoTraffic, IpVersion, TransProtocol};

/// The calling thread enters in a loop in which it waits for network packets, parses them according
/// to the user specified filters, and inserts them into the shared map variable.
pub fn parse_packets_loop(
    current_capture_id: &Arc<Mutex<u16>>,
    device: Device,
    mut cap: Capture<Active>,
    filters: &Filters,
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
) {
    let capture_id = *current_capture_id.lock().unwrap();

    let mut my_interface_addresses = Vec::new();
    for address in device.addresses {
        my_interface_addresses.push(address.addr.to_string());
    }

    let network_layer_filter = filters.ip;
    let transport_layer_filter = filters.transport;
    let app_layer_filter = filters.application;

    let mut port1 = 0;
    let mut port2 = 0;
    let mut exchanged_bytes: u128 = 0;
    let mut network_protocol;
    let mut transport_protocol;
    let mut application_protocol;
    let mut traffic_type;
    let mut skip_packet;
    let mut reported_packet;

    let country_db_reader = maxminddb::Reader::from_source(COUNTRY_MMDB).unwrap();

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
                        let mut address1 = String::new();
                        let mut address2 = String::new();
                        network_protocol = IpVersion::Other;
                        transport_protocol = TransProtocol::Other;
                        application_protocol = AppProtocol::Other;
                        traffic_type = TrafficType::Other;
                        skip_packet = false;
                        reported_packet = false;

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

                        if my_interface_addresses.contains(&address1) {
                            traffic_type = TrafficType::Outgoing;
                        } else if my_interface_addresses.contains(&address2) {
                            traffic_type = TrafficType::Incoming;
                        } else if is_multicast_address(&address2) {
                            traffic_type = TrafficType::Multicast;
                        } else if is_broadcast_address(&address2) {
                            traffic_type = TrafficType::Broadcast;
                        }

                        let key: AddressPortPair = AddressPortPair::new(
                            address1,
                            port1,
                            address2,
                            port2,
                            transport_protocol,
                        );

                        if (network_layer_filter.eq(&IpVersion::Other)
                            || network_layer_filter.eq(&network_protocol))
                            && (transport_layer_filter.eq(&TransProtocol::Other)
                                || transport_layer_filter.eq(&transport_protocol))
                            && (app_layer_filter.eq(&AppProtocol::Other)
                                || app_layer_filter.eq(&application_protocol))
                        {
                            // if (port1 >= lowest_port && port1 <= highest_port)
                            //     || (port2 >= lowest_port && port2 <= highest_port) {
                            modify_or_insert_in_map(
                                info_traffic_mutex,
                                key,
                                exchanged_bytes,
                                traffic_type,
                                application_protocol,
                                &country_db_reader,
                            );
                            reported_packet = true;
                            // }
                        }

                        let mut info_traffic = info_traffic_mutex
                            .lock()
                            .expect("Error acquiring mutex\n\r");
                        //increment number of sniffed packets and bytes
                        info_traffic.all_packets += 1;
                        info_traffic.all_bytes += exchanged_bytes;

                        if reported_packet {
                            //increment the packet count for the sniffed app protocol
                            info_traffic
                                .app_protocols
                                .entry(application_protocol)
                                .and_modify(|n| *n += 1)
                                .or_insert(1);

                            if traffic_type == TrafficType::Outgoing {
                                //increment number of sent packets and bytes
                                info_traffic.tot_sent_packets += 1;
                                info_traffic.tot_sent_bytes += exchanged_bytes;
                            } else {
                                //increment number of received packets and bytes
                                info_traffic.tot_received_packets += 1;
                                info_traffic.tot_received_bytes += exchanged_bytes;
                            }
                        }
                    }
                }
            }
        }
    }
}
