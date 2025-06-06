//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::collections::HashMap;

use crate::Service;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::types::timestamp::Timestamp;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each `InfoAddressPortPair` struct is associated to a single address:port pair.
#[derive(Clone, Default, Debug)]
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
    pub initial_timestamp: Timestamp,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: Timestamp,
    /// Upper layer service carried by the associated address:port pair.
    pub service: Service,
    /// Determines if the connection is incoming or outgoing
    pub traffic_direction: TrafficDirection,
    /// Types of the ICMP messages exchanged, with the relative count (this is empty if not ICMP)
    pub icmp_types: HashMap<IcmpType, usize>,
    /// Types of the ARP operations, with the relative count (this is empty if not ARP)
    pub arp_types: HashMap<ArpType, usize>,
}

impl InfoAddressPortPair {
    pub fn refresh(&mut self, other: &Self) {
        self.transmitted_bytes += other.transmitted_bytes;
        self.transmitted_packets += other.transmitted_packets;
        self.final_timestamp = other.final_timestamp;
        self.service = other.service;
        self.traffic_direction = other.traffic_direction;
        for (icmp_type, count) in &other.icmp_types {
            self.icmp_types
                .entry(*icmp_type)
                .and_modify(|v| *v += count)
                .or_insert(*count);
        }
        for (arp_type, count) in &other.arp_types {
            self.arp_types
                .entry(*arp_type)
                .and_modify(|v| *v += count)
                .or_insert(*count);
        }
    }
}
