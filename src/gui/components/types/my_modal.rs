use crate::gui::types::message::Message;
use crate::networking::types::address_port_pair::AddressPortPair;

/// This enum defines the currently displayed modal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MyModal {
    /// Reset or quit modal.
    ResetOrQuit(ResetOrQuit),
    /// Clear all modal.
    ClearAll,
    /// Connection details modal.
    ConnectionDetails(AddressPortPair),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResetOrQuit {
    Reset,
    Quit,
}

impl ResetOrQuit {
    pub fn get_message(&self) -> Message {
        match self {
            ResetOrQuit::Reset => Message::Reset,
            ResetOrQuit::Quit => Message::Quit,
        }
    }
}
