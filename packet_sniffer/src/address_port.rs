//! Module defining the `AddressPort` struct, which represents a network address:port pair.

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AddressPort {
    /// Network layer IPv4 or IPv6 address.
    pub address: String,
    /// Transport layer port number (in the range 0..=65535).
    pub port: u16,
}

impl AddressPort {

    /// Returns a new AddressPort element.
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the network layer IPv4 or IPv6 address.
    ///
    /// * `port` - An integer representing the transport layer port number (in the range 0..=65535).
    pub fn new (address: String, port: u16) -> Self {
        AddressPort {
            address,
            port,
        }
    }
}

