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
            u128::from(self.by_adapter + self.by_sniffnet),
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
