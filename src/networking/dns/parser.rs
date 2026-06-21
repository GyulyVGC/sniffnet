//! Manual parser for the DNS message format (RFC 1035), operating directly on
//! the raw bytes of a UDP/TCP payload.
//!
//! No external DNS parsing crate is used: every field is read by hand from the
//! byte slice. The parser is defensive — any malformed or truncated input
//! yields `None` (or a partial result) instead of panicking.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::networking::dns::types::{
    DnsFlags, DnsMessage, DnsQuestion, DnsRCode, DnsRData, DnsRecord, DnsRecordType,
};

/// Maximum number of compression-pointer jumps allowed while decoding a single
/// name, to guard against maliciously crafted pointer loops.
const MAX_NAME_JUMPS: usize = 64;

/// Parses a DNS message from the bytes of a transport payload.
///
/// `buf` must start at the DNS header (for TCP, strip the 2-byte length prefix
/// beforehand). Returns `None` if the input is too short or the header/question
/// section is malformed.
pub fn parse_dns(buf: &[u8]) -> Option<DnsMessage> {
    // The header is a fixed 12 bytes.
    if buf.len() < 12 {
        return None;
    }

    let id = u16::from_be_bytes([buf[0], buf[1]]);
    let flags = u16::from_be_bytes([buf[2], buf[3]]);
    let qr = (flags >> 15) & 1;
    let opcode = ((flags >> 11) & 0x0F) as u8;
    let aa = (flags >> 10) & 1 == 1;
    let tc = (flags >> 9) & 1 == 1;
    let rd = (flags >> 8) & 1 == 1;
    let ra = (flags >> 7) & 1 == 1;
    let rcode = (flags & 0x0F) as u8;
    let qdcount = u16::from_be_bytes([buf[4], buf[5]]);
    let ancount = u16::from_be_bytes([buf[6], buf[7]]);
    // NSCOUNT (buf[8..10]) and ARCOUNT (buf[10..12]) are not parsed.

    let mut pos = 12;

    // Question section.
    let mut questions = Vec::with_capacity(qdcount as usize);
    for _ in 0..qdcount {
        let (name, next) = read_name(buf, pos)?;
        pos = next;
        let qtype = read_u16(buf, pos)?;
        pos += 2;
        let qclass = read_u16(buf, pos)?;
        pos += 2;
        questions.push(DnsQuestion {
            name,
            qtype: DnsRecordType::from_u16(qtype),
            qclass,
        });
    }

    // Answer section. Tolerate truncation: stop at the first record we cannot
    // fully read, keeping whatever we already decoded.
    let mut answers = Vec::with_capacity(ancount as usize);
    for _ in 0..ancount {
        match read_record(buf, pos) {
            Some((record, next)) => {
                pos = next;
                answers.push(record);
            }
            None => break,
        }
    }

    Some(DnsMessage {
        id,
        is_response: qr == 1,
        opcode,
        flags: DnsFlags { aa, tc, rd, ra },
        rcode: DnsRCode::from_u8(rcode),
        questions,
        answers,
    })
}

/// Decodes a DNS name starting at `start`, following compression pointers
/// (RFC 1035 §4.1.4). Returns the decoded name and the position immediately
/// after the name in the byte stream (i.e. after the first pointer, if any).
fn read_name(buf: &[u8], start: usize) -> Option<(String, usize)> {
    let mut labels: Vec<String> = Vec::new();
    let mut pos = start;
    // Position to resume reading the outer stream once the name is decoded.
    // Set when the first pointer is encountered; otherwise it is the byte after
    // the terminating zero length.
    let mut next_pos: Option<usize> = None;
    let mut jumps = 0;

    loop {
        let len = *buf.get(pos)?;
        match len & 0xC0 {
            // Regular label: top two bits are 00.
            0x00 => {
                if len == 0 {
                    pos += 1;
                    next_pos.get_or_insert(pos);
                    break;
                }
                let label_len = len as usize;
                let label = buf.get(pos + 1..pos + 1 + label_len)?;
                labels.push(String::from_utf8_lossy(label).into_owned());
                pos += 1 + label_len;
            }
            // Compression pointer: top two bits are 11.
            0xC0 => {
                let second = *buf.get(pos + 1)?;
                let offset = (((len & 0x3F) as usize) << 8) | second as usize;
                next_pos.get_or_insert(pos + 2);
                jumps += 1;
                if jumps > MAX_NAME_JUMPS || offset >= buf.len() {
                    return None;
                }
                pos = offset;
            }
            // 0x40 and 0x80 are reserved and must not appear.
            _ => return None,
        }
    }

    let name = if labels.is_empty() {
        ".".to_string()
    } else {
        labels.join(".")
    };
    Some((name, next_pos?))
}

