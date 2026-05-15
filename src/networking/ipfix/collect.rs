//! IPFIX collector runtime — binds a UDP socket, decodes incoming IPFIX
//! datagrams, and projects flow records into the same `InfoTraffic` shape the
//! pcap pipeline produces.

use async_channel::Sender;
use pcap::Address;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::Receiver;

use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::ipfix::MyIpfixSocket;
use crate::networking::ipfix::templates::TemplateCache;
use crate::networking::ipfix::wire::{
    self, FlowRecord, IPFIX_VERSION, Set, decode_data_record, format_mac, parse_message,
};
use crate::networking::manage_packets::{
    get_address_to_lookup, get_traffic_type, is_local_connection, modify_or_insert_in_map,
};
use crate::networking::parse_packets::{
    AddressesResolutionState, BackendTrafficMessage, reverse_dns_lookups,
};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::bogon::is_bogon;
use crate::networking::types::capture_context::{CaptureContext, CaptureType};
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::networking::types::protocol::Protocol;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::timestamp::Timestamp;

/// Buffer size for a single UDP datagram. RFC 7011 §10.3.1 recommends at least
/// 1500; we size larger to accommodate jumbo-framed exporters.
const RECV_BUF_LEN: usize = 65_535;

/// Receive timeout — short enough to allow tick emission and freeze checks at
/// roughly the same cadence as the pcap loop.
const RECV_TIMEOUT: Duration = Duration::from_millis(150);

/// Entry point for the IPFIX collector thread. Mirrors `parse_packets` in
/// terms of channel contracts: it emits `BackendTrafficMessage::TickRun` every
/// second with the accumulated `InfoTraffic`.
pub fn collect_ipfix(
    cap_id: usize,
    socket: UdpSocket,
    mmdb_readers: MmdbReaders,
    ip_blacklist: &IpBlacklist,
    tx: &Sender<BackendTrafficMessage>,
    freeze_rxs: (Receiver<()>, Receiver<()>),
) {
    let (mut freeze_rx, _freeze_rx_2) = freeze_rxs;

    let mut info_traffic_msg = InfoTraffic::default();
    let mut templates = TemplateCache::new();
    let mut buf = vec![0u8; RECV_BUF_LEN];
    let mut first_packet_ticks: Option<Instant> = None;

    let (lookup_request_tx, lookup_request_rx) = std::sync::mpsc::channel();
    let (lookup_result_tx, lookup_result_rx) = std::sync::mpsc::channel();
    let mut resolutions_state = AddressesResolutionState::new(lookup_request_tx, lookup_result_rx);
    let _ = thread::Builder::new()
        .name("thread_reverse_dns_lookups".to_string())
        .spawn(move || {
            reverse_dns_lookups(&lookup_request_rx, &lookup_result_tx, &mmdb_readers);
        })
        .log_err(location!());

    loop {
        if tx.is_closed() {
            return;
        }

        if freeze_rx.try_recv().is_ok() {
            let _ = freeze_rx.blocking_recv();
            first_packet_ticks = Some(Instant::now());
        }

        maybe_send_tick(
            cap_id,
            &mut info_traffic_msg,
            &mut first_packet_ticks,
            tx,
            &mut resolutions_state,
        );

        match socket.recv_from(&mut buf) {
            Ok((len, peer)) => {
                if first_packet_ticks.is_none() {
                    first_packet_ticks = Some(Instant::now());
                }
                info_traffic_msg.last_packet_timestamp = current_timestamp();
                process_datagram(
                    &buf[..len],
                    peer,
                    &mut templates,
                    &mut info_traffic_msg,
                    ip_blacklist,
                    &mut resolutions_state,
                );
            }
            Err(e) => match e.kind() {
                // expected — timeout fires regularly so we can tick and check freeze
                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {}
                _ => {
                    // Real socket error: log and keep listening.
                    let _: Result<(), std::io::Error> = Err(e).log_err(location!());
                }
            },
        }
    }
}

fn current_timestamp() -> Timestamp {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    #[allow(clippy::cast_possible_wrap)]
    Timestamp::new(now.as_secs() as i64, i64::from(now.subsec_micros()))
}

