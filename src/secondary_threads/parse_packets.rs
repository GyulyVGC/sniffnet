//! Module containing functions executed by the thread in charge of parsing sniffed packets and
//! inserting them in the shared map.

use std::sync::{Arc, Mutex};
use std::thread;

use etherparse::{PacketHeaders, ReadError};
use pcap::{Active, Capture, Linktype, Packet};

use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::manage_packets::{
    analyze_headers, get_address_to_lookup, get_app_protocol, modify_or_insert_in_map,
    reverse_dns_lookup,
};
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::filters::Filters;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::InfoTraffic;

/// The calling thread enters in a loop in which it waits for network packets, parses them according
/// to the user specified filters, and inserts them into the shared map variable.
pub fn parse_packets(
    current_capture_id: &Arc<Mutex<usize>>,
    device: &MyDevice,
    mut cap: Capture<Active>,
    filters: &Filters,
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
    country_mmdb_reader: &Arc<MmdbReader>,
    asn_mmdb_reader: &Arc<MmdbReader>,
) {
    let capture_id = *current_capture_id.lock().unwrap();

    // pcap seems to assign ethernet to all interfaces (at least on macOS)...
    let mut link_type = cap.get_datalink();
    // ...it'll be confirmed after having parsed the first packet!
    let mut is_link_type_confirmed = false;

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
                if let Ok(headers) = get_sniffable_headers(
                    &packet,
                    &mut link_type,
                    info_traffic_mutex,
                    &mut is_link_type_confirmed,
                ) {
                    let mut exchanged_bytes = 0;
                    let mut mac_addresses = (None, None);
                    let mut icmp_type = IcmpType::default();
                    let mut packet_filters_fields = PacketFiltersFields::default();

                    let key_option = analyze_headers(
                        headers,
                        &mut mac_addresses,
                        &mut exchanged_bytes,
                        &mut icmp_type,
                        &mut packet_filters_fields,
                    );
                    if key_option.is_none() {
                        continue;
                    }

                    let key = key_option.unwrap();
                    let application_protocol = get_app_protocol(key.port1, key.port2);
                    let mut new_info = InfoAddressPortPair::default();

                    let passed_filters = filters.matches(&packet_filters_fields);
                    if passed_filters {
                        new_info = modify_or_insert_in_map(
                            info_traffic_mutex,
                            &key,
                            device,
                            mac_addresses,
                            icmp_type,
                            exchanged_bytes,
                            application_protocol,
                        );
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

                    if passed_filters {
                        info_traffic.add_packet(exchanged_bytes, new_info.traffic_direction);

                        // check the rDNS status of this address and act accordingly
                        let address_to_lookup =
                            get_address_to_lookup(&key, new_info.traffic_direction);
                        let r_dns_already_resolved = info_traffic
                            .addresses_resolved
                            .contains_key(&address_to_lookup);
                        let mut r_dns_waiting_resolution = false;
                        if !r_dns_already_resolved {
                            r_dns_waiting_resolution = info_traffic
                                .addresses_waiting_resolution
                                .contains_key(&address_to_lookup);
                        }

                        match (r_dns_waiting_resolution, r_dns_already_resolved) {
                            (false, false) => {
                                // rDNS not requested yet (first occurrence of this address to lookup)

                                // Add this address to the map of addresses waiting for a resolution
                                // Useful to NOT perform again a rDNS lookup for this entry
                                info_traffic.addresses_waiting_resolution.insert(
                                    address_to_lookup,
                                    DataInfo::new_with_first_packet(
                                        exchanged_bytes,
                                        new_info.traffic_direction,
                                    ),
                                );

                                // launch new thread to resolve host name
                                let key2 = key.clone();
                                let info_traffic2 = info_traffic_mutex.clone();
                                let device2 = device.clone();
                                let country_db_reader_2 = country_mmdb_reader.clone();
                                let asn_db_reader_2 = asn_mmdb_reader.clone();
                                thread::Builder::new()
                                    .name("thread_reverse_dns_lookup".to_string())
                                    .spawn(move || {
                                        reverse_dns_lookup(
                                            &info_traffic2,
                                            &key2,
                                            new_info.traffic_direction,
                                            &device2,
                                            &country_db_reader_2,
                                            &asn_db_reader_2,
                                        );
                                    })
                                    .unwrap();
                            }
                            (true, false) => {
                                // waiting for a previously requested rDNS resolution
                                // update the corresponding waiting address data
                                info_traffic
                                    .addresses_waiting_resolution
                                    .entry(address_to_lookup)
                                    .and_modify(|data_info| {
                                        data_info.add_packet(
                                            exchanged_bytes,
                                            new_info.traffic_direction,
                                        );
                                    });
                            }
                            (_, true) => {
                                // rDNS already resolved
                                // update the corresponding host's data info
                                let host = info_traffic
                                    .addresses_resolved
                                    .get(&address_to_lookup)
                                    .unwrap()
                                    .1
                                    .clone();
                                info_traffic.hosts.entry(host).and_modify(|data_info_host| {
                                    data_info_host
                                        .data_info
                                        .add_packet(exchanged_bytes, new_info.traffic_direction);
                                });
                            }
                        }

                        //increment the packet count for the sniffed app protocol
                        info_traffic
                            .app_protocols
                            .entry(application_protocol)
                            .and_modify(|data_info| {
                                data_info.add_packet(exchanged_bytes, new_info.traffic_direction);
                            })
                            .or_insert(DataInfo::new_with_first_packet(
                                exchanged_bytes,
                                new_info.traffic_direction,
                            ));
                    }
                }
            }
        }
    }
}

