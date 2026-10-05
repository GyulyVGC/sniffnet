//! Module defining the `DataInfo` struct, which represents incoming and outgoing packets and bytes.

use crate::networking::types::data_representation::DataUnit;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::types::sort_type::SortType;
use std::cmp::Ordering;
use std::time::Instant;

/// Amount of exchanged data (packets and bytes) incoming and outgoing, with the timestamp of the latest occurrence
// data fields are private to make them only editable via the provided methods: needed to correctly refresh timestamps
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct DataInfo {
    /// Incoming packets
    incoming_packets: u128,
    /// Outgoing packets
    outgoing_packets: u128,
    /// Incoming bytes
    incoming_bytes: u128,
    /// Outgoing bytes
    outgoing_bytes: u128,
    /// Latest instant of occurrence. Initialized to None by Default.
    final_instant: Option<Instant>,
}

impl DataInfo {
    pub fn incoming_data(&self, data_repr: DataUnit) -> u128 {
        match data_repr {
            DataUnit::Packets => self.incoming_packets,
            DataUnit::Bytes => self.incoming_bytes,
            DataUnit::Bits => self.incoming_bytes * 8,
        }
    }

    pub fn outgoing_data(&self, data_repr: DataUnit) -> u128 {
        match data_repr {
            DataUnit::Packets => self.outgoing_packets,
            DataUnit::Bytes => self.outgoing_bytes,
            DataUnit::Bits => self.outgoing_bytes * 8,
        }
    }

    pub fn tot_data(&self, data_repr: DataUnit) -> u128 {
        self.incoming_data(data_repr) + self.outgoing_data(data_repr)
    }

    pub fn add_packets(
        &mut self,
        packets: u128,
        bytes: u128,
        traffic_direction: TrafficDirection,
        final_instant: Instant,
    ) {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            self.outgoing_packets += packets;
            self.outgoing_bytes += bytes;
        } else {
            self.incoming_packets += packets;
            self.incoming_bytes += bytes;
        }
        self.final_instant = Some(final_instant);
    }

    pub fn refresh(&mut self, rhs: Self) {
        let Self {
            incoming_packets,
            outgoing_packets,
            incoming_bytes,
            outgoing_bytes,
            final_instant,
        } = rhs;

        self.incoming_packets += incoming_packets;
        self.outgoing_packets += outgoing_packets;
        self.incoming_bytes += incoming_bytes;
        self.outgoing_bytes += outgoing_bytes;
        // None < Some(_) by Option's Ord, so a Some will always win over a default None
        if final_instant > self.final_instant {
            self.final_instant = final_instant;
        }
    }

    pub fn compare(&self, other: &Self, sort_type: SortType, data_repr: DataUnit) -> Ordering {
        match sort_type {
            SortType::Ascending => self.tot_data(data_repr).cmp(&other.tot_data(data_repr)),
            SortType::Descending => other.tot_data(data_repr).cmp(&self.tot_data(data_repr)),
            SortType::Neutral => other.final_instant.cmp(&self.final_instant),
        }
    }

    pub fn is_within_same_second(&self, other: &Self) -> bool {
        if let (Some(self_instant), Some(other_instant)) = (self.final_instant, other.final_instant)
        {
            if self_instant < other_instant {
                other_instant.duration_since(self_instant).as_secs() == 0
            } else {
                self_instant.duration_since(other_instant).as_secs() == 0
            }
        } else {
            false
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
            final_instant: Some(Instant::now()),
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
        let mut data_info_1 = DataInfo::default();
        data_info_1.add_packets(1, 123, TrafficDirection::Incoming, Instant::now());
        // 1, 0, 123, 0
        data_info_1.add_packets(1, 100, TrafficDirection::Incoming, Instant::now());
        // 2, 0, 223, 0
        data_info_1.add_packets(1, 200, TrafficDirection::Outgoing, Instant::now());
        // 2, 1, 223, 200
        data_info_1.add_packets(11, 1200, TrafficDirection::Outgoing, Instant::now());
        // 2, 12, 223, 1400
        data_info_1.add_packets(5, 500, TrafficDirection::Incoming, Instant::now());
        // 7, 12, 723, 1400

        assert_eq!(data_info_1.incoming_packets, 7);
        assert_eq!(data_info_1.outgoing_packets, 12);
        assert_eq!(data_info_1.incoming_bytes, 723);
        assert_eq!(data_info_1.outgoing_bytes, 1400);

        assert_eq!(data_info_1.tot_data(DataUnit::Packets), 19);
        assert_eq!(data_info_1.tot_data(DataUnit::Bytes), 2123);
        assert_eq!(data_info_1.tot_data(DataUnit::Bits), 16984);

        assert_eq!(data_info_1.incoming_data(DataUnit::Packets), 7);
        assert_eq!(data_info_1.incoming_data(DataUnit::Bytes), 723);
        assert_eq!(data_info_1.incoming_data(DataUnit::Bits), 5784);

        assert_eq!(data_info_1.outgoing_data(DataUnit::Packets), 12);
        assert_eq!(data_info_1.outgoing_data(DataUnit::Bytes), 1400);
        assert_eq!(data_info_1.outgoing_data(DataUnit::Bits), 11200);

        // sleep a little to have a different final_instant
        std::thread::sleep(std::time::Duration::from_millis(10));
        let mut data_info_2 = DataInfo::default();
        data_info_2.add_packets(1, 100, TrafficDirection::Outgoing, Instant::now());
        // 0, 1, 0, 100
        data_info_2.add_packets(19, 300, TrafficDirection::Outgoing, Instant::now());
        // 0, 20, 0, 400

        assert_eq!(data_info_2.incoming_packets, 0);
        assert_eq!(data_info_2.outgoing_packets, 20);
        assert_eq!(data_info_2.incoming_bytes, 0);
        assert_eq!(data_info_2.outgoing_bytes, 400);

        assert_eq!(data_info_2.tot_data(DataUnit::Packets), 20);
        assert_eq!(data_info_2.tot_data(DataUnit::Bytes), 400);
        assert_eq!(data_info_2.tot_data(DataUnit::Bits), 3200);

        assert_eq!(data_info_2.incoming_data(DataUnit::Packets), 0);
        assert_eq!(data_info_2.incoming_data(DataUnit::Bytes), 0);
        assert_eq!(data_info_2.incoming_data(DataUnit::Bits), 0);

        assert_eq!(data_info_2.outgoing_data(DataUnit::Packets), 20);
        assert_eq!(data_info_2.outgoing_data(DataUnit::Bytes), 400);
        assert_eq!(data_info_2.outgoing_data(DataUnit::Bits), 3200);

        // compare data_info_1 and data_info_2

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataUnit::Packets),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataUnit::Packets),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataUnit::Packets),
            Ordering::Greater
        );

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataUnit::Bytes),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataUnit::Bytes),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataUnit::Bytes),
            Ordering::Greater
        );

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataUnit::Bits),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataUnit::Bits),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataUnit::Bits),
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
