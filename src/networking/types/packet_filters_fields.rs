use crate::{IpVersion, TransProtocol};
use std::net::IpAddr;
use std::str::FromStr;

/// Fields extracted from a packet to determine if this packet matches the defined filters
#[derive(Clone)]
pub struct PacketFiltersFields {
    /// Internet Protocol version
    pub ip: IpVersion,
    /// Transport layer protocol
    pub transport: TransProtocol,
    /// Source IP address
    pub source: IpAddr,
    /// Destination IP address
    pub dest: IpAddr,
    /// Source port
    pub sport: u16,
    /// Destination port
    pub dport: u16,
}

impl Default for PacketFiltersFields {
    fn default() -> Self {
        Self {
            ip: IpVersion::IPv4,
            transport: TransProtocol::TCP,
            source: IpAddr::from_str("::").unwrap(),
            dest: IpAddr::from_str("::").unwrap(),
            sport: 0,
            dport: 0,
        }
    }
}
