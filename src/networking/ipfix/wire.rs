//! RFC 7011 IPFIX wire format decoding.

use crate::networking::ipfix::field_priority::{
    FieldPriority, bytes_delta_rank, bytes_total_rank, mac_rank, packets_delta_rank,
    packets_total_rank, timestamp_rank,
};
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::types::timestamp::Timestamp;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::take;
use nom::combinator::verify;
use nom::multi::many0;
use nom::number::complete::{be_u8, be_u16, be_u32};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// IPFIX version number (10)
pub(super) const IPFIX_VERSION: u16 = 0x000A;
/// The set id for a template set
pub(super) const SET_ID_TEMPLATE: u16 = 2;
/// Variable-length field length sentinel
pub(super) const VARIABLE_LENGTH: u16 = 0xFFFF;
/// Direction flag to export reverse-direction counterparts of standard IEs
pub(super) const REVERSE_PEN: u32 = 29305;
/// The minimum set id for a data set
const MIN_DATA_SET_ID: u16 = 256;

/// IANA-assigned IPFIX Information Element identifiers used by Sniffnet
pub(super) mod ie {
    pub(in crate::networking::ipfix) const OCTET_DELTA_COUNT: u16 = 1;
    pub(in crate::networking::ipfix) const PACKET_DELTA_COUNT: u16 = 2;
    pub(in crate::networking::ipfix) const PROTOCOL_IDENTIFIER: u16 = 4;
    pub(in crate::networking::ipfix) const SOURCE_TRANSPORT_PORT: u16 = 7;
    pub(in crate::networking::ipfix) const SOURCE_IPV4_ADDRESS: u16 = 8;
    pub(in crate::networking::ipfix) const DESTINATION_TRANSPORT_PORT: u16 = 11;
    pub(in crate::networking::ipfix) const DESTINATION_IPV4_ADDRESS: u16 = 12;
    pub(in crate::networking::ipfix) const POST_OCTET_DELTA_COUNT: u16 = 23;
    pub(in crate::networking::ipfix) const POST_PACKET_DELTA_COUNT: u16 = 24;
    pub(in crate::networking::ipfix) const SOURCE_IPV6_ADDRESS: u16 = 27;
    pub(in crate::networking::ipfix) const DESTINATION_IPV6_ADDRESS: u16 = 28;
    pub(in crate::networking::ipfix) const SOURCE_MAC_ADDRESS: u16 = 56;
    pub(in crate::networking::ipfix) const POST_DESTINATION_MAC_ADDRESS: u16 = 57;
    pub(in crate::networking::ipfix) const FLOW_DIRECTION: u16 = 61;
    pub(in crate::networking::ipfix) const DESTINATION_MAC_ADDRESS: u16 = 80;
    pub(in crate::networking::ipfix) const POST_SOURCE_MAC_ADDRESS: u16 = 81;
    pub(in crate::networking::ipfix) const OCTET_TOTAL_COUNT: u16 = 85;
    pub(in crate::networking::ipfix) const PACKET_TOTAL_COUNT: u16 = 86;
    pub(in crate::networking::ipfix) const FLOW_START_SECONDS: u16 = 150;
    pub(in crate::networking::ipfix) const FLOW_END_SECONDS: u16 = 151;
    pub(in crate::networking::ipfix) const FLOW_START_MILLISECONDS: u16 = 152;
    pub(in crate::networking::ipfix) const FLOW_END_MILLISECONDS: u16 = 153;
    pub(in crate::networking::ipfix) const FLOW_START_MICROSECONDS: u16 = 154;
    pub(in crate::networking::ipfix) const FLOW_END_MICROSECONDS: u16 = 155;
    pub(in crate::networking::ipfix) const FLOW_START_NANOSECONDS: u16 = 156;
    pub(in crate::networking::ipfix) const FLOW_END_NANOSECONDS: u16 = 157;
    pub(in crate::networking::ipfix) const POST_OCTET_TOTAL_COUNT: u16 = 171;
    pub(in crate::networking::ipfix) const POST_PACKET_TOTAL_COUNT: u16 = 172;
    pub(in crate::networking::ipfix) const LAYER2_OCTET_DELTA_COUNT: u16 = 352;
    pub(in crate::networking::ipfix) const LAYER2_OCTET_TOTAL_COUNT: u16 = 353;
    pub(in crate::networking::ipfix) const POST_LAYER2_OCTET_DELTA_COUNT: u16 = 417;
    pub(in crate::networking::ipfix) const POST_LAYER2_OCTET_TOTAL_COUNT: u16 = 420;
}

