//! Module defining the `DataInfo` struct, which represents incoming and outgoing packets and bytes.

use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::types::sort_type::SortType;
use std::cmp::Ordering;
use std::time::Instant;

/// Amount of exchanged data (packets and bytes) incoming and outgoing, with the timestamp of the latest occurrence
// data fields are private to make them only editable via the provided methods: needed to correctly refresh timestamps
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DataInfo {
    /// Incoming packets
    incoming_packets: u128,
    /// Outgoing packets
    outgoing_packets: u128,
    /// Incoming bytes
    incoming_bytes: u128,
    /// Outgoing bytes
    outgoing_bytes: u128,
    /// Latest instant of occurrence
    final_instant: Instant,
}

impl DataInfo {
    pub fn incoming_data(&self, data_repr: DataRepr) -> u128 {
        match data_repr {
            DataRepr::Packets => self.incoming_packets,
            DataRepr::Bytes => self.incoming_bytes,
            DataRepr::Bits => self.incoming_bytes * 8,
        }
    }

    pub fn outgoing_data(&self, data_repr: DataRepr) -> u128 {
        match data_repr {
            DataRepr::Packets => self.outgoing_packets,
            DataRepr::Bytes => self.outgoing_bytes,
            DataRepr::Bits => self.outgoing_bytes * 8,
        }
    }

    pub fn tot_data(&self, data_repr: DataRepr) -> u128 {
        self.incoming_data(data_repr) + self.outgoing_data(data_repr)
    }

    pub fn add_packet(&mut self, bytes: u128, traffic_direction: TrafficDirection) {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            self.outgoing_packets += 1;
            self.outgoing_bytes += bytes;
        } else {
            self.incoming_packets += 1;
            self.incoming_bytes += bytes;
        }
        self.final_instant = Instant::now();
    }

    pub fn add_packets(&mut self, packets: u128, bytes: u128, traffic_direction: TrafficDirection) {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            self.outgoing_packets += packets;
            self.outgoing_bytes += bytes;
        } else {
            self.incoming_packets += packets;
            self.incoming_bytes += bytes;
        }
    }

    pub fn new_with_first_packet(bytes: u128, traffic_direction: TrafficDirection) -> Self {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            Self {
                incoming_packets: 0,
                outgoing_packets: 1,
                incoming_bytes: 0,
                outgoing_bytes: bytes,
                final_instant: Instant::now(),
            }
        } else {
            Self {
                incoming_packets: 1,
                outgoing_packets: 0,
                incoming_bytes: bytes,
                outgoing_bytes: 0,
                final_instant: Instant::now(),
            }
        }
    }

    pub fn refresh(&mut self, rhs: Self) {
        self.incoming_packets += rhs.incoming_packets;
        self.outgoing_packets += rhs.outgoing_packets;
        self.incoming_bytes += rhs.incoming_bytes;
        self.outgoing_bytes += rhs.outgoing_bytes;
        self.final_instant = rhs.final_instant;
    }

    pub fn compare(&self, other: &Self, sort_type: SortType, data_repr: DataRepr) -> Ordering {
        match sort_type {
            SortType::Ascending => self.tot_data(data_repr).cmp(&other.tot_data(data_repr)),
            SortType::Descending => other.tot_data(data_repr).cmp(&self.tot_data(data_repr)),
            SortType::Neutral => other.final_instant.cmp(&self.final_instant),
        }
    }

    #[cfg(test)]
    pub fn new_for_tests(
        incoming_packets: u128,
        outgoing_packets: u128,
        incoming_bytes: u128,
        outgoing_bytes: u128,
    ) -> Self {
        Self {
            incoming_packets,
            outgoing_packets,
            incoming_bytes,
            outgoing_bytes,
            final_instant: Instant::now(),
        }
    }
}

