//! RFC 7011 IPFIX wire format decoding.
//!
//! This module parses the bytes of an IPFIX datagram into structured records.
//! It is deliberately minimal: only the Information Elements relevant for
//! 5-tuple flow visualization are interpreted; unknown IEs are skipped by
//! their declared length so future-template fields don't break decoding.
//!
//! All parsers are built on `nom` combinators so every byte read is bounded
//! by construction — malformed datagrams produce parse errors rather than
//! panics.

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::take;
use nom::combinator::verify;
use nom::multi::many0;
use nom::number::complete::{be_u8, be_u16, be_u32};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::types::timestamp::Timestamp;

pub const IPFIX_VERSION: u16 = 0x000A;
pub const SET_ID_TEMPLATE: u16 = 2;
pub const SET_ID_OPTIONS_TEMPLATE: u16 = 3;
pub const MIN_DATA_SET_ID: u16 = 256;
pub const VARIABLE_LENGTH: u16 = 0xFFFF;

pub mod ie {
    //! IANA-assigned IPFIX Information Element identifiers used by Sniffnet.
    //!
    //! Note the crossed naming in the IANA registry: the "post" counterpart of
    //! `sourceMacAddress` (56) is 81, while the one of `destinationMacAddress`
    //! (80) is 57.
    pub const OCTET_DELTA_COUNT: u16 = 1;
    pub const PACKET_DELTA_COUNT: u16 = 2;
    pub const PROTOCOL_IDENTIFIER: u16 = 4;
    pub const SOURCE_TRANSPORT_PORT: u16 = 7;
    pub const SOURCE_IPV4_ADDRESS: u16 = 8;
    pub const DESTINATION_TRANSPORT_PORT: u16 = 11;
    pub const DESTINATION_IPV4_ADDRESS: u16 = 12;
    pub const SOURCE_IPV6_ADDRESS: u16 = 27;
    pub const DESTINATION_IPV6_ADDRESS: u16 = 28;
    pub const SOURCE_MAC_ADDRESS: u16 = 56;
    pub const POST_DESTINATION_MAC_ADDRESS: u16 = 57;
    pub const FLOW_DIRECTION: u16 = 61;
    pub const DESTINATION_MAC_ADDRESS: u16 = 80;
    pub const POST_SOURCE_MAC_ADDRESS: u16 = 81;
    pub const OCTET_TOTAL_COUNT: u16 = 85;
    pub const PACKET_TOTAL_COUNT: u16 = 86;
    pub const FLOW_START_SECONDS: u16 = 150;
    pub const FLOW_END_SECONDS: u16 = 151;
    pub const FLOW_START_MILLISECONDS: u16 = 152;
    pub const FLOW_END_MILLISECONDS: u16 = 153;
    pub const FLOW_START_MICROSECONDS: u16 = 154;
    pub const FLOW_END_MICROSECONDS: u16 = 155;
    pub const FLOW_START_NANOSECONDS: u16 = 156;
    pub const FLOW_END_NANOSECONDS: u16 = 157;
    pub const LAYER2_OCTET_DELTA_COUNT: u16 = 352;
    pub const LAYER2_OCTET_TOTAL_COUNT: u16 = 353;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageHeader {
    pub version: u16,
    pub length: u16,
    pub export_time: u32,
    pub sequence_number: u32,
    pub observation_domain_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldSpec {
    pub ie_id: u16,
    pub length: u16,
    pub enterprise: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateRecord {
    pub template_id: u16,
    pub fields: Vec<FieldSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Set<'a> {
    Template(Vec<TemplateRecord>),
    /// Options templates are parsed but not interpreted; the collector skips them.
    OptionsTemplate,
    /// Reserved or unrecognised set id — consumed and skipped.
    Ignored,
    /// Data set: the payload is left as raw bytes and decoded against the
    /// referenced template by the collector layer.
    Data {
        template_id: u16,
        payload: &'a [u8],
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpfixMessage<'a> {
    pub header: MessageHeader,
    pub sets: Vec<Set<'a>>,
}

/// Decoded fields from a single data record. Each `Option` is `None` when the
/// template doesn't carry that IE.
///
/// `bytes` / `packets` come from the delta counters, which are already the
/// increment since the exporter's previous report. `bytes_total` /
/// `packets_total` come from the cumulative counters instead, and mean nothing
/// on their own — the collector differences them against the same flow's
/// previous report (see `totals.rs`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlowRecord {
    pub src_ip: Option<IpAddr>,
    pub dst_ip: Option<IpAddr>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Option<u8>,
    pub bytes: u128,
    pub packets: u128,
    pub bytes_total: Option<u128>,
    pub packets_total: Option<u128>,
    pub src_mac: Option<[u8; 6]>,
    pub dst_mac: Option<[u8; 6]>,
    pub direction: Option<TrafficDirection>,
    pub flow_start: Option<Timestamp>,
    pub flow_end: Option<Timestamp>,
}

/// Parse a complete IPFIX message (header + sets). Fails on anything that
/// isn't IPFIX, a `NetFlow` v9 datagram included.
pub fn parse_message(input: &[u8]) -> IResult<&[u8], IpfixMessage<'_>> {
    let (input, header) = parse_message_header(input)?;
    // header.length is the total message length including the 16-byte header
    let payload_len = (header.length as usize).saturating_sub(16);
    let (rest, payload) = take(payload_len)(input)?;
    let (_, sets) = many0(parse_set).parse(payload)?;
    Ok((rest, IpfixMessage { header, sets }))
}

fn parse_message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {
    // The version is what the parse hinges on: nothing further down would
    // reject a NetFlow v9 datagram, whose header is a different shape entirely,
    // so it would otherwise be read as IPFIX and decode into garbage.
    let (input, version) = verify(be_u16, |v: &u16| *v == IPFIX_VERSION).parse(input)?;
    let (input, length) = be_u16(input)?;
    let (input, export_time) = be_u32(input)?;
    let (input, sequence_number) = be_u32(input)?;
    let (input, observation_domain_id) = be_u32(input)?;
    Ok((
        input,
        MessageHeader {
            version,
            length,
            export_time,
            sequence_number,
            observation_domain_id,
        },
    ))
}

fn parse_set(input: &[u8]) -> IResult<&[u8], Set<'_>> {
    let (input, set_id) = be_u16(input)?;
    let (input, set_length) = be_u16(input)?;
    // set_length includes the 4-byte set header
    let body_len = (set_length as usize).saturating_sub(4);
    let (rest, body) = take(body_len)(input)?;

    let set = match set_id {
        SET_ID_TEMPLATE => {
            let (_, templates) = many0(parse_template_record).parse(body)?;
            Set::Template(templates)
        }
        SET_ID_OPTIONS_TEMPLATE => Set::OptionsTemplate,
        id if id >= MIN_DATA_SET_ID => Set::Data {
            template_id: id,
            payload: body,
        },
        // reserved set ids 0, 1, and 4..=255 — skip silently
        _ => Set::Ignored,
    };
    Ok((rest, set))
}

fn parse_template_record(input: &[u8]) -> IResult<&[u8], TemplateRecord> {
    let (input, template_id) = be_u16(input)?;
    let (input, field_count) = be_u16(input)?;
    let mut remaining = input;
    let mut fields = Vec::with_capacity(field_count as usize);
    for _ in 0..field_count {
        let (next, spec) = parse_field_spec(remaining)?;
        remaining = next;
        fields.push(spec);
    }
    Ok((
        remaining,
        TemplateRecord {
            template_id,
            fields,
        },
    ))
}

fn parse_field_spec(input: &[u8]) -> IResult<&[u8], FieldSpec> {
    let (input, raw_ie) = be_u16(input)?;
    let (input, length) = be_u16(input)?;
    let enterprise_bit = raw_ie & 0x8000 != 0;
    let ie_id = raw_ie & 0x7FFF;
    if enterprise_bit {
        let (input, enterprise) = be_u32(input)?;
        Ok((
            input,
            FieldSpec {
                ie_id,
                length,
                enterprise: Some(enterprise),
            },
        ))
    } else {
        Ok((
            input,
            FieldSpec {
                ie_id,
                length,
                enterprise: None,
            },
        ))
    }
}

/// Decode a single data record against its template. Returns the consumed
/// number of bytes alongside the parsed `FlowRecord`.
pub fn decode_data_record<'a>(
    template: &[FieldSpec],
    input: &'a [u8],
) -> IResult<&'a [u8], FlowRecord> {
    let mut record = FlowRecord::default();
    let mut priority = FieldPriority::default();
    let mut remaining = input;

    for spec in template {
        let (after, raw) = read_field_bytes(remaining, spec.length)?;
        remaining = after;

        // Enterprise-specific IEs and unknown IEs are skipped; the bytes were
        // already consumed above by `read_field_bytes`.
        if spec.enterprise.is_some() {
            continue;
        }

        apply_ie(spec.ie_id, raw, &mut record, &mut priority);
    }

    Ok((remaining, record))
}

