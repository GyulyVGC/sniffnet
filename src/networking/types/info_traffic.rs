use crate::Service;
use crate::chart::types::chart_type::ChartType;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::utils::types::timestamp::Timestamp;
use std::collections::HashMap;

/// Struct containing overall traffic statistics and data.
#[derive(Debug, Default, Clone)]
pub struct InfoTraffic {
    /// Total amount of exchanged data
    pub tot_data_info: DataInfo,
    /// Total packets including those not filtered
    pub all_packets: u128,
    /// Total bytes including those not filtered
    pub all_bytes: u128,
    /// Number of dropped packets
    pub dropped_packets: u32,
    /// Timestamp of the latest parsed packet
    pub last_packet_timestamp: Timestamp,
    /// Map of the filtered traffic
    pub map: HashMap<AddressPortPair, InfoAddressPortPair>,
    /// Map of the upper layer services with their data info
    pub services: HashMap<Service, DataInfo>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, DataInfoHost>,
}

impl InfoTraffic {
    pub fn refresh(&mut self, msg: &mut InfoTraffic) {
        self.tot_data_info.refresh(msg.tot_data_info);

        self.all_packets += msg.all_packets;
        self.all_bytes += msg.all_bytes;
        self.dropped_packets = msg.dropped_packets;

        // it can happen they're equal due to dis-alignments in the PCAP timestamp
        if self.last_packet_timestamp.secs() == msg.last_packet_timestamp.secs() {
            msg.last_packet_timestamp.add_secs(1);
        }
        self.last_packet_timestamp = msg.last_packet_timestamp;

        for (key, value) in &msg.map {
            self.map
                .entry(*key)
                .and_modify(|x| x.refresh(value))
                .or_insert_with(|| value.clone());
        }

        for (key, value) in &msg.services {
            self.services
                .entry(*key)
                .and_modify(|x| x.refresh(*value))
                .or_insert(*value);
        }

        for (key, value) in &msg.hosts {
            self.hosts
                .entry(key.clone())
                .and_modify(|x| x.refresh(value))
                .or_insert(*value);
        }
    }

    pub fn get_thumbnail_data(&self, chart_type: ChartType) -> (u128, u128, u128, u128) {
        if chart_type.eq(&ChartType::Bytes) {
            (
                self.tot_data_info.incoming_bytes(),
                self.tot_data_info.outgoing_bytes(),
                self.all_bytes
                    - self.tot_data_info.outgoing_bytes()
                    - self.tot_data_info.incoming_bytes(),
                // assume that the dropped packets have the same size as the average packet
                u128::from(self.dropped_packets) * self.all_bytes / self.all_packets,
            )
        } else {
            (
                self.tot_data_info.incoming_packets(),
                self.tot_data_info.outgoing_packets(),
                self.all_packets
                    - self.tot_data_info.outgoing_packets()
                    - self.tot_data_info.incoming_packets(),
                u128::from(self.dropped_packets),
            )
        }
    }

    pub fn take_but_leave_something(&mut self) -> Self {
        let info_traffic = Self {
            last_packet_timestamp: self.last_packet_timestamp,
            dropped_packets: self.dropped_packets,
            ..Self::default()
        };
        std::mem::replace(self, info_traffic)
    }
}
