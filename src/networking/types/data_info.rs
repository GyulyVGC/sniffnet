//! Module defining the `DataInfo` struct, which represents incoming and outgoing packets and bytes.

use crate::networking::types::data_representation::{DataRepr, DataUnit};
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::types::sort_type::SortType;
use std::cmp::Ordering;
use std::time::Instant;

/// Amount of exchanged data (packets and bytes) incoming and outgoing, with the timestamp of the latest occurrence
// data fields are private to make them only editable via the provided methods: needed to correctly refresh timestamps
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct DataInfo {
    /// Incoming packets
    in_packets: u128,
    /// Outgoing packets
    out_packets: u128,
    /// Incoming bytes
    in_bytes: u128,
    /// Outgoing bytes
    out_bytes: u128,
    /// Previous interval's incoming packets
    previous_in_packets: u128,
    /// Previous interval's outgoing packets
    previous_out_packets: u128,
    /// Previous interval's incoming bytes
    previous_in_bytes: u128,
    /// Previous interval's outgoing bytes
    previous_out_bytes: u128,
    /// Latest instant of occurrence. Initialized to None by Default.
    final_instant: Option<Instant>,
}

impl DataInfo {
    pub fn incoming_data(&self, data_repr: DataRepr) -> u128 {
        let (packets, bytes) = if data_repr.per_second {
            (
                self.in_packets.saturating_sub(self.previous_in_packets),
                self.in_bytes.saturating_sub(self.previous_in_bytes),
            )
        } else {
            (self.in_packets, self.in_bytes)
        };
        match data_repr.data_unit {
            DataUnit::Packets => packets,
            DataUnit::Bytes => bytes,
            DataUnit::Bits => bytes * 8,
        }
    }

    pub fn outgoing_data(&self, data_repr: DataRepr) -> u128 {
        let (packets, bytes) = if data_repr.per_second {
            (
                self.out_packets.saturating_sub(self.previous_out_packets),
                self.out_bytes.saturating_sub(self.previous_out_bytes),
            )
        } else {
            (self.out_packets, self.out_bytes)
        };
        match data_repr.data_unit {
            DataUnit::Packets => packets,
            DataUnit::Bytes => bytes,
            DataUnit::Bits => bytes * 8,
        }
    }

