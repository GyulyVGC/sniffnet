//! Module defining the `InfoAddressPortPair` struct, useful to format the output report file and
//! to keep track of statistics about the sniffed traffic.

use crate::Service;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::message_type::MessageType;
use crate::networking::types::program::Program;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::types::sort_type::SortType;
use crate::utils::types::timestamp::Timestamp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

/// Struct useful to format the output report file and to keep track of statistics about the sniffed traffic.
///
/// Each `InfoAddressPortPair` struct is associated to a single address:port pair.
#[derive(Clone, Debug)]
pub struct InfoAddressPortPair {
    // TODO: overridden with the latest values, should be a list of all MAC addresses with their counts?
    /// Source MAC address (last seen), if available
    pub src_mac: Option<[u8; 6]>,
    /// Destination MAC address (last seen), if available
    pub dst_mac: Option<[u8; 6]>,
    /// Amount of bytes transmitted between the pair.
    pub bytes: u128,
    /// Amount of packets transmitted between the pair.
    pub packets: u128,
    /// First occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub initial_timestamp: Timestamp,
    /// Last occurrence of information exchange featuring the associate address:port pair as a source or destination.
    pub final_timestamp: Timestamp,
    /// Final instance of information exchange featuring the associate address:port pair as a source or destination (used for Program).
    pub final_instant: Instant,
    /// Upper layer service carried by the associated address:port pair.
    pub service: Service,
    /// Determines if the connection is incoming or outgoing
    pub traffic_direction: TrafficDirection,
    /// Types of the messages exchanged, with the relative count (this is empty for protocols without message types)
    pub message_types: HashMap<MessageType, usize>,
    // TODO: vlan_id is overridden with the latest value, maybe should be moved in AddressPortPair
    /// Latest VLAN ID of the associated address:port pair, if any
    pub vlan_id: Option<u16>,
    /// Whether the remote address is blacklisted
    pub is_blacklisted: bool,
    /// The program associated to this pair
    pub program: Program,
}

impl InfoAddressPortPair {
    pub fn refresh(&mut self, other: &Self) {
        let Self {
            src_mac,
            dst_mac,
            bytes,
            packets,
            initial_timestamp,
            final_timestamp,
            final_instant,
            service,
            traffic_direction,
            message_types,
            vlan_id,
            is_blacklisted,
            // self.program MUST NOT be refreshed here
            program: _,
        } = other;

        self.bytes += bytes;
        self.packets += packets;
        self.src_mac = *src_mac;
        self.dst_mac = *dst_mac;
        self.vlan_id = *vlan_id;
        if *initial_timestamp < self.initial_timestamp {
            self.initial_timestamp = *initial_timestamp;
        }
        if *final_timestamp > self.final_timestamp {
            self.final_timestamp = *final_timestamp;
        }
        if *final_instant > self.final_instant {
            self.final_instant = *final_instant;
        }
        self.service = *service;
        self.is_blacklisted = *is_blacklisted;
        self.traffic_direction = *traffic_direction;
        for (message_type, count) in message_types {
            self.message_types
                .entry(*message_type)
                .and_modify(|v| *v += count)
                .or_insert(*count);
        }
    }

    pub fn transmitted_data(&self, data_repr: DataRepr) -> u128 {
        match data_repr {
            DataRepr::Packets => self.packets,
            DataRepr::Bytes => self.bytes,
            DataRepr::Bits => self.bytes * 8,
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

    pub fn data_info(&self) -> DataInfo {
        let mut data_info = DataInfo::default();
        data_info.add_packets(
            self.packets,
            self.bytes,
            self.traffic_direction,
            self.final_instant,
        );
        data_info
    }
}

impl Default for InfoAddressPortPair {
    fn default() -> Self {
        Self {
            src_mac: None,
            dst_mac: None,
            bytes: 0,
            packets: 0,
            initial_timestamp: Timestamp::default(),
            final_timestamp: Timestamp::default(),
            final_instant: Instant::now(),
            service: Service::default(),
            traffic_direction: TrafficDirection::default(),
            message_types: HashMap::new(),
            vlan_id: None,
            is_blacklisted: false,
            program: Program::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::types::data_representation::DataRepr;
    use crate::report::types::sort_type::SortType;

    #[test]
    fn test_refresh_only_widens_the_timestamp_window() {
        let mut pair = InfoAddressPortPair {
            initial_timestamp: Timestamp::new(10, 0),
            final_timestamp: Timestamp::new(20, 0),
            ..Default::default()
        };

        // inside the window does not change anything
        pair.refresh(&InfoAddressPortPair {
            initial_timestamp: Timestamp::new(12, 0),
            final_timestamp: Timestamp::new(18, 0),
            ..Default::default()
        });
        assert_eq!(pair.initial_timestamp, Timestamp::new(10, 0));
        assert_eq!(pair.final_timestamp, Timestamp::new(20, 0));

        // outside the window extends
        pair.refresh(&InfoAddressPortPair {
            initial_timestamp: Timestamp::new(5, 0),
            final_timestamp: Timestamp::new(25, 0),
            ..Default::default()
        });
        assert_eq!(pair.initial_timestamp, Timestamp::new(5, 0));
        assert_eq!(pair.final_timestamp, Timestamp::new(25, 0));
    }

    #[test]
    fn test_info_address_port_pair_data() {
        let pair1 = InfoAddressPortPair {
            bytes: 1000,
            packets: 10,
            final_timestamp: Timestamp::new(8, 1300),
            ..Default::default()
        };
        let pair2 = InfoAddressPortPair {
            bytes: 1100,
            packets: 8,
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
