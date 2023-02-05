use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::structs::notifications::{BytesNotification, PacketsNotification};

/// Enum representing the possible observed values of IP protocol version.
pub enum LoggedNotification {
    /// Packets threshold exceeded
    PacketsThresholdExceeded(PacketsThresholdExceeded),
    /// Byte threshold exceeded
    BytesThresholdExceeded(BytesThresholdExceeded),
    /// Favorite connection exchanged data
    FavoriteTransmitted(FavoriteTransmitted),
}

#[derive(Clone)]
pub struct PacketsThresholdExceeded {
    pub(crate) notification: PacketsNotification,
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct BytesThresholdExceeded {
    pub(crate) notification: BytesNotification,
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) connection: (AddressPortPair, InfoAddressPortPair),
    pub(crate) timestamp: String,
}