/// IPFIX complete message
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IpfixMessage<'a> {
    pub(super) header: MessageHeader,
    pub(super) sets: Vec<Set<'a>>,
}

/// IPFIX message header
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MessageHeader {
    version: u16,
    length: u16,
    export_time: u32,
    sequence_number: u32,
    pub(super) observation_domain_id: u32,
}

/// IPFIX set: either a template set or a data set
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Set<'a> {
    Template(Vec<TemplateRecord>),
    Data { template_id: u16, payload: &'a [u8] },
    Ignored,
}

/// IPFIX template record, as carried in a template set
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TemplateRecord {
    pub(super) template_id: u16,
    pub(super) fields: Vec<FieldSpec>,
}

/// IPFIX field specification, as carried in a template record
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct FieldSpec {
    pub(super) ie_id: u16,
    pub(super) length: u16,
    pub(super) enterprise: Option<u32>,
}

/// Decoded fields from a single data record
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct FlowRecord {
    pub(super) src_ip: Option<IpAddr>,
    pub(super) dst_ip: Option<IpAddr>,
    pub(super) src_port: Option<u16>,
    pub(super) dst_port: Option<u16>,
    pub(super) protocol: Option<u8>,
    pub(super) bytes_delta: Option<u128>,
    pub(super) packets_delta: Option<u128>,
    pub(super) bytes_total: Option<u128>,
    pub(super) packets_total: Option<u128>,
    pub(super) src_mac: Option<[u8; 6]>,
    pub(super) dst_mac: Option<[u8; 6]>,
    pub(super) direction: Option<TrafficDirection>,
    pub(super) flow_start: Option<Timestamp>,
    pub(super) flow_end: Option<Timestamp>,
    /// Set only when the exporter sends a biflow (same flow in the opposite direction)
    pub(super) reverse: Option<ReverseCounters>,
}

/// The counters of an RFC 5103 biflow's reverse direction
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ReverseCounters {
    pub(super) bytes_delta: Option<u128>,
    pub(super) packets_delta: Option<u128>,
    pub(super) bytes_total: Option<u128>,
    pub(super) packets_total: Option<u128>,
}

/// Parse a complete IPFIX message (header + sets)
pub(super) fn parse_message(input: &[u8]) -> IResult<&[u8], IpfixMessage<'_>> {
    let (input, header) = parse_message_header(input)?;
    // header.length includes the 16-byte header already parsed
    let payload_len = header.length.saturating_sub(16);
    let (rest, payload) = take(payload_len)(input)?;
    let (_, sets) = many0(parse_set).parse(payload)?;
    Ok((rest, IpfixMessage { header, sets }))
}

/// Parse an IPFIX message header (16 bytes)
fn parse_message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {
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

/// Parse a single IPFIX set
fn parse_set(input: &[u8]) -> IResult<&[u8], Set<'_>> {
    let (input, set_id) = be_u16(input)?;
    let (input, set_length) = be_u16(input)?;
    // set_length includes the 4-byte set header already parsed
    let body_len = set_length.saturating_sub(4);
    let (rest, body) = take(body_len)(input)?;

    let set = match set_id {
        SET_ID_TEMPLATE => {
            let (_, templates) = many0(parse_template_record).parse(body)?;
            Set::Template(templates)
        }
        id if id >= MIN_DATA_SET_ID => Set::Data {
            template_id: id,
            payload: body,
        },
        _ => Set::Ignored,
    };

    Ok((rest, set))
}

/// Parse a single template record from a template set
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

/// Parse a single field specification from a template record
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

/// Decode a single data record against its template
pub(super) fn decode_data_record<'a>(
    template: &[FieldSpec],
    input: &'a [u8],
) -> IResult<&'a [u8], FlowRecord> {
    let mut record = FlowRecord::default();
    let mut priority = FieldPriority::default();
    let mut reverse = FlowRecord::default();
    let mut reverse_priority = FieldPriority::default();
    let mut saw_reverse = false;

    let mut remaining = input;

    for spec in template {
        let (after, raw) = read_field_bytes(remaining, spec.length)?;
        remaining = after;

        match spec.enterprise {
            None => apply_ie(spec.ie_id, raw, &mut record, &mut priority),
            Some(REVERSE_PEN) => {
                saw_reverse = true;
                apply_ie(spec.ie_id, raw, &mut reverse, &mut reverse_priority);
            }
            Some(_) => {}
        }
    }

    record.reverse = saw_reverse.then_some(ReverseCounters {
        bytes_delta: reverse.bytes_delta,
        packets_delta: reverse.packets_delta,
        bytes_total: reverse.bytes_total,
        packets_total: reverse.packets_total,
    });

    Ok((remaining, record))
}

