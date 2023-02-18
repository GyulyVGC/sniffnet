use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::ByteMultiple;

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
    pub(crate) threshold: u32,
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct BytesThresholdExceeded {
    pub(crate) threshold: u64,
    pub(crate) byte_multiple: ByteMultiple,
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) connection: (AddressPortPair, InfoAddressPortPair),
    pub(crate) timestamp: String,
}