fn maybe_send_tick(
    cap_id: usize,
    info_traffic_msg: &mut InfoTraffic,
    first_packet_ticks: &mut Option<Instant>,
    tx: &Sender<BackendTrafficMessage>,
    resolutions_state: &mut AddressesResolutionState,
) {
    if first_packet_ticks.is_some_and(|i| i.elapsed() >= Duration::from_secs(1)) {
        *first_packet_ticks =
            first_packet_ticks.and_then(|i| i.checked_add(Duration::from_secs(1)));
        let _ = tx.send_blocking(BackendTrafficMessage::TickRun(
            cap_id,
            info_traffic_msg.take_but_leave_something(),
            resolutions_state.new_hosts_to_send(),
            false,
        ));
    }
}

fn process_datagram(
    bytes: &[u8],
    peer: SocketAddr,
    templates: &mut TemplateCache,
    info_traffic_msg: &mut InfoTraffic,
    ip_blacklist: &IpBlacklist,
    resolutions_state: &mut AddressesResolutionState,
) {
    let Ok((_, message)) = parse_message(bytes) else {
        // Malformed datagram — log and drop, no panic.
        return;
    };
    if message.header.version != IPFIX_VERSION {
        return;
    }

    let exporter_addresses = exporter_as_addresses(peer.ip());

    // First pass: register all templates so later data sets in the same
    // datagram can reference them.
    for set in &message.sets {
        if let Set::Template(records) = set {
            for record in records {
                templates.insert(
                    peer,
                    message.header.observation_domain_id,
                    record.template_id,
                    record.fields.clone(),
                );
            }
        }
    }

    // Second pass: decode data sets and project records into InfoTraffic.
    for set in &message.sets {
        if let Set::Data {
            template_id,
            payload,
        } = *set
        {
            let Some(template) =
                templates.get(peer, message.header.observation_domain_id, template_id)
            else {
                // Data record references a template we haven't seen — skip
                // silently per RFC 7011 §8.
                continue;
            };
            let mut remaining = payload;
            // Decode records until the remaining bytes can no longer fit a
            // record, treating any trailing bytes as padding (RFC 7011 §3.3.1).
            while record_fits(template, remaining) {
                let Ok((rest, record)) = decode_data_record(template, remaining) else {
                    break;
                };
                if rest.len() == remaining.len() {
                    // No progress — guard against infinite loops on templates
                    // with all-zero-length fields.
                    break;
                }
                remaining = rest;
                ingest_flow_record(
                    &record,
                    &exporter_addresses,
                    info_traffic_msg,
                    ip_blacklist,
                    resolutions_state,
                );
            }
        }
    }
}

fn record_fits(template: &[wire::FieldSpec], remaining: &[u8]) -> bool {
    // A template with at least one fixed-length field can be sized
    // statically; variable-length fields can never satisfy a strict
    // "remaining >= min_size" check below their 1-byte length prefix so we
    // fall back to "at least the variable-length prefix is present."
    let mut needed = 0usize;
    for field in template {
        if field.length == wire::VARIABLE_LENGTH {
            needed += 1; // at minimum the 1-byte length prefix
        } else {
            needed += field.length as usize;
        }
    }
    remaining.len() >= needed && needed > 0
}

