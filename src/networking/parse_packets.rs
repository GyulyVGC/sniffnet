//! Module containing functions executed by the thread in charge of parsing sniffed packets

use crate::location;
use crate::mmdb::asn::get_asn;
use crate::mmdb::country::get_country;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::manage_packets::{
    analyze_headers, get_address_to_lookup, get_traffic_type, is_local_connection,
    modify_or_insert_in_map,
};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::bogon::is_bogon;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource, CaptureType};
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::{Host, HostMessage};
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::my_link_type::MyLinkType;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::formatted_strings::get_domain_from_r_dns;
use crate::utils::types::timestamp::Timestamp;
use async_channel::Sender;
use dns_lookup::lookup_addr;
use etherparse::{EtherType, LaxPacketHeaders};
use pcap::{Address, Packet, PacketHeader};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// The calling thread enters a loop in which it waits for network packets
pub fn parse_packets(
    cap_id: usize,
    mut cs: CaptureSource,
    mmdb_readers: &MmdbReaders,
    capture_context: CaptureContext,
    tx: &Sender<BackendTrafficMessage>,
) {
    let my_link_type = capture_context.my_link_type();
    let (cap, mut savefile) = capture_context.consume();

    let mut info_traffic_msg = InfoTraffic::default();
    let resolutions_state = Arc::new(Mutex::new(AddressesResolutionState::default()));
    // list of newly resolved hosts to be sent (batched to avoid UI updates too often)
    let new_hosts_to_send = Arc::new(Mutex::new(Vec::new()));

    // instant of the first parsed packet plus multiples of 1 second (only used in live captures)
    let mut first_packet_ticks = None;

    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);
    let _ = thread::Builder::new()
        .name("thread_packet_stream".to_string())
        .spawn(move || packet_stream(cap, &pcap_tx))
        .log_err(location!());

    loop {
        let (packet_res, cap_stats) = pcap_rx
            .recv_timeout(Duration::from_millis(150))
            .unwrap_or((Err(pcap::Error::TimeoutExpired), None));

        if tx.is_closed() {
            return;
        }

        if matches!(cs, CaptureSource::Device(_)) {
            maybe_send_tick_run_live(
                cap_id,
                &mut info_traffic_msg,
                &new_hosts_to_send,
                &mut cs,
                &mut first_packet_ticks,
                tx,
            );
        }

        match packet_res {
            Err(e) => {
                if e == pcap::Error::NoMorePackets {
                    // send a message including data from the last interval (only happens in offline captures)
                    let _ = tx.send_blocking(BackendTrafficMessage::TickRun(
                        cap_id,
                        info_traffic_msg,
                        new_hosts_to_send.lock().unwrap().drain(..).collect(),
                        true,
                    ));
                    // wait until there is still some thread doing rdns
                    while tx.sender_count() > 1 {
                        thread::sleep(Duration::from_millis(1000));
                    }
                    // send one last message including all pending hosts
                    let _ = tx.send_blocking(BackendTrafficMessage::PendingHosts(
                        cap_id,
                        new_hosts_to_send.lock().unwrap().drain(..).collect(),
                    ));
                    return;
                }
            }
            Ok(packet) => {
                if let Some(headers) = get_sniffable_headers(&packet.data, my_link_type) {
                    #[allow(clippy::useless_conversion)]
                    let secs = i64::from(packet.header.ts.tv_sec);
                    #[allow(clippy::useless_conversion)]
                    let usecs = i64::from(packet.header.ts.tv_usec);
                    let next_packet_timestamp = Timestamp::new(secs, usecs);

                    if matches!(cs, CaptureSource::File(_)) {
                        maybe_send_tick_run_offline(
                            cap_id,
                            &mut info_traffic_msg,
                            &new_hosts_to_send,
                            next_packet_timestamp,
                            tx,
                        );
                    } else if first_packet_ticks.is_none() {
                        first_packet_ticks = Some(Instant::now());
                    }

                    info_traffic_msg.last_packet_timestamp = next_packet_timestamp;

                    let mut exchanged_bytes = 0;
                    let mut mac_addresses = (None, None);
                    let mut icmp_type = IcmpType::default();
                    let mut arp_type = ArpType::default();
                    let mut packet_filters_fields = PacketFiltersFields::default();

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

                    // save this packet to PCAP file
                    if let Some(file) = savefile.as_mut() {
                        file.write(&Packet {
                            header: &packet.header,
                            data: &packet.data,
                        });
                    }
                    // update the map
                    let (traffic_direction, service) = modify_or_insert_in_map(
                        &mut info_traffic_msg,
                        &key,
                        &cs,
                        mac_addresses,
                        icmp_type,
                        arp_type,
                        exchanged_bytes,
                    );

                    info_traffic_msg
                        .tot_data_info
                        .add_packet(exchanged_bytes, traffic_direction);

                    // check the rDNS status of this address and act accordingly
                    let address_to_lookup = get_address_to_lookup(&key, traffic_direction);
                    let mut r_dns_waiting_resolution = false;
                    let mut resolutions_lock = resolutions_state.lock().unwrap();
                    let r_dns_already_resolved = resolutions_lock
                        .addresses_resolved
                        .contains_key(&address_to_lookup);
                    if !r_dns_already_resolved {
                        r_dns_waiting_resolution = resolutions_lock
                            .addresses_waiting_resolution
                            .contains_key(&address_to_lookup);
                    }

                    match (r_dns_waiting_resolution, r_dns_already_resolved) {
                        (false, false) => {
                            // rDNS not requested yet (first occurrence of this address to lookup)

                            // Add this address to the map of addresses waiting for a resolution
                            // Useful to NOT perform again a rDNS lookup for this entry
                            resolutions_lock.addresses_waiting_resolution.insert(
                                address_to_lookup,
                                DataInfo::new_with_first_packet(exchanged_bytes, traffic_direction),
                            );
                            drop(resolutions_lock);

                            // launch new thread to resolve host name
                            let key2 = key;
                            let resolutions_state2 = resolutions_state.clone();
                            let new_hosts_to_send2 = new_hosts_to_send.clone();
                            let interface_addresses = cs.get_addresses().clone();
                            let mmdb_readers_2 = mmdb_readers.clone();
                            let tx2 = tx.clone();
                            let _ = thread::Builder::new()
                                .name("thread_reverse_dns_lookup".to_string())
                                .spawn(move || {
                                    reverse_dns_lookup(
                                        &resolutions_state2,
                                        &new_hosts_to_send2,
                                        &key2,
                                        traffic_direction,
                                        &interface_addresses,
                                        &mmdb_readers_2,
                                        &tx2,
                                    );
                                })
                                .log_err(location!());
                        }
                        (true, false) => {
                            // waiting for a previously requested rDNS resolution
                            // update the corresponding waiting address data
                            resolutions_lock
                                .addresses_waiting_resolution
                                .entry(address_to_lookup)
                                .and_modify(|data_info| {
                                    data_info.add_packet(exchanged_bytes, traffic_direction);
                                });
                            drop(resolutions_lock);
                        }
                        (_, true) => {
                            // rDNS already resolved
                            // update the corresponding host's data info
                            let host = resolutions_lock
                                .addresses_resolved
                                .get(&address_to_lookup)
                                .unwrap_or(&Host::default())
                                .clone();
                            drop(resolutions_lock);
                            info_traffic_msg
                                .hosts
                                .entry(host)
                                .and_modify(|data_info_host| {
                                    data_info_host
                                        .data_info
                                        .add_packet(exchanged_bytes, traffic_direction);
                                })
                                .or_insert_with(|| {
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
                        .entry(service)
                        .and_modify(|data_info| {
                            data_info.add_packet(exchanged_bytes, traffic_direction);
                        })
                        .or_insert_with(|| {
                            DataInfo::new_with_first_packet(exchanged_bytes, traffic_direction)
                        });

                    // update dropped packets number
                    if let Some(stats) = cap_stats {
                        info_traffic_msg.dropped_packets = stats.dropped;
                    }
                }
            }
        }
    }
}

fn get_sniffable_headers(packet: &[u8], my_link_type: MyLinkType) -> Option<LaxPacketHeaders<'_>> {
    match my_link_type {
        MyLinkType::Ethernet(_) | MyLinkType::Unsupported(_) | MyLinkType::NotYetAssigned => {
            LaxPacketHeaders::from_ethernet(packet).ok()
        }
        MyLinkType::RawIp(_) | MyLinkType::IPv4(_) | MyLinkType::IPv6(_) => {
            LaxPacketHeaders::from_ip(packet).ok()
        }
        MyLinkType::LinuxSll(_) => from_linux_sll(packet, true),
        MyLinkType::LinuxSll2(_) => from_linux_sll(packet, false),
        MyLinkType::Null(_) | MyLinkType::Loop(_) => from_null(packet),
    }
}

fn from_null(packet: &[u8]) -> Option<LaxPacketHeaders<'_>> {
    if packet.len() <= 4 {
        return None;
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
        LaxPacketHeaders::from_ip(&packet[4..]).ok()
    } else {
        None
    }
}

fn from_linux_sll(packet: &[u8], is_v1: bool) -> Option<LaxPacketHeaders<'_>> {
    let header_len = if is_v1 { 16 } else { 20 };
    if packet.len() <= header_len {
        return None;
    }

    let protocol_type = u16::from_be_bytes(if is_v1 {
        [packet[14], packet[15]]
    } else {
        [packet[0], packet[1]]
    });
    let payload = &packet[header_len..];

    Some(LaxPacketHeaders::from_ether_type(
        EtherType(protocol_type),
        payload,
    ))
}

