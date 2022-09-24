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

        let addr_string_1 = if self.address1.contains(':') { // IPv6 address is enclosed in [brackets]
            format!("|  [{}]:{}  |", self.address1, self.port1)
        }
        else {
            format!("|  {}:{}  |", self.address1, self.port1)
        };

        let addr_string_2 = if self.address2.contains(':') { // IPv6 address is enclosed in [brackets]
            format!("|  [{}]:{}  |", self.address2, self.port2)
        }
        else {
            format!("|  {}:{}  |", self.address2, self.port2)
        };

        let addresses_string = format!("{}  --->  {}", addr_string_1, addr_string_2);

        let spaces = " ".to_string().repeat(addr_string_1.len()-6);
        let my_interface_string = match self.traffic_type {
            TrafficType::Incoming => {format!("   Remote address{}Your interface", spaces)}
            TrafficType::Outgoing => {format!("   Your interface{}Remote Address", spaces)}
            TrafficType::Multicast => {format!("   Remote address{}Multicast address", spaces)}
            TrafficType::Other => {format!("   Remote address{}Remote address", spaces)}
        };

        let cornice_up_string = format!(" /{}\\          /{}\\", "-".to_string().repeat(addr_string_1.len() - 4),
                                                                "-".to_string().repeat(addr_string_2.len() - 4));

        let cornice_down_string = format!(" \\{}/          \\{}/", "-".to_string().repeat(addr_string_1.len() - 4),
                                        "-".to_string().repeat(addr_string_2.len() - 4));

        write!(f,"{}\n{}\n{}\n{}", my_interface_string, cornice_up_string, addresses_string, cornice_down_string)
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