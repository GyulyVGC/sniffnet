use std::cmp::Ordering;

pub struct AddressPortPair {
    address1: String,
    port1: u16,
    address2: String,
    port2: u16,
}

impl AddressPortPair {

    pub fn new (address1: String, port1: u16, address2: String, port2: u16) -> Self {
        if address1.cmp(&address2) == Ordering::Less {
            AddressPortPair {
                address1,
                port1,
                address2,
                port2
            }
        }
        else {
            AddressPortPair {
                address1: address2,
                port1: port2,
                address2: address1,
                port2: port1
            }
        }
    }

}

