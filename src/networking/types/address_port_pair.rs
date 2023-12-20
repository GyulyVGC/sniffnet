//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use std::fmt;

use crate::Protocol;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AddressPortPair {
    /// Network layer IPv4 or IPv6 source address.
    pub address1: String,
    /// Transport layer source port number (in the range 0..=65535).
    pub port1: Option<u16>,
    /// Network layer IPv4 or IPv6 destination address.
    pub address2: String,
    /// Transport layer destination port number (in the range 0..=65535).
    pub port2: Option<u16>,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub protocol: Protocol,
}

impl AddressPortPair {
    /// Returns a new `AddressPort` element.
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the network layer IPv4 or IPv6 address.
    ///
    /// * `port` - An integer representing the transport layer port number (in the range 0..=65535).
    pub fn new(
        address1: String,
        port1: Option<u16>,
        address2: String,
        port2: Option<u16>,
        protocol: Protocol,
    ) -> Self {
        AddressPortPair {
            address1,
            port1,
            address2,
            port2,
            protocol,
        }
    }
}

impl fmt::Display for AddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (port_1_str, port_2_str) = if self.port1.is_some() && self.port2.is_some() {
            (
                self.port1.unwrap().to_string(),
                self.port2.unwrap().to_string(),
            )
        } else {
            ("-".to_string(), "-".to_string())
        };
        if self.address1.len() > 25 || self.address2.len() > 25 {
            write!(
                f,
                "{:^45}{:>8}  {:^45}{:>8}    {:>4}   ",
                self.address1,
                port_1_str,
                self.address2,
                port_2_str,
                self.protocol.to_string()
            )
        } else {
            write!(
                f,
                "{:^25}{:>8}  {:^25}{:>8}    {:>4}   ",
                self.address1,
                port_1_str,
                self.address2,
                port_2_str,
                self.protocol.to_string()
            )
        }
    }
}
