use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use crate::translations::translations::favorite_transmitted_translation;
use crate::translations::types::language::Language;
use serde_json::json;

/// Enum representing the possible notification events.
#[derive(Clone)]
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

    pub fn to_json(&self) -> String {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.to_json(),
            LoggedNotification::FavoriteTransmitted(f) => f.to_json(),
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

impl DataThresholdExceeded {
    pub fn to_json(&self) -> String {
        json!({
            "info": self.data_repr.data_exceeded_translation(Language::EN),
            "timestamp": self.timestamp,
            "threshold": self.data_repr.formatted_string(self.threshold.into()),
            "data": self.data_repr.formatted_string(self.data_info.tot_data(self.data_repr)),
        })
        .to_string()
    }
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) id: usize,
    pub(crate) host: Host,
    pub(crate) data_info_host: DataInfoHost,
    pub(crate) timestamp: String,
}

impl FavoriteTransmitted {
    pub fn to_json(&self) -> String {
        json!({
            "info": favorite_transmitted_translation(Language::EN),
            "timestamp": self.timestamp,
            "favorite": {
                "country": self.host.country.to_string(),
                "domain": self.host.domain,
                "asn": self.host.asn.name,
            },
            "data": DataRepr::Bytes.formatted_string(self.data_info_host.data_info.tot_data(DataRepr::Bytes)),
        })
        .to_string()
    }
}
