use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use serde::Serialize;
use serde::ser::SerializeStruct;

/// Enum representing the possible notification events.
pub enum LoggedNotification {
    /// Data threshold exceeded
    DataThresholdExceeded(DataThresholdExceeded),
    /// Favorite connection exchanged data
    FavoriteTransmitted(FavoriteTransmitted),
}

impl LoggedNotification {
    pub fn id(&self) -> usize {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.id,
            LoggedNotification::FavoriteTransmitted(f) => f.id,
        }
    }

    pub fn data_info(&self) -> DataInfo {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.data_info,
            LoggedNotification::FavoriteTransmitted(f) => f.data_info_host.data_info,
        }
    }

    pub fn expand(&mut self, expand: bool) {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.is_expanded = expand,
            LoggedNotification::FavoriteTransmitted(_) => {}
        }
    }
}

#[derive(Clone)]
pub struct DataThresholdExceeded {
    pub(crate) id: usize,
    pub(crate) data_repr: DataRepr,
    pub(crate) threshold: u64,
    pub(crate) data_info: DataInfo,
    pub(crate) timestamp: String,
    pub(crate) is_expanded: bool,
    pub(crate) hosts: Vec<(Host, DataInfoHost)>,
    pub(crate) services: Vec<(Service, DataInfo)>,
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) id: usize,
    pub(crate) host: Host,
    pub(crate) data_info_host: DataInfoHost,
    pub(crate) timestamp: String,
}

impl Serialize for LoggedNotification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.serialize(serializer),
            LoggedNotification::FavoriteTransmitted(f) => f.serialize(serializer),
        }
    }
}

impl Serialize for DataThresholdExceeded {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DataThresholdExceeded", 6)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        // TODO: info message translated
        state.serialize_field("info", &self.info)?;
        // TODO: pretty print
        state.serialize_field("threshold", &self.threshold)?;
        // TODO: data information
        state.serialize_field("data", &self.data_info)?;
        // TODO: host & data information (only this data repr)
        state.serialize_field("hosts", &self.hosts)?;
        // TODO: service & data information (only this data repr)
        state.serialize_field("services", &self.services)?;
        state.end()
    }
}

impl Serialize for FavoriteTransmitted {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DataExchangedFromFavorites", 4)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        // TODO: info message translated
        state.serialize_field("info", &self.info)?;
        // TODO: host information
        state.serialize_field("host", &self.host)?;
        // TODO: data information
        state.serialize_field("data", &self.data_info_host.data_info)?;
        state.end()
    }
}