/// Rank of the IE that supplied the value currently held in each record slot.
///
/// A template may legitimately carry several IEs that fill the same slot (e.g.
/// `octetDeltaCount` alongside `layer2OctetDeltaCount`, or both a second- and a
/// millisecond-granularity flow start). Ranking them means the outcome no
/// longer depends on which one happens to appear last in the template.
#[derive(Default)]
struct FieldPriority {
    bytes: u8,
    packets: u8,
    bytes_total: u8,
    packets_total: u8,
    flow_start: u8,
    flow_end: u8,
}

/// Higher wins. Layer-2 deltas match what the pcap pipeline counts — frame
/// bytes including the link header — so they outrank IP-layer deltas.
fn octet_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::LAYER2_OCTET_DELTA_COUNT => 2,
        ie::OCTET_DELTA_COUNT => 1,
        _ => 0,
    }
}

fn packet_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::PACKET_DELTA_COUNT => 1,
        _ => 0,
    }
}

/// Same layer-2-over-IP preference as `octet_rank`, for the cumulative counters.
fn octet_total_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::LAYER2_OCTET_TOTAL_COUNT => 2,
        ie::OCTET_TOTAL_COUNT => 1,
        _ => 0,
    }
}

fn packet_total_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::PACKET_TOTAL_COUNT => 1,
        _ => 0,
    }
}

