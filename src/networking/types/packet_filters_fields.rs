use std::net::IpAddr;
use std::str::FromStr;

use crate::{IpVersion, Protocol};

/// Fields extracted from a packet to determine if this packet matches the defined filters
#[derive(Clone)]
pub struct PacketFiltersFields {
    /// Internet Protocol version
    pub ip: IpVersion,
    /// Protocol
    pub protocol: Protocol,
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
            protocol: Protocol::TCP,
            source: IpAddr::from_str("::").unwrap(),
            dest: IpAddr::from_str("::").unwrap(),
            sport: 0,
            dport: 0,
        }
    }
}
