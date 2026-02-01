use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use crate::translations::translations::favorite_transmitted_translation;
use crate::translations::translations_5::blacklisted_transmitted_translation;
use crate::translations::types::language::Language;
use serde_json::json;
use std::collections::VecDeque;
use std::net::IpAddr;

#[derive(Default)]
pub struct LoggedNotifications {
    /// Logged notifications during this capture session (max 30 kept)
    notifications: VecDeque<LoggedNotification>,
    /// Total number of notifications ever logged during this capture session
    total_notifications: usize,
}

impl LoggedNotifications {
    pub fn push(&mut self, notification: &LoggedNotification) {
        self.total_notifications += 1;
        if self.notifications.len() >= 30 {
            self.notifications.pop_back();
        }
        self.notifications.push_front(notification.clone());
    }

    pub fn tot(&self) -> usize {
        self.total_notifications
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn len(&self) -> usize {
        self.notifications.len()
    }

    pub fn notifications(&self) -> &VecDeque<LoggedNotification> {
        &self.notifications
    }

    pub fn notifications_mut(&mut self) -> &mut VecDeque<LoggedNotification> {
        &mut self.notifications
    }

    pub fn clear_notifications(&mut self) {
        self.notifications = VecDeque::new();
    }

    #[cfg(test)]
    pub fn set_notifications(&mut self, notifications: VecDeque<LoggedNotification>) {
        self.notifications = notifications;
    }
}

/// Enum representing the possible notification events.
#[derive(Clone)]
pub enum LoggedNotification {
    /// Data threshold exceeded
    DataThresholdExceeded(DataThresholdExceeded),
    /// Favorite connection exchanged data
    FavoriteTransmitted(FavoriteTransmitted),
    /// Blacklisted connection exchanged data
    BlacklistedTransmitted(BlacklistedTransmitted),
}

impl LoggedNotification {
    pub fn id(&self) -> usize {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.id,
            LoggedNotification::FavoriteTransmitted(f) => f.id,
            LoggedNotification::BlacklistedTransmitted(b) => b.id,
        }
    }

    pub fn data_info(&self) -> DataInfo {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.data_info,
            LoggedNotification::FavoriteTransmitted(f) => f.data_info_host.data_info,
            LoggedNotification::BlacklistedTransmitted(b) => b.data_info_host.data_info,
        }
    }

    pub fn expand(&mut self, expand: bool) {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.is_expanded = expand,
            LoggedNotification::FavoriteTransmitted(_)
            | LoggedNotification::BlacklistedTransmitted(_) => {}
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            LoggedNotification::DataThresholdExceeded(d) => d.to_json(),
            LoggedNotification::FavoriteTransmitted(f) => f.to_json(),
            LoggedNotification::BlacklistedTransmitted(b) => b.to_json(),
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
    fn to_json(&self) -> String {
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
    fn to_json(&self) -> String {
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

#[derive(Clone)]
pub struct BlacklistedTransmitted {
    pub(crate) id: usize,
    pub(crate) ip: IpAddr,
    pub(crate) host: Host,
    pub(crate) data_info_host: DataInfoHost,
    pub(crate) timestamp: String,
}

impl BlacklistedTransmitted {
    fn to_json(&self) -> String {
        json!({
            "info": blacklisted_transmitted_translation(Language::EN),
            "timestamp": self.timestamp,
            "ip": self.ip.to_string(),
            "host": {
                "country": self.host.country.to_string(),
                "domain": self.host.domain,
                "asn": self.host.asn.name,
            },
            "data": DataRepr::Bytes.formatted_string(self.data_info_host.data_info.tot_data(DataRepr::Bytes)),
        })
        .to_string()
    }
}
