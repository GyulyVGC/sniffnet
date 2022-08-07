use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct AddressPort {
    address1: String,
    port1: u16,
}

impl AddressPort {

    pub fn new (address1: String, port1: u16) -> Self {
        AddressPort {
                address1,
                port1,
        }
    }
    pub fn get_ip(&self) -> String { address1 }
    pub fn get_port(&self) -> u16 { port1 }
}