/// Reads a single Resource Record starting at `start`. Returns the record and
/// the position immediately after it.
fn read_record(buf: &[u8], start: usize) -> Option<(DnsRecord, usize)> {
    let (name, mut pos) = read_name(buf, start)?;
    let rtype_raw = read_u16(buf, pos)?;
    pos += 2;
    let class = read_u16(buf, pos)?;
    pos += 2;
    let ttl = read_u32(buf, pos)?;
    pos += 4;
    let rdlength = read_u16(buf, pos)? as usize;
    pos += 2;

    let rdata_start = pos;
    let rdata_end = rdata_start.checked_add(rdlength)?;
    if rdata_end > buf.len() {
        return None;
    }

    let rtype = DnsRecordType::from_u16(rtype_raw);
    let rdata = parse_rdata(buf, rtype, rdata_start, rdlength)?;

    Some((
        DnsRecord {
            name,
            rtype,
            class,
            ttl,
            rdata,
        },
        rdata_end,
    ))
}

/// Interprets the RDATA of a record according to its type. `start`/`len`
/// delimit the RDATA inside `buf`; names inside RDATA may use compression and
/// therefore are resolved against the whole message.
fn parse_rdata(buf: &[u8], rtype: DnsRecordType, start: usize, len: usize) -> Option<DnsRData> {
    let end = start.checked_add(len)?;
    let data = buf.get(start..end)?;

    let rdata = match rtype {
        DnsRecordType::A => {
            let octets: [u8; 4] = data.try_into().ok()?;
            DnsRData::A(Ipv4Addr::from(octets))
        }
        DnsRecordType::Aaaa => {
            let octets: [u8; 16] = data.try_into().ok()?;
            DnsRData::Aaaa(Ipv6Addr::from(octets))
        }
        DnsRecordType::Cname | DnsRecordType::Ns | DnsRecordType::Ptr => {
            let (name, _) = read_name(buf, start)?;
            DnsRData::Name(name)
        }
        DnsRecordType::Mx => {
            let preference = read_u16(buf, start)?;
            let (exchange, _) = read_name(buf, start + 2)?;
            DnsRData::Mx {
                preference,
                exchange,
            }
        }
        DnsRecordType::Txt => {
            // RDATA is one or more length-prefixed <character-string>s.
            let mut strings = Vec::new();
            let mut p = start;
            while p < end {
                let str_len = *buf.get(p)? as usize;
                p += 1;
                let bytes = buf.get(p..p + str_len)?;
                strings.push(String::from_utf8_lossy(bytes).into_owned());
                p += str_len;
            }
            DnsRData::Txt(strings)
        }
        DnsRecordType::Soa => {
            let (mname, p) = read_name(buf, start)?;
            let (rname, p) = read_name(buf, p)?;
            let serial = read_u32(buf, p)?;
            let refresh = read_u32(buf, p + 4)?;
            let retry = read_u32(buf, p + 8)?;
            let expire = read_u32(buf, p + 12)?;
            let minimum = read_u32(buf, p + 16)?;
            DnsRData::Soa {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            }
        }
        DnsRecordType::Srv | DnsRecordType::Other(_) => DnsRData::Other(data.to_vec()),
    };

    Some(rdata)
}

fn read_u16(buf: &[u8], pos: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*buf.get(pos)?, *buf.get(pos + 1)?]))
}