/// Higher wins, so the finest granularity the exporter offers is the one kept.
fn timestamp_rank(ie_id: u16) -> u8 {
    match ie_id {
        ie::FLOW_START_NANOSECONDS | ie::FLOW_END_NANOSECONDS => 4,
        ie::FLOW_START_MICROSECONDS | ie::FLOW_END_MICROSECONDS => 3,
        ie::FLOW_START_MILLISECONDS | ie::FLOW_END_MILLISECONDS => 2,
        ie::FLOW_START_SECONDS | ie::FLOW_END_SECONDS => 1,
        _ => 0,
    }
}

/// Read the bytes belonging to a single field, accounting for the
/// variable-length encoding (RFC 7011 §7).
fn read_field_bytes(input: &[u8], declared_length: u16) -> IResult<&[u8], &[u8]> {
    if declared_length != VARIABLE_LENGTH {
        return take(declared_length as usize)(input);
    }
    // Variable length: 1-byte length, with 0xFF sentinel switching to 2-byte length
    let (input, first) = be_u8(input)?;
    if first == 0xFF {
        let (input, actual_len) = be_u16(input)?;
        return take(actual_len as usize)(input);
    }
    take(first as usize)(input)
}

fn apply_ie(ie_id: u16, raw: &[u8], record: &mut FlowRecord, priority: &mut FieldPriority) {
    if apply_delta_counter_ie(ie_id, raw, record, priority)
        || apply_total_counter_ie(ie_id, raw, record, priority)
        || apply_timestamp_ie(ie_id, raw, record, priority)
    {
        return;
    }

    match ie_id {
        ie::PROTOCOL_IDENTIFIER => {
            if let Some(b) = raw.first() {
                record.protocol = Some(*b);
            }
        }
        ie::SOURCE_TRANSPORT_PORT => {
            if let Some(v) = read_u16(raw) {
                record.src_port = Some(v);
            }
        }
        ie::DESTINATION_TRANSPORT_PORT => {
            if let Some(v) = read_u16(raw) {
                record.dst_port = Some(v);
            }
        }
        ie::SOURCE_IPV4_ADDRESS => {
            if let Some(v) = read_ipv4(raw) {
                record.src_ip = Some(v);
            }
        }
        ie::DESTINATION_IPV4_ADDRESS => {
            if let Some(v) = read_ipv4(raw) {
                record.dst_ip = Some(v);
            }
        }
        ie::SOURCE_IPV6_ADDRESS => {
            if let Some(v) = read_ipv6(raw) {
                record.src_ip = Some(v);
            }
        }
        ie::DESTINATION_IPV6_ADDRESS => {
            if let Some(v) = read_ipv6(raw) {
                record.dst_ip = Some(v);
            }
        }
        ie::SOURCE_MAC_ADDRESS | ie::POST_SOURCE_MAC_ADDRESS => {
            if let Some(v) = read_mac(raw) {
                record.src_mac = Some(v);
            }
        }
        ie::DESTINATION_MAC_ADDRESS | ie::POST_DESTINATION_MAC_ADDRESS => {
            if let Some(v) = read_mac(raw) {
                record.dst_mac = Some(v);
            }
        }
        ie::FLOW_DIRECTION => {
            // IANA: 0x00 = ingress, 0x01 = egress, 0xFF = undefined.
            // Unknown values are treated as undefined.
            record.direction = match raw.first() {
                Some(0x00) => Some(TrafficDirection::Incoming),
                Some(0x01) => Some(TrafficDirection::Outgoing),
                _ => None,
            };
        }
        _ => {}
    }
}

