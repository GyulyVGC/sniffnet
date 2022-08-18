//! Module defining the `AddressPort` struct, which represents a network address:port pair.

use std::fmt;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AddressPort {
    /// Network layer IPv4 or IPv6 address.
    pub address: String,
    /// Transport layer port number (in the range 0..=65535).
    pub port: u16,
    /// Flag to determine if the address is that of the sniffed adapter or remote
    pub my_interface: bool,
}

impl AddressPort {

    /// Returns a new AddressPort element.
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the network layer IPv4 or IPv6 address.
    ///
    /// * `port` - An integer representing the transport layer port number (in the range 0..=65535).
    pub fn new (address: String, port: u16, my_interface: bool) -> Self {
        AddressPort {
            address,
            port,
            my_interface,
        }
    }
}


impl fmt::Display for AddressPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let my_interface_string =
            if self.my_interface {
                " (your network interface)".to_string()
            }
            else {
                "".to_string()
            };

        write!(f, "{}:{}{}", self.address, self.port, my_interface_string)

    }
}