fn read_u32(buf: &[u8], pos: usize) -> Option<u32> {
    Some(u32::from_be_bytes([
        *buf.get(pos)?,
        *buf.get(pos + 1)?,
        *buf.get(pos + 2)?,
        *buf.get(pos + 3)?,
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Query for `google.com`, type A, class IN. Header: id=0x1234, RD set,
    /// QDCOUNT=1.
    const QUERY_A: &[u8] = &[
        0x12, 0x34, // ID
        0x01, 0x00, // flags: RD=1
        0x00, 0x01, // QDCOUNT
        0x00, 0x00, // ANCOUNT
        0x00, 0x00, // NSCOUNT
        0x00, 0x00, // ARCOUNT
        0x06, b'g', b'o', b'o', b'g', b'l', b'e', // "google"
        0x03, b'c', b'o', b'm', // "com"
        0x00, // end of name
        0x00, 0x01, // QTYPE = A
        0x00, 0x01, // QCLASS = IN
    ];

    #[test]
    fn parses_query_a() {
        let msg = parse_dns(QUERY_A).expect("should parse");
        assert_eq!(msg.id, 0x1234);
        assert!(!msg.is_response);
        assert_eq!(msg.opcode, 0);
        assert!(msg.flags.rd);
        assert_eq!(msg.rcode, DnsRCode::NoError);
        assert_eq!(msg.questions.len(), 1);
        assert_eq!(msg.query_name(), Some("google.com"));
        assert_eq!(msg.query_type(), Some(DnsRecordType::A));
        assert_eq!(msg.questions[0].qclass, 1);
        assert!(msg.answers.is_empty());
    }

    /// Response for `google.com` A, using a compression pointer (0xC00C) in the
    /// answer's NAME field to reference the question's name at offset 12.
    const RESPONSE_A_COMPRESSED: &[u8] = &[
        0x12, 0x34, // ID
        0x81, 0x80, // flags: QR=1, RD=1, RA=1, RCODE=0
        0x00, 0x01, // QDCOUNT
        0x00, 0x01, // ANCOUNT
        0x00, 0x00, // NSCOUNT
        0x00, 0x00, // ARCOUNT
        // Question (name starts at offset 12)
        0x06, b'g', b'o', b'o', b'g', b'l', b'e', 0x03, b'c', b'o', b'm', 0x00, 0x00, 0x01,
        0x00, 0x01, // QTYPE=A, QCLASS=IN
        // Answer
        0xC0, 0x0C, // NAME -> pointer to offset 12
        0x00, 0x01, // TYPE=A
        0x00, 0x01, // CLASS=IN
        0x00, 0x00, 0x01, 0x2C, // TTL=300
        0x00, 0x04, // RDLENGTH=4
        0x08, 0x08, 0x08, 0x08, // RDATA = 8.8.8.8
    ];

    #[test]
    fn parses_response_a_with_compression() {
        let msg = parse_dns(RESPONSE_A_COMPRESSED).expect("should parse");
        assert!(msg.is_response);
        assert!(msg.flags.ra);
        assert_eq!(msg.rcode, DnsRCode::NoError);
        assert_eq!(msg.query_name(), Some("google.com"));
        assert_eq!(msg.answers.len(), 1);
        let answer = &msg.answers[0];
        assert_eq!(answer.name, "google.com");
        assert_eq!(answer.rtype, DnsRecordType::A);
        assert_eq!(answer.ttl, 300);
        assert_eq!(answer.rdata, DnsRData::A(Ipv4Addr::new(8, 8, 8, 8)));
    }

    /// AAAA response (::1) with a compression pointer.
    const RESPONSE_AAAA: &[u8] = &[
        0x00, 0x01, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0x04, b'i', b'p', b'v', b'6', 0x00, // name "ipv6"
        0x00, 0x1C, 0x00, 0x01, // QTYPE=AAAA, QCLASS=IN
        0xC0, 0x0C, // pointer to "ipv6"
        0x00, 0x1C, // TYPE=AAAA
        0x00, 0x01, // CLASS=IN
        0x00, 0x00, 0x00, 0x3C, // TTL=60
        0x00, 0x10, // RDLENGTH=16
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, // ::1
    ];

    #[test]
    fn parses_aaaa() {
        let msg = parse_dns(RESPONSE_AAAA).expect("should parse");
        assert_eq!(msg.answers.len(), 1);
        assert_eq!(
            msg.answers[0].rdata,
            DnsRData::Aaaa(Ipv6Addr::LOCALHOST)
        );
    }

    /// MX response: preference 10, exchange "mail" (compressed back to "ex").
    const RESPONSE_MX: &[u8] = &[
        0x00, 0x02, 0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, //
        0x02, b'e', b'x', 0x00, // name "ex"
        0x00, 0x0F, 0x00, 0x01, // QTYPE=MX, QCLASS=IN
        0xC0, 0x0C, // pointer to "ex"
        0x00, 0x0F, // TYPE=MX
        0x00, 0x01, // CLASS=IN
        0x00, 0x00, 0x00, 0x3C, // TTL=60
        0x00, 0x09, // RDLENGTH=9
        0x00, 0x0A, // preference=10
        0x04, b'm', b'a', b'i', b'l', 0xC0, 0x0C, // "mail" + pointer to "ex"
    ];

    #[test]
    fn parses_mx() {
        let msg = parse_dns(RESPONSE_MX).expect("should parse");
        assert_eq!(msg.answers.len(), 1);
        assert_eq!(
            msg.answers[0].rdata,
            DnsRData::Mx {
                preference: 10,
                exchange: "mail.ex".to_string(),
            }
        );
    }

    /// NXDOMAIN response (RCODE=3) with no answers.
    const RESPONSE_NXDOMAIN: &[u8] = &[
        0x00, 0x03, 0x81, 0x83, // QR=1, RD=1, RA=1, RCODE=3
        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        0x07, b'm', b'i', b's', b's', b'i', b'n', b'g', 0x03, b'c', b'o', b'm', 0x00, //
        0x00, 0x01, 0x00, 0x01, // QTYPE=A, QCLASS=IN
    ];

    #[test]
    fn parses_nxdomain() {
        let msg = parse_dns(RESPONSE_NXDOMAIN).expect("should parse");
        assert!(msg.is_response);
        assert_eq!(msg.rcode, DnsRCode::NxDomain);
        assert_eq!(msg.query_name(), Some("missing.com"));
        assert!(msg.answers.is_empty());
    }

    #[test]
    fn rejects_too_short() {
        assert!(parse_dns(&[0x00, 0x01, 0x02]).is_none());
        assert!(parse_dns(&[]).is_none());
    }

    #[test]
    fn truncated_question_returns_none() {
        // Header claims one question but the name never terminates.
        let buf = [
            0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x06, b'g', b'o', b'o', // truncated label, no terminator
        ];
        assert!(parse_dns(&buf).is_none());
    }

    #[test]
    fn pointer_loop_does_not_hang() {
        // A name at offset 12 that points to itself.
        let buf = [
            0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0xC0, 0x0C, // pointer -> offset 12 (itself)
            0x00, 0x01, 0x00, 0x01,
        ];
        // Must terminate (return None) rather than loop forever.
        assert!(parse_dns(&buf).is_none());
    }
}
