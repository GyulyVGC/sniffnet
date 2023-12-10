use crate::networking::types::address_port_pair::AddressPortPair;

/// This enum defines the currently displayed modal.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum MyModal {
    /// Quit modal.
    Quit,
    /// Clear all modal.
    ClearAll,
    /// Connection details modal.
    ConnectionDetails(AddressPortPair),
}
