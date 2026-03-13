use crate::gui::types::favorite::FavoriteItem;
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
            LoggedNotification::FavoriteTransmitted(f) => f.favorite.data_info(),
            LoggedNotification::BlacklistedTransmitted(b) => {
                b.data_info_host.data_info_fav.data_info
            }
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
    pub(crate) favorite: FavoriteItem,
    pub(crate) timestamp: String,
}

impl FavoriteTransmitted {
    fn to_json(&self) -> String {
        json!({
            "info": favorite_transmitted_translation(Language::EN),
            "timestamp": self.timestamp,
            "favorite": match &self.favorite {
                FavoriteItem::Host((host, _)) => json!({
                    "country": host.country.to_string(),
                    "domain": host.domain,
                    "asn": host.asn.name,
                }),
                FavoriteItem::Service((service, _)) => json!({
                    "service": service.to_string(),
                }),
                FavoriteItem::Program((program, _)) => json!({
                    "program": program.to_string(),
                }),
            },
            "data": DataRepr::Bytes.formatted_string(self.favorite.data_info().tot_data(DataRepr::Bytes)),
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
            "data": DataRepr::Bytes.formatted_string(self.data_info_host.data_info_fav.data_info.tot_data(DataRepr::Bytes)),
        })
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::countries::types::country::Country;
    use crate::networking::types::asn::Asn;
    use crate::networking::types::data_info_fav::DataInfoFav;
    use crate::networking::types::program::Program;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use crate::networking::types::traffic_type::TrafficType;
    use std::time::Instant;

    #[test]
    fn test_data_threshold_exceeded_to_json() {
        let mut data_info = DataInfo::default();
        data_info.add_packets(12, 1500, TrafficDirection::Incoming, Instant::now());
        let notification = DataThresholdExceeded {
            id: 1,
            data_repr: DataRepr::Packets,
            threshold: 10,
            data_info,
            timestamp: "2024-06-01T12:00:00Z".to_string(),
            is_expanded: false,
            hosts: vec![],
            services: vec![],
        };
        assert_eq!(
            notification.to_json(),
            r#"{"info":"Packets threshold exceeded","timestamp":"2024-06-01T12:00:00Z","threshold":"10","data":"12"}"#
        );
    }

    #[test]
    fn test_favorite_host_transmitted_to_json() {
        let host = Host {
            country: Country::AE,
            domain: "example.com".to_string(),
            asn: Asn {
                code: "12345".to_string(),
                name: "AS12345".to_string(),
            },
        };
        let data_info_host = DataInfoHost {
            data_info_fav: DataInfoFav {
                data_info: {
                    let mut di = DataInfo::default();
                    di.add_packets(5, 500, TrafficDirection::Outgoing, Instant::now());
                    di
                },
                is_favorite: true,
            },
            is_loopback: false,
            is_local: false,
            is_bogon: None,
            traffic_type: TrafficType::Unicast,
        };
        let favorite_item = FavoriteItem::Host((host, data_info_host));
        let notification = FavoriteTransmitted {
            id: 2,
            favorite: favorite_item,
            timestamp: "2024-06-01T12:05:00Z".to_string(),
        };
        assert_eq!(
            notification.to_json(),
            r#"{"info":"New data exchanged from favorites","timestamp":"2024-06-01T12:05:00Z","favorite":{"country":"AE","domain":"example.com","asn":"AS12345"},"data":"500 B"}"#
        );
    }

    #[test]
    fn test_favorite_service_transmitted_to_json() {
        let service = Service::Name("https");
        let mut data_info = DataInfo::default();
        data_info.add_packets(12, 1500, TrafficDirection::Incoming, Instant::now());
        let favorite_item = FavoriteItem::Service((
            service,
            DataInfoFav {
                data_info,
                is_favorite: true,
            },
        ));
        let notification = FavoriteTransmitted {
            id: 3,
            favorite: favorite_item,
            timestamp: "2024-06-01T12:10:00Z".to_string(),
        };
        assert_eq!(
            notification.to_json(),
            r#"{"info":"New data exchanged from favorites","timestamp":"2024-06-01T12:10:00Z","favorite":{"service":"https"},"data":"1.5 KB"}"#
        );
    }

    #[test]
    fn test_favorite_program_transmitted_to_json() {
        let program = Program::NamePath((
            "example.exe".to_string(),
            "/usr/bin/example.exe".to_string(),
        ));
        let mut data_info = DataInfo::default();
        data_info.add_packets(20, 2_500_000, TrafficDirection::Outgoing, Instant::now());
        let favorite_item = FavoriteItem::Program((
            program,
            DataInfoFav {
                data_info,
                is_favorite: true,
            },
        ));
        let notification = FavoriteTransmitted {
            id: 4,
            favorite: favorite_item,
            timestamp: "2024-06-01T12:15:00Z".to_string(),
        };
        assert_eq!(
            notification.to_json(),
            r#"{"info":"New data exchanged from favorites","timestamp":"2024-06-01T12:15:00Z","favorite":{"program":"example.exe"},"data":"2.5 MB"}"#
        );
    }

    #[test]
    fn test_blacklisted_transmitted_to_json() {
        let host = Host {
            country: Country::US,
            domain: "malicious.com".to_string(),
            asn: Asn {
                code: "54321".to_string(),
                name: "AS54321".to_string(),
            },
        };
        let data_info_host = DataInfoHost {
            data_info_fav: DataInfoFav {
                data_info: {
                    let mut di = DataInfo::default();
                    di.add_packets(50, 10_000, TrafficDirection::Incoming, Instant::now());
                    di
                },
                is_favorite: false,
            },
            is_loopback: false,
            is_local: false,
            is_bogon: None,
            traffic_type: TrafficType::Unicast,
        };
        let notification = BlacklistedTransmitted {
            id: 5,
            ip: IpAddr::from([8, 8, 8, 8]),
            host,
            data_info_host,
            timestamp: "2024-06-01T12:20:00Z".to_string(),
        };
        assert_eq!(
            notification.to_json(),
            r#"{"info":"New data exchanged from a blacklisted IP","timestamp":"2024-06-01T12:20:00Z","ip":"8.8.8.8","host":{"country":"US","domain":"malicious.com","asn":"AS54321"},"data":"10 KB"}"#
        );
    }
}
