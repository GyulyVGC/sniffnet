//! IPFIX collector runtime

use async_channel::Sender;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use tokio::sync::broadcast::Receiver;

use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::capture::{
    AddressesResolutionState, BackendTrafficMessage, maybe_send_tick, spawn_reverse_dns_pool,
};
use crate::networking::ipfix::baseline_cache::BaselineCache;
use crate::networking::ipfix::flow_record::FlowRecord;
use crate::networking::ipfix::template_cache::TemplateCache;
use crate::networking::ipfix::wire::{Set, decode_data_record, parse_message};
use crate::networking::manage_packets::{modify_or_insert_in_map, update_connection_stats};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::utils::types::timestamp::Timestamp;

/// Buffer size for a single UDP datagram
const RECV_BUF_LEN: usize = 65_535;

/// Max number of datagrams discarded when resuming a capture (queued on the socket while paused)
const MAX_RESUME_DRAIN: usize = 10_000;

/// State the collector needs to carry across datagrams: registered templates and cumulative counters
struct CollectorState {
    templates: TemplateCache,
    baselines: BaselineCache,
}

impl CollectorState {
    fn new(now: Instant) -> Self {
        Self {
            templates: TemplateCache::new(now),
            baselines: BaselineCache::new(now),
        }
    }
}

/// Entry point for the IPFIX collector thread (IPFIX mirror of PCAP's `parse_packets`)
pub(crate) fn collect_ipfix(
    cap_id: usize,
    socket: &UdpSocket,
    mmdb_readers: &MmdbReaders,
    ip_blacklist: &IpBlacklist,
    tx: &Sender<BackendTrafficMessage>,
    freeze_rxs: (Receiver<()>, Receiver<()>),
) {
    let (mut freeze_rx, _) = freeze_rxs;

    let mut info_traffic_msg = InfoTraffic::default();

    let mut resolutions_state = spawn_reverse_dns_pool(mmdb_readers);

    // instant of the first parsed packet plus multiples of 1 second
    let mut first_packet_ticks = None;

    // whether we've already reported to the GUI that a datagram was rejected as undecodable
    let mut rejection_reported = false;
    let mut state = CollectorState::new(Instant::now());
    let mut buf = vec![0u8; RECV_BUF_LEN];

    loop {
        // check if we need to freeze the parsing
        if freeze_rx.try_recv().is_ok() {
            // wait until unfreeze
            let _ = freeze_rx.blocking_recv();
            // discard whatever the socket queued while we were frozen
            if socket.set_nonblocking(true).is_ok() {
                let mut left = MAX_RESUME_DRAIN;
                while left > 0 && socket.recv_from(&mut buf).is_ok() {
                    left -= 1;
                }
                let _ = socket.set_nonblocking(false);
            }
            // reset the first packet ticks
            first_packet_ticks = Some(Instant::now());
        }

        let recv_res = socket.recv_from(&mut buf);

        if tx.is_closed() {
            return;
        }

        maybe_send_tick(
            cap_id,
            &mut info_traffic_msg,
            &mut first_packet_ticks,
            tx,
            &mut resolutions_state,
        );

        if let Ok((len, peer)) = recv_res {
            let Some(datagram) = buf.get(..len) else {
                continue;
            };

            if first_packet_ticks.is_none() {
                first_packet_ticks = Some(Instant::now());
            }

            info_traffic_msg.last_packet_timestamp = current_timestamp();

            let success = process_datagram(
                datagram,
                peer,
                &mut state,
                &mut info_traffic_msg,
                ip_blacklist,
                &mut resolutions_state,
            );

            if !success && !rejection_reported {
                rejection_reported = true;
                let _ = tx.send_blocking(BackendTrafficMessage::IpfixUndecodable(cap_id));
            }
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

/// Process the whole datagram and returns whether the parsing succeeded
fn process_datagram(
    bytes: &[u8],
    peer: SocketAddr,
    state: &mut CollectorState,
    info_traffic_msg: &mut InfoTraffic,
    ip_blacklist: &IpBlacklist,
    resolutions_state: &mut AddressesResolutionState,
) -> bool {
    let Ok((_, message)) = parse_message(bytes) else {
        return false;
    };

    let now = Instant::now();
    let od_id = message.header.observation_domain_id;

    // first pass: parse templates so that later data sets in this datagram can reference them
    for set in &message.sets {
        if let Set::Template(records) = set {
            for record in records {
                state
                    .templates
                    .insert(peer, od_id, record.template_id, record.fields.clone(), now);
            }
        }
    }

    // second pass: parse data sets and populate InfoTraffic
    for set in &message.sets {
        if let Set::Data {
            template_id,
            payload,
        } = *set
        {
            let Some(template) = state.templates.get(peer, od_id, template_id, now) else {
                // no such template seen
                continue;
            };
            let mut remaining = payload;
            while !remaining.is_empty() {
                let Ok((rest, record)) = decode_data_record(template, remaining) else {
                    break;
                };
                if rest.len() >= remaining.len() {
                    // no progress: guard against infinite loops
                    break;
                }
                remaining = rest;
                // a biflow is accounted as two records
                let reverse = record.get_reverse_record();
                for flow in [Some(record), reverse].into_iter().flatten() {
                    ingest_flow_record(
                        &flow,
                        peer,
                        od_id,
                        &mut state.baselines,
                        now,
                        info_traffic_msg,
                        ip_blacklist,
                        resolutions_state,
                    );
                }
            }
        }
    }

    true
}

#[allow(clippy::too_many_arguments)]
fn ingest_flow_record(
    record: &FlowRecord,
    peer: SocketAddr,
    observation_domain_id: u32,
    baselines: &mut BaselineCache,
    now: Instant,
    info_traffic_msg: &mut InfoTraffic,
    ip_blacklist: &IpBlacklist,
    resolutions_state: &mut AddressesResolutionState,
) {
    let Some(key) = record.get_key() else {
        return;
    };

    let (exchanged_bytes, exchanged_packets) =
        resolve_counters(record, peer, observation_domain_id, &key, baselines, now);
    // Both counters have to be there for the record to be worth accounting: a
    // flow with no bytes has nothing to report, and one with no packets would
    // add bytes that never show up in the packet totals the rest of the
    // application counts by.
    if exchanged_bytes == 0 || exchanged_packets == 0 {
        return;
    }

    let mac_addresses = (record.src_mac, record.dst_mac);

    let timestamps_hint = record.flow_start.zip(record.flow_end);
    let (traffic_direction, service) = modify_or_insert_in_map(
        info_traffic_msg,
        &key,
        &[],
        mac_addresses,
        None,
        ArpType::default(),
        exchanged_packets,
        exchanged_bytes,
        ip_blacklist,
        record.direction,
        timestamps_hint,
    );

    update_connection_stats(
        info_traffic_msg,
        resolutions_state,
        &key,
        &[],
        exchanged_packets,
        exchanged_bytes,
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
    baselines: &mut BaselineCache,
    now: Instant,
) -> (u128, u128) {
    let (bytes_from_baseline, packets_from_baseline) = baselines.delta(
        peer,
        observation_domain_id,
        key,
        record.bytes_total,
        record.packets_total,
        now,
    );

    let bytes = if let Some(bytes_delta) = record.bytes_delta {
        bytes_delta
    } else {
        bytes_from_baseline
    };
    let packets = if let Some(packets_delta) = record.packets_delta {
        packets_delta
    } else {
        packets_from_baseline
    };

    (bytes, packets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::ipfix::flow_record::ReverseCounters;
    use crate::networking::ipfix::wire::{self, IPFIX_VERSION};
    use crate::networking::types::data_representation::DataRepr;
    use crate::networking::types::protocol::Protocol;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    /// The exporter every test collects from
    const PEER: &str = "203.0.113.9:4739";

    fn peer() -> SocketAddr {
        PEER.parse().unwrap()
    }

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

    /// The same fields with the forward counters doubled up under `pen`
    fn totals_fields_with_pen(pen: u32) -> Vec<(u16, u16, Option<u32>)> {
        let mut fields: Vec<_> = TOTALS_FIELDS[..5]
            .iter()
            .map(|(ie, len)| (*ie, *len, None))
            .collect();
        fields.extend([
            (1, 8, None),
            (2, 8, None),
            (1, 8, Some(pen)),
            (2, 8, Some(pen)),
        ]);
        fields
    }

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
        let fields: Vec<_> = fields.iter().map(|(ie, len)| (*ie, *len, None)).collect();
        template_set_with_pens(template_id, &fields)
    }

    /// A template whose fields are `(ie, length, enterprise)` triples; an
    /// enterprise field sets the high bit of the ie id and appends the PEN.
    fn template_set_with_pens(template_id: u16, fields: &[(u16, u16, Option<u32>)]) -> Vec<u8> {
        let mut body = template_id.to_be_bytes().to_vec();
        body.extend_from_slice(&u16::try_from(fields.len()).unwrap().to_be_bytes());
        for (ie, len, pen) in fields {
            let raw_ie = if pen.is_some() { ie | 0x8000 } else { *ie };
            body.extend_from_slice(&raw_ie.to_be_bytes());
            body.extend_from_slice(&len.to_be_bytes());
            if let Some(pen) = pen {
                body.extend_from_slice(&pen.to_be_bytes());
            }
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

    /// A record for `TOTALS_FIELDS`, or the head of any template that starts
    /// with the same 5-tuple fields.
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

    /// The flow every record built here belongs to
    fn totals_key() -> AddressPortPair {
        AddressPortPair {
            source: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            sport: Some(443),
            dest: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            dport: Some(50_000),
            protocol: Protocol::TCP,
        }
    }

    /// `totals_key`'s flow as an already-decoded record
    fn flow_record(bytes_delta: Option<u128>, packets_delta: Option<u128>) -> FlowRecord {
        FlowRecord {
            src_ip: Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
            dst_ip: Some(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))),
            src_port: Some(443),
            dst_port: Some(50_000),
            protocol: Some(Protocol::TCP),
            bytes_delta,
            packets_delta,
            ..FlowRecord::default()
        }
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

    fn run(bytes: &[u8]) -> (InfoTraffic, AddressesResolutionState) {
        let (info, resolutions, _) = run_all(&[bytes]);
        (info, resolutions)
    }

    /// Feed a sequence of datagrams to one collector, so state that spans
    /// datagrams (templates, counter baselines) behaves as it would live. The
    /// last element reports whether every datagram succeeded.
    fn run_all(datagrams: &[&[u8]]) -> (InfoTraffic, AddressesResolutionState, bool) {
        let mut state = CollectorState::new(Instant::now());
        let mut info = InfoTraffic::default();
        let mut resolutions = AddressesResolutionState::new_for_tests();
        let mut all_succeeded = true;
        for bytes in datagrams {
            all_succeeded &= process_datagram(
                bytes,
                peer(),
                &mut state,
                &mut info,
                &IpBlacklist::default(),
                &mut resolutions,
            );
        }
        (info, resolutions, all_succeeded)
    }

    #[test]
    fn test_current_timestamp() {
        let seconds_now = || {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        };

        let before = i64::try_from(seconds_now()).unwrap();
        let stamped = current_timestamp();
        let after = i64::try_from(seconds_now()).unwrap();

        // the wall clock the rest of the application reads flows by, not an uptime
        assert!(
            (before..=after).contains(&stamped.secs()),
            "{stamped:?} outside {before}..={after}"
        );
        assert!((0..1_000_000).contains(&(stamped.to_usecs().unwrap() % 1_000_000)));
    }

    #[test]
    fn test_process_datagram() {
        // an agent-shaped datagram: a template set, then a data set against it
        let mut addrs = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        addrs.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());
        let (info, resolutions, succeeded) =
            run_all(&[&agent_datagram(256, [(8, 4), (12, 4)], &addrs, 1500, 10)]);
        assert!(succeeded);

        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 1500);
        assert_eq!(entry.transmitted_packets, 10);
        // flowDirection 0x00 is ingress, and it overrides any address guess
        assert_eq!(entry.traffic_direction, TrafficDirection::Incoming);
        assert_eq!(entry.mac_address1, Some([0xAA; 6]));
        assert_eq!(entry.mac_address2, None, "all-zero MAC means not observed");
        assert_eq!(entry.initial_timestamp, Timestamp::new(20, 0));
        assert_eq!(entry.final_timestamp, Timestamp::new(25, 0));

        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 1500);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 10);
        assert_eq!(info.services.len(), 1);
        // no rDNS threads are running, so the address is left awaiting lookup
        assert_eq!(resolutions.addresses_waiting_resolution.len(), 1);
        assert!(info.hosts.is_empty());

        // a data set holds its records back to back, and may precede the
        // template it references: templates are registered in a first pass
        let mut records = addrs.clone();
        records.extend_from_slice(&record_tail(1500, 10));
        records.extend_from_slice(&Ipv4Addr::new(10, 0, 0, 2).octets());
        records.extend_from_slice(&Ipv4Addr::new(8, 8, 4, 4).octets());
        records.extend_from_slice(&record_tail(600, 4));
        let bytes = datagram(&[
            set(256, &records),
            template_set(256, &agent_fields([(8, 4), (12, 4)])),
        ]);
        let (info, _, _) = run_all(&[&bytes]);

        assert_eq!(info.map.len(), 2, "both records are accounted");
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 2100);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 14);

        // a template only has to arrive once: later datagrams may carry data alone
        let mut record = addrs;
        record.extend_from_slice(&record_tail(800, 5));
        let (info, _, _) = run_all(&[&bytes, &datagram(&[set(256, &record)])]);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 19);
    }

    #[test]
    fn test_process_datagram_ipv6() {
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
    fn test_process_datagram_biflow() {
        // what YAF sends: forward counters, then the same IEs under PEN 29305
        let fields = totals_fields_with_pen(wire::REVERSE_PEN);
        let biflow = |reverse_bytes: u64, reverse_packets: u64| {
            let mut record = totals_record(1500, 10);
            record.extend_from_slice(&reverse_bytes.to_be_bytes());
            record.extend_from_slice(&reverse_packets.to_be_bytes());
            datagram(&[template_set_with_pens(300, &fields), set(300, &record)])
        };

        let (info, _, _) = run_all(&[&biflow(9000, 60)]);
        assert_eq!(info.map.len(), 2, "one entry per direction");

        let forward = info.map.get(&totals_key()).expect("forward flow present");
        assert_eq!(forward.transmitted_bytes, 1500);
        assert_eq!(forward.transmitted_packets, 10);

        // the reverse half is the same conversation with the tuple swapped
        let reverse_key = AddressPortPair {
            source: totals_key().dest,
            sport: totals_key().dport,
            dest: totals_key().source,
            dport: totals_key().sport,
            protocol: Protocol::TCP,
        };
        let reverse = info.map.get(&reverse_key).expect("reverse flow present");
        assert_eq!(reverse.transmitted_bytes, 9000);
        assert_eq!(reverse.transmitted_packets, 60);

        // ...and it travels the other way
        assert_eq!(forward.traffic_direction, TrafficDirection::Outgoing);
        assert_eq!(reverse.traffic_direction, TrafficDirection::Incoming);

        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 10_500);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 70);

        // a biflow template on a conversation that only ever went one way
        let (info, _, _) = run_all(&[&biflow(0, 0)]);
        assert_eq!(info.map.len(), 1, "the empty reverse half is dropped");
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 10);

        // the same shape under a vendor PEN is not a biflow at all: those
        // counters must not be read as a reverse direction
        let cisco = totals_fields_with_pen(9);
        let mut record = totals_record(1500, 10);
        record.extend_from_slice(&9000u64.to_be_bytes());
        record.extend_from_slice(&60u64.to_be_bytes());
        let bytes = datagram(&[template_set_with_pens(300, &cisco), set(300, &record)]);
        let (info, _, _) = run_all(&[&bytes]);

        assert_eq!(info.map.len(), 1);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 1500);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 10);
    }

    #[test]
    fn test_process_datagram_cumulative_counters() {
        // the single-record-per-flow-at-expiry case: the total is the flow
        let first = datagram(&[
            template_set(300, &TOTALS_FIELDS),
            set(300, &totals_record(1500, 10)),
        ]);
        let (info, _, _) = run_all(&[&first]);
        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 1500);
        assert_eq!(entry.transmitted_packets, 10);

        // a flow reported repeatedly is differenced against its own baseline,
        // which only holds because the collector keeps state between datagrams
        let grown = datagram(&[set(300, &totals_record(4000, 25))]);
        let unchanged = datagram(&[set(300, &totals_record(4000, 25))]);
        let (info, _, _) = run_all(&[&first, &grown, &unchanged]);

        // 1500 + 2500 + 0 — not 1500 + 4000 + 4000
        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 4000);
        assert_eq!(entry.transmitted_packets, 25);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 4000);
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 25);
    }

    #[test]
    fn test_process_datagram_rejects_undecodable() {
        // a NetFlow v9 header is a different shape entirely: the version check
        // in `parse_message_header` is what turns it into a rejection
        let mut v9 = 9u16.to_be_bytes().to_vec();
        v9.extend_from_slice(&[0; 18]);
        let (info, _, succeeded) = run_all(&[&v9, &v9, &v9]);
        assert!(info.map.is_empty());
        assert!(!succeeded);

        // claims a 200-byte message but carries only the header: exactly the
        // kind of input that must never take the application down
        let truncated = vec![0x00, 0x0A, 0x00, 0xC8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let (info, _, succeeded) = run_all(&[&truncated, &truncated]);
        assert!(info.map.is_empty());
        assert!(!succeeded);

        // an empty datagram
        assert!(!run_all(&[&[]]).2);
    }

    #[test]
    fn test_process_datagram_skips_unusable_sets() {
        // data referencing a template that hasn't arrived is skipped silently
        // per RFC 7011 §8 — the exporter is fine, we just can't read it yet
        let bytes = datagram(&[set(256, &[0xAA; 58])]);
        let (info, _, succeeded) = run_all(&[&bytes]);
        assert!(info.map.is_empty());
        assert!(succeeded, "an unknown template is not a rejection");

        // bytes left over below one record's worth are padding (RFC 7011 §3.3.1),
        // not the start of another record
        let mut record = Ipv4Addr::new(10, 0, 0, 1).octets().to_vec();
        record.extend_from_slice(&Ipv4Addr::new(8, 8, 8, 8).octets());
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

    #[test]
    fn test_data_set_record_loop_terminates() {
        // a variable-length field (RFC 7011 §7) makes the record size readable
        // only from the record itself, so the padding after it is whatever is
        // left below the smallest record the template admits
        let mut fields: Vec<(u16, u16)> = TOTALS_FIELDS.to_vec();
        fields.push((82, wire::VARIABLE_LENGTH)); // interfaceName
        let mut body = totals_record(1500, 10);
        body.push(0x00); // the variable-length field, empty
        body.extend_from_slice(&[0; 2]); // pad to a 4-byte boundary
        let bytes = datagram(&[template_set(256, &fields), set(256, &body)]);
        let (info, _, succeeded) = run_all(&[&bytes]);
        assert!(succeeded);
        assert_eq!(info.map.len(), 1, "the padding is not a second record");
        assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 10);

        // a record cut short mid-field is dropped, not partially accounted
        let bytes = datagram(&[
            template_set(257, &TOTALS_FIELDS),
            set(257, &totals_record(1500, 10)[..20]),
        ]);
        let (info, _, succeeded) = run_all(&[&bytes]);
        assert!(succeeded);
        assert!(info.map.is_empty());

        // templates that consume no bytes would otherwise be decoded forever
        for degenerate in [&[(8, 0)][..], &[]] {
            let bytes = datagram(&[
                template_set(258, degenerate),
                set(258, &totals_record(1500, 10)),
            ]);
            let (info, _, succeeded) = run_all(&[&bytes]);
            assert!(succeeded);
            assert!(info.map.is_empty());
        }
    }

    #[test]
    fn test_reverse_record() {
        // a uniflow has no other half
        assert_eq!(flow_record(Some(1500), Some(10)).get_reverse_record(), None);

        let mut record = flow_record(Some(1500), Some(10));
        record.bytes_total = Some(4000);
        record.packets_total = Some(25);
        record.src_mac = Some([0xAA; 6]);
        record.dst_mac = Some([0xBB; 6]);
        record.direction = Some(TrafficDirection::Incoming);
        record.flow_start = Some(Timestamp::new(20, 0));
        record.flow_end = Some(Timestamp::new(25, 0));
        record.reverse = Some(ReverseCounters {
            bytes_delta: Some(9000),
            packets_delta: Some(60),
            bytes_total: Some(12_000),
            packets_total: Some(80),
        });

        let reverse = record
            .get_reverse_record()
            .expect("a biflow has a reverse half");
        assert_eq!(
            reverse,
            FlowRecord {
                // the 5-tuple and the MAC addresses are swapped...
                src_ip: record.dst_ip,
                dst_ip: record.src_ip,
                src_port: Some(50_000),
                dst_port: Some(443),
                protocol: Some(Protocol::TCP),
                src_mac: Some([0xBB; 6]),
                dst_mac: Some([0xAA; 6]),
                // ...the reverse counters move into the forward slots, so that
                // everything downstream applies to this half unchanged...
                bytes_delta: Some(9000),
                packets_delta: Some(60),
                bytes_total: Some(12_000),
                packets_total: Some(80),
                // ...IE 61 describes the forward direction at the observation
                // point, so this half travels the other way...
                direction: Some(TrafficDirection::Outgoing),
                // ...and both halves share the conversation's lifetime
                flow_start: Some(Timestamp::new(20, 0)),
                flow_end: Some(Timestamp::new(25, 0)),
                // the reverse of the reverse would be the forward half again
                reverse: None,
            }
        );

        // a direction the exporter never sent can't be flipped into one
        record.direction = None;
        assert_eq!(record.get_reverse_record().unwrap().direction, None);
    }

    #[test]
    fn test_ingest_flow_record() {
        let ingest = |record: &FlowRecord| {
            let mut info = InfoTraffic::default();
            let mut resolutions = AddressesResolutionState::new_for_tests();
            let mut baselines = BaselineCache::new(Instant::now());
            ingest_flow_record(
                record,
                peer(),
                0,
                &mut baselines,
                Instant::now(),
                &mut info,
                &IpBlacklist::default(),
                &mut resolutions,
            );
            (info, resolutions)
        };

        // both counters have to be there for the record to be worth accounting:
        // bytes without packets would grow the byte totals while leaving the
        // packet ones the rest of the application counts by at zero
        for (bytes, packets) in [(0, 0), (1500, 0), (0, 10)] {
            let (info, _) = ingest(&flow_record(Some(bytes), Some(packets)));
            assert!(info.map.is_empty(), "{bytes} bytes / {packets} packets");
            assert_eq!(info.tot_data_info.tot_data(DataRepr::Bytes), 0);
            assert_eq!(info.tot_data_info.tot_data(DataRepr::Packets), 0);
        }

        // and so does a key: a record without a 5-tuple names no flow
        let mut keyless = flow_record(Some(1500), Some(10));
        keyless.protocol = None;
        assert!(ingest(&keyless).0.map.is_empty());

        // IE 61 decides the direction whenever the exporter sends it
        let mut record = flow_record(Some(1500), Some(10));
        record.direction = Some(TrafficDirection::Incoming);
        let (info, resolutions) = ingest(&record);
        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.transmitted_bytes, 1500);
        assert_eq!(entry.transmitted_packets, 10);
        assert_eq!(entry.traffic_direction, TrafficDirection::Incoming);
        // no rDNS threads are running, so the address is left awaiting lookup
        assert_eq!(resolutions.addresses_waiting_resolution.len(), 1);

        // without it there is no local interface to classify a remote flow
        // against, so the bogon heuristic decides — as it does for PCAP import
        record.direction = None;
        let (info, _) = ingest(&record);
        let entry = info.map.get(&totals_key()).expect("flow present");
        assert_eq!(entry.traffic_direction, TrafficDirection::Outgoing);
    }

    #[test]
    fn test_resolve_counters() {
        let now = Instant::now();
        let key = totals_key();
        let mut baselines = BaselineCache::new(now);
        let mut resolve =
            |record: &FlowRecord| resolve_counters(record, peer(), 0, &key, &mut baselines, now);

        // deltas are already increments, so they're used as they stand
        assert_eq!(resolve(&flow_record(Some(1500), Some(10))), (1500, 10));

        // cumulative counters are differenced against the flow's own previous
        // report: the first is the flow so far, the rest are what it grew by
        let totals = |bytes: u128, packets: u128| FlowRecord {
            bytes_total: Some(bytes),
            packets_total: Some(packets),
            ..flow_record(None, None)
        };
        assert_eq!(resolve(&totals(1500, 10)), (1500, 10));
        assert_eq!(resolve(&totals(4000, 25)), (2500, 15));
        assert_eq!(resolve(&totals(4000, 25)), (0, 0), "an unchanged report");

        // a template carrying both kinds keeps the baseline current while still
        // preferring the deltas, so a later totals-only record isn't over-counted
        let both = FlowRecord {
            bytes_total: Some(6000),
            packets_total: Some(40),
            ..flow_record(Some(900), Some(6))
        };
        assert_eq!(resolve(&both), (900, 6));
        assert_eq!(resolve(&totals(6500, 45)), (500, 5));

        // a record with no counters at all adds nothing
        assert_eq!(resolve(&flow_record(None, None)), (0, 0));
    }
}