/// Read a field's bytes, handling the variable-length encoding if necessary
fn read_field_bytes(input: &[u8], declared_length: u16) -> IResult<&[u8], &[u8]> {
    if declared_length != VARIABLE_LENGTH {
        return take(declared_length)(input);
    }
    // variable length: 1-byte length, with 0xFF sentinel switching to 2-byte length
    let (input, first) = be_u8(input)?;
    if first == 0xFF {
        let (input, actual_len) = be_u16(input)?;
        return take(actual_len)(input);
    }
    take(first)(input)
}

/// Apply a single IE to the flow record, using the field priority to resolve conflicts
fn apply_ie(ie_id: u16, raw: &[u8], record: &mut FlowRecord, priority: &mut FieldPriority) {
    if apply_delta_counter_ie(ie_id, raw, record, priority)
        || apply_total_counter_ie(ie_id, raw, record, priority)
        || apply_timestamp_ie(ie_id, raw, record, priority)
        || apply_mac_ie(ie_id, raw, record, priority)
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
        ie::FLOW_DIRECTION => {
            // 0x00 is ingress, 0x01 is egress
            if let Some(dir) = match raw.first() {
                Some(0x00) => Some(TrafficDirection::Incoming),
                Some(0x01) => Some(TrafficDirection::Outgoing),
                _ => None,
            } {
                record.direction = Some(dir);
            }
        }
        _ => {}
    }
}

