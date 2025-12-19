//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use crate::Protocol;
use std::net::{IpAddr, Ipv4Addr};

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AddressPortPair {
    /// Network layer IPv4 or IPv6 source address.
    pub source: IpAddr,
    /// Transport layer source port number (in the range 0..=65535).
    pub sport: Option<u16>,
    /// Network layer IPv4 or IPv6 destination address.
    pub dest: IpAddr,
    /// Transport layer destination port number (in the range 0..=65535).
    pub dport: Option<u16>,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub protocol: Protocol,
}

#[cfg(test)]
impl AddressPortPair {
    pub fn new(
        source: IpAddr,
        sport: Option<u16>,
        dest: IpAddr,
        dport: Option<u16>,
        protocol: Protocol,
    ) -> Self {
        AddressPortPair {
            source,
            sport,
            dest,
            dport,
            protocol,
        }
    }
}

impl Default for AddressPortPair {
    fn default() -> Self {
        AddressPortPair {
            source: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            dest: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            sport: None,
            dport: None,
            protocol: Protocol::ARP,
        }
    }
}
