//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use std::fmt;

use crate::TransProtocol;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct AddressPortPair {
    /// Network layer IPv4 or IPv6 source address.
    pub address1: String,
    /// Transport layer source port number (in the range 0..=65535).
    pub port1: u16,
    /// Network layer IPv4 or IPv6 destination address.
    pub address2: String,
    /// Transport layer destination port number (in the range 0..=65535).
    pub port2: u16,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub trans_protocol: TransProtocol,
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
        port1: u16,
        address2: String,
        port2: u16,
        trans_protocol: TransProtocol,
    ) -> Self {
        AddressPortPair {
            address1,
            port1,
            address2,
            port2,
            trans_protocol,
        }
    }

    pub fn print_gui(&self) -> String {
        self.to_string().replace('|', "")
    }
}

impl fmt::Display for AddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.address1.len() > 25 || self.address2.len() > 25 {
            write!(
                f,
                "|{:^45}|{:>8}  |{:^45}|{:>8}  |   {}   |",
                self.address1, self.port1, self.address2, self.port2, self.trans_protocol
            )
        } else {
            write!(
                f,
                "|{:^25}|{:>8}  |{:^25}|{:>8}  |   {}   |",
                self.address1, self.port1, self.address2, self.port2, self.trans_protocol
            )
        }
    }
}