/// Apply a delta counter IE, returning whether `ie_id` was one
fn apply_delta_counter_ie(
    ie_id: u16,
    raw: &[u8],
    record: &mut FlowRecord,
    priority: &mut FieldPriority,
) -> bool {
    let (rank, slot, slot_priority) = match ie_id {
        ie::OCTET_DELTA_COUNT
        | ie::LAYER2_OCTET_DELTA_COUNT
        | ie::POST_OCTET_DELTA_COUNT
        | ie::POST_LAYER2_OCTET_DELTA_COUNT => (
            bytes_delta_rank(ie_id),
            &mut record.bytes_delta,
            &mut priority.bytes_delta,
        ),
        ie::PACKET_DELTA_COUNT | ie::POST_PACKET_DELTA_COUNT => (
            packets_delta_rank(ie_id),
            &mut record.packets_delta,
            &mut priority.packets_delta,
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

/// Apply a cumulative counter IE, returning whether `ie_id` was one
fn apply_total_counter_ie(
    ie_id: u16,
    raw: &[u8],
    record: &mut FlowRecord,
    priority: &mut FieldPriority,
) -> bool {
    let (rank, slot, slot_priority) = match ie_id {
        ie::OCTET_TOTAL_COUNT
        | ie::LAYER2_OCTET_TOTAL_COUNT
        | ie::POST_OCTET_TOTAL_COUNT
        | ie::POST_LAYER2_OCTET_TOTAL_COUNT => (
            bytes_total_rank(ie_id),
            &mut record.bytes_total,
            &mut priority.bytes_total,
        ),
        ie::PACKET_TOTAL_COUNT | ie::POST_PACKET_TOTAL_COUNT => (
            packets_total_rank(ie_id),
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

/// Apply a flow start timestamp or flow end timestamp IE, returning whether `ie_id` was one
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

/// Apply a MAC address IE, returning whether `ie_id` was one
fn apply_mac_ie(
    ie_id: u16,
    raw: &[u8],
    record: &mut FlowRecord,
    priority: &mut FieldPriority,
) -> bool {
    let (slot, slot_priority) = match ie_id {
        ie::SOURCE_MAC_ADDRESS | ie::POST_SOURCE_MAC_ADDRESS => {
            (&mut record.src_mac, &mut priority.src_mac)
        }
        ie::DESTINATION_MAC_ADDRESS | ie::POST_DESTINATION_MAC_ADDRESS => {
            (&mut record.dst_mac, &mut priority.dst_mac)
        }
        _ => return false,
    };

    let rank = mac_rank(ie_id);
    if rank >= *slot_priority
        && let Some(v) = read_mac(raw)
    {
        *slot = Some(v);
        *slot_priority = rank;
    }
    true
}

/// Read a timestamp field using the encoding its IE prescribes
fn read_timestamp(ie_id: u16, raw: &[u8]) -> Option<Timestamp> {
    match ie_id {
        ie::FLOW_START_SECONDS | ie::FLOW_END_SECONDS => read_timestamp_secs(raw),
        ie::FLOW_START_MILLISECONDS | ie::FLOW_END_MILLISECONDS => read_timestamp_ms(raw),
        ie::FLOW_START_MICROSECONDS
        | ie::FLOW_END_MICROSECONDS
        | ie::FLOW_START_NANOSECONDS
        | ie::FLOW_END_NANOSECONDS => read_timestamp_ntp(raw),
        _ => None,
    }
}

/// IPFIX `dateTimeSeconds` is 4 bytes big-endian, seconds since UNIX epoch
fn read_timestamp_secs(raw: &[u8]) -> Option<Timestamp> {
    if raw.len() != 4 {
        return None;
    }
    let secs = u32::from_be_bytes(raw.try_into().ok()?);
    Some(Timestamp::new(i64::from(secs), 0))
}

/// IPFIX `dateTimeMilliseconds` is 8 bytes big-endian, ms since UNIX epoch
fn read_timestamp_ms(raw: &[u8]) -> Option<Timestamp> {
    if raw.len() != 8 {
        return None;
    }
    let ms = u64::from_be_bytes(raw.try_into().ok()?);
    let secs = i64::try_from(ms / 1_000).ok()?;
    let usecs = i64::try_from((ms % 1_000) * 1_000).ok()?;
    Some(Timestamp::new(secs, usecs))
}

/// Seconds between the NTP epoch (1900-01-01) and the UNIX epoch (1970-01-01)
const NTP_UNIX_OFFSET: u64 = 2_208_988_800;

/// IPFIX `dateTimeMicroseconds` and `dateTimeNanoseconds` are 8-byte NTP timestamps:
/// 32 bits of seconds since 1900 followed by a 32-bit binary fraction of a second
fn read_timestamp_ntp(raw: &[u8]) -> Option<Timestamp> {
    if raw.len() != 8 {
        return None;
    }
    let ntp = u64::from_be_bytes(raw.try_into().ok()?);
    let ntp_secs = ntp >> 32;
    let fraction = ntp & 0xFFFF_FFFF;
    // pre-1970 timestamps aren't representable as a UNIX instant
    let secs = i64::try_from(ntp_secs.checked_sub(NTP_UNIX_OFFSET)?).ok()?;
    let usecs = i64::try_from((fraction * 1_000_000) >> 32).ok()?;
    Some(Timestamp::new(secs, usecs))
}

/// Read a big-endian unsigned integer of 1 to 8 bytes into a `u128`
fn read_unsigned(raw: &[u8]) -> Option<u128> {
    if raw.is_empty() || raw.len() > 8 {
        return None;
    }
    let mut buf = [0u8; 8];
    buf[8 - raw.len()..].copy_from_slice(raw);
    Some(u128::from(u64::from_be_bytes(buf)))
}

/// Read a big-endian unsigned integer of 1 or 2 bytes into a `u16`
fn read_u16(raw: &[u8]) -> Option<u16> {
    match raw.len() {
        1 => Some(u16::from(raw[0])),
        2 => Some(u16::from_be_bytes([raw[0], raw[1]])),
        _ => None,
    }
}

/// Read a big-endian IPv4 address from 4 bytes
fn read_ipv4(raw: &[u8]) -> Option<IpAddr> {
    if raw.len() != 4 {
        return None;
    }
    Some(IpAddr::V4(Ipv4Addr::new(raw[0], raw[1], raw[2], raw[3])))
}

/// Read a big-endian IPv6 address from 16 bytes
fn read_ipv6(raw: &[u8]) -> Option<IpAddr> {
    if raw.len() != 16 {
        return None;
    }
    let mut octets = [0u8; 16];
    octets.copy_from_slice(raw);
    Some(IpAddr::V6(Ipv6Addr::from(octets)))
}

/// Read a big-endian MAC address from 6 bytes, returning `None` for all-zero addresses
fn read_mac(raw: &[u8]) -> Option<[u8; 6]> {
    if raw.len() != 6 || raw.iter().all(|b| *b == 0) {
        return None;
    }
    let mut mac = [0u8; 6];
    mac.copy_from_slice(raw);
    Some(mac)
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
        let tset_len_bytes = tset_len.to_be_bytes();
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
        assert_eq!(record.bytes_delta, Some(1500));
        assert_eq!(record.packets_delta, Some(10));
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
    fn reverse_pen_ies_decode_into_the_reverse_counters() {
        // Same IE ids as the forward counters, under PEN 29305.
        let reverse_spec = |ie_id: u16| FieldSpec {
            ie_id,
            length: 8,
            enterprise: Some(REVERSE_PEN),
        };
        let template = [
            FieldSpec {
                ie_id: ie::OCTET_DELTA_COUNT,
                length: 8,
                enterprise: None,
            },
            reverse_spec(ie::OCTET_DELTA_COUNT),
            reverse_spec(ie::PACKET_DELTA_COUNT),
        ];
        let mut payload = 1500u64.to_be_bytes().to_vec();
        payload.extend_from_slice(&9000u64.to_be_bytes());
        payload.extend_from_slice(&60u64.to_be_bytes());

        let (rest, record) = decode_data_record(&template, &payload).unwrap();
        assert!(rest.is_empty());
        // the forward slots must be untouched by the reverse fields
        assert_eq!(record.bytes_delta, Some(1500));
        assert_eq!(record.packets_delta, None);
        assert_eq!(
            record.reverse,
            Some(ReverseCounters {
                bytes_delta: Some(9000),
                packets_delta: Some(60),
                bytes_total: None,
                packets_total: None,
            })
        );
    }

    #[test]
    fn a_uniflow_record_has_no_reverse_counters() {
        let record = decode(&[(ie::OCTET_DELTA_COUNT, 8)], &1500u64.to_be_bytes());
        assert_eq!(record.bytes_delta, Some(1500));
        assert_eq!(record.reverse, None, "not a biflow");
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
            .bytes_delta,
            Some(1500),
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
            .bytes_delta,
            Some(1500),
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
        assert_eq!(record.bytes_delta, None);
        assert_eq!(record.packets_delta, None);
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
    fn post_counters_are_used_when_no_others_are_exported() {
        // An exporter observing the flow after a middlebox may carry only the
        // post counters; without them the record would decode to zero bytes and
        // the collector would drop it outright.
        let payload = [0, 0, 0, 0, 0, 0, 0x05, 0xDC, 0, 0, 0, 0, 0, 0, 0, 10];
        let record = decode(
            &[
                (ie::POST_OCTET_DELTA_COUNT, 8),
                (ie::POST_PACKET_DELTA_COUNT, 8),
            ],
            &payload,
        );
        assert_eq!(record.bytes_delta, Some(1500));
        assert_eq!(record.packets_delta, Some(10));

        let record = decode(
            &[
                (ie::POST_OCTET_TOTAL_COUNT, 8),
                (ie::POST_PACKET_TOTAL_COUNT, 8),
            ],
            &payload,
        );
        assert_eq!(record.bytes_total, Some(1500));
        assert_eq!(record.packets_total, Some(10));
    }

    #[test]
    fn layer2_post_octets_win_over_ip_octets_in_either_order() {
        // The layer-2 preference outranks the pre/post one: a post-middlebox
        // frame count still includes the link header, an IP-layer count never
        // does.
        let l2_post = [0, 0, 0, 0, 0, 0, 0x05, 0xDC]; // 1500
        let ip = [0, 0, 0, 0, 0, 0, 0x03, 0xE8]; // 1000

        let mut l2_first = Vec::new();
        l2_first.extend_from_slice(&l2_post);
        l2_first.extend_from_slice(&ip);
        assert_eq!(
            decode(
                &[
                    (ie::POST_LAYER2_OCTET_DELTA_COUNT, 8),
                    (ie::OCTET_DELTA_COUNT, 8)
                ],
                &l2_first,
            )
            .bytes_delta,
            Some(1500),
        );

        let mut ip_first = Vec::new();
        ip_first.extend_from_slice(&ip);
        ip_first.extend_from_slice(&l2_post);
        assert_eq!(
            decode(
                &[
                    (ie::OCTET_TOTAL_COUNT, 8),
                    (ie::POST_LAYER2_OCTET_TOTAL_COUNT, 8)
                ],
                &ip_first,
            )
            .bytes_total,
            Some(1500),
        );
    }

    #[test]
    fn plain_layer2_octets_win_over_their_post_counterparts() {
        let plain = [0, 0, 0, 0, 0, 0, 0x05, 0xDC]; // 1500
        let post = [0, 0, 0, 0, 0, 0, 0x03, 0xE8]; // 1000

        let mut payload = Vec::new();
        payload.extend_from_slice(&post);
        payload.extend_from_slice(&plain);
        let record = decode(
            &[
                (ie::POST_LAYER2_OCTET_DELTA_COUNT, 8),
                (ie::LAYER2_OCTET_DELTA_COUNT, 8),
            ],
            &payload,
        );
        assert_eq!(record.bytes_delta, Some(1500));

        let record = decode(
            &[
                (ie::POST_LAYER2_OCTET_TOTAL_COUNT, 8),
                (ie::LAYER2_OCTET_TOTAL_COUNT, 8),
            ],
            &payload,
        );
        assert_eq!(record.bytes_total, Some(1500));
    }

    #[test]
    fn plain_deltas_win_over_post_deltas_in_either_order() {
        let plain = [0, 0, 0, 0, 0, 0, 0x05, 0xDC]; // 1500
        let post = [0, 0, 0, 0, 0, 0, 0x03, 0xE8]; // 1000

        let mut plain_first = Vec::new();
        plain_first.extend_from_slice(&plain);
        plain_first.extend_from_slice(&post);
        assert_eq!(
            decode(
                &[(ie::OCTET_DELTA_COUNT, 8), (ie::POST_OCTET_DELTA_COUNT, 8)],
                &plain_first,
            )
            .bytes_delta,
            Some(1500),
        );

        let mut post_first = Vec::new();
        post_first.extend_from_slice(&post);
        post_first.extend_from_slice(&plain);
        assert_eq!(
            decode(
                &[(ie::POST_OCTET_DELTA_COUNT, 8), (ie::OCTET_DELTA_COUNT, 8)],
                &post_first,
            )
            .bytes_delta,
            Some(1500),
        );
    }

    #[test]
    fn plain_macs_win_over_post_macs_in_either_order() {
        let plain = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let post = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        let mut plain_first = Vec::new();
        plain_first.extend_from_slice(&plain);
        plain_first.extend_from_slice(&post);
        assert_eq!(
            decode(
                &[
                    (ie::SOURCE_MAC_ADDRESS, 6),
                    (ie::POST_SOURCE_MAC_ADDRESS, 6)
                ],
                &plain_first,
            )
            .src_mac,
            Some(plain),
        );

        let mut post_first = Vec::new();
        post_first.extend_from_slice(&post);
        post_first.extend_from_slice(&plain);
        assert_eq!(
            decode(
                &[
                    (ie::POST_DESTINATION_MAC_ADDRESS, 6),
                    (ie::DESTINATION_MAC_ADDRESS, 6),
                ],
                &post_first,
            )
            .dst_mac,
            Some(plain),
        );
    }

    #[test]
    fn post_macs_are_used_when_no_others_are_exported() {
        let post = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
        let record = decode(&[(ie::POST_SOURCE_MAC_ADDRESS, 6)], &post);
        assert_eq!(record.src_mac, Some(post));
    }

    #[test]
    fn plain_totals_win_over_post_totals_in_either_order() {
        let plain = [0, 0, 0, 0, 0, 0, 0x05, 0xDC]; // 1500
        let post = [0, 0, 0, 0, 0, 0, 0x03, 0xE8]; // 1000

        let mut plain_first = Vec::new();
        plain_first.extend_from_slice(&plain);
        plain_first.extend_from_slice(&post);
        assert_eq!(
            decode(
                &[
                    (ie::PACKET_TOTAL_COUNT, 8),
                    (ie::POST_PACKET_TOTAL_COUNT, 8)
                ],
                &plain_first,
            )
            .packets_total,
            Some(1500),
        );

        let mut post_first = Vec::new();
        post_first.extend_from_slice(&post);
        post_first.extend_from_slice(&plain);
        assert_eq!(
            decode(
                &[
                    (ie::POST_PACKET_TOTAL_COUNT, 8),
                    (ie::PACKET_TOTAL_COUNT, 8)
                ],
                &post_first,
            )
            .packets_total,
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