/// Apply a delta counter IE, which is already an increment over the exporter's
/// previous report. Returns whether `ie_id` was one.
fn apply_delta_counter_ie(
    ie_id: u16,
    raw: &[u8],
    record: &mut FlowRecord,
    priority: &mut FieldPriority,
) -> bool {
    let (rank, slot, slot_priority): (u8, &mut u128, &mut u8) = match ie_id {
        ie::OCTET_DELTA_COUNT | ie::LAYER2_OCTET_DELTA_COUNT => {
            (octet_rank(ie_id), &mut record.bytes, &mut priority.bytes)
        }
        ie::PACKET_DELTA_COUNT => (
            packet_rank(ie_id),
            &mut record.packets,
            &mut priority.packets,
        ),
        _ => return false,
    };

    if rank >= *slot_priority
        && let Some(v) = read_unsigned(raw)
    {
        *slot = v;
        *slot_priority = rank;
    }
    true
}

/// Apply a cumulative counter IE. These are counted for the lifetime of the
/// flow, so they are kept apart from the deltas: the collector turns them into
/// an increment by differencing against the flow's previous report. Returns
/// whether `ie_id` was one.
fn apply_total_counter_ie(
    ie_id: u16,
    raw: &[u8],
    record: &mut FlowRecord,
    priority: &mut FieldPriority,
) -> bool {
    let (rank, slot, slot_priority): (u8, &mut Option<u128>, &mut u8) = match ie_id {
        ie::OCTET_TOTAL_COUNT | ie::LAYER2_OCTET_TOTAL_COUNT => (
            octet_total_rank(ie_id),
            &mut record.bytes_total,
            &mut priority.bytes_total,
        ),
        ie::PACKET_TOTAL_COUNT => (
            packet_total_rank(ie_id),
            &mut record.packets_total,
            &mut priority.packets_total,
        ),
        _ => return false,
    };

    if rank >= *slot_priority
        && let Some(v) = read_unsigned(raw)
    {
        *slot = Some(v);
        *slot_priority = rank;
    }
    true
}

/// Apply a flow start or end timestamp IE. Returns whether `ie_id` was one.
fn apply_timestamp_ie(
    ie_id: u16,
    raw: &[u8],
    record: &mut FlowRecord,
    priority: &mut FieldPriority,
) -> bool {
    let (slot, slot_priority) = match ie_id {
        ie::FLOW_START_SECONDS
        | ie::FLOW_START_MILLISECONDS
        | ie::FLOW_START_MICROSECONDS
        | ie::FLOW_START_NANOSECONDS => (&mut record.flow_start, &mut priority.flow_start),
        ie::FLOW_END_SECONDS
        | ie::FLOW_END_MILLISECONDS
        | ie::FLOW_END_MICROSECONDS
        | ie::FLOW_END_NANOSECONDS => (&mut record.flow_end, &mut priority.flow_end),
        _ => return false,
    };

    let rank = timestamp_rank(ie_id);
    if rank >= *slot_priority
        && let Some(ts) = read_timestamp(ie_id, raw)
    {
        *slot = Some(ts);
        *slot_priority = rank;
    }
    true
}

/// Read a timestamp field using the encoding its IE prescribes. The four
/// granularities use three different wire formats (RFC 7011 §6.1.7-6.1.10),
/// so the IE id has to pick the reader.
fn read_timestamp(ie_id: u16, raw: &[u8]) -> Option<Timestamp> {
    match ie_id {
        ie::FLOW_START_SECONDS | ie::FLOW_END_SECONDS => read_timestamp_secs(raw),
        ie::FLOW_START_MILLISECONDS | ie::FLOW_END_MILLISECONDS => read_timestamp_ms(raw),
        // Both the microsecond and the nanosecond IEs carry an NTP timestamp;
        // they differ only in how many of the fraction bits the exporter is
        // allowed to set, which doesn't change how we read them.
        ie::FLOW_START_MICROSECONDS
        | ie::FLOW_END_MICROSECONDS
        | ie::FLOW_START_NANOSECONDS
        | ie::FLOW_END_NANOSECONDS => read_timestamp_ntp(raw),
        _ => None,
    }
}

/// IPFIX `dateTimeSeconds` is 4 bytes big-endian, seconds since UNIX epoch.
fn read_timestamp_secs(raw: &[u8]) -> Option<Timestamp> {
    if raw.len() != 4 {
        return None;
    }
    let secs = u32::from_be_bytes(raw.try_into().ok()?);
    Some(Timestamp::new(i64::from(secs), 0))
}

/// IPFIX `dateTimeMilliseconds` is 8 bytes big-endian, ms since UNIX epoch.
/// Converted to Sniffnet's `Timestamp(secs, usecs)` representation.
fn read_timestamp_ms(raw: &[u8]) -> Option<Timestamp> {
    if raw.len() != 8 {
        return None;
    }
    let ms = u64::from_be_bytes(raw.try_into().ok()?);
    let secs = i64::try_from(ms / 1_000).ok()?;
    let usecs = i64::try_from((ms % 1_000) * 1_000).ok()?;
    Some(Timestamp::new(secs, usecs))
}

