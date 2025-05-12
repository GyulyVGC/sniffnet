//! Module defining the `ReportInfo` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use crate::Service;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use std::collections::{HashMap, HashSet};

/// Struct to be shared between the threads in charge of parsing packets and update reports.
#[derive(Debug, Clone, Default)]
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
    /// Timestamp of the latest packet
    pub last_packet_timestamp: i64,
    /// Map of the filtered traffic
    pub map: HashMap<AddressPortPair, InfoAddressPortPair>,
    /// Map of the upper layer services with their data info
    pub services: HashMap<Service, DataInfo>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, DataInfoHost>,

    // todo!!!
    /// Collection of the favorite hosts
    pub favorite_hosts: HashSet<Host>,
    /// Collection of favorite hosts that exchanged data in the last interval
    pub favorites_last_interval: HashSet<Host>,
}

impl InfoTraffic {
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

    pub fn refresh(&mut self, other: Self) {
        self.tot_in_bytes += other.tot_in_bytes;
        self.tot_out_bytes += other.tot_out_bytes;
        self.tot_in_packets += other.tot_in_packets;
        self.tot_out_packets += other.tot_out_packets;
        self.all_packets += other.all_packets;
        self.all_bytes += other.all_bytes;
        self.dropped_packets = other.dropped_packets;
        self.last_packet_timestamp = other.last_packet_timestamp;

        for (key, value) in other.map {
            self.map
                .entry(key)
                .and_modify(|x| x.refresh(&value))
                .or_insert(value);
        }

        for (key, value) in other.services {
            self.services
                .entry(key)
                .and_modify(|x| *x += value)
                .or_insert(value);
        }

        for (key, value) in other.hosts {
            self.hosts
                .entry(key)
                .and_modify(|x| x.data_info += value.data_info)
                .or_insert(value);
        }

        // todo: remove this
        // let mut total_packets = 0;
        // for (_, data_info_host) in self.hosts.iter() {
        //     total_packets += data_info_host.data_info.tot_packets();
        // }
        // println!("Total packets from all hosts: {}", total_packets);
    }

    pub fn take(&mut self, last_packet_timestamp: i64) -> Self {
        let info_traffic = InfoTraffic {
            last_packet_timestamp,
            ..InfoTraffic::default()
        };
        std::mem::replace(self, info_traffic)
    }
}
