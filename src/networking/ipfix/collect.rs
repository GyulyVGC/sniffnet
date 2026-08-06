//! IPFIX collector runtime — binds a UDP socket, decodes incoming IPFIX
//! datagrams, and projects flow records into the same `InfoTraffic` shape the
//! pcap pipeline produces.

use async_channel::Sender;
use pcap::Address;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use tokio::sync::broadcast::Receiver;

use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::ipfix::templates::TemplateCache;
use crate::networking::ipfix::totals::TotalsCache;
use crate::networking::ipfix::wire::{
    self, FlowRecord, Set, decode_data_record, format_mac, parse_message,
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

/// Per-exporter state the collector carries between datagrams.
struct CollectorState {
    templates: TemplateCache,
    totals: TotalsCache,
}

impl CollectorState {
    fn new(now: Instant) -> Self {
        Self {
            templates: TemplateCache::new(now),
            totals: TotalsCache::new(now),
        }
    }
}

/// Entry point for the IPFIX collector thread. Mirrors `parse_packets` in
/// terms of channel contracts: it emits `BackendTrafficMessage::TickRun` every
/// second with the accumulated `InfoTraffic`.
pub fn collect_ipfix(
    cap_id: usize,
    socket: &UdpSocket,
    mmdb_readers: &MmdbReaders,
    ip_blacklist: &IpBlacklist,
    tx: &Sender<BackendTrafficMessage>,
    freeze_rxs: (Receiver<()>, Receiver<()>),
) {
    let (mut freeze_rx, _freeze_rx_2) = freeze_rxs;

    let mut info_traffic_msg = InfoTraffic::default();
    let mut state = CollectorState::new(Instant::now());
    let mut buf = vec![0u8; RECV_BUF_LEN];
    let mut first_packet_ticks: Option<Instant> = None;
    // the GUI only needs telling once: a misconfigured exporter keeps sending at
    // its normal rate, and every rejection says exactly the same thing
    let mut rejection_reported = false;

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
                let rejected = process_datagram(
                    &buf[..len],
                    peer,
                    &mut state,
                    &mut info_traffic_msg,
                    ip_blacklist,
                    &mut resolutions_state,
                );
                if rejected && !rejection_reported {
                    rejection_reported = true;
                    let _ = tx.send_blocking(BackendTrafficMessage::IpfixRejection(cap_id));
                }
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

/// Returns whether the whole datagram was thrown away because it isn't
/// decodable as IPFIX — which means the exporter is misconfigured rather than
/// merely chatty, so it's worth surfacing to the user.
fn process_datagram(
    bytes: &[u8],
    peer: SocketAddr,
    state: &mut CollectorState,
    info_traffic_msg: &mut InfoTraffic,
    ip_blacklist: &IpBlacklist,
    resolutions_state: &mut AddressesResolutionState,
) -> bool {
    let Ok((_, message)) = parse_message(bytes) else {
        return true;
    };

    let now = Instant::now();

    // First pass: register all templates so later data sets in the same
    // datagram can reference them.
    for set in &message.sets {
        if let Set::Template(records) = set {
            for record in records {
                state.templates.insert(
                    peer,
                    message.header.observation_domain_id,
                    record.template_id,
                    record.fields.clone(),
                    now,
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
                state
                    .templates
                    .get(peer, message.header.observation_domain_id, template_id, now)
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
                    peer,
                    message.header.observation_domain_id,
                    &mut state.totals,
                    now,
                    info_traffic_msg,
                    ip_blacklist,
                    resolutions_state,
                );
            }
        }
    }

    false
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

#[allow(clippy::too_many_arguments)]
fn ingest_flow_record(
    record: &FlowRecord,
    peer: SocketAddr,
    observation_domain_id: u32,
    totals: &mut TotalsCache,
    now: Instant,
    info_traffic_msg: &mut InfoTraffic,
    ip_blacklist: &IpBlacklist,
    resolutions_state: &mut AddressesResolutionState,
) {
    let Some(key) = build_key(record) else {
        return;
    };

    let (exchanged_bytes, exchanged_packets) =
        resolve_counters(record, peer, observation_domain_id, &key, totals, now);
    // Both counters have to be there for the record to be worth accounting: a
    // flow with no bytes has nothing to report, and one with no packets would
    // add bytes that never show up in the packet totals the rest of the
    // application counts by.
    if exchanged_bytes == 0 || exchanged_packets == 0 {
        return;
    }

    let mac_addresses = (
        record.src_mac.map(format_mac),
        record.dst_mac.map(format_mac),
    );

    let timestamps_hint = record.flow_start.zip(record.flow_end);
    let (traffic_direction, service) = modify_or_insert_in_map(
        info_traffic_msg,
        &key,
        NO_INTERFACE_ADDRESSES,
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
        NO_INTERFACE_ADDRESSES,
        exchanged_bytes,
        exchanged_packets,
        traffic_direction,
        service,
    );
}

/// Work out how much traffic this record actually adds.
///
/// Delta counters are already increments, so they're used as they stand.
/// Cumulative counters have to be differenced against the flow's previous
/// report. The totals are handed to the cache either way, so that a template
/// carrying both kinds keeps the baseline current for the records that need it.
fn resolve_counters(
    record: &FlowRecord,
    peer: SocketAddr,
    observation_domain_id: u32,
    key: &AddressPortPair,
    totals: &mut TotalsCache,
    now: Instant,
) -> (u128, u128) {
    let (bytes_from_totals, packets_from_totals) = totals.delta(
        peer,
        observation_domain_id,
        key,
        record.bytes_total,
        record.packets_total,
        now,
    );

    let bytes = if record.bytes > 0 {
        record.bytes
    } else {
        bytes_from_totals
    };
    let packets = if record.packets > 0 {
        record.packets
    } else {
        packets_from_totals
    };

    (bytes, packets)
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

/// Flows are observed somewhere else entirely, so there is no local interface
/// to classify them against — the exporter's own IP is no help either, since a
/// router exports flows between hosts that are both remote to it.
///
/// Passing no addresses is what PCAP import does, and it makes the downstream
/// classifiers fall back to their bogon heuristic. Flow direction proper comes
/// from IE 61 whenever the exporter sends it, which overrides the heuristic.
const NO_INTERFACE_ADDRESSES: &[Address] = &[];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::ipfix::wire::IPFIX_VERSION;
    use crate::networking::types::data_representation::DataRepr;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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
        let (info, resolutions, _) = run_all(&[bytes]);
        (info, resolutions)
    }

    /// Feed a sequence of datagrams to one collector, so state that spans
    /// datagrams (templates, counter baselines) behaves as it would live. The
    /// last element reports whether every datagram was rejected.
    fn run_all(datagrams: &[&[u8]]) -> (InfoTraffic, AddressesResolutionState, bool) {
        let mut state = CollectorState::new(Instant::now());
        let mut info = InfoTraffic::default();
        let mut resolutions = AddressesResolutionState::new_detached();
        let mut all_rejected = true;
        for bytes in datagrams {
            all_rejected &= process_datagram(
                bytes,
                "203.0.113.9:4739".parse().unwrap(),
                &mut state,
                &mut info,
                &IpBlacklist::default(),
                &mut resolutions,
            );
        }
        (info, resolutions, all_rejected)
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
    fn a_record_with_either_counter_at_zero_is_skipped() {
        let mut addrs = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        addrs.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());

        // Bytes without packets would grow the byte totals while leaving the
        // packet ones at zero, which the rest of the application counts by.
        for (bytes, packets) in [(0, 0), (1500, 0), (0, 10)] {
            let (info, _) = run(&agent_datagram(
                256,
                [(8, 4), (12, 4)],
                &addrs,
                bytes,
                packets,
            ));

            assert!(info.map.is_empty(), "no traffic to account for");
            assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 0);
            assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 0);
        }
    }

    /// A template in the shape exporters that only report cumulative counters
    /// use: no delta IEs, no flowDirection.
    const TOTALS_FIELDS: [(u16, u16); 7] = [
        (8, 4),
        (12, 4),
        (7, 2),
        (11, 2),
        (4, 1),
        (85, 8), // octetTotalCount
        (86, 8), // packetTotalCount
    ];

    fn totals_record(bytes: u64, packets: u64) -> Vec<u8> {
        let mut r = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        r.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());
        r.extend_from_slice(&443u16.to_be_bytes());
        r.extend_from_slice(&50_000u16.to_be_bytes());
        r.push(6); // TCP
        r.extend_from_slice(&bytes.to_be_bytes());
        r.extend_from_slice(&packets.to_be_bytes());
        r
    }

    fn totals_key() -> AddressPortPair {
        AddressPortPair {
            source: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            sport: Some(443),
            dest: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            dport: Some(50_000),
            protocol: Protocol::TCP,
        }
    }

    #[test]
    fn a_lone_totals_record_is_accounted_in_full() {
        // The single-record-per-flow-at-expiry case: the total is the flow.
        let bytes = datagram(&[
            template_set(300, &TOTALS_FIELDS),
            set(300, &totals_record(1500, 10)),
        ]);
        let (info, _, _) = run_all(&[&bytes]);

        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 1500);
        assert_eq!(entry.transmitted_packets, 10);
    }

    #[test]
    fn repeated_totals_are_differenced_not_re_added() {
        let first = datagram(&[
            template_set(300, &TOTALS_FIELDS),
            set(300, &totals_record(1500, 10)),
        ]);
        let grown = datagram(&[set(300, &totals_record(4000, 25))]);
        let unchanged = datagram(&[set(300, &totals_record(4000, 25))]);
        let (info, _, _) = run_all(&[&first, &grown, &unchanged]);

        // 1500 + 2500 + 0 — not 1500 + 4000 + 4000.
        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 4000);
        assert_eq!(entry.transmitted_packets, 25);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 4000);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 25);
    }

    #[test]
    fn deltas_are_preferred_when_a_template_carries_both() {
        // layer2OctetDeltaCount + packetDeltaCount alongside the totals: the
        // deltas are already increments, so they're what gets accounted.
        let fields = [
            (8, 4),
            (12, 4),
            (7, 2),
            (11, 2),
            (4, 1),
            (352, 8), // layer2OctetDeltaCount
            (2, 8),   // packetDeltaCount
            (85, 8),  // octetTotalCount
            (86, 8),  // packetTotalCount
        ];
        let record =
            |delta_bytes: u64, delta_packets: u64, total_bytes: u64, total_packets: u64| {
                let mut r = totals_record(delta_bytes, delta_packets);
                r.extend_from_slice(&total_bytes.to_be_bytes());
                r.extend_from_slice(&total_packets.to_be_bytes());
                r
            };

        let first = datagram(&[
            template_set(300, &fields),
            set(300, &record(600, 4, 600, 4)),
        ]);
        let second = datagram(&[set(300, &record(900, 6, 1500, 10))]);
        let (info, _, _) = run_all(&[&first, &second]);

        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 1500);
        assert_eq!(entry.transmitted_packets, 10);
    }

    #[test]
    fn direction_falls_back_to_the_bogon_heuristic_without_ie_61() {
        // No flowDirection in this template and no interface addresses to
        // compare against, so the private source has to carry the decision —
        // the same way PCAP import classifies it.
        let bytes = datagram(&[
            template_set(300, &TOTALS_FIELDS),
            set(300, &totals_record(1500, 10)),
        ]);
        let (info, _, _) = run_all(&[&bytes]);

        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.traffic_direction, TrafficDirection::Outgoing);
    }

    #[test]
    fn a_netflow_v9_exporter_is_rejected() {
        // A v9 header, which is a different shape entirely: the version check
        // in `parse_message_header` is what turns it into a rejection.
        let mut v9 = 9u16.to_be_bytes().to_vec();
        v9.extend_from_slice(&[0; 18]);
        let (info, _, rejected) = run_all(&[&v9, &v9, &v9]);

        assert!(info.map.is_empty());
        assert!(rejected);
    }

    #[test]
    fn an_undecodable_datagram_is_rejected_without_panicking() {
        // Claims a 200-byte message but carries only the header: exactly the
        // kind of input that must never take the application down.
        let truncated = vec![0x00, 0x0A, 0x00, 0xC8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let (info, _, rejected) = run_all(&[&truncated, &truncated]);

        assert!(info.map.is_empty());
        assert!(rejected);
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
