//! Module containing functions executed by the thread in charge of parsing sniffed packets and
//! inserting them in the shared map.

use async_channel::Sender;
use etherparse::err::ip::{HeaderError, LaxHeaderSliceError};
use etherparse::err::{Layer, LenError};
use etherparse::{LaxPacketHeaders, LenSource};
use pcap::{Device, Packet};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::gui::types::message::Message;
use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::manage_packets::{
    analyze_headers, get_address_to_lookup, get_traffic_type, is_local_connection,
    modify_or_insert_in_map, reverse_dns_lookup,
};
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::bogon::is_bogon;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource};
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::filters::Filters;
use crate::networking::types::host::Host;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::info_traffic::InfoTrafficMessage;
use crate::networking::types::my_link_type::MyLinkType;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::utils::error_logger::{ErrorLogger, Location};

/// The calling thread enters a loop in which it waits for network packets, parses them according
/// to the user specified filters, and inserts them into the shared map variable.
pub fn parse_packets(
    current_capture_id: &Mutex<usize>,
    mut cs: CaptureSource,
    filters: &Filters,
    mmdb_readers: &MmdbReaders,
    capture_context: CaptureContext,
    tx: &Sender<Message>,
) {
    let my_link_type = capture_context.my_link_type();
    let (mut cap, mut savefile) = capture_context.consume();

    let capture_id = *current_capture_id.lock().unwrap();
    let mut info_traffic_msg = InfoTrafficMessage::default();

    let resolutions_state = Arc::new(Mutex::new(AddressesResolutionState::default()));

    loop {
        match cap.next_packet() {
            Err(e) => {
                if e == pcap::Error::NoMorePackets {
                    let _ = tx.send_blocking(Message::TickRun(info_traffic_msg));
                    return;
                }
                if *current_capture_id.lock().unwrap() != capture_id {
                    return;
                }
            }
            Ok(packet) => {
                if *current_capture_id.lock().unwrap() != capture_id {
                    return;
                }
                if let Ok(headers) = get_sniffable_headers(&packet, my_link_type) {
                    let mut exchanged_bytes = 0;
                    let mut mac_addresses = (None, None);
                    let mut icmp_type = IcmpType::default();
                    let mut arp_type = ArpType::default();
                    let mut packet_filters_fields = PacketFiltersFields::default();

                    // todo!!!
                    #[allow(clippy::useless_conversion)]
                    let this_packet_timestamp = i64::from(packet.header.ts.tv_sec);
                    if info_traffic_msg.last_packet_timestamp != this_packet_timestamp {
                        if matches!(cs, CaptureSource::Device(_)) {
                            for dev in Device::list().log_err(location!()).unwrap_or_default() {
                                if dev.name.eq(&cs.get_name()) {
                                    cs.set_addresses(dev.addresses.clone());
                                    info_traffic_msg.device_addresses = dev.addresses;
                                    break;
                                }
                            }
                        }

                        let _ = tx.send_blocking(Message::TickRun(
                            info_traffic_msg.take(this_packet_timestamp),
                        ));
                    }

                    let key_option = analyze_headers(
                        headers,
                        &mut mac_addresses,
                        &mut exchanged_bytes,
                        &mut icmp_type,
                        &mut arp_type,
                        &mut packet_filters_fields,
                    );

                    let Some(key) = key_option else {
                        continue;
                    };

                    let mut new_info = InfoAddressPortPair::default();

                    let passed_filters = filters.matches(&packet_filters_fields);
                    if passed_filters {
                        // save this packet to PCAP file
                        if let Some(file) = savefile.as_mut() {
                            file.write(&packet);
                        }
                        // update the shared map
                        new_info = modify_or_insert_in_map(
                            &mut info_traffic_msg,
                            &key,
                            &cs,
                            mac_addresses,
                            icmp_type,
                            arp_type,
                            exchanged_bytes,
                            &resolutions_state,
                        );
                    }

                    //increment number of sniffed packets and bytes
                    info_traffic_msg.all_packets += 1;
                    info_traffic_msg.all_bytes += exchanged_bytes;
                    // update dropped packets number
                    if let Ok(stats) = cap.stats() {
                        info_traffic_msg.dropped_packets = stats.dropped;
                    }

                    if passed_filters {
                        info_traffic_msg.add_packet(exchanged_bytes, new_info.traffic_direction);

                        // check the rDNS status of this address and act accordingly
                        let address_to_lookup =
                            get_address_to_lookup(&key, new_info.traffic_direction);
                        let r_dns_already_resolved = resolutions_state
                            .lock()
                            .unwrap()
                            .addresses_resolved
                            .contains_key(&address_to_lookup);
                        let mut r_dns_waiting_resolution = false;
                        if !r_dns_already_resolved {
                            r_dns_waiting_resolution = resolutions_state
                                .lock()
                                .unwrap()
                                .addresses_waiting_resolution
                                .contains_key(&address_to_lookup);
                        }

                        match (r_dns_waiting_resolution, r_dns_already_resolved) {
                            (false, false) => {
                                // rDNS not requested yet (first occurrence of this address to lookup)

                                // Add this address to the map of addresses waiting for a resolution
                                // Useful to NOT perform again a rDNS lookup for this entry
                                resolutions_state
                                    .lock()
                                    .unwrap()
                                    .addresses_waiting_resolution
                                    .insert(
                                        address_to_lookup,
                                        DataInfo::new_with_first_packet(
                                            exchanged_bytes,
                                            new_info.traffic_direction,
                                        ),
                                    );

                                // launch new thread to resolve host name
                                let key2 = key;
                                let tx2 = tx.clone();
                                let resolutions_state2 = resolutions_state.clone();
                                let cs2 = cs.clone();
                                let mmdb_readers_2 = mmdb_readers.clone();
                                let _ = thread::Builder::new()
                                    .name("thread_reverse_dns_lookup".to_string())
                                    .spawn(move || {
                                        reverse_dns_lookup(
                                            &tx2,
                                            &resolutions_state2,
                                            &key2,
                                            new_info.traffic_direction,
                                            &cs2,
                                            &mmdb_readers_2,
                                        );
                                    })
                                    .log_err(location!());
                            }
                            (true, false) => {
                                // waiting for a previously requested rDNS resolution
                                // update the corresponding waiting address data
                                resolutions_state
                                    .lock()
                                    .unwrap()
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
                                let host = resolutions_state
                                    .lock()
                                    .unwrap()
                                    .addresses_resolved
                                    .get(&address_to_lookup)
                                    .unwrap_or(&(String::new(), Host::default()))
                                    .1
                                    .clone();
                                info_traffic_msg
                                    .hosts
                                    .entry(host)
                                    .and_modify(|data_info_host| {
                                        data_info_host.data_info.add_packet(
                                            exchanged_bytes,
                                            new_info.traffic_direction,
                                        );
                                    })
                                    .or_insert_with(|| {
                                        let traffic_direction = new_info.traffic_direction;
                                        let my_interface_addresses = cs.get_addresses();
                                        let traffic_type = get_traffic_type(
                                            &address_to_lookup,
                                            my_interface_addresses,
                                            traffic_direction,
                                        );
                                        let is_loopback = address_to_lookup.is_loopback();
                                        let is_local = is_local_connection(
                                            &address_to_lookup,
                                            my_interface_addresses,
                                        );
                                        let is_bogon = is_bogon(&address_to_lookup);
                                        DataInfoHost {
                                            data_info: DataInfo::new_with_first_packet(
                                                exchanged_bytes,
                                                traffic_direction,
                                            ),
                                            is_favorite: false,
                                            is_loopback,
                                            is_local,
                                            is_bogon,
                                            traffic_type,
                                        }
                                    });
                            }
                        }

                        //increment the packet count for the sniffed service
                        info_traffic_msg
                            .services
                            .entry(new_info.service)
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
) -> Result<LaxPacketHeaders<'a>, LaxHeaderSliceError> {
    match my_link_type {
        MyLinkType::Ethernet(_) | MyLinkType::Unsupported(_) | MyLinkType::NotYetAssigned => {
            LaxPacketHeaders::from_ethernet(packet).map_err(LaxHeaderSliceError::Len)
        }
        MyLinkType::RawIp(_) | MyLinkType::IPv4(_) | MyLinkType::IPv6(_) => {
            LaxPacketHeaders::from_ip(packet)
        }
        MyLinkType::Null(_) | MyLinkType::Loop(_) => from_null(packet),
    }
}

fn from_null(packet: &[u8]) -> Result<LaxPacketHeaders, LaxHeaderSliceError> {
    if packet.len() <= 4 {
        return Err(LaxHeaderSliceError::Len(LenError {
            required_len: 4,
            len: packet.len(),
            len_source: LenSource::Slice,
            layer: Layer::Ethernet2Header,
            layer_start_offset: 0,
        }));
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
        // as some OS'es use native endianness and others use big endian
        matches(u32::from_le_bytes(b)) || matches(u32::from_be_bytes(b))
    };

    if is_valid_af_inet {
        LaxPacketHeaders::from_ip(&packet[4..])
    } else {
        Err(LaxHeaderSliceError::Content(
            HeaderError::UnsupportedIpVersion { version_number: 0 },
        ))
    }
}

#[derive(Default)]
pub struct AddressesResolutionState {
    /// Map of the addresses waiting for a rDNS resolution; used to NOT send multiple rDNS for the same address
    pub addresses_waiting_resolution: HashMap<IpAddr, DataInfo>,
    /// Map of the resolved addresses with their full rDNS value and the corresponding host
    pub addresses_resolved: HashMap<IpAddr, (String, Host)>,
}
