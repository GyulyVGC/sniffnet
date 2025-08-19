use crate::Service;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::utils::types::timestamp::Timestamp;
use std::collections::HashMap;

/// Struct containing overall traffic statistics and data.
#[derive(Debug, Default, Clone)]
pub struct InfoTraffic {
    /// Total amount of exchanged data
    pub tot_data_info: DataInfo,
    /// Number of dropped packets
    pub dropped_packets: u32,
    /// Timestamp of the latest parsed packet
    pub last_packet_timestamp: Timestamp,
    /// Map of the traffic
    pub map: HashMap<AddressPortPair, InfoAddressPortPair>,
    /// Map of the upper layer services with their data info
    pub services: HashMap<Service, DataInfo>,
    /// Map of the hosts with their data info
    pub hosts: HashMap<Host, DataInfoHost>,
}

impl InfoTraffic {
    pub fn refresh(&mut self, msg: &mut InfoTraffic) {
        self.tot_data_info.refresh(msg.tot_data_info);

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

    pub fn get_thumbnail_data(&self, data_repr: DataRepr) -> (u128, u128, u128) {
        let incoming = self.tot_data_info.incoming_data(data_repr);
        let outgoing = self.tot_data_info.outgoing_data(data_repr);
        let all = incoming + outgoing;
        let all_packets = self.tot_data_info.tot_data(DataRepr::Packets);
        let dropped = match data_repr {
            DataRepr::Packets => u128::from(self.dropped_packets),
            DataRepr::Bytes | DataRepr::Bits => {
                // assume that the dropped packets have the same size as the average packet
                u128::from(self.dropped_packets) * all / all_packets
            }
        };

        (incoming, outgoing, dropped)
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