/// Seconds between the NTP epoch (1900-01-01) and the UNIX epoch (1970-01-01).
const NTP_UNIX_OFFSET: u64 = 2_208_988_800;

/// IPFIX `dateTimeMicroseconds` and `dateTimeNanoseconds` are 8-byte NTP
/// timestamps (RFC 5905): 32 bits of seconds since 1900 followed by a 32-bit
/// binary fraction of a second.
///
/// Only NTP era 0 is decoded, so timestamps beyond 2036-02-07 read as `None`
/// rather than silently wrapping to 1900.
fn read_timestamp_ntp(raw: &[u8]) -> Option<Timestamp> {
    if raw.len() != 8 {
        return None;
    }
    let ntp = u64::from_be_bytes(raw.try_into().ok()?);
    let ntp_secs = ntp >> 32;
    let fraction = ntp & 0xFFFF_FFFF;
    // Pre-1970 timestamps aren't representable as a UNIX instant here, and in
    // practice only show up on exporters with a broken clock.
    let secs = i64::try_from(ntp_secs.checked_sub(NTP_UNIX_OFFSET)?).ok()?;
    let usecs = i64::try_from((fraction * 1_000_000) >> 32).ok()?;
    Some(Timestamp::new(secs, usecs))
}

/// Read a big-endian unsigned integer of 1..=8 bytes into a `u128`.
fn read_unsigned(raw: &[u8]) -> Option<u128> {
    if raw.is_empty() || raw.len() > 8 {
        return None;
    }
    let mut buf = [0u8; 8];
    buf[8 - raw.len()..].copy_from_slice(raw);
    Some(u128::from(u64::from_be_bytes(buf)))
}

fn read_u16(raw: &[u8]) -> Option<u16> {
    match raw.len() {
        1 => Some(u16::from(raw[0])),
        2 => Some(u16::from_be_bytes([raw[0], raw[1]])),
        _ => None,
    }
}

fn read_ipv4(raw: &[u8]) -> Option<IpAddr> {
    if raw.len() != 4 {
        return None;
    }
    Some(IpAddr::V4(Ipv4Addr::new(raw[0], raw[1], raw[2], raw[3])))
}

fn read_ipv6(raw: &[u8]) -> Option<IpAddr> {
    if raw.len() != 16 {
        return None;
    }
    let mut octets = [0u8; 16];
    octets.copy_from_slice(raw);
    Some(IpAddr::V6(Ipv6Addr::from(octets)))
}

/// An all-zero MAC is how exporters spell "not observed" — `sniffnet-agent`
/// writes it whenever a flow has no link header — so it decodes to `None`
/// rather than being shown as a genuine `00:00:00:00:00:00` address.
fn read_mac(raw: &[u8]) -> Option<[u8; 6]> {
    if raw.len() != 6 || raw.iter().all(|b| *b == 0) {
        return None;
    }
    let mut mac = [0u8; 6];
    mac.copy_from_slice(raw);
    Some(mac)
}

