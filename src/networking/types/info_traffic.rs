//! Module defining the `ReportInfo` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use crate::Service;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::types::timestamp::Timestamp;
use std::collections::{HashMap, HashSet};

/// Struct containing overall traffic statistics and data.
#[derive(Debug, Default)]
pub struct InfoTraffic {
    /// Total amount of filtered bytes received.
    pub tot_in_bytes: u128,
    /// Total amount of filtered bytes sent.
    pub tot_out_bytes: u128,
    /// Total amount of filtered packets received.
    pub tot_in_packets: u128,
    /// Total amount of filtered packets sent.
    pub tot_out_packets: u128,
    /// Total packets including those not filtered
    pub all_packets: u128,
    /// Total bytes including those not filtered
    pub all_bytes: u128,
    /// Number of dropped packets
    pub dropped_packets: u32,
    /// Timestamp of the latest filtered packet
    pub last_filtered_packet_timestamp: Timestamp,
    /// Total sent bytes filtered before the current time interval
    pub tot_out_bytes_prev: u128,
    /// Total received bytes filtered before the current time interval
    pub tot_in_bytes_prev: u128,
    /// Total sent packets filtered before the current time interval
    pub tot_out_packets_prev: u128,
    /// Total received packets filtered before the current time interval
    pub tot_in_packets_prev: u128,
    /// Map of the filtered traffic
    pub map: HashMap<AddressPortPair, InfoAddressPortPair>,
    /// Map of the upper layer services with their data info
    pub services: HashMap<Service, DataInfo>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, DataInfoHost>,
    /// Collection of favorite hosts that exchanged data in the last interval
    pub favorites_last_interval: HashSet<Host>,
}

impl InfoTraffic {
    pub fn refresh(&mut self, msg: InfoTrafficMessage, favorites: &HashSet<Host>) {
        self.tot_out_bytes_prev = self.tot_out_bytes;
        self.tot_in_bytes_prev = self.tot_in_bytes;
        self.tot_out_packets_prev = self.tot_out_packets;
        self.tot_in_packets_prev = self.tot_in_packets;

        self.tot_in_bytes += msg.tot_in_bytes;
        self.tot_out_bytes += msg.tot_out_bytes;
        self.tot_in_packets += msg.tot_in_packets;
        self.tot_out_packets += msg.tot_out_packets;
        self.all_packets += msg.all_packets;
        self.all_bytes += msg.all_bytes;
        self.dropped_packets = msg.dropped_packets;

        // it can happen they're equal due to dis-alignments caused by the timeout in the packet capture
        if self.last_filtered_packet_timestamp.secs() == msg.last_filtered_packet_timestamp.secs() {
            self.last_filtered_packet_timestamp.add_one_sec();
        } else {
            self.last_filtered_packet_timestamp = msg.last_filtered_packet_timestamp;
        }

        for (key, value) in msg.map {
            self.map
                .entry(key)
                .and_modify(|x| x.refresh(&value))
                .or_insert(value);
        }

        for (key, value) in msg.services {
            self.services
                .entry(key)
                .and_modify(|x| x.refresh(value))
                .or_insert(value);
        }

        for (key, value) in msg.hosts {
            self.hosts
                .entry(key)
                .and_modify(|x| x.refresh(&value))
                .or_insert(value);
        }

        self.favorites_last_interval = msg
            .potential_favorites
            .into_iter()
            .filter(|h| favorites.contains(h))
            .collect();
    }
}

/// Struct containing traffic statistics and data related to the last time interval.
#[derive(Debug, Clone, Default)]
pub struct InfoTrafficMessage {
    /// Total amount of filtered bytes received.
    pub tot_in_bytes: u128,
    /// Total amount of filtered bytes sent.
    pub tot_out_bytes: u128,
    /// Total amount of filtered packets received.
    pub tot_in_packets: u128,
    /// Total amount of filtered packets sent.
    pub tot_out_packets: u128,
    /// Total packets including those not filtered
    pub all_packets: u128,
    /// Total bytes including those not filtered
    pub all_bytes: u128,
    /// Number of dropped packets
    pub dropped_packets: u32,
    /// Timestamp of the latest filtered packet
    pub last_filtered_packet_timestamp: Timestamp,
    /// Map of the filtered traffic
    pub map: HashMap<AddressPortPair, InfoAddressPortPair>,
    /// Map of the upper layer services with their data info
    pub services: HashMap<Service, DataInfo>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, DataInfoHost>,
    /// Collection of favorite hosts that exchanged data in the last interval
    pub potential_favorites: HashSet<Host>,
}

impl InfoTrafficMessage {
    pub fn add_packet(&mut self, bytes: u128, traffic_direction: TrafficDirection) {
        if traffic_direction == TrafficDirection::Outgoing {
            //increment number of sent packets and bytes
            self.tot_out_packets += 1;
            self.tot_out_bytes += bytes;
        } else {
            //increment number of received packets and bytes
            self.tot_in_packets += 1;
            self.tot_in_bytes += bytes;
        }
    }

    pub fn take_but_leave_timestamp(&mut self) -> Self {
        let info_traffic = Self {
            last_filtered_packet_timestamp: self.last_filtered_packet_timestamp,
            ..Self::default()
        };
        std::mem::replace(self, info_traffic)
    }
}
