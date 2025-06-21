use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;

/// Enum representing the possible notification events.
pub enum LoggedNotification {
    /// Packets threshold exceeded
    PacketsThresholdExceeded(DataThresholdExceeded),
    /// Byte threshold exceeded
    BytesThresholdExceeded(DataThresholdExceeded),
    /// Favorite connection exchanged data
    FavoriteTransmitted(FavoriteTransmitted),
}

impl LoggedNotification {
    pub fn data_info(&self) -> DataInfo {
        match self {
            LoggedNotification::BytesThresholdExceeded(d)
            | LoggedNotification::PacketsThresholdExceeded(d) => d.data_info,
            LoggedNotification::FavoriteTransmitted(f) => f.data_info_host.data_info,
        }
    }
}

#[derive(Clone)]
pub struct DataThresholdExceeded {
    pub(crate) threshold: u64,
    pub(crate) data_info: DataInfo,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) host: Host,
    pub(crate) data_info_host: DataInfoHost,
    pub(crate) timestamp: String,
}
