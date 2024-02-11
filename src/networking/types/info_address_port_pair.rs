//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::Service;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each `InfoAddressPortPair` struct is associated to a single address:port pair.
#[derive(Clone, Default)]
pub struct InfoAddressPortPair {
    /// Source MAC address
    pub mac_address1: Option<String>,
    /// Destination MAC address
    pub mac_address2: Option<String>,
    /// Amount of bytes transmitted between the pair.
    pub transmitted_bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub transmitted_packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: DateTime<Local>,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: DateTime<Local>,
    /// Upper layer service carried by the associated address:port pair.
    pub service: Service,
    /// Determines if the connection is incoming or outgoing
    pub traffic_direction: TrafficDirection,
    /// Types of the ICMP messages exchanged, with the relative count (this is empty if not ICMP)
    pub icmp_types: HashMap<IcmpType, usize>,
}
