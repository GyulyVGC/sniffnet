//! RFC 7011 IPFIX wire format decoding

use crate::Protocol;
use crate::networking::ipfix::field_priority::{
    FieldPriority, bytes_delta_rank, bytes_total_rank, mac_rank, packets_delta_rank,
    packets_total_rank, timestamp_rank,
};
use crate::networking::ipfix::flow_record::{FlowRecord, ReverseCounters};
use crate::networking::ipfix::ie;
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
            if let Some(n) = raw.first()
                && let Some(proto) = Protocol::from_number(*n)
            {
                record.protocol = Some(proto);
            }
        }
        ie::SOURCE_TRANSPORT_PORT => {
            if let Some(v) = read_port(raw) {
                record.src_port = Some(v);
            }
        }
        ie::DESTINATION_TRANSPORT_PORT => {
            if let Some(v) = read_port(raw) {
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

/// Read a big-endian unsigned integer of 1 or 2 bytes into a `u16`, returning `None` for port 0
fn read_port(raw: &[u8]) -> Option<u16> {
    if raw.iter().all(|b| *b == 0) {
        return None;
    }
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

    /// 1500 as an 8-byte counter
    const C1500: [u8; 8] = [0, 0, 0, 0, 0, 0, 0x05, 0xDC];
    /// 1000 as an 8-byte counter
    const C1000: [u8; 8] = [0, 0, 0, 0, 0, 0, 0x03, 0xE8];
    const MAC_A: [u8; 6] = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    const MAC_B: [u8; 6] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

    /// Build a template of plain IANA fields from `(ie_id, length)` pairs
    fn template(fields: &[(u16, u16)]) -> Vec<FieldSpec> {
        fields
            .iter()
            .map(|(ie_id, length)| FieldSpec {
                ie_id: *ie_id,
                length: *length,
                enterprise: None,
            })
            .collect()
    }

    /// Decode `payload` against a template built from `(ie_id, length)` pairs
    fn decode(fields: &[(u16, u16)], payload: &[u8]) -> FlowRecord {
        decode_data_record(&template(fields), payload).unwrap().1
    }

    /// Wrap `body` in a set header carrying `set_id`
    fn set(set_id: u16, body: &[u8]) -> Vec<u8> {
        let length = u16::try_from(body.len() + 4).unwrap();
        [&set_id.to_be_bytes()[..], &length.to_be_bytes(), body].concat()
    }

    /// Wrap `sets` in an IPFIX message header (export time, sequence and odid all zero)
    fn datagram(sets: &[Vec<u8>]) -> Vec<u8> {
        let body = sets.concat();
        let length = u16::try_from(body.len() + 16).unwrap();
        [
            &IPFIX_VERSION.to_be_bytes()[..],
            &length.to_be_bytes(),
            &[0; 12],
            &body,
        ]
        .concat()
    }

    /// An 8-byte NTP timestamp: `unix_secs` past the UNIX epoch plus `fraction`
    fn ntp_bytes(unix_secs: u64, fraction: u32) -> [u8; 8] {
        let secs = u32::try_from(NTP_UNIX_OFFSET + unix_secs).unwrap();
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&secs.to_be_bytes());
        bytes[4..].copy_from_slice(&fraction.to_be_bytes());
        bytes
    }

    /// Assert that `winner` fills its record slot even when the template lists `loser` first
    fn assert_wins<T: PartialEq + std::fmt::Debug>(
        winner: (u16, &[u8]),
        loser: (u16, &[u8]),
        slot: fn(&FlowRecord) -> Option<T>,
        expected: &T,
    ) {
        for order in [[winner, loser], [loser, winner]] {
            let fields: Vec<(u16, u16)> = order
                .iter()
                .map(|(ie_id, raw)| (*ie_id, u16::try_from(raw.len()).unwrap()))
                .collect();
            let payload: Vec<u8> = order.iter().flat_map(|(_, raw)| *raw).copied().collect();
            let actual = slot(&decode(&fields, &payload));
            assert_eq!(actual.as_ref(), Some(expected), "template {fields:?}");
        }
    }

    #[test]
    fn test_parse_message_header() {
        // version, length=16, export_time=0xDEADBEEF, seq=0x01020304, odid=7
        let header = |version: u16| {
            [
                &version.to_be_bytes()[..],
                &16u16.to_be_bytes(),
                &0xDEAD_BEEFu32.to_be_bytes(),
                &0x0102_0304u32.to_be_bytes(),
                &7u32.to_be_bytes(),
            ]
            .concat()
        };

        let valid = header(IPFIX_VERSION);
        let (rest, hdr) = parse_message_header(&valid).unwrap();
        assert!(rest.is_empty());
        assert_eq!(hdr.version, IPFIX_VERSION);
        assert_eq!(hdr.length, 16);
        assert_eq!(hdr.export_time, 0xDEAD_BEEF);
        assert_eq!(hdr.sequence_number, 0x0102_0304);
        assert_eq!(hdr.observation_domain_id, 7);

        // a NetFlow v9 header is rejected
        assert!(parse_message_header(&header(9)).is_err());
        // so is a header cut short
        assert!(parse_message_header(&valid[..15]).is_err());
    }

    #[test]
    fn test_parse_message() {
        // a template (id 256: srcIPv4, dstIPv4, octetDelta, packetDelta) followed
        // by a data set carrying one record against it
        let template_body = [
            &256u16.to_be_bytes()[..],
            &4u16.to_be_bytes(),
            &[0, 8, 0, 4],
            &[0, 12, 0, 4],
            &[0, 1, 0, 8],
            &[0, 2, 0, 8],
        ]
        .concat();
        let record_body = [
            &[10, 0, 0, 1][..],
            &[192, 168, 1, 5],
            &C1500,
            &10u64.to_be_bytes(),
        ]
        .concat();
        let bytes = datagram(&[set(SET_ID_TEMPLATE, &template_body), set(256, &record_body)]);

        let (rest, message) = parse_message(&bytes).unwrap();
        assert!(rest.is_empty());
        assert_eq!(message.header.version, IPFIX_VERSION);
        assert_eq!(message.sets.len(), 2);

        let Set::Template(templates) = &message.sets[0] else {
            panic!("expected a template set, got {:?}", message.sets[0]);
        };
        let Set::Data {
            template_id,
            payload,
        } = &message.sets[1]
        else {
            panic!("expected a data set, got {:?}", message.sets[1]);
        };
        assert_eq!(*template_id, 256);

        let (rest, flow) = decode_data_record(&templates[0].fields, payload).unwrap();
        assert!(rest.is_empty());
        assert_eq!(flow.src_ip, Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
        assert_eq!(flow.dst_ip, Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5))));
        assert_eq!(
            (flow.bytes_delta, flow.packets_delta),
            (Some(1500), Some(10))
        );

        // bytes past the declared length belong to the caller, not to this message
        let padded = [bytes.clone(), vec![0xAA]].concat();
        let (rest, _) = parse_message(&padded).unwrap();
        assert_eq!(rest, &[0xAA][..]);

        // a datagram shorter than its header claims is an error, not a panic
        assert!(parse_message(&bytes[..bytes.len() - 1]).is_err());
    }

    #[test]
    fn test_parse_set() {
        // a template set is parsed into its records
        let body = [
            &256u16.to_be_bytes()[..],
            &1u16.to_be_bytes(),
            &[0, 8, 0, 4],
        ]
        .concat();
        let bytes = set(SET_ID_TEMPLATE, &body);
        let (rest, parsed) = parse_set(&bytes).unwrap();
        assert!(rest.is_empty());
        assert_eq!(
            parsed,
            Set::Template(vec![TemplateRecord {
                template_id: 256,
                fields: vec![FieldSpec {
                    ie_id: ie::SOURCE_IPV4_ADDRESS,
                    length: 4,
                    enterprise: None,
                }],
            }])
        );

        // a data set keeps its payload for the caller to decode against a template
        let bytes = set(MIN_DATA_SET_ID, &[0xDE, 0xAD]);
        let (_, parsed) = parse_set(&bytes).unwrap();
        assert_eq!(
            parsed,
            Set::Data {
                template_id: MIN_DATA_SET_ID,
                payload: &[0xDE, 0xAD],
            }
        );

        // ids below the data range other than 2 are skipped
        for set_id in [0, 1, 3, MIN_DATA_SET_ID - 1] {
            let bytes = set(set_id, &[0xDE, 0xAD]);
            let (rest, parsed) = parse_set(&bytes).unwrap();
            assert_eq!(parsed, Set::Ignored, "set id {set_id}");
            assert!(rest.is_empty(), "set id {set_id} left its body unconsumed");
        }

        // a set claiming less than its own 4-byte header doesn't underflow
        let (rest, parsed) = parse_set(&[0x00, 0x02, 0x00, 0x00, 0xAA]).unwrap();
        assert_eq!(parsed, Set::Template(vec![]));
        assert_eq!(rest, &[0xAA][..]);
    }

    #[test]
    fn test_parse_template_record() {
        // two records back to back in the same template set
        let bytes = [
            &256u16.to_be_bytes()[..],
            &1u16.to_be_bytes(),
            &[0, 8, 0, 4],
            &257u16.to_be_bytes(),
            &2u16.to_be_bytes(),
            &[0, 1, 0, 8],
            &[0, 2, 0, 8],
        ]
        .concat();

        let (rest, first) = parse_template_record(&bytes).unwrap();
        assert_eq!(first.template_id, 256);
        assert_eq!(
            first.fields,
            vec![FieldSpec {
                ie_id: ie::SOURCE_IPV4_ADDRESS,
                length: 4,
                enterprise: None,
            }]
        );

        let (rest, second) = parse_template_record(rest).unwrap();
        assert!(rest.is_empty());
        assert_eq!(second.template_id, 257);
        assert_eq!(second.fields.len(), 2);

        // a record announcing more fields than it carries is an error, not a panic
        let truncated = [
            &257u16.to_be_bytes()[..],
            &2u16.to_be_bytes(),
            &[0, 1, 0, 8],
        ]
        .concat();
        assert!(parse_template_record(&truncated).is_err());
    }

    #[test]
    fn test_parse_field_spec() {
        // a plain IANA field spec is 4 bytes
        let (rest, spec) = parse_field_spec(&[0x00, 0x08, 0x00, 0x04, 0xAA]).unwrap();
        assert_eq!(
            spec,
            FieldSpec {
                ie_id: ie::SOURCE_IPV4_ADDRESS,
                length: 4,
                enterprise: None,
            }
        );
        assert_eq!(rest, &[0xAA][..]);

        // the top bit of the IE id marks 4 further bytes of enterprise number
        let (rest, spec) =
            parse_field_spec(&[0x80, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x2A]).unwrap();
        assert_eq!(
            spec,
            FieldSpec {
                ie_id: ie::OCTET_DELTA_COUNT,
                length: 4,
                enterprise: Some(42),
            }
        );
        assert!(rest.is_empty());

        // the variable-length sentinel is carried through as the declared length
        let (_, spec) = parse_field_spec(&[0x00, 0x08, 0xFF, 0xFF]).unwrap();
        assert_eq!(spec.length, VARIABLE_LENGTH);
    }

    #[test]
    fn test_read_field_bytes() {
        // a fixed-length field takes exactly its declared length
        let (rest, bytes) = read_field_bytes(&[0xDE, 0xAD, 0xBE, 0xEF, 0xAA], 4).unwrap();
        assert_eq!(bytes, &[0xDE, 0xAD, 0xBE, 0xEF][..]);
        assert_eq!(rest, &[0xAA][..]);

        // variable length, short form: a 1-byte length prefix
        let (rest, bytes) =
            read_field_bytes(&[0x03, b'a', b'b', b'c', 0xAA], VARIABLE_LENGTH).unwrap();
        assert_eq!(bytes, b"abc");
        assert_eq!(rest, &[0xAA][..]);

        // variable length, long form: 0xFF switches to a 2-byte length prefix
        let long = [0xFF, 0x00, 0x04, b'w', b'x', b'y', b'z', 0xBB];
        let (rest, bytes) = read_field_bytes(&long, VARIABLE_LENGTH).unwrap();
        assert_eq!(bytes, b"wxyz");
        assert_eq!(rest, &[0xBB][..]);

        // a field longer than what's left is an error, not a panic
        assert!(read_field_bytes(&[0xDE, 0xAD], 4).is_err());
        assert!(read_field_bytes(&[0x04, 0xDE], VARIABLE_LENGTH).is_err());
    }

    #[test]
    fn test_decode_data_record() {
        let fields = [
            (ie::SOURCE_IPV4_ADDRESS, 4),
            (ie::DESTINATION_IPV4_ADDRESS, 4),
            (ie::SOURCE_TRANSPORT_PORT, 2),
            (ie::DESTINATION_TRANSPORT_PORT, 2),
            (ie::PROTOCOL_IDENTIFIER, 1),
            (ie::FLOW_DIRECTION, 1),
            (ie::SOURCE_MAC_ADDRESS, 6),
            (ie::DESTINATION_MAC_ADDRESS, 6),
            (ie::OCTET_DELTA_COUNT, 8),
            (ie::PACKET_DELTA_COUNT, 8),
            (ie::OCTET_TOTAL_COUNT, 8),
            (ie::PACKET_TOTAL_COUNT, 8),
            (ie::FLOW_START_SECONDS, 4),
            (ie::FLOW_END_SECONDS, 4),
        ];
        let payload = [
            &[10, 0, 0, 1][..],
            &[192, 168, 1, 5],
            &[0x01, 0xBB],
            &[0xC8, 0x22],
            &[6],
            &[0x01],
            &MAC_A,
            &MAC_B,
            &C1500,
            &10u64.to_be_bytes(),
            &9000u64.to_be_bytes(),
            &60u64.to_be_bytes(),
            &10u32.to_be_bytes(),
            &20u32.to_be_bytes(),
            // a data set holds its records back to back, so what follows is the next one
            &[0xAA],
        ]
        .concat();

        let (rest, record) = decode_data_record(&template(&fields), &payload).unwrap();
        assert_eq!(rest, &[0xAA][..]);
        assert_eq!(
            record,
            FlowRecord {
                src_ip: Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
                dst_ip: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5))),
                src_port: Some(443),
                dst_port: Some(51234),
                protocol: Some(Protocol::Tcp),
                bytes_delta: Some(1500),
                packets_delta: Some(10),
                bytes_total: Some(9000),
                packets_total: Some(60),
                src_mac: Some(MAC_A),
                dst_mac: Some(MAC_B),
                direction: Some(TrafficDirection::Outgoing),
                flow_start: Some(Timestamp::new(10, 0)),
                flow_end: Some(Timestamp::new(20, 0)),
                reverse: None,
            }
        );
    }

    #[test]
    fn test_decode_data_record_enterprise_ies() {
        let spec = |ie_id: u16, enterprise: Option<u32>| FieldSpec {
            ie_id,
            length: 8,
            enterprise,
        };

        // a vendor IE is skipped, but still consumes its bytes
        let fields = [
            spec(ie::OCTET_DELTA_COUNT, Some(42)),
            spec(ie::OCTET_DELTA_COUNT, None),
        ];
        let payload = [C1000, C1500].concat();
        let (rest, record) = decode_data_record(&fields, &payload).unwrap();
        assert!(rest.is_empty());
        assert_eq!(record.bytes_delta, Some(1500));
        assert_eq!(record.reverse, None, "a vendor PEN is not a biflow");

        // RFC 5103 reverse IEs fill the reverse counters, leaving the forward ones alone
        let fields = [
            spec(ie::OCTET_DELTA_COUNT, None),
            spec(ie::OCTET_DELTA_COUNT, Some(REVERSE_PEN)),
            spec(ie::PACKET_DELTA_COUNT, Some(REVERSE_PEN)),
        ];
        let payload = [&C1500[..], &C1000, &60u64.to_be_bytes()].concat();
        let (_, record) = decode_data_record(&fields, &payload).unwrap();
        assert_eq!(record.bytes_delta, Some(1500));
        assert_eq!(record.packets_delta, None);
        assert_eq!(
            record.reverse,
            Some(ReverseCounters {
                bytes_delta: Some(1000),
                packets_delta: Some(60),
                bytes_total: None,
                packets_total: None,
            })
        );

        // a uniflow has no reverse half at all
        assert_eq!(decode(&[(ie::OCTET_DELTA_COUNT, 8)], &C1500).reverse, None);
    }

    #[test]
    fn test_decode_data_record_malformed_field() {
        // srcIPv4 declared as 3 bytes can't be read, but the field still consumes
        // its declared length, so every field after it stays aligned
        let payload = [10, 0, 0, 192, 168, 1, 5, 0x01, 0xBB];
        let record = decode(
            &[
                (ie::SOURCE_IPV4_ADDRESS, 3),
                (ie::DESTINATION_IPV4_ADDRESS, 4),
                (ie::SOURCE_TRANSPORT_PORT, 2),
            ],
            &payload,
        );
        assert_eq!(record.src_ip, None);
        assert_eq!(
            record.dst_ip,
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5)))
        );
        assert_eq!(record.src_port, Some(443));
    }

    #[test]
    fn test_apply_ie() {
        for (raw, expected) in [
            (6, Some(Protocol::Tcp)),
            (17, Some(Protocol::Udp)),
            (1, Some(Protocol::Icmpv4)),
            (58, Some(Protocol::Icmpv6)),
            (47, None),
        ] {
            let record = decode(&[(ie::PROTOCOL_IDENTIFIER, 1)], &[raw]);
            assert_eq!(record.protocol, expected, "protocolIdentifier {raw}");
        }

        // 0x00 is ingress, 0x01 is egress, anything else is undefined
        for (raw, expected) in [
            (0x00, Some(TrafficDirection::Incoming)),
            (0x01, Some(TrafficDirection::Outgoing)),
            (0xFF, None),
        ] {
            let record = decode(&[(ie::FLOW_DIRECTION, 1)], &[raw]);
            assert_eq!(record.direction, expected, "flowDirection {raw:#04x}");
        }

        let record = decode(
            &[
                (ie::SOURCE_TRANSPORT_PORT, 2),
                (ie::DESTINATION_TRANSPORT_PORT, 2),
            ],
            &[0x01, 0xBB, 0xC8, 0x22],
        );
        assert_eq!((record.src_port, record.dst_port), (Some(443), Some(51234)));

        let src = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        let dst = Ipv6Addr::LOCALHOST;
        let record = decode(
            &[
                (ie::SOURCE_IPV6_ADDRESS, 16),
                (ie::DESTINATION_IPV6_ADDRESS, 16),
            ],
            &[src.octets(), dst.octets()].concat(),
        );
        assert_eq!(
            (record.src_ip, record.dst_ip),
            (Some(IpAddr::V6(src)), Some(IpAddr::V6(dst)))
        );

        // an IE Sniffnet doesn't decode leaves the record untouched
        let record = decode(&[(999, 4)], &[0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(record, FlowRecord::default());
    }

    #[test]
    fn test_apply_delta_counter_ie() {
        let bytes_delta: fn(&FlowRecord) -> Option<u128> = |r| r.bytes_delta;
        assert_wins(
            (ie::LAYER2_OCTET_DELTA_COUNT, &C1500),
            (ie::POST_LAYER2_OCTET_DELTA_COUNT, &C1000),
            bytes_delta,
            &1500,
        );
        assert_wins(
            (ie::POST_LAYER2_OCTET_DELTA_COUNT, &C1500),
            (ie::OCTET_DELTA_COUNT, &C1000),
            bytes_delta,
            &1500,
        );
        assert_wins(
            (ie::OCTET_DELTA_COUNT, &C1500),
            (ie::POST_OCTET_DELTA_COUNT, &C1000),
            bytes_delta,
            &1500,
        );
        assert_wins(
            (ie::PACKET_DELTA_COUNT, &C1500),
            (ie::POST_PACKET_DELTA_COUNT, &C1000),
            |r| r.packets_delta,
            &1500,
        );

        let record = decode(
            &[
                (ie::POST_OCTET_DELTA_COUNT, 8),
                (ie::POST_PACKET_DELTA_COUNT, 8),
            ],
            &[C1500, C1000].concat(),
        );
        assert_eq!(
            (record.bytes_delta, record.packets_delta),
            (Some(1500), Some(1000))
        );
        assert_eq!((record.bytes_total, record.packets_total), (None, None));

        // RFC 7011 reduced-size encoding: a counter may be narrowed to fewer bytes
        let record = decode(&[(ie::OCTET_DELTA_COUNT, 4)], &[0x00, 0x00, 0x05, 0xDC]);
        assert_eq!(record.bytes_delta, Some(1500));
    }

    #[test]
    fn test_apply_total_counter_ie() {
        let bytes_total: fn(&FlowRecord) -> Option<u128> = |r| r.bytes_total;
        assert_wins(
            (ie::LAYER2_OCTET_TOTAL_COUNT, &C1500),
            (ie::POST_LAYER2_OCTET_TOTAL_COUNT, &C1000),
            bytes_total,
            &1500,
        );
        assert_wins(
            (ie::POST_LAYER2_OCTET_TOTAL_COUNT, &C1500),
            (ie::OCTET_TOTAL_COUNT, &C1000),
            bytes_total,
            &1500,
        );
        assert_wins(
            (ie::OCTET_TOTAL_COUNT, &C1500),
            (ie::POST_OCTET_TOTAL_COUNT, &C1000),
            bytes_total,
            &1500,
        );
        assert_wins(
            (ie::PACKET_TOTAL_COUNT, &C1500),
            (ie::POST_PACKET_TOTAL_COUNT, &C1000),
            |r| r.packets_total,
            &1500,
        );

        let record = decode(
            &[
                (ie::POST_OCTET_TOTAL_COUNT, 8),
                (ie::POST_PACKET_TOTAL_COUNT, 8),
            ],
            &[C1500, C1000].concat(),
        );
        assert_eq!(
            (record.bytes_total, record.packets_total),
            (Some(1500), Some(1000))
        );
        assert_eq!((record.bytes_delta, record.packets_delta), (None, None));
    }

    #[test]
    fn test_apply_timestamp_ie() {
        let secs = 10u32.to_be_bytes();
        let millis = 20_000u64.to_be_bytes();
        let micros = ntp_bytes(30, 0);
        let nanos = ntp_bytes(40, 0);

        let flow_start: fn(&FlowRecord) -> Option<Timestamp> = |r| r.flow_start;
        assert_wins(
            (ie::FLOW_START_MICROSECONDS, &micros),
            (ie::FLOW_START_MILLISECONDS, &millis),
            flow_start,
            &Timestamp::new(30, 0),
        );
        assert_wins(
            (ie::FLOW_START_MILLISECONDS, &millis),
            (ie::FLOW_START_SECONDS, &secs),
            flow_start,
            &Timestamp::new(20, 0),
        );
        assert_wins(
            (ie::FLOW_END_NANOSECONDS, &nanos),
            (ie::FLOW_END_MICROSECONDS, &micros),
            |r| r.flow_end,
            &Timestamp::new(40, 0),
        );

        let record = decode(
            &[(ie::FLOW_START_SECONDS, 4), (ie::FLOW_END_SECONDS, 4)],
            &[0, 0, 0, 10, 0, 0, 0, 20],
        );
        assert_eq!(record.flow_start, Some(Timestamp::new(10, 0)));
        assert_eq!(record.flow_end, Some(Timestamp::new(20, 0)));
    }

    #[test]
    fn test_apply_mac_ie() {
        assert_wins(
            (ie::SOURCE_MAC_ADDRESS, &MAC_A),
            (ie::POST_SOURCE_MAC_ADDRESS, &MAC_B),
            |r| r.src_mac,
            &MAC_A,
        );
        assert_wins(
            (ie::DESTINATION_MAC_ADDRESS, &MAC_A),
            (ie::POST_DESTINATION_MAC_ADDRESS, &MAC_B),
            |r| r.dst_mac,
            &MAC_A,
        );

        assert_eq!(
            decode(&[(ie::POST_SOURCE_MAC_ADDRESS, 6)], &MAC_B).src_mac,
            Some(MAC_B)
        );

        let record = decode(
            &[
                (ie::POST_SOURCE_MAC_ADDRESS, 6),
                (ie::SOURCE_MAC_ADDRESS, 6),
            ],
            &[&MAC_B[..], &[0; 6]].concat(),
        );
        assert_eq!(record.src_mac, Some(MAC_B));
    }

    #[test]
    fn test_read_timestamp() {
        let secs = 10u32.to_be_bytes();
        let millis = 20_500u64.to_be_bytes();
        let ntp = ntp_bytes(30, 0x8000_0000);

        for ie_id in [ie::FLOW_START_SECONDS, ie::FLOW_END_SECONDS] {
            assert_eq!(read_timestamp(ie_id, &secs), Some(Timestamp::new(10, 0)));
        }
        for ie_id in [ie::FLOW_START_MILLISECONDS, ie::FLOW_END_MILLISECONDS] {
            assert_eq!(
                read_timestamp(ie_id, &millis),
                Some(Timestamp::new(20, 500_000))
            );
        }
        for ie_id in [
            ie::FLOW_START_MICROSECONDS,
            ie::FLOW_END_MICROSECONDS,
            ie::FLOW_START_NANOSECONDS,
            ie::FLOW_END_NANOSECONDS,
        ] {
            assert_eq!(
                read_timestamp(ie_id, &ntp),
                Some(Timestamp::new(30, 500_000))
            );
        }

        assert_eq!(read_timestamp(ie::OCTET_DELTA_COUNT, &millis), None);
    }

    #[test]
    fn test_read_timestamp_secs() {
        assert_eq!(
            read_timestamp_secs(&10u32.to_be_bytes()),
            Some(Timestamp::new(10, 0))
        );
        assert_eq!(
            read_timestamp_secs(&u32::MAX.to_be_bytes()),
            Some(Timestamp::new(i64::from(u32::MAX), 0))
        );
        assert_eq!(read_timestamp_secs(&[0, 0, 10]), None);
        assert_eq!(read_timestamp_secs(&10u64.to_be_bytes()), None);
    }

    #[test]
    fn test_read_timestamp_ms() {
        assert_eq!(
            read_timestamp_ms(&20_500u64.to_be_bytes()),
            Some(Timestamp::new(20, 500_000))
        );
        assert_eq!(
            read_timestamp_ms(&0u64.to_be_bytes()),
            Some(Timestamp::new(0, 0))
        );
        assert_eq!(read_timestamp_ms(&20u32.to_be_bytes()), None);
    }

    #[test]
    fn test_read_timestamp_ntp() {
        assert_eq!(
            read_timestamp_ntp(&ntp_bytes(20, 0x8000_0000)),
            Some(Timestamp::new(20, 500_000))
        );
        assert_eq!(
            read_timestamp_ntp(&ntp_bytes(0, 0)),
            Some(Timestamp::new(0, 0))
        );
        assert_eq!(
            read_timestamp_ntp(&ntp_bytes(0, u32::MAX)),
            Some(Timestamp::new(0, 999_999))
        );
        assert_eq!(read_timestamp_ntp(&1_000u64.to_be_bytes()), None);
        assert_eq!(read_timestamp_ntp(&[0; 4]), None);
    }

    #[test]
    fn test_read_unsigned() {
        assert_eq!(read_unsigned(&C1500), Some(1500));
        // RFC 7011 reduced-size encoding: any width from 1 to 8 bytes
        assert_eq!(read_unsigned(&[0x05, 0xDC]), Some(1500));
        assert_eq!(read_unsigned(&[0x00, 0x00, 0x05, 0xDC]), Some(1500));
        assert_eq!(read_unsigned(&[0xFF]), Some(255));
        assert_eq!(read_unsigned(&[0xFF; 8]), Some(u128::from(u64::MAX)));
        assert_eq!(read_unsigned(&[]), None);
        assert_eq!(read_unsigned(&[0; 9]), None);
    }

    #[test]
    fn test_read_port() {
        assert_eq!(read_port(&[0x01, 0xBB]), Some(443));
        // a port narrowed to a single byte by reduced-size encoding
        assert_eq!(read_port(&[0xFF]), Some(255));
        assert_eq!(read_port(&[]), None);
        assert_eq!(read_port(&[0, 0, 1]), None);
        // port 0 is invalid
        assert_eq!(read_port(&[0, 0]), None);
        assert_eq!(read_port(&[0]), None);
    }

    #[test]
    fn test_read_ipv4() {
        assert_eq!(
            read_ipv4(&[10, 0, 0, 1]),
            Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)))
        );
        assert_eq!(read_ipv4(&[10, 0, 0]), None);
        assert_eq!(read_ipv4(&[10, 0, 0, 1, 0]), None);
    }

    #[test]
    fn test_read_ipv6() {
        let addr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
        assert_eq!(read_ipv6(&addr.octets()), Some(IpAddr::V6(addr)));
        assert_eq!(read_ipv6(&addr.octets()[..15]), None);
        // an IPv4 address is never widened into this slot
        assert_eq!(read_ipv6(&[10, 0, 0, 1]), None);
    }

    #[test]
    fn test_read_mac() {
        assert_eq!(read_mac(&MAC_A), Some(MAC_A));
        // `sniffnet-agent` writes all-zero when a flow carries no link header
        assert_eq!(read_mac(&[0; 6]), None);
        assert_eq!(read_mac(&MAC_A[..5]), None);
    }
}
