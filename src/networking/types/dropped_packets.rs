use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_representation::DataRepr;

/// Represents the number of dropped packets
#[derive(Debug, Default, Clone, Copy)]
pub struct DroppedPackets {
    /// Number of packets dropped by the network interface or its driver
    by_adapter: u32,
    /// Packets dropped because they weren't being read fast enough (PCAP buffer was full)
    by_sniffnet: u32,
}

impl DroppedPackets {
    /// Creates a `DroppedPackets` instance from a `pcap::Stat` struct
    pub fn from_pcap_stats(stats: &pcap::Stat) -> Self {
        Self {
            by_adapter: stats.if_dropped,
            by_sniffnet: stats.dropped,
        }
    }

    /// Returns the total number of dropped data,
    /// assuming that dropped packets have the same size as the average packet
    pub fn total(self, data_repr: DataRepr, tot_data_info: DataInfo) -> u128 {
        averaged_data(
            u128::from(self.by_adapter) + u128::from(self.by_sniffnet),
            data_repr,
            tot_data_info,
        )
    }

    /// Returns the number of data dropped by the network adapter,
    /// assuming that dropped packets have the same size as the average packet
    pub fn by_adapter(self, data_repr: DataRepr, tot_data_info: DataInfo) -> u128 {
        averaged_data(u128::from(self.by_adapter), data_repr, tot_data_info)
    }

    /// Returns the number of data dropped by the Sniffnet,
    /// assuming that dropped packets have the same size as the average packet
    pub fn by_sniffnet(self, data_repr: DataRepr, tot_data_info: DataInfo) -> u128 {
        averaged_data(u128::from(self.by_sniffnet), data_repr, tot_data_info)
    }
}

fn averaged_data(dropped_packets: u128, data_repr: DataRepr, tot_data_info: DataInfo) -> u128 {
    if data_repr == DataRepr::Packets {
        return dropped_packets;
    }

    let all = tot_data_info.tot_data(data_repr);
    let all_packets = tot_data_info.tot_data(DataRepr::Packets);
    dropped_packets
        .saturating_mul(all)
        .checked_div(all_packets)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_averaged_data() {
        // 6 packets carrying 100 bytes in total
        let data_info = DataInfo::new_for_tests(2, 4, 30, 70);

        assert_eq!(averaged_data(3, DataRepr::Packets, data_info), 3);
        // multiply before dividing to avoid truncating the average packet size.
        assert_eq!(averaged_data(3, DataRepr::Bytes, data_info), 50);
        assert_eq!(averaged_data(3, DataRepr::Bits, data_info), 400);

        // fractional results are truncated only after scaling by dropped packets.
        assert_eq!(averaged_data(1, DataRepr::Bytes, data_info), 16);
        assert_eq!(averaged_data(1, DataRepr::Bits, data_info), 133);
        assert_eq!(averaged_data(9, DataRepr::Bytes, data_info), 150);

        assert_eq!(averaged_data(0, DataRepr::Packets, data_info), 0);
        assert_eq!(averaged_data(0, DataRepr::Bytes, data_info), 0);
        assert_eq!(averaged_data(0, DataRepr::Bits, data_info), 0);
    }

    #[test]
    fn test_averaged_data_no_observed_packets() {
        let data_info = DataInfo::default();

        assert_eq!(averaged_data(3, DataRepr::Packets, data_info), 3);
        assert_eq!(averaged_data(3, DataRepr::Bytes, data_info), 0);
        assert_eq!(averaged_data(3, DataRepr::Bits, data_info), 0);
    }

    #[test]
    fn test_dropped_packets_total_overflow() {
        let dropped = DroppedPackets {
            by_adapter: u32::MAX,
            by_sniffnet: u32::MAX,
        };
        let data_info = DataInfo::new_for_tests(1, 1, 100, 100);

        assert_eq!(dropped.total(DataRepr::Packets, data_info), 8_589_934_590);
        assert_eq!(dropped.total(DataRepr::Bytes, data_info), 858_993_459_000);
        assert_eq!(dropped.total(DataRepr::Bits, data_info), 6_871_947_672_000);
    }
}