/// Format a MAC address as a colon-separated hex string.
pub fn format_mac(mac: [u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_message_header() {
        // version=10, length=16, export_time=0xDEADBEEF, seq=0x01020304, odid=0
        let bytes: Vec<u8> = vec![
            0x00, 0x0A, 0x00, 0x10, 0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x02, 0x03, 0x04, 0x00, 0x00,
            0x00, 0x00,
        ];
        let (_, hdr) = parse_message_header(&bytes).unwrap();
        assert_eq!(hdr.version, IPFIX_VERSION);
        assert_eq!(hdr.length, 16);
        assert_eq!(hdr.export_time, 0xDEAD_BEEF);
        assert_eq!(hdr.sequence_number, 0x0102_0304);
        assert_eq!(hdr.observation_domain_id, 0);
    }

    #[test]
    fn rejects_a_header_that_is_not_ipfix() {
        // A NetFlow v9 header: no later step of the parse would catch it, so
        // the version has to be what fails.
        let mut v9 = 9u16.to_be_bytes().to_vec();
        v9.extend_from_slice(&[0; 18]);
        assert!(parse_message_header(&v9).is_err());
        assert!(parse_message(&v9).is_err());
    }

    #[test]
    fn parses_template_set_and_data_set() {
        // Message containing a template (id=256, fields: srcIPv4, dstIPv4, octets, packets)
        // followed by a data set with one record.
        let mut bytes = Vec::new();
        // Header (will fill length later)
        bytes.extend_from_slice(&[0x00, 0x0A]); // version
        let len_off = bytes.len();
        bytes.extend_from_slice(&[0x00, 0x00]); // length placeholder
        bytes.extend_from_slice(&[0, 0, 0, 0]); // export time
        bytes.extend_from_slice(&[0, 0, 0, 1]); // seq
        bytes.extend_from_slice(&[0, 0, 0, 0]); // odid

        // Template set
        bytes.extend_from_slice(&[0x00, 0x02]); // set id = 2
        let tset_len_off = bytes.len();
        bytes.extend_from_slice(&[0x00, 0x00]); // set length placeholder
        bytes.extend_from_slice(&[0x01, 0x00]); // template id = 256
        bytes.extend_from_slice(&[0x00, 0x04]); // field count = 4
        bytes.extend_from_slice(&[0x00, 8, 0x00, 4]); // IE 8 (srcIPv4), len 4
        bytes.extend_from_slice(&[0x00, 12, 0x00, 4]); // IE 12 (dstIPv4), len 4
        bytes.extend_from_slice(&[0x00, 1, 0x00, 8]); // IE 1 (octetDelta), len 8
        bytes.extend_from_slice(&[0x00, 2, 0x00, 8]); // IE 2 (packetDelta), len 8
        let tset_len = (bytes.len() - tset_len_off + 2) as u16; // includes the 4-byte set header
        let tset_len_bytes = (tset_len).to_be_bytes();
        bytes[tset_len_off] = tset_len_bytes[0];
        bytes[tset_len_off + 1] = tset_len_bytes[1];

        // Data set
        bytes.extend_from_slice(&[0x01, 0x00]); // set id = 256
        let dset_len_off = bytes.len();
        bytes.extend_from_slice(&[0x00, 0x00]); // set length placeholder
        // record: src=10.0.0.1, dst=192.168.1.5, bytes=1500, packets=10
        bytes.extend_from_slice(&[10, 0, 0, 1]);
        bytes.extend_from_slice(&[192, 168, 1, 5]);
        bytes.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0x05, 0xDC]); // 1500
        bytes.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 10]); // 10
        let dset_len = (bytes.len() - dset_len_off + 2) as u16;
        let dset_len_bytes = dset_len.to_be_bytes();
        bytes[dset_len_off] = dset_len_bytes[0];
        bytes[dset_len_off + 1] = dset_len_bytes[1];

        // Finalize message length
        let msg_len = (bytes.len() as u16).to_be_bytes();
        bytes[len_off] = msg_len[0];
        bytes[len_off + 1] = msg_len[1];

        let (_, msg) = parse_message(&bytes).expect("parse");
        assert_eq!(msg.header.version, IPFIX_VERSION);
        assert_eq!(msg.sets.len(), 2);

        let template = match &msg.sets[0] {
            Set::Template(t) => t,
            other => panic!("expected template, got {other:?}"),
        };
        assert_eq!(template.len(), 1);
        assert_eq!(template[0].template_id, 256);
        assert_eq!(template[0].fields.len(), 4);

        let (template_id, payload) = match &msg.sets[1] {
            Set::Data {
                template_id,
                payload,
            } => (*template_id, *payload),
            other => panic!("expected data, got {other:?}"),
        };
        assert_eq!(template_id, 256);

        let (rest, record) =
            decode_data_record(&template[0].fields, payload).expect("decode record");
        assert!(rest.is_empty());
        assert_eq!(record.src_ip, Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
        assert_eq!(
            record.dst_ip,
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5)))
        );
        assert_eq!(record.bytes, 1500);
        assert_eq!(record.packets, 10);
    }

    #[test]
    fn enterprise_ie_is_skipped_but_consumes_bytes() {
        // Template: one enterprise IE of length 4
        let bytes: Vec<u8> = vec![
            0x80, 0x01, 0x00, 0x04, // ie_id=1 with enterprise bit, length=4
            0x00, 0x00, 0x00, 0x2A, // enterprise number = 42
        ];
        let (_, spec) = parse_field_spec(&bytes).unwrap();
        assert_eq!(spec.ie_id, 1);
        assert_eq!(spec.length, 4);
        assert_eq!(spec.enterprise, Some(42));

        // Decoding a data record with this single enterprise field should
        // consume 4 bytes and leave the record untouched.
        let payload: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let (rest, record) = decode_data_record(&[spec], &payload).unwrap();
        assert!(rest.is_empty());
        assert_eq!(record, FlowRecord::default());
    }

    #[test]
    fn variable_length_short_and_long_forms() {
        // Short form: 1-byte length = 3, then 3 bytes
        let short: Vec<u8> = vec![0x03, b'a', b'b', b'c', 0xAA];
        let (rest, bytes) = read_field_bytes(&short, VARIABLE_LENGTH).unwrap();
        assert_eq!(bytes, b"abc");
        assert_eq!(rest, &[0xAAu8][..]);

        // Long form: sentinel 0xFF, then 2-byte length = 4, then 4 bytes
        let long: Vec<u8> = vec![0xFF, 0x00, 0x04, b'w', b'x', b'y', b'z', 0xBB];
        let (rest, bytes) = read_field_bytes(&long, VARIABLE_LENGTH).unwrap();
        assert_eq!(bytes, b"wxyz");
        assert_eq!(rest, &[0xBBu8][..]);
    }

    #[test]
    fn truncated_datagram_returns_error_not_panic() {
        // Header claims length 200 but only the 16-byte header is present
        let bytes: Vec<u8> = vec![0x00, 0x0A, 0x00, 0xC8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let result = parse_message(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn options_template_set_is_returned_as_marker() {
        // Options template set with a 2-byte body — content doesn't matter
        // for our purposes; we just want it skipped cleanly.
        let bytes: Vec<u8> = vec![0x00, 0x03, 0x00, 0x06, 0xAA, 0xBB];
        let (_, set) = parse_set(&bytes).unwrap();
        assert_eq!(set, Set::OptionsTemplate);
    }

    /// Decode `payload` against a template built from `(ie, length)` pairs.
    fn decode(fields: &[(u16, u16)], payload: &[u8]) -> FlowRecord {
        let template: Vec<FieldSpec> = fields
            .iter()
            .map(|(ie_id, length)| FieldSpec {
                ie_id: *ie_id,
                length: *length,
                enterprise: None,
            })
            .collect();
        decode_data_record(&template, payload).expect("decode").1
    }

    #[test]
    fn layer2_octets_win_over_ip_octets_in_either_order() {
        // 1500 as layer2OctetDeltaCount, 1000 as octetDeltaCount
        let l2 = [0, 0, 0, 0, 0, 0, 0x05, 0xDC];
        let ip = [0, 0, 0, 0, 0, 0, 0x03, 0xE8];

        let mut l2_first = Vec::new();
        l2_first.extend_from_slice(&l2);
        l2_first.extend_from_slice(&ip);
        assert_eq!(
            decode(
                &[
                    (ie::LAYER2_OCTET_DELTA_COUNT, 8),
                    (ie::OCTET_DELTA_COUNT, 8)
                ],
                &l2_first,
            )
            .bytes,
            1500,
        );

        let mut ip_first = Vec::new();
        ip_first.extend_from_slice(&ip);
        ip_first.extend_from_slice(&l2);
        assert_eq!(
            decode(
                &[
                    (ie::OCTET_DELTA_COUNT, 8),
                    (ie::LAYER2_OCTET_DELTA_COUNT, 8)
                ],
                &ip_first,
            )
            .bytes,
            1500,
        );
    }

    #[test]
    fn cumulative_totals_are_kept_apart_from_deltas() {
        // Totals are cumulative, so they must not land in the delta slots the
        // collector adds straight onto its running tally.
        let payload = [0, 0, 0, 0, 0, 0, 0x05, 0xDC, 0, 0, 0, 0, 0, 0, 0, 10];
        let record = decode(
            &[(ie::OCTET_TOTAL_COUNT, 8), (ie::PACKET_TOTAL_COUNT, 8)],
            &payload,
        );
        assert_eq!(record.bytes, 0);
        assert_eq!(record.packets, 0);
        assert_eq!(record.bytes_total, Some(1500));
        assert_eq!(record.packets_total, Some(10));
    }

    #[test]
    fn layer2_totals_win_over_ip_totals_in_either_order() {
        let l2 = [0, 0, 0, 0, 0, 0, 0x05, 0xDC]; // 1500
        let ip = [0, 0, 0, 0, 0, 0, 0x03, 0xE8]; // 1000

        let mut l2_first = Vec::new();
        l2_first.extend_from_slice(&l2);
        l2_first.extend_from_slice(&ip);
        assert_eq!(
            decode(
                &[
                    (ie::LAYER2_OCTET_TOTAL_COUNT, 8),
                    (ie::OCTET_TOTAL_COUNT, 8)
                ],
                &l2_first,
            )
            .bytes_total,
            Some(1500),
        );

        let mut ip_first = Vec::new();
        ip_first.extend_from_slice(&ip);
        ip_first.extend_from_slice(&l2);
        assert_eq!(
            decode(
                &[
                    (ie::OCTET_TOTAL_COUNT, 8),
                    (ie::LAYER2_OCTET_TOTAL_COUNT, 8)
                ],
                &ip_first,
            )
            .bytes_total,
            Some(1500),
        );
    }

    #[test]
    fn ntp_timestamps_decode_against_the_unix_epoch() {
        // 1900-01-01 + NTP_UNIX_OFFSET seconds == the UNIX epoch, so this is 20s
        // past the UNIX epoch with a half-second fraction.
        let ntp_secs = u32::try_from(NTP_UNIX_OFFSET + 20).unwrap();
        let mut payload = ntp_secs.to_be_bytes().to_vec();
        payload.extend_from_slice(&0x8000_0000u32.to_be_bytes()); // 0.5s

        let record = decode(&[(ie::FLOW_START_MICROSECONDS, 8)], &payload);
        assert_eq!(record.flow_start, Some(Timestamp::new(20, 500_000)));

        // The nanosecond IEs use the very same encoding.
        let record = decode(&[(ie::FLOW_END_NANOSECONDS, 8)], &payload);
        assert_eq!(record.flow_end, Some(Timestamp::new(20, 500_000)));
    }

    #[test]
    fn ntp_timestamps_before_the_unix_epoch_are_rejected() {
        // Era-0 NTP seconds below the offset would otherwise decode to a
        // negative UNIX instant.
        let payload = 1_000u64.to_be_bytes();
        let record = decode(&[(ie::FLOW_START_MICROSECONDS, 8)], &payload);
        assert_eq!(record.flow_start, None);
    }

    #[test]
    fn finest_timestamp_granularity_wins_regardless_of_order() {
        let secs = [0x00, 0x00, 0x00, 0x0A]; // 10s
        let millis = 20_000u64.to_be_bytes(); // 20s
        let micros = {
            let mut v = u32::try_from(NTP_UNIX_OFFSET + 30)
                .unwrap()
                .to_be_bytes()
                .to_vec();
            v.extend_from_slice(&0u32.to_be_bytes()); // 30s
            v
        };
        let expected = Some(Timestamp::new(30, 0));

        let mut coarse_first = secs.to_vec();
        coarse_first.extend_from_slice(&millis);
        coarse_first.extend_from_slice(&micros);
        assert_eq!(
            decode(
                &[
                    (ie::FLOW_START_SECONDS, 4),
                    (ie::FLOW_START_MILLISECONDS, 8),
                    (ie::FLOW_START_MICROSECONDS, 8),
                ],
                &coarse_first,
            )
            .flow_start,
            expected,
        );

        let mut fine_first = micros.clone();
        fine_first.extend_from_slice(&millis);
        fine_first.extend_from_slice(&secs);
        assert_eq!(
            decode(
                &[
                    (ie::FLOW_START_MICROSECONDS, 8),
                    (ie::FLOW_START_MILLISECONDS, 8),
                    (ie::FLOW_START_SECONDS, 4),
                ],
                &fine_first,
            )
            .flow_start,
            expected,
        );
    }

    #[test]
    fn milliseconds_win_over_seconds_in_either_order() {
        let secs = [0x00, 0x00, 0x00, 0x0A]; // 10s
        let millis = [0, 0, 0, 0, 0, 0, 0x4E, 0x20]; // 20_000ms == 20s
        let expected = Timestamp::new(20, 0);

        let mut secs_first = Vec::new();
        secs_first.extend_from_slice(&secs);
        secs_first.extend_from_slice(&millis);
        assert_eq!(
            decode(
                &[
                    (ie::FLOW_START_SECONDS, 4),
                    (ie::FLOW_START_MILLISECONDS, 8)
                ],
                &secs_first,
            )
            .flow_start,
            Some(expected),
        );

        let mut millis_first = Vec::new();
        millis_first.extend_from_slice(&millis);
        millis_first.extend_from_slice(&secs);
        assert_eq!(
            decode(
                &[
                    (ie::FLOW_START_MILLISECONDS, 8),
                    (ie::FLOW_START_SECONDS, 4)
                ],
                &millis_first,
            )
            .flow_start,
            Some(expected),
        );
    }

    #[test]
    fn second_granularity_timestamps_decode_on_their_own() {
        let payload = [0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x14];
        let record = decode(
            &[(ie::FLOW_START_SECONDS, 4), (ie::FLOW_END_SECONDS, 4)],
            &payload,
        );
        assert_eq!(record.flow_start, Some(Timestamp::new(10, 0)));
        assert_eq!(record.flow_end, Some(Timestamp::new(20, 0)));
    }

    #[test]
    fn all_zero_mac_decodes_to_none() {
        // `sniffnet-agent` writes all-zero when a flow carries no link header.
        let payload = [0, 0, 0, 0, 0, 0, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let record = decode(
            &[
                (ie::SOURCE_MAC_ADDRESS, 6),
                (ie::DESTINATION_MAC_ADDRESS, 6),
            ],
            &payload,
        );
        assert_eq!(record.src_mac, None);
        assert_eq!(record.dst_mac, Some([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]));
    }

    #[test]
    fn flow_direction_maps_ingress_egress_and_undefined() {
        let cases = [
            (0x00, Some(TrafficDirection::Incoming)),
            (0x01, Some(TrafficDirection::Outgoing)),
            (0xFF, None),
        ];
        for (raw, expected) in cases {
            let record = decode(&[(ie::FLOW_DIRECTION, 1)], &[raw]);
            assert_eq!(record.direction, expected, "flowDirection {raw:#04x}");
        }
    }
}
