use crate::networking::types::address_port_pair::AddressPortPair;

/// This enum defines the currently displayed modal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MyModal {
    /// Reset modal.
    Reset,
    /// Quit modal.
    Quit,
    /// Clear all modal.
    ClearAll,
    /// Connection details modal.
    ConnectionDetails(AddressPortPair),
}
