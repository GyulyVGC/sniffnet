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
use nom::multi::many0;
use nom::number::complete::{be_u8, be_u16, be_u32};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub const IPFIX_VERSION: u16 = 0x000A;
pub const SET_ID_TEMPLATE: u16 = 2;
pub const SET_ID_OPTIONS_TEMPLATE: u16 = 3;
pub const MIN_DATA_SET_ID: u16 = 256;
pub const VARIABLE_LENGTH: u16 = 0xFFFF;

#[allow(dead_code)]
pub mod ie {
    //! IANA-assigned IPFIX Information Element identifiers used by Sniffnet.
    //!
    //! Constants for flow-timestamp IEs are kept even though we don't decode
    //! them today — they document which fields a future maintainer would wire
    //! up if per-flow timestamps from the exporter become useful.
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
    pub const POST_DESTINATION_MAC_ADDRESS: u16 = 80;
    pub const POST_SOURCE_MAC_ADDRESS: u16 = 81;
    pub const OCTET_TOTAL_COUNT: u16 = 85;
    pub const PACKET_TOTAL_COUNT: u16 = 86;
    pub const FLOW_START_SECONDS: u16 = 150;
    pub const FLOW_END_SECONDS: u16 = 151;
    pub const FLOW_START_MILLISECONDS: u16 = 152;
    pub const FLOW_END_MILLISECONDS: u16 = 153;
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlowRecord {
    pub src_ip: Option<IpAddr>,
    pub dst_ip: Option<IpAddr>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Option<u8>,
    pub bytes: u128,
    pub packets: u128,
    pub src_mac: Option<[u8; 6]>,
    pub dst_mac: Option<[u8; 6]>,
}

/// Parse a complete IPFIX message (header + sets).
pub fn parse_message(input: &[u8]) -> IResult<&[u8], IpfixMessage<'_>> {
    let (input, header) = parse_message_header(input)?;
    // header.length is the total message length including the 16-byte header
    let payload_len = (header.length as usize).saturating_sub(16);
    let (rest, payload) = take(payload_len)(input)?;
    let (_, sets) = many0(parse_set).parse(payload)?;
    Ok((rest, IpfixMessage { header, sets }))
}

fn parse_message_header(input: &[u8]) -> IResult<&[u8], MessageHeader> {
    let (input, version) = be_u16(input)?;
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
        _ => Set::OptionsTemplate,
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
    let mut remaining = input;

    for spec in template {
        let (after, raw) = read_field_bytes(remaining, spec.length)?;
        remaining = after;

        // Enterprise-specific IEs and unknown IEs are skipped; the bytes were
        // already consumed above by `read_field_bytes`.
        if spec.enterprise.is_some() {
            continue;
        }

        apply_ie(spec.ie_id, raw, &mut record);
    }

    Ok((remaining, record))
}

/// Read the bytes belonging to a single field, accounting for the
/// variable-length encoding (RFC 7011 §7).
fn read_field_bytes(input: &[u8], declared_length: u16) -> IResult<&[u8], &[u8]> {
    if declared_length != VARIABLE_LENGTH {
        return take(declared_length as usize)(input);
    }
    // Variable length: 1-byte length, with 0xFF sentinel switching to 2-byte length
    let (input, first) = be_u8(input)?;
    let actual_len = if first == 0xFF {
        let (input2, real) = be_u16(input)?;
        let (input2, bytes) = take(real as usize)(input2)?;
        return Ok((input2, bytes));
    } else {
        first as usize
    };
    let (input, bytes) = take(actual_len)(input)?;
    Ok((input, bytes))
}

fn apply_ie(ie_id: u16, raw: &[u8], record: &mut FlowRecord) {
    match ie_id {
        ie::OCTET_DELTA_COUNT | ie::OCTET_TOTAL_COUNT => {
            if let Some(v) = read_unsigned(raw) {
                record.bytes = v;
            }
        }
        ie::PACKET_DELTA_COUNT | ie::PACKET_TOTAL_COUNT => {
            if let Some(v) = read_unsigned(raw) {
                record.packets = v;
            }
        }
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
        ie::POST_DESTINATION_MAC_ADDRESS => {
            if let Some(v) = read_mac(raw) {
                record.dst_mac = Some(v);
            }
        }
        _ => {}
    }
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

fn read_mac(raw: &[u8]) -> Option<[u8; 6]> {
    if raw.len() != 6 {
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
}
