use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;

/// Enum representing the possible notification events.
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
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) host: Host,
    pub(crate) data_info_host: DataInfoHost,
    pub(crate) timestamp: String,
}
