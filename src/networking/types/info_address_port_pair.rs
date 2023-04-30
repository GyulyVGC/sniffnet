//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::fmt;
use std::net::IpAddr;
use std::ops::Add;

use chrono::{DateTime, Local};

use crate::networking::types::asn::Asn;
use crate::networking::types::host::Host;
use crate::networking::types::traffic_type::TrafficType;
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
    /// Check if source or destination is an IPv6 address longer than 25 bytes (used for Display
    pub very_long_address: bool,
    /// Flag to determine which of the address is that of the sniffed adapter or remote
    pub traffic_type: TrafficType,
    /// Country of the remote IP address
    pub country: String,
    /// Autonomous System of the remote IP address
    pub asn: Asn,
    /// Reverse DNS lookup of the remote address. It is set to `None` if not requested yet.
    pub r_dns: Option<String>,
    /// Integer corresponding to the index inside the connections map
    pub index: usize,
}

impl Default for InfoAddressPortPair {
    fn default() -> Self {
        Self {
            mac_address1: String::new(),
            mac_address2: String::new(),
            transmitted_bytes: 0,
            transmitted_packets: 0,
            initial_timestamp: Default::default(),
            final_timestamp: Default::default(),
            app_protocol: AppProtocol::Other,
            very_long_address: false,
            traffic_type: TrafficType::Other,
            country: String::new(),
            asn: Default::default(),
            r_dns: None,
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
            .add(&*format!(" {} ", &self.country))
    }

    pub fn get_host(&self) -> Host {
        if self.r_dns.is_none() {
            return Host::default();
        }

        let r_dns = self.r_dns.as_ref().unwrap().clone();

        let domain = if r_dns.parse::<IpAddr>().is_ok() || r_dns.is_empty() {
            // rDNS is equal to the corresponding IP address
            r_dns
        } else {
            let parts: Vec<&str> = r_dns.split('.').collect();
            parts
                .get(parts.len() - 2..)
                .unwrap_or(&parts)
                .iter()
                .fold(Vec::new(), |mut vec, part| {
                    vec.push(part.to_string());
                    vec
                })
                .join(".")
        };

        Host {
            domain,
            asn: self.asn.clone(),
            country: self.country.clone(),
        }
    }

    /// An rDNS resolution has already been requested for this host
    pub fn r_dns_already_requested(&self) -> bool {
        self.r_dns.is_some()
    }

    /// An rDNS resolution has already completed for this host
    pub fn r_dns_already_resolved(&self) -> bool {
        self.r_dns.is_some() && !self.r_dns.as_ref().unwrap().is_empty()
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
