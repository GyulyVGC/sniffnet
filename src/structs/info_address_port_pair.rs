//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use chrono::{DateTime, Local};
use std::fmt;
use std::ops::Add;

use crate::enums::traffic_type::TrafficType;
use crate::utility::get_formatted_strings::get_formatted_bytes_string;
use crate::AppProtocol;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each `InfoAddressPortPair` struct is associated to a single address:port pair.
#[derive(Clone)]
pub struct InfoAddressPortPair {
    /// Amount of bytes transmitted between the pair.
    pub transmitted_bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub transmitted_packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: DateTime<Local>,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: DateTime<Local>,
    /// Set of application layer protocols carried by the associated address:port pair.
    pub app_protocol: AppProtocol,
    /// Check if source or destination is an IPv6 address longer than 25 bytes (used for Display
    pub very_long_address: bool,
    /// Flag to determine which of the address is that of the sniffed adapter or remote
    pub traffic_type: TrafficType,
    /// Country of the remote IP address
    pub country: String,
    /// Integer corresponding to the index inside the connections map
    pub index: usize,
    /// Flag that indicates if this connection is marked as favourite
    pub is_favorite: bool,
}

impl InfoAddressPortPair {
    pub fn print_gui(&self) -> String {
        self.to_string()
            .get(0..37)
            .unwrap()
            .to_string()
            .replace('|', "")
            .add(&*format!(" {} ", &self.country))
    }
}

impl fmt::Display for InfoAddressPortPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes_string = get_formatted_bytes_string(self.transmitted_bytes);

        let app_string = match self.app_protocol {
            AppProtocol::Other => "Other".to_string(),
            _ => self.app_protocol.to_string(),
        };

        if self.very_long_address {
            write!(
                f,
                "{:^9}|{:>10}  |{:>10}  | {} | {} |",
                app_string,
                self.transmitted_packets,
                bytes_string,
                self.initial_timestamp.to_string().get(0..19).unwrap(),
                self.final_timestamp.to_string().get(0..19).unwrap()
            )
        } else {
            write!(
                f,
                "{:^9}|{:>10}  |{:>10}  | {} | {} |{}",
                app_string,
                self.transmitted_packets,
                bytes_string,
                self.initial_timestamp.to_string().get(0..19).unwrap(),
                self.final_timestamp.to_string().get(0..19).unwrap(),
                " ".repeat(40)
            )
        }
    }
}