fn reverse_dns_lookup(
    resolutions_state: &Arc<Mutex<AddressesResolutionState>>,
    new_hosts_to_send: &Arc<Mutex<Vec<HostMessage>>>,
    key: &AddressPortPair,
    traffic_direction: TrafficDirection,
    interface_addresses: &Vec<Address>,
    mmdb_readers: &MmdbReaders,
    // needed to know that this thread is still running!
    _tx: &Sender<BackendTrafficMessage>,
) {
    let address_to_lookup = get_address_to_lookup(key, traffic_direction);

    // perform rDNS lookup
    let lookup_result = lookup_addr(&address_to_lookup);

    // get new host info and build the new host
    let traffic_type = get_traffic_type(&address_to_lookup, interface_addresses, traffic_direction);
    let is_loopback = address_to_lookup.is_loopback();
    let is_local = is_local_connection(&address_to_lookup, interface_addresses);
    let is_bogon = is_bogon(&address_to_lookup);
    let country = get_country(&address_to_lookup, &mmdb_readers.country);
    let asn = get_asn(&address_to_lookup, &mmdb_readers.asn);
    let rdns = if let Ok(result) = lookup_result {
        if result.is_empty() {
            address_to_lookup.to_string()
        } else {
            result
        }
    } else {
        address_to_lookup.to_string()
    };
    let new_host = Host {
        domain: get_domain_from_r_dns(rdns.clone()),
        asn,
        country,
    };

    // collect the data exchanged from the same address so far and remove the address from the collection of addresses waiting a rDNS
    let mut resolutions_lock = resolutions_state.lock().unwrap();
    let other_data = resolutions_lock
        .addresses_waiting_resolution
        .remove(&address_to_lookup)
        .unwrap_or_default();
    // insert the newly resolved host in the collections, with the data it exchanged so far
    resolutions_lock
        .addresses_resolved
        .insert(address_to_lookup, new_host.clone());
    drop(resolutions_lock);

    let data_info_host = DataInfoHost {
        data_info: other_data,
        is_favorite: false,
        is_local,
        is_bogon,
        is_loopback,
        traffic_type,
    };

    let msg_data = HostMessage {
        host: new_host,
        data_info_host,
        address_to_lookup,
        rdns,
    };

    // add the new host to the list of hosts to be sent
    new_hosts_to_send.lock().unwrap().push(msg_data);
}

