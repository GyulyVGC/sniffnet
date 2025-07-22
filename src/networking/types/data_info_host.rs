//! Module defining the `DataInfoHost` struct related to hosts.

use crate::networking::types::data_info::DataInfo;
use crate::networking::types::traffic_type::TrafficType;

/// Host-related information.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash)]
pub struct DataInfoHost {
    /// Incoming and outgoing packets and bytes
    pub data_info: DataInfo,
    /// Determine if this host is one of the favorites
    pub is_favorite: bool,
    /// Determine if the connection is loopback (the "remote" is loopback)
    pub is_loopback: bool,
    /// Determine if the connection with this host is local
    pub is_local: bool,
    /// Determine if the connection is with a bogon address
    pub is_bogon: Option<&'static str>,
    /// Determine if the connection with this host is unicast, multicast, or broadcast
    pub traffic_type: TrafficType,
}

impl DataInfoHost {
    pub fn refresh(&mut self, other: &Self) {
        self.data_info.refresh(other.data_info);
        self.is_loopback = other.is_loopback;
        self.is_local = other.is_local;
        self.is_bogon = other.is_bogon;
        self.traffic_type = other.traffic_type;
    }
}
