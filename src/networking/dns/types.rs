//! Data types representing a parsed DNS message (RFC 1035) and the capture
//! metadata associated with it.
//!
//! These types are intentionally independent of any external DNS parsing
//! crate: the parsing is performed manually over the raw bytes (see
//! [`super::parser`]), so every field here maps directly to a field of the
//! on-the-wire DNS format.

use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::networking::types::protocol::Protocol;
use crate::utils::types::timestamp::Timestamp;

/// A DNS message captured from the network, enriched with the capture context
/// (when it was seen and between which endpoints).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DnsEvent {
    /// Timestamp of the packet carrying this DNS message.
    pub timestamp: Timestamp,
    /// Source IP address of the packet.
    pub src: IpAddr,
    /// Destination IP address of the packet (the DNS server, for a query).
    pub dst: IpAddr,
    /// Transport protocol carrying the DNS message (UDP or TCP).
    pub transport: Protocol,
    /// The parsed DNS message itself.
    pub message: DnsMessage,
}

/// A fully parsed DNS message: the 12-byte header plus the Question and Answer
/// sections. The Authority and Additional sections are not retained.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DnsMessage {
    /// Transaction ID, used to correlate a query with its response.
    pub id: u16,
    /// `QR` bit: `false` = query, `true` = response.
    pub is_response: bool,
    /// `OPCODE` field (0 = standard QUERY, 1 = IQUERY, 2 = STATUS, ...).
    pub opcode: u8,
    /// Header flags (AA, TC, RD, RA).
    pub flags: DnsFlags,
    /// `RCODE` field (response code).
    pub rcode: DnsRCode,
    /// Entries of the Question section.
    pub questions: Vec<DnsQuestion>,
    /// Resource Records of the Answer section.
    pub answers: Vec<DnsRecord>,
}

impl DnsMessage {
    /// Name of the first question, if any (the domain being looked up).
    pub fn query_name(&self) -> Option<&str> {
        self.questions.first().map(|q| q.name.as_str())
    }

    /// Type of the first question, if any.
    pub fn query_type(&self) -> Option<DnsRecordType> {
        self.questions.first().map(|q| q.qtype)
    }

    /// Human-readable, comma-separated summary of the answer records, each
    /// prefixed by its record type (e.g. "CNAME cdn.example.net, A 1.2.3.4"),
    /// suitable for a single table cell.
    pub fn answers_summary(&self) -> String {
        self.answers
            .iter()
            .map(|r| format!("{} {}", r.rtype, r.rdata))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Header flags retained from the DNS header.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DnsFlags {
    /// Authoritative Answer.
    pub aa: bool,
    /// TrunCation: the message was truncated.
    pub tc: bool,
    /// Recursion Desired.
    pub rd: bool,
    /// Recursion Available.
    pub ra: bool,
}

/// An entry of the Question section.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DnsQuestion {
    /// Queried domain name (already decoded, dots between labels).
    pub name: String,
    /// Query type (QTYPE).
    pub qtype: DnsRecordType,
    /// Query class (QCLASS); 1 = IN (Internet).
    pub qclass: u16,
}

/// A Resource Record (used here for the Answer section).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DnsRecord {
    /// Owner name of the record.
    pub name: String,
    /// Record type.
    pub rtype: DnsRecordType,
    /// Record class; 1 = IN (Internet).
    pub class: u16,
    /// Time to live, in seconds.
    pub ttl: u32,
    /// Interpreted record data.
    pub rdata: DnsRData,
}

/// DNS record/query type (TYPE / QTYPE field).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DnsRecordType {
    A,
    Ns,
    Cname,
    Soa,
    Ptr,
    Mx,
    Txt,
    Aaaa,
    Srv,
    /// Any other type, keeping its numeric value.
    Other(u16),
}

impl DnsRecordType {
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::Ns,
            5 => Self::Cname,
            6 => Self::Soa,
            12 => Self::Ptr,
            15 => Self::Mx,
            16 => Self::Txt,
            28 => Self::Aaaa,
            33 => Self::Srv,
            other => Self::Other(other),
        }
    }
}

impl Display for DnsRecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::Ns => write!(f, "NS"),
            Self::Cname => write!(f, "CNAME"),
            Self::Soa => write!(f, "SOA"),
            Self::Ptr => write!(f, "PTR"),
            Self::Mx => write!(f, "MX"),
            Self::Txt => write!(f, "TXT"),
            Self::Aaaa => write!(f, "AAAA"),
            Self::Srv => write!(f, "SRV"),
            Self::Other(n) => write!(f, "TYPE{n}"),
        }
    }
}

/// DNS response code (RCODE field).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DnsRCode {
    NoError,
    FormErr,
    ServFail,
    NxDomain,
    NotImpl,
    Refused,
    /// Any other code, keeping its numeric value.
    Other(u8),
}

impl DnsRCode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormErr,
            2 => Self::ServFail,
            3 => Self::NxDomain,
            4 => Self::NotImpl,
            5 => Self::Refused,
            other => Self::Other(other),
        }
    }
}

impl Display for DnsRCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoError => write!(f, "NOERROR"),
            Self::FormErr => write!(f, "FORMERR"),
            Self::ServFail => write!(f, "SERVFAIL"),
            Self::NxDomain => write!(f, "NXDOMAIN"),
            Self::NotImpl => write!(f, "NOTIMP"),
            Self::Refused => write!(f, "REFUSED"),
            Self::Other(n) => write!(f, "RCODE{n}"),
        }
    }
}

/// Interpreted record data (RDATA), by record type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DnsRData {
    /// IPv4 address (type A).
    A(Ipv4Addr),
    /// IPv6 address (type AAAA).
    Aaaa(Ipv6Addr),
    /// A domain name (types CNAME, NS, PTR).
    Name(String),
    /// Mail exchange (type MX): preference and mail server name.
    Mx { preference: u16, exchange: String },
    /// Text strings (type TXT).
    Txt(Vec<String>),
    /// Start of authority (type SOA).
    Soa {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    /// Raw, uninterpreted data for unsupported types.
    Other(Vec<u8>),
}

impl Display for DnsRData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(ip) => write!(f, "{ip}"),
            Self::Aaaa(ip) => write!(f, "{ip}"),
            Self::Name(name) => write!(f, "{name}"),
            Self::Mx {
                preference,
                exchange,
            } => write!(f, "{preference} {exchange}"),
            Self::Txt(strings) => write!(f, "{}", strings.join(" ")),
            Self::Soa { mname, rname, .. } => write!(f, "{mname} {rname}"),
            Self::Other(bytes) => write!(f, "{} bytes", bytes.len()),
        }
    }
}
