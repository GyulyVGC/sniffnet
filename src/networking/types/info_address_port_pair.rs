//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::fmt;

use chrono::{DateTime, Local};

use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::formatted_strings::get_formatted_bytes_string;
use crate::AppProtocol;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each `InfoAddressPortPair` struct is associated to a single address:port pair.
#[derive(Clone)]
pub struct InfoAddressPortPair {
    /// Source MAC address
    pub mac_address1: String,
    /// Destination MAC address
    pub mac_address2: String,
    /// Amount of bytes transmitted between the pair.
    pub transmitted_bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub transmitted_packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: DateTime<Local>,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: DateTime<Local>,
    /// Application layer protocol carried by the associated address:port pair.
    pub app_protocol: AppProtocol,
    /// Determines if the connection is incoming or outgoing
    pub traffic_direction: TrafficDirection,
}

impl Default for InfoAddressPortPair {
    fn default() -> Self {
        Self {
            mac_address1: String::new(),
            mac_address2: String::new(),
            transmitted_bytes: 0,
            transmitted_packets: 0,
            initial_timestamp: DateTime::default(),
            final_timestamp: DateTime::default(),
            app_protocol: AppProtocol::Other,
            traffic_direction: TrafficDirection::default(),
        }
    }
}

impl fmt::Display for InfoAddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes_string = get_formatted_bytes_string(self.transmitted_bytes);

        let app_string = match self.app_protocol {
            AppProtocol::Other => "Other".to_string(),
            _ => self.app_protocol.to_string(),
        };

        write!(
            f,
            "{:^9}{:>10}  {:>9}   ",
            app_string, self.transmitted_packets, bytes_string,
        )
    }
}