impl Default for DataInfo {
    fn default() -> Self {
        Self {
            incoming_packets: 0,
            outgoing_packets: 0,
            incoming_bytes: 0,
            outgoing_bytes: 0,
            final_instant: Instant::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::types::traffic_direction::TrafficDirection;

    #[test]
    fn test_data_info() {
        // in_packets: 0, out_packets: 0, in_bytes: 0, out_bytes: 0
        let mut data_info_1 = DataInfo::new_with_first_packet(123, TrafficDirection::Incoming);
        // 1, 0, 123, 0
        data_info_1.add_packet(100, TrafficDirection::Incoming);
        // 2, 0, 223, 0
        data_info_1.add_packet(200, TrafficDirection::Outgoing);
        // 2, 1, 223, 200
        data_info_1.add_packets(11, 1200, TrafficDirection::Outgoing);
        // 2, 12, 223, 1400
        data_info_1.add_packets(5, 500, TrafficDirection::Incoming);
        // 7, 12, 723, 1400

        assert_eq!(data_info_1.incoming_packets, 7);
        assert_eq!(data_info_1.outgoing_packets, 12);
        assert_eq!(data_info_1.incoming_bytes, 723);
        assert_eq!(data_info_1.outgoing_bytes, 1400);

        assert_eq!(data_info_1.tot_data(DataRepr::Packets), 19);
        assert_eq!(data_info_1.tot_data(DataRepr::Bytes), 2123);
        assert_eq!(data_info_1.tot_data(DataRepr::Bits), 16984);

        assert_eq!(data_info_1.incoming_data(DataRepr::Packets), 7);
        assert_eq!(data_info_1.incoming_data(DataRepr::Bytes), 723);
        assert_eq!(data_info_1.incoming_data(DataRepr::Bits), 5784);

        assert_eq!(data_info_1.outgoing_data(DataRepr::Packets), 12);
        assert_eq!(data_info_1.outgoing_data(DataRepr::Bytes), 1400);
        assert_eq!(data_info_1.outgoing_data(DataRepr::Bits), 11200);

        // sleep a little to have a different final_instant
        std::thread::sleep(std::time::Duration::from_millis(10));
        let mut data_info_2 = DataInfo::new_with_first_packet(100, TrafficDirection::Outgoing);
        // 0, 1, 0, 100
        data_info_2.add_packets(19, 300, TrafficDirection::Outgoing);
        // 0, 20, 0, 400

        assert_eq!(data_info_2.incoming_packets, 0);
        assert_eq!(data_info_2.outgoing_packets, 20);
        assert_eq!(data_info_2.incoming_bytes, 0);
        assert_eq!(data_info_2.outgoing_bytes, 400);

        assert_eq!(data_info_2.tot_data(DataRepr::Packets), 20);
        assert_eq!(data_info_2.tot_data(DataRepr::Bytes), 400);
        assert_eq!(data_info_2.tot_data(DataRepr::Bits), 3200);

        assert_eq!(data_info_2.incoming_data(DataRepr::Packets), 0);
        assert_eq!(data_info_2.incoming_data(DataRepr::Bytes), 0);
        assert_eq!(data_info_2.incoming_data(DataRepr::Bits), 0);

        assert_eq!(data_info_2.outgoing_data(DataRepr::Packets), 20);
        assert_eq!(data_info_2.outgoing_data(DataRepr::Bytes), 400);
        assert_eq!(data_info_2.outgoing_data(DataRepr::Bits), 3200);

        // compare data_info_1 and data_info_2

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataRepr::Packets),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataRepr::Packets),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataRepr::Packets),
            Ordering::Greater
        );

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataRepr::Bytes),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataRepr::Bytes),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataRepr::Bytes),
            Ordering::Greater
        );

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataRepr::Bits),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataRepr::Bits),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataRepr::Bits),
            Ordering::Greater
        );

        // refresh data_info_1 with data_info_2
        assert!(data_info_1.final_instant < data_info_2.final_instant);
        data_info_1.refresh(data_info_2);

        // data_info_1 should now contain the sum of both data_info_1 and data_info_2
        assert_eq!(data_info_1.incoming_packets, 7);
        assert_eq!(data_info_1.outgoing_packets, 32);
        assert_eq!(data_info_1.incoming_bytes, 723);
        assert_eq!(data_info_1.outgoing_bytes, 1800);
        assert_eq!(data_info_1.final_instant, data_info_2.final_instant);
    }
}
