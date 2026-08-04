//! IPFIX collector runtime — binds a UDP socket, decodes incoming IPFIX
//! datagrams, and projects flow records into the same `InfoTraffic` shape the
//! pcap pipeline produces.

use async_channel::Sender;
use pcap::Address;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Instant;
use tokio::sync::broadcast::Receiver;

use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::ipfix::templates::TemplateCache;
use crate::networking::ipfix::wire::{
    self, FlowRecord, IPFIX_VERSION, Set, decode_data_record, format_mac, parse_message,
};
use crate::networking::manage_packets::{account_flow, modify_or_insert_in_map};
use crate::networking::parse_packets::{
    AddressesResolutionState, BackendTrafficMessage, maybe_send_tick, spawn_reverse_dns_pool,
};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::networking::types::protocol::Protocol;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::timestamp::Timestamp;

/// Buffer size for a single UDP datagram. RFC 7011 §10.3.1 recommends at least
/// 1500; we size larger to accommodate jumbo-framed exporters.
const RECV_BUF_LEN: usize = 65_535;

/// Entry point for the IPFIX collector thread. Mirrors `parse_packets` in
/// terms of channel contracts: it emits `BackendTrafficMessage::TickRun` every
/// second with the accumulated `InfoTraffic`.
pub fn collect_ipfix(
    cap_id: usize,
    socket: UdpSocket,
    mmdb_readers: &MmdbReaders,
    ip_blacklist: &IpBlacklist,
    tx: &Sender<BackendTrafficMessage>,
    freeze_rxs: (Receiver<()>, Receiver<()>),
) {
    let (mut freeze_rx, _freeze_rx_2) = freeze_rxs;

    let mut info_traffic_msg = InfoTraffic::default();
    let mut templates = TemplateCache::new();
    let mut buf = vec![0u8; RECV_BUF_LEN];
    let mut first_packet_ticks: Option<Instant> = None;

    let mut resolutions_state = spawn_reverse_dns_pool(mmdb_readers);

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
    // A record with neither counter carries nothing to account for — an
    // exporter whose counters we can't read (see the total-counter note in
    // `wire.rs`) would otherwise contribute a phantom packet per record.
    if record.bytes == 0 && record.packets == 0 {
        return;
    }
    let exchanged_bytes = record.bytes;
    let exchanged_packets = record.packets;
    let mac_addresses = (
        record.src_mac.map(format_mac),
        record.dst_mac.map(format_mac),
    );

    let timestamps_hint = record.flow_start.zip(record.flow_end);
    let (traffic_direction, service) = modify_or_insert_in_map(
        info_traffic_msg,
        &key,
        exporter_addresses,
        mac_addresses,
        None,
        ArpType::default(),
        exchanged_bytes,
        exchanged_packets,
        ip_blacklist,
        record.direction,
        timestamps_hint,
    );

    account_flow(
        info_traffic_msg,
        resolutions_state,
        &key,
        exporter_addresses,
        exchanged_bytes,
        exchanged_packets,
        traffic_direction,
        service,
    );
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

/// Build a `[Address]` slice carrying just the exporter's IP, so host
/// classification treats the exporter as the local anchor. Flow direction
/// itself comes from IE 61 when the exporter sends it; this is only the
/// fallback for exporters that report `undefined`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::types::data_representation::DataRepr;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use std::net::{Ipv4Addr, Ipv6Addr};

    /// The field specifiers `sniffnet-agent` puts in its templates, after the
    /// two address fields that differ between the IPv4 and IPv6 variants.
    const AGENT_COMMON_FIELDS: [(u16, u16); 10] = [
        (7, 2),
        (11, 2),
        (4, 1),
        (56, 6),
        (80, 6),
        (61, 1),
        (352, 8),
        (2, 8),
        (152, 8),
        (153, 8),
    ];

    fn agent_fields(addr_fields: [(u16, u16); 2]) -> Vec<(u16, u16)> {
        addr_fields.into_iter().chain(AGENT_COMMON_FIELDS).collect()
    }

    fn set(set_id: u16, body: &[u8]) -> Vec<u8> {
        let mut out = set_id.to_be_bytes().to_vec();
        out.extend_from_slice(&u16::try_from(body.len() + 4).unwrap().to_be_bytes());
        out.extend_from_slice(body);
        out
    }

    fn template_set(template_id: u16, fields: &[(u16, u16)]) -> Vec<u8> {
        let mut body = template_id.to_be_bytes().to_vec();
        body.extend_from_slice(&u16::try_from(fields.len()).unwrap().to_be_bytes());
        for (ie, len) in fields {
            body.extend_from_slice(&ie.to_be_bytes());
            body.extend_from_slice(&len.to_be_bytes());
        }
        set(wire::SET_ID_TEMPLATE, &body)
    }

    /// Message header plus the given sets, with the length backfilled.
    fn datagram(sets: &[Vec<u8>]) -> Vec<u8> {
        let mut out = IPFIX_VERSION.to_be_bytes().to_vec();
        out.extend_from_slice(&[0, 0]); // length placeholder
        out.extend_from_slice(&[0; 4]); // export time
        out.extend_from_slice(&[0; 4]); // sequence number
        out.extend_from_slice(&[0; 4]); // observation domain
        for s in sets {
            out.extend_from_slice(s);
        }
        let len = u16::try_from(out.len()).unwrap().to_be_bytes();
        out[2] = len[0];
        out[3] = len[1];
        out
    }

    /// Record tail shared by both address families, in the agent's field order.
    fn record_tail(bytes: u64, packets: u64) -> Vec<u8> {
        let mut r = Vec::new();
        r.extend_from_slice(&443u16.to_be_bytes()); // source port
        r.extend_from_slice(&50_000u16.to_be_bytes()); // destination port
        r.push(6); // TCP
        r.extend_from_slice(&[0xAA; 6]); // source MAC
        r.extend_from_slice(&[0; 6]); // destination MAC: not observed
        r.push(0x00); // flowDirection: ingress
        r.extend_from_slice(&bytes.to_be_bytes());
        r.extend_from_slice(&packets.to_be_bytes());
        r.extend_from_slice(&20_000u64.to_be_bytes()); // flow start: 20s
        r.extend_from_slice(&25_000u64.to_be_bytes()); // flow end: 25s
        r
    }

    fn run(bytes: &[u8]) -> (InfoTraffic, AddressesResolutionState) {
        let mut templates = TemplateCache::new();
        let mut info = InfoTraffic::default();
        let mut resolutions = AddressesResolutionState::new_detached();
        process_datagram(
            bytes,
            "203.0.113.9:4739".parse().unwrap(),
            &mut templates,
            &mut info,
            &IpBlacklist::default(),
            &mut resolutions,
        );
        (info, resolutions)
    }

    /// A template set plus a one-record data set, shaped exactly as the agent
    /// emits them.
    fn agent_datagram(
        template_id: u16,
        addr_fields: [(u16, u16); 2],
        addrs: &[u8],
        bytes: u64,
        packets: u64,
    ) -> Vec<u8> {
        let mut record = addrs.to_vec();
        record.extend_from_slice(&record_tail(bytes, packets));
        datagram(&[
            template_set(template_id, &agent_fields(addr_fields)),
            set(template_id, &record),
        ])
    }

    #[test]
    fn decodes_an_agent_shaped_ipv4_datagram() {
        let mut addrs = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        addrs.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());
        let (info, resolutions) = run(&agent_datagram(256, [(8, 4), (12, 4)], &addrs, 1500, 10));

        let key = AddressPortPair {
            source: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            sport: Some(443),
            dest: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            dport: Some(50_000),
            protocol: Protocol::TCP,
        };
        let entry = info.map.get(&key).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 1500);
        assert_eq!(entry.transmitted_packets, 10);
        // flowDirection 0x00 is ingress, and it overrides any address guess
        assert_eq!(entry.traffic_direction, TrafficDirection::Incoming);
        assert_eq!(entry.mac_address1, Some("aa:aa:aa:aa:aa:aa".to_string()));
        assert_eq!(entry.mac_address2, None, "all-zero MAC means not observed");
        assert_eq!(entry.initial_timestamp, Timestamp::new(20, 0));
        assert_eq!(entry.final_timestamp, Timestamp::new(25, 0));

        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 1500);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 10);
        assert_eq!(info.services.len(), 1);
        // no rDNS threads are running, so the address is left awaiting lookup
        assert_eq!(resolutions.addresses_waiting_resolution.len(), 1);
        assert!(info.hosts.is_empty());
    }

    #[test]
    fn decodes_an_agent_shaped_ipv6_datagram() {
        let src = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let dst = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2);
        let mut addrs = src.octets().to_vec();
        addrs.extend_from_slice(&dst.octets());
        let (info, _) = run(&agent_datagram(257, [(27, 16), (28, 16)], &addrs, 800, 4));

        let key = AddressPortPair {
            source: IpAddr::V6(src),
            sport: Some(443),
            dest: IpAddr::V6(dst),
            dport: Some(50_000),
            protocol: Protocol::TCP,
        };
        let entry = info.map.get(&key).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 800);
        assert_eq!(entry.transmitted_packets, 4);
    }

    #[test]
    fn record_without_counters_is_skipped() {
        let mut addrs = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        addrs.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());
        let (info, _) = run(&agent_datagram(256, [(8, 4), (12, 4)], &addrs, 0, 0));

        assert!(info.map.is_empty(), "no traffic to account for");
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 0);
    }

    #[test]
    fn data_set_without_a_known_template_is_skipped() {
        // Data referencing template 256 before any template set has arrived.
        let bytes = datagram(&[set(256, &[0xAA; 58])]);
        let (info, _) = run(&bytes);
        assert!(info.map.is_empty());
    }

    #[test]
    fn trailing_padding_does_not_produce_an_extra_record() {
        let mut addrs = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        addrs.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());
        let mut record = addrs;
        record.extend_from_slice(&record_tail(1500, 10));
        record.extend_from_slice(&[0; 3]); // pad to a 4-byte boundary

        let bytes = datagram(&[
            template_set(256, &agent_fields([(8, 4), (12, 4)])),
            set(256, &record),
        ]);
        let (info, _) = run(&bytes);

        assert_eq!(info.map.len(), 1);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 10);
    }
}
