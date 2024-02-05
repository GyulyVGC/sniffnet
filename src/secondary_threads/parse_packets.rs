//! Module containing functions executed by the thread in charge of parsing sniffed packets and
//! inserting them in the shared map.

use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::thread;

use etherparse::{PacketHeaders, ReadError};
use pcap::{Active, Capture, Packet};

use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::networking::manage_packets::{
    analyze_headers, get_address_to_lookup, modify_or_insert_in_map, reverse_dns_lookup,
};
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::filters::Filters;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::my_link_type::MyLinkType;
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

    let my_link_type = MyLinkType::from_pcap_link_type(cap.get_datalink());

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
                if let Ok(headers) = get_sniffable_headers(&packet, my_link_type) {
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
                            .entry(new_info.app_protocol)
                            .and_modify(|data_info| {
                                data_info.add_packet(exchanged_bytes, new_info.traffic_direction);
                            })
                            .or_insert_with(|| {
                                DataInfo::new_with_first_packet(
                                    exchanged_bytes,
                                    new_info.traffic_direction,
                                )
                            });
                    }
                }
            }
        }
    }
}

fn get_sniffable_headers<'a>(
    packet: &'a Packet,
    my_link_type: MyLinkType,
) -> Result<PacketHeaders<'a>, ReadError> {
    match my_link_type {
        MyLinkType::Ethernet(_) => PacketHeaders::from_ethernet_slice(packet),
        MyLinkType::RawIp(_) | MyLinkType::IPv4(_) | MyLinkType::IPv6(_) => {
            PacketHeaders::from_ip_slice(packet)
        }
        MyLinkType::Null(_) | MyLinkType::Loop(_) => from_null_slice(packet),
        MyLinkType::Unsupported(_) | MyLinkType::NotYetAssigned => {
            PacketHeaders::from_ethernet_slice(packet)
        }
    }
}

fn from_null_slice(packet: &[u8]) -> Result<PacketHeaders, ReadError> {
    if packet.len() <= 4 {
        return Err(ReadError::UnexpectedEndOfSlice(packet.len()));
    }

    let is_valid_af_inet = {
        // based on https://wiki.wireshark.org/NullLoopback.md (2023-12-31)
        fn matches(value: u32) -> bool {
            match value {
                // 2 = IPv4 on all platforms
                // 24, 28, or 30 = IPv6 depending on platform
                2 | 24 | 28 | 30 => true,
                _ => false,
            }
        }
        let h = &packet[..4];
        let b = [h[0], h[1], h[2], h[3]];
        // check both big endian and little endian representations
        // as some OS'es use native endianess and others use big endian
        matches(u32::from_le_bytes(b)) || matches(u32::from_be_bytes(b))
    };

    if is_valid_af_inet {
        PacketHeaders::from_ip_slice(&packet[4..])
    } else {
        Err(ReadError::IoError(std::io::Error::new(
            ErrorKind::InvalidData,
            "Invalid AF_INET / AF_INET6 value",
        )))
    }
}