#[derive(Default)]
pub struct AddressesResolutionState {
    /// Map of the addresses waiting for a rDNS resolution; used to NOT send multiple rDNS for the same address
    addresses_waiting_resolution: HashMap<IpAddr, DataInfo>,
    /// Map of the resolved addresses with the corresponding host
    pub addresses_resolved: HashMap<IpAddr, Host>,
}

#[allow(clippy::large_enum_variant)]
pub enum BackendTrafficMessage {
    TickRun(usize, InfoTraffic, Vec<HostMessage>, bool),
    PendingHosts(usize, Vec<HostMessage>),
    OfflineGap(usize, u32),
}

fn maybe_send_tick_run_live(
    cap_id: usize,
    info_traffic_msg: &mut InfoTraffic,
    new_hosts_to_send: &Arc<Mutex<Vec<HostMessage>>>,
    cs: &mut CaptureSource,
    first_packet_ticks: &mut Option<Instant>,
    tx: &Sender<BackendTrafficMessage>,
) {
    if first_packet_ticks.is_some_and(|i| i.elapsed() >= Duration::from_millis(1000)) {
        *first_packet_ticks =
            first_packet_ticks.and_then(|i| i.checked_add(Duration::from_millis(1000)));
        let _ = tx.send_blocking(BackendTrafficMessage::TickRun(
            cap_id,
            info_traffic_msg.take_but_leave_something(),
            new_hosts_to_send.lock().unwrap().drain(..).collect(),
            false,
        ));
        cs.set_addresses();
    }
}

fn maybe_send_tick_run_offline(
    cap_id: usize,
    info_traffic_msg: &mut InfoTraffic,
    new_hosts_to_send: &Arc<Mutex<Vec<HostMessage>>>,
    next_packet_timestamp: Timestamp,
    tx: &Sender<BackendTrafficMessage>,
) {
    if info_traffic_msg.last_packet_timestamp == Timestamp::default() {
        info_traffic_msg.last_packet_timestamp = next_packet_timestamp;
    }
    if info_traffic_msg.last_packet_timestamp.secs() < next_packet_timestamp.secs() {
        let diff_secs =
            next_packet_timestamp.secs() - info_traffic_msg.last_packet_timestamp.secs();
        let _ = tx.send_blocking(BackendTrafficMessage::TickRun(
            cap_id,
            info_traffic_msg.take_but_leave_something(),
            new_hosts_to_send.lock().unwrap().drain(..).collect(),
            false,
        ));
        if diff_secs > 1 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let _ = tx.send_blocking(BackendTrafficMessage::OfflineGap(
                cap_id,
                diff_secs as u32 - 1,
            ));
        }
    }
}

fn packet_stream(
    mut cap: CaptureType,
    tx: &std::sync::mpsc::SyncSender<(Result<PacketOwned, pcap::Error>, Option<pcap::Stat>)>,
) {
    loop {
        let packet_res = cap.next_packet();
        let packet_owned = packet_res.map(|p| PacketOwned {
            header: *p.header,
            data: p.data.into(),
        });
        if tx.send((packet_owned, cap.stats().ok())).is_err() {
            return;
        }
    }
}

struct PacketOwned {
    pub header: PacketHeader,
    pub data: Box<[u8]>,
}
