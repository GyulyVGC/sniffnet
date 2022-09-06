//! Module defining the `AddressPort` struct, which represents a network address:port pair.

use std::fmt;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AddressPort {
    /// Network layer IPv4 or IPv6 source address.
    pub address1: String,
    /// Transport layer source port number (in the range 0..=65535).
    pub port1: u16,
    /// Network layer IPv4 or IPv6 destination address.
    pub address2: String,
    /// Transport layer destination port number (in the range 0..=65535).
    pub port2: u16,
    /// Flag to determine which of the address is that of the sniffed adapter or remote
    pub my_interface: u8,
}

impl AddressPort {

    /// Returns a new AddressPort element.
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the network layer IPv4 or IPv6 address.
    ///
    /// * `port` - An integer representing the transport layer port number (in the range 0..=65535).
    pub fn new (address1: String, port1: u16, address2: String, port2: u16, my_interface: u8) -> Self {
        AddressPort {
            address1,
            port1,
            address2,
            port2,
            my_interface,
        }
    }
}


impl fmt::Display for AddressPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let addr_string_1 = if self.address1.contains(":") { // IPv6 address is enclosed in [brackets]
            format!("|  [{}]:{}  |", self.address1, self.port1)
        }
        else {
            format!("|  {}:{}  |", self.address1, self.port1)
        };

        let addr_string_2 = if self.address2.contains(":") { // IPv6 address is enclosed in [brackets]
            format!("|  [{}]:{}  |", self.address2, self.port2)
        }
        else {
            format!("|  {}:{}  |", self.address2, self.port2)
        };

        let addresses_string = format!("{}  --->  {}", addr_string_1, addr_string_2);

        let spaces = " ".to_string().repeat(addr_string_1.len()-6);
        let my_interface_string =
            if self.my_interface == 1 {
                format!("   Your interface{}Remote Address", spaces)
            }
            else if self.my_interface ==2 {
                format!("   Remote address{}Your interface", spaces)
            }
            else {
                format!("   Remote address{}Remote address", spaces)
            };

        let cornice_up_string = format!(" /{}\\          /{}\\", "-".to_string().repeat(addr_string_1.len() - 4),
                                                                "-".to_string().repeat(addr_string_2.len() - 4));

        let cornice_down_string = format!(" \\{}/          \\{}/", "-".to_string().repeat(addr_string_1.len() - 4),
                                        "-".to_string().repeat(addr_string_2.len() - 4));

        write!(f,"{}\n{}\n{}\n{}", my_interface_string, cornice_up_string, addresses_string, cornice_down_string)
    }
}


