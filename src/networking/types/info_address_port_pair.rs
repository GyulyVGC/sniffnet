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
    /// Set of application layer protocols carried by the associated address:port pair.
    pub app_protocol: AppProtocol,
    /// Check if source or destination is an IPv6 address longer than 25 bytes (used for layout)
    pub very_long_address: bool,
    // /// Country of the remote IP address
    // pub country: String,
    // /// Autonomous System of the remote IP address
    // pub asn: Asn,
    /// Integer corresponding to the index inside the connections map
    pub index: usize,
    /// Determines if the connection is incoming or outgoing
    pub traffic_direction: TrafficDirection,
    // /// Determines if the connection is unicast, multicast or broadcast
    // pub traffic_type: TrafficType,
    // /// Determines if the connection is local
    // pub is_local: bool,
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
            very_long_address: false,
            traffic_direction: TrafficDirection::default(),
            index: 0,
        }
    }
}

impl InfoAddressPortPair {
    pub fn print_gui(&self) -> String {
        self.to_string()
            .get(0..37)
            .unwrap()
            .to_string()
            .replace('|', "")
    }

    // pub fn get_host(&self) -> Host {
    //     if self.r_dns.is_none() {
    //         return Host::default();
    //     }
    //
    //     let r_dns = self.r_dns.as_ref().unwrap().clone();
    //
    //     let domain = if r_dns.parse::<IpAddr>().is_ok() || r_dns.is_empty() {
    //         // rDNS is equal to the corresponding IP address
    //         r_dns
    //     } else {
    //         let parts: Vec<&str> = r_dns.split('.').collect();
    //         if parts.len() >= 2 {
    //             parts
    //                 .get(parts.len() - 2..)
    //                 .unwrap_or(&parts)
    //                 .iter()
    //                 .fold(Vec::new(), |mut vec, part| {
    //                     vec.push((*part).to_string());
    //                     vec
    //                 })
    //                 .join(".")
    //         } else {
    //             r_dns
    //         }
    //     };
    //
    //     Host {
    //         domain,
    //         asn: self.asn.clone(),
    //         country: self.country.clone(),
    //     }
    // }
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
                "{:^9}|{:>10}  |{:>9}   | {} | {} |",
                app_string,
                self.transmitted_packets,
                bytes_string,
                self.initial_timestamp.to_string().get(0..19).unwrap(),
                self.final_timestamp.to_string().get(0..19).unwrap()
            )
        } else {
            write!(
                f,
                "{:^9}|{:>10}  |{:>9}   | {} | {} |{}",
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
