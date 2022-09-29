//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use std::fmt;
use crate::TransProtocol;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Debug)]
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
    /// Flag to determine which of the address is that of the sniffed adapter or remote
    pub traffic_type: TrafficType,
}

impl AddressPortPair {

    /// Returns a new AddressPort element.
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the network layer IPv4 or IPv6 address.
    ///
    /// * `port` - An integer representing the transport layer port number (in the range 0..=65535).
    pub fn new (address1: String, port1: u16, address2: String, port2: u16, trans_protocol: TransProtocol, traffic_type: TrafficType) -> Self {
        AddressPortPair {
            address1,
            port1,
            address2,
            port2,
            trans_protocol,
            traffic_type,
        }
    }
}


impl fmt::Display for AddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"|{:^25}|{:>8}  |{:^25}|{:>8}  |",
               self.address1, self.port1, self.address2, self.port2)
    }
}


impl Clone for AddressPortPair {
    fn clone(&self) -> Self {
        AddressPortPair {
            address1: self.address1.clone(),
            port1: self.port1,
            address2: self.address2.clone(),
            port2: self.port2,
            trans_protocol: self.trans_protocol,
            traffic_type: self.traffic_type
        }
    }
}


/// Enum representing the possible traffic type (incoming, outgoing or multicast).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrafficType {
    /// Incoming traffic (from remote address to local interface)
    Incoming,
    /// Outgoing traffic (from local interface to remote address)
    Outgoing,
    /// Multicast traffic (from remote address to multicast address)
    Multicast,
    /// Not identified
    Other
}