//! Module defining the `ReportInfo` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::collections::{HashMap, HashSet};

use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::Service;

/// Struct to be shared between the threads in charge of parsing packets and update reports.
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
    /// Map of the filtered traffic
    pub map: HashMap<AddressPortPair, InfoAddressPortPair>,
    /// Collection of the favorite hosts
    pub favorite_hosts: HashSet<Host>,
    /// Collection of favorite hosts that exchanged data in the last interval
    pub favorites_last_interval: HashSet<Host>,
    /// Map of the upper layer services with their data info
    pub services: HashMap<Service, DataInfo>,
    /// Map of the addresses waiting for a rDNS resolution; used to NOT send multiple rDNS for the same address
    pub addresses_waiting_resolution: HashMap<String, DataInfo>,
    /// Map of the resolved addresses with their full rDNS value and the corresponding host
    pub addresses_resolved: HashMap<String, (String, Host)>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, DataInfoHost>,
}

impl InfoTraffic {
    /// Constructs a new `InfoTraffic` element.
    pub fn new() -> Self {
        InfoTraffic {
            tot_in_bytes: 0,
            tot_out_bytes: 0,
            tot_in_packets: 0,
            tot_out_packets: 0,
            all_packets: 0,
            all_bytes: 0,
            dropped_packets: 0,
            map: HashMap::new(),
            favorite_hosts: HashSet::new(),
            favorites_last_interval: HashSet::new(),
            services: HashMap::new(),
            addresses_waiting_resolution: HashMap::new(),
            addresses_resolved: HashMap::new(),
            hosts: HashMap::new(),
        }
    }

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
}