    pub fn tot_data(&self, data_repr: DataRepr) -> u128 {
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
            self.out_packets += packets;
            self.out_bytes += bytes;
        } else {
            self.in_packets += packets;
            self.in_bytes += bytes;
        }
        self.final_instant = Some(final_instant);
    }

    /// Merge totals and their baselines together, preserving rates during reassignment.
    /// Backend interval updates have zero baselines.
    pub fn refresh(&mut self, rhs: Self) {
        let Self {
            in_packets,
            out_packets,
            in_bytes,
            out_bytes,
            final_instant,
            previous_in_packets,
            previous_out_packets,
            previous_in_bytes,
            previous_out_bytes,
        } = rhs;

        self.previous_in_packets += previous_in_packets;
        self.previous_out_packets += previous_out_packets;
        self.previous_in_bytes += previous_in_bytes;
        self.previous_out_bytes += previous_out_bytes;
        self.in_packets += in_packets;
        self.out_packets += out_packets;
        self.in_bytes += in_bytes;
        self.out_bytes += out_bytes;
        // None < Some(_) by Option's Ord, so a Some will always win over a default None
        if final_instant > self.final_instant {
            self.final_instant = final_instant;
        }
    }

    /// Assign current totals to previous values, to be used as baselines for the next interval
    pub fn start_interval(&mut self) {
        self.previous_in_packets = self.in_packets;
        self.previous_out_packets = self.out_packets;
        self.previous_in_bytes = self.in_bytes;
        self.previous_out_bytes = self.out_bytes;
    }

    pub fn compare(&self, other: &Self, sort_type: SortType, data_repr: DataRepr) -> Ordering {
        let self_data = self.tot_data(data_repr);
        let other_data = other.tot_data(data_repr);

        // handle cases where one or both have no data
        if self_data == 0 && other_data == 0 {
            return other.final_instant.cmp(&self.final_instant);
        } else if self_data == 0 && other_data > 0 {
            return Ordering::Greater;
        } else if self_data > 0 && other_data == 0 {
            return Ordering::Less;
        }

        match sort_type {
            SortType::Ascending => self_data.cmp(&other_data),
            SortType::Descending => other_data.cmp(&self_data),
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
        in_packets: u128,
        out_packets: u128,
        in_bytes: u128,
        out_bytes: u128,
    ) -> Self {
        Self {
            in_packets,
            out_packets,
            in_bytes,
            out_bytes,
            final_instant: Some(Instant::now()),
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::types::traffic_direction::TrafficDirection;

    #[test]
    fn interval_values_and_sorting_use_selected_counters() {
        let mut busy = DataInfo::new_for_tests(100, 100, 10_000, 10_000);
        let mut active = DataInfo::default();
        let interval = DataInfo::new_for_tests(2, 3, 200, 600);
        busy.start_interval();
        busy.refresh(interval);
        active.refresh(DataInfo::new_for_tests(4, 4, 400, 400));
        assert_eq!(busy.incoming_data(DataRepr::packets(true)), 2);
        assert_eq!(busy.outgoing_data(DataRepr::bytes(true)), 600);
        assert_eq!(busy.tot_data(DataRepr::bits(true)), 6400);
        assert_eq!(busy.tot_data(DataRepr::bytes(false)), 20_800);
        assert_eq!(
            busy.compare(&active, SortType::Descending, DataRepr::packets(true)),
            Ordering::Greater
        );
        assert_eq!(
            busy.compare(&active, SortType::Descending, DataRepr::packets(false)),
            Ordering::Less
        );
        busy.start_interval();
        for sort in [SortType::Ascending, SortType::Descending, SortType::Neutral] {
            assert_eq!(
                busy.compare(&active, sort, DataRepr::packets(true)),
                Ordering::Greater
            );
            assert_eq!(
                active.compare(&busy, sort, DataRepr::packets(true)),
                Ordering::Less
            );
        }
        assert_eq!(busy.tot_data(DataRepr::packets(false)), 205);
    }

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

        assert_eq!(data_info_1.in_packets, 7);
        assert_eq!(data_info_1.out_packets, 12);
        assert_eq!(data_info_1.in_bytes, 723);
        assert_eq!(data_info_1.out_bytes, 1400);

        assert_eq!(data_info_1.tot_data(DataRepr::packets(false)), 19);
        assert_eq!(data_info_1.tot_data(DataRepr::bytes(false)), 2123);
        assert_eq!(data_info_1.tot_data(DataRepr::bits(false)), 16984);

        assert_eq!(data_info_1.incoming_data(DataRepr::packets(false)), 7);
        assert_eq!(data_info_1.incoming_data(DataRepr::bytes(false)), 723);
        assert_eq!(data_info_1.incoming_data(DataRepr::bits(false)), 5784);

        assert_eq!(data_info_1.outgoing_data(DataRepr::packets(false)), 12);
        assert_eq!(data_info_1.outgoing_data(DataRepr::bytes(false)), 1400);
        assert_eq!(data_info_1.outgoing_data(DataRepr::bits(false)), 11200);

        // sleep a little to have a different final_instant
        std::thread::sleep(std::time::Duration::from_millis(10));
        let mut data_info_2 = DataInfo::default();
        data_info_2.add_packets(1, 100, TrafficDirection::Outgoing, Instant::now());
        // 0, 1, 0, 100
        data_info_2.add_packets(19, 300, TrafficDirection::Outgoing, Instant::now());
        // 0, 20, 0, 400

        assert_eq!(data_info_2.in_packets, 0);
        assert_eq!(data_info_2.out_packets, 20);
        assert_eq!(data_info_2.in_bytes, 0);
        assert_eq!(data_info_2.out_bytes, 400);

        assert_eq!(data_info_2.tot_data(DataRepr::packets(false)), 20);
        assert_eq!(data_info_2.tot_data(DataRepr::bytes(false)), 400);
        assert_eq!(data_info_2.tot_data(DataRepr::bits(false)), 3200);

        assert_eq!(data_info_2.incoming_data(DataRepr::packets(false)), 0);
        assert_eq!(data_info_2.incoming_data(DataRepr::bytes(false)), 0);
        assert_eq!(data_info_2.incoming_data(DataRepr::bits(false)), 0);

        assert_eq!(data_info_2.outgoing_data(DataRepr::packets(false)), 20);
        assert_eq!(data_info_2.outgoing_data(DataRepr::bytes(false)), 400);
        assert_eq!(data_info_2.outgoing_data(DataRepr::bits(false)), 3200);

        // compare data_info_1 and data_info_2

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataRepr::packets(false)),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataRepr::packets(false)),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataRepr::packets(false)),
            Ordering::Greater
        );

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataRepr::bytes(false)),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataRepr::bytes(false)),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataRepr::bytes(false)),
            Ordering::Greater
        );

        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Ascending, DataRepr::bits(false)),
            Ordering::Greater
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Descending, DataRepr::bits(false)),
            Ordering::Less
        );
        assert_eq!(
            data_info_1.compare(&data_info_2, SortType::Neutral, DataRepr::bits(false)),
            Ordering::Greater
        );

        // refresh data_info_1 with data_info_2
        assert!(data_info_1.final_instant < data_info_2.final_instant);
        data_info_1.refresh(data_info_2);

        // data_info_1 should now contain the sum of both data_info_1 and data_info_2
        assert_eq!(data_info_1.in_packets, 7);
        assert_eq!(data_info_1.out_packets, 32);
        assert_eq!(data_info_1.in_bytes, 723);
        assert_eq!(data_info_1.out_bytes, 1800);
        assert_eq!(data_info_1.final_instant, data_info_2.final_instant);
    }
}
