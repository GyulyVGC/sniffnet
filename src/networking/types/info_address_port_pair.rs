//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::Service;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::types::sort_type::SortType;
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

    pub fn transmitted_data(&self, data_repr: DataRepr) -> u128 {
        match data_repr {
            DataRepr::Packets => self.transmitted_packets,
            DataRepr::Bytes => self.transmitted_bytes,
            DataRepr::Bits => self.transmitted_bytes * 8,
        }
    }

    pub fn compare(&self, other: &Self, sort_type: SortType, data_repr: DataRepr) -> Ordering {
        match sort_type {
            SortType::Ascending => self
                .transmitted_data(data_repr)
                .cmp(&other.transmitted_data(data_repr)),
            SortType::Descending => other
                .transmitted_data(data_repr)
                .cmp(&self.transmitted_data(data_repr)),
            SortType::Neutral => other.final_timestamp.cmp(&self.final_timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::types::data_representation::DataRepr;
    use crate::report::types::sort_type::SortType;

    #[test]
    fn test_info_address_port_pair_data() {
        let pair1 = InfoAddressPortPair {
            transmitted_bytes: 1000,
            transmitted_packets: 10,
            final_timestamp: Timestamp::new(8, 1300),
            ..Default::default()
        };
        let pair2 = InfoAddressPortPair {
            transmitted_bytes: 1100,
            transmitted_packets: 8,
            final_timestamp: Timestamp::new(15, 0),
            ..Default::default()
        };

        assert_eq!(pair1.transmitted_data(DataRepr::Bytes), 1000);
        assert_eq!(pair1.transmitted_data(DataRepr::Packets), 10);
        assert_eq!(pair1.transmitted_data(DataRepr::Bits), 8000);

        assert_eq!(pair2.transmitted_data(DataRepr::Bytes), 1100);
        assert_eq!(pair2.transmitted_data(DataRepr::Packets), 8);
        assert_eq!(pair2.transmitted_data(DataRepr::Bits), 8800);

        assert_eq!(
            pair1.compare(&pair2, SortType::Ascending, DataRepr::Bytes),
            Ordering::Less
        );
        assert_eq!(
            pair1.compare(&pair2, SortType::Descending, DataRepr::Bytes),
            Ordering::Greater
        );
        assert_eq!(
            pair1.compare(&pair2, SortType::Neutral, DataRepr::Bytes),
            Ordering::Greater
        );

        assert_eq!(
            pair1.compare(&pair2, SortType::Ascending, DataRepr::Packets),
            Ordering::Greater
        );
        assert_eq!(
            pair1.compare(&pair2, SortType::Descending, DataRepr::Packets),
            Ordering::Less
        );
        assert_eq!(
            pair1.compare(&pair2, SortType::Neutral, DataRepr::Packets),
            Ordering::Greater
        );

        assert_eq!(
            pair1.compare(&pair2, SortType::Ascending, DataRepr::Bits),
            Ordering::Less
        );
        assert_eq!(
            pair1.compare(&pair2, SortType::Descending, DataRepr::Bits),
            Ordering::Greater
        );
        assert_eq!(
            pair1.compare(&pair2, SortType::Neutral, DataRepr::Bits),
            Ordering::Greater
        );
    }
}