fn get_sniffable_headers<'a>(
    packet: &'a Packet,
    link_type: &mut Linktype,
    info_traffic_mutex: &Arc<Mutex<InfoTraffic>>,
    is_link_type_confirmed: &mut bool,
) -> Result<PacketHeaders<'a>, ReadError> {
    match is_link_type_confirmed {
        false => {
            *is_link_type_confirmed = true;

            let ethernet_result = PacketHeaders::from_ethernet_slice(packet);
            let ip_result = PacketHeaders::from_ip_slice(packet);
            let null_result = PacketHeaders::from_ip_slice(&packet[4..]);

            let is_ethernet_sniffable = are_headers_sniffable(&ethernet_result);
            let is_ip_sniffable = are_headers_sniffable(&ip_result);
            let is_null_sniffable = are_headers_sniffable(&null_result);

            match (is_ethernet_sniffable, is_ip_sniffable, is_null_sniffable) {
                (true, _, _) => {
                    *link_type = Linktype::ETHERNET;
                    info_traffic_mutex.lock().unwrap().link_type = Linktype::ETHERNET;
                    ethernet_result
                }
                (_, true, _) => {
                    // it could be IPV4 as well as IPV6 but it should be the same
                    *link_type = Linktype::IPV4;
                    info_traffic_mutex.lock().unwrap().link_type = Linktype::IPV4;
                    ip_result
                }
                (_, _, true) => {
                    // it could be NULL as well as LOOP but it should be the same
                    *link_type = Linktype::NULL;
                    info_traffic_mutex.lock().unwrap().link_type = Linktype::NULL;
                    null_result
                }
                (false, false, false) => ethernet_result,
            }
        }
        true => match *link_type {
            Linktype::IPV4 | Linktype::IPV6 => PacketHeaders::from_ip_slice(packet),
            Linktype::NULL | Linktype::LOOP => PacketHeaders::from_ip_slice(&packet[4..]),
            _ => PacketHeaders::from_ethernet_slice(packet),
        },
    }
}

fn are_headers_sniffable(headers_result: &Result<PacketHeaders, ReadError>) -> bool {
    if let Ok(headers) = headers_result {
        headers.ip.is_some() && headers.transport.is_some()
    } else {
        false
    }
}