fn ingest_flow_record(
    record: &FlowRecord,
    exporter_addresses: &[Address],
    info_traffic_msg: &mut InfoTraffic,
    ip_blacklist: &IpBlacklist,
    resolutions_state: &mut AddressesResolutionState,
) {
    let Some(key) = build_key(record) else {
        return;
    };
    let exchanged_bytes = record.bytes;
    let exchanged_packets = if record.packets == 0 {
        1
    } else {
        record.packets
    };
    let mac_addresses = (
        record.src_mac.map(format_mac),
        record.dst_mac.map(format_mac),
    );

    let (traffic_direction, service) = modify_or_insert_in_map(
        info_traffic_msg,
        &key,
        exporter_addresses,
        mac_addresses,
        IcmpType::default(),
        ArpType::default(),
        exchanged_bytes,
        exchanged_packets,
        ip_blacklist,
    );

    info_traffic_msg.tot_data_info.add_packets(
        exchanged_packets,
        exchanged_bytes,
        traffic_direction,
        Instant::now(),
    );

    let address_to_lookup = get_address_to_lookup(&key, traffic_direction);
    let already_resolved = resolutions_state
        .addresses_resolved
        .contains_key(&address_to_lookup);
    let waiting_resolution = resolutions_state
        .addresses_waiting_resolution
        .contains_key(&address_to_lookup);

    match (waiting_resolution, already_resolved) {
        (false, false) => {
            let mut data_info = DataInfo::default();
            data_info.add_packets(
                exchanged_packets,
                exchanged_bytes,
                traffic_direction,
                Instant::now(),
            );
            resolutions_state
                .addresses_waiting_resolution
                .insert(address_to_lookup, data_info);
            let _ = resolutions_state.lookup_request_tx.send((
                key,
                traffic_direction,
                exporter_addresses.to_vec(),
            ));
        }
        (true, false) => {
            resolutions_state
                .addresses_waiting_resolution
                .entry(address_to_lookup)
                .and_modify(|data_info| {
                    data_info.add_packets(
                        exchanged_packets,
                        exchanged_bytes,
                        traffic_direction,
                        Instant::now(),
                    );
                });
        }
        (_, true) => {
            let host = resolutions_state
                .addresses_resolved
                .get(&address_to_lookup)
                .cloned()
                .unwrap_or_default();
            info_traffic_msg
                .hosts
                .entry(host)
                .and_modify(|data_info_host| {
                    data_info_host.data_info.add_packets(
                        exchanged_packets,
                        exchanged_bytes,
                        traffic_direction,
                        Instant::now(),
                    );
                })
                .or_insert_with(|| {
                    let traffic_type =
                        get_traffic_type(&address_to_lookup, exporter_addresses, traffic_direction);
                    let is_loopback = address_to_lookup.is_loopback();
                    let is_local = is_local_connection(&address_to_lookup, exporter_addresses);
                    let is_bogon = is_bogon(&address_to_lookup);
                    let mut data_info = DataInfo::default();
                    data_info.add_packets(
                        exchanged_packets,
                        exchanged_bytes,
                        traffic_direction,
                        Instant::now(),
                    );
                    DataInfoHost {
                        data_info,
                        is_loopback,
                        is_local,
                        is_bogon,
                        traffic_type,
                    }
                });
        }
    }

    info_traffic_msg
        .services
        .entry(service)
        .and_modify(|data_info| {
            data_info.add_packets(
                exchanged_packets,
                exchanged_bytes,
                traffic_direction,
                Instant::now(),
            );
        })
        .or_insert_with(|| {
            let mut data_info = DataInfo::default();
            data_info.add_packets(
                exchanged_packets,
                exchanged_bytes,
                traffic_direction,
                Instant::now(),
            );
            data_info
        });
}

fn build_key(record: &FlowRecord) -> Option<AddressPortPair> {
    let src = record.src_ip?;
    let dst = record.dst_ip?;
    let proto = match record.protocol {
        Some(6) => Protocol::TCP,
        Some(17) => Protocol::UDP,
        Some(1 | 58) => Protocol::ICMP, // ICMP / ICMPv6
        _ => return None,
    };
    let sport = match proto {
        Protocol::TCP | Protocol::UDP => record.src_port,
        _ => None,
    };
    let dport = match proto {
        Protocol::TCP | Protocol::UDP => record.dst_port,
        _ => None,
    };
    Some(AddressPortPair {
        source: src,
        sport,
        dest: dst,
        dport,
        protocol: proto,
    })
}

/// Build a `[Address]` slice carrying just the exporter's IP, so direction
/// classification treats the exporter as the local anchor.
fn exporter_as_addresses(peer: IpAddr) -> Vec<Address> {
    if peer.is_loopback() || peer.is_unspecified() {
        return vec![];
    }

    vec![Address {
        addr: peer,
        netmask: None,
        broadcast_addr: None,
        dst_addr: None,
    }]
}
