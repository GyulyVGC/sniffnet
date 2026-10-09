use crate::Service;
use crate::networking::manage_packets::get_local_port;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::dropped_packets::DroppedPackets;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::program_lookup::ProgramLookup;
use crate::utils::types::timestamp::Timestamp;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// Struct containing overall traffic statistics and data.
#[derive(Debug, Default, Clone)]
pub struct InfoTraffic {
    /// Total amount of exchanged data
    pub tot_data_info: DataInfo,
    /// Number of dropped packets, if applicable
    pub dropped_packets: Option<DroppedPackets>,
    /// Dropped packets during the latest capture interval, if available.
    pub latest_dropped_packets: Option<DroppedPackets>,
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
    pub fn refresh(&mut self, msg: &mut Self, program_lookup_opt: &mut Option<ProgramLookup>) {
        let Self {
            tot_data_info,
            dropped_packets,
            latest_dropped_packets: _,
            last_packet_timestamp,
            map,
            services,
            hosts,
        } = msg;

        self.start_interval();
        self.tot_data_info.refresh(*tot_data_info);
        if let Some(program_lookup) = program_lookup_opt {
            program_lookup.start_interval();
        }

        self.latest_dropped_packets =
            dropped_packets.map(|current| current.since(self.dropped_packets.unwrap_or_default()));
        self.dropped_packets = *dropped_packets;

        // it can happen they're equal due to dis-alignments in the PCAP timestamp
        if self.last_packet_timestamp.secs() == last_packet_timestamp.secs() {
            last_packet_timestamp.add_secs(1);
        }
        self.last_packet_timestamp = *last_packet_timestamp;

        for (key, value) in &mut *map {
            let local_port = get_local_port(key, value.traffic_direction);
            let entry = self.map.entry(*key);
            match entry {
                Entry::Occupied(mut o) => {
                    if let Some(program_lookup) = program_lookup_opt
                        && let Some(local_port) = local_port
                    {
                        let program = program_lookup.lookup_and_add_data(
                            local_port,
                            false,
                            value.data_info(),
                        );
                        // set program in msg (used for favorite notifications)
                        value.program = program;
                    }

                    o.get_mut().refresh(value);
                }
                Entry::Vacant(v) => {
                    if let Some(program_lookup) = program_lookup_opt
                        && let Some(local_port) = local_port
                    {
                        let program =
                            program_lookup.lookup_and_add_data(local_port, true, value.data_info());
                        // set program in msg (used for favorite notifications)
                        value.program = program;
                    }

                    v.insert(value.clone());
                }
            }
        }

        for (key, value) in &*services {
            self.services
                .entry(*key)
                .and_modify(|x| x.refresh(*value))
                .or_insert(*value);
        }

        for (key, value) in &*hosts {
            self.hosts
                .entry(key.clone())
                .and_modify(|x| x.refresh(value))
                .or_insert(*value);
        }
    }

    /// Snapshot totals so inactive entries display zero in the next interval.
    pub fn start_interval(&mut self) {
        self.tot_data_info.start_interval();
        self.latest_dropped_packets = self.dropped_packets.map(|_| DroppedPackets::default());
        for value in self.map.values_mut() {
            value.start_interval();
        }
        for value in self.services.values_mut() {
            value.start_interval();
        }
        for value in self.hosts.values_mut() {
            value.data_info.start_interval();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Protocol;
    use crate::networking::types::data_representation::{DataRepr, DataUnit};
    use crate::networking::types::program::Program;
    use crate::networking::types::traffic_direction::TrafficDirection;
    use listeners::Process;
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::mpsc;

    fn key(port: u16) -> AddressPortPair {
        AddressPortPair::new(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            Some(port),
            IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
            Some(443),
            Protocol::Tcp,
        )
    }

    fn update(port: u16, packets: u128) -> InfoTraffic {
        let pair = InfoAddressPortPair {
            packets,
            bytes: packets * 100,
            traffic_direction: TrafficDirection::Outgoing,
            final_timestamp: Timestamp::new(i64::from(port), 0),
            ..Default::default()
        };
        let data_info = pair.data_info();
        InfoTraffic {
            tot_data_info: data_info,
            map: HashMap::from([(key(port), pair)]),
            services: HashMap::from([(Service::Name("https"), data_info)]),
            hosts: HashMap::from([(
                Host {
                    domain: port.to_string(),
                    ..Default::default()
                },
                DataInfoHost {
                    data_info,
                    ..Default::default()
                },
            )]),
            ..Default::default()
        }
    }

    fn program_lookup() -> ProgramLookup {
        let (port_tx, _) = mpsc::channel();
        let (_, program_rx) = mpsc::channel();
        let (icon_tx, _) = mpsc::channel();
        let (_, icon_rx) = mpsc::channel();
        ProgramLookup::new(port_tx, program_rx, icon_tx, icon_rx)
    }

    #[test]
    fn refresh_replaces_rates_preserves_totals_and_keeps_message_entries() {
        let mut traffic = InfoTraffic::default();
        let mut lookup = None;
        let mut first = update(1000, 10);
        traffic.refresh(&mut first, &mut lookup);
        assert_eq!(
            traffic.map[&key(1000)].transmitted_data(DataRepr::packets(true)),
            10
        );
        assert_eq!(
            traffic
                .hosts
                .values()
                .next()
                .unwrap()
                .data_info
                .tot_data(DataRepr::bytes(true)),
            1000
        );

        let mut next = update(1000, 3);
        let new = update(2000, 5);
        next.tot_data_info.refresh(new.tot_data_info);
        next.map.extend(new.map);
        next.hosts.extend(new.hosts);
        next.services
            .get_mut(&Service::Name("https"))
            .unwrap()
            .refresh(new.tot_data_info);
        traffic.refresh(&mut next, &mut lookup);
        assert_eq!(traffic.tot_data_info.tot_data(DataRepr::packets(false)), 18);
        assert_eq!(traffic.tot_data_info.tot_data(DataRepr::packets(true)), 8);
        assert_eq!(traffic.map[&key(1000)].packets, 13);
        assert_eq!(
            traffic.map[&key(1000)].transmitted_data(DataRepr::packets(true)),
            3
        );
        assert_eq!(
            traffic.map[&key(2000)].transmitted_data(DataRepr::packets(true)),
            5
        );
        let mut connections: Vec<_> = traffic.map.iter().collect();
        connections.sort_by(|(_, a), (_, b)| {
            a.compare(
                b,
                crate::report::types::sort_type::SortType::Descending,
                DataRepr::packets(true),
            )
        });
        assert_eq!(*connections[0].0, key(2000));
        assert_eq!(
            traffic.services[&Service::Name("https")].tot_data(DataRepr::packets(true)),
            8
        );
        // Backend messages remain usable by notifications and charts and contain zero baselines.
        assert_eq!(next.map.len(), 2);
        assert_eq!(next.map[&key(1000)].packets, 3);
        assert_eq!(next.map[&key(1000)].previous_packets, 0);

        traffic.refresh(&mut update(2000, 2), &mut lookup);
        assert_eq!(
            traffic.map[&key(1000)].transmitted_data(DataRepr::packets(true)),
            0
        );
        assert_eq!(traffic.map[&key(1000)].packets, 13);
        for sort in [
            crate::report::types::sort_type::SortType::Ascending,
            crate::report::types::sort_type::SortType::Descending,
            crate::report::types::sort_type::SortType::Neutral,
        ] {
            assert_eq!(
                traffic.map[&key(1000)].compare(
                    &traffic.map[&key(2000)],
                    sort,
                    DataRepr::packets(true)
                ),
                std::cmp::Ordering::Greater
            );
        }
        let old_host = Host {
            domain: "1000".into(),
            ..Default::default()
        };
        assert_eq!(
            traffic.hosts[&old_host]
                .data_info
                .tot_data(DataRepr::packets(true)),
            0
        );
        traffic.refresh(&mut InfoTraffic::default(), &mut lookup);
        assert_eq!(traffic.tot_data_info.tot_data(DataRepr::packets(false)), 20);
        assert_eq!(traffic.tot_data_info.tot_data(DataRepr::packets(true)), 0);
        assert_eq!(
            traffic.services[&Service::Name("https")].tot_data(DataRepr::packets(true)),
            0
        );
        assert!(
            traffic
                .map
                .values()
                .all(|p| p.transmitted_data(DataRepr::packets(true)) == 0)
        );
    }

    #[test]
    fn program_reassignment_moves_only_current_interval_rates() {
        let mut traffic = InfoTraffic::default();
        let mut lookup = Some(program_lookup());
        let mut first = update(1000, 10);
        let second = update(2000, 5);
        first.tot_data_info.refresh(second.tot_data_info);
        first.map.extend(second.map);
        traffic.refresh(&mut first, &mut lookup);
        let programs = lookup.as_ref().unwrap().programs();
        assert_eq!(
            programs[&Program::Unknown].tot_data(DataRepr::packets(true)),
            15
        );

        // Both connections share Unknown, but only one is active this interval.
        traffic.refresh(&mut update(1000, 3), &mut lookup);
        let proc = Process {
            pid: 1,
            name: "test".into(),
            path: "/test".into(),
        };
        let known = Program::from_proc(Some(&proc));
        lookup.as_mut().unwrap().update(
            (1000, listeners::Protocol::TCP, Some(proc)),
            &mut traffic.map,
        );
        let programs = lookup.as_ref().unwrap().programs();
        assert_eq!(programs[&known].tot_data(DataRepr::packets(false)), 13);
        assert_eq!(programs[&known].tot_data(DataRepr::packets(true)), 3);
        assert_eq!(
            programs[&Program::Unknown].tot_data(DataRepr::packets(false)),
            5
        );
        assert_eq!(
            programs[&Program::Unknown].tot_data(DataRepr::packets(true)),
            0
        );
        assert_eq!(traffic.map[&key(1000)].program, known);

        // An offline gap invalidates rates, retaining all cumulative counters.
        traffic.start_interval();
        lookup.as_mut().unwrap().start_interval();
        assert_eq!(traffic.tot_data_info.tot_data(DataRepr::packets(true)), 0);
        assert_eq!(traffic.tot_data_info.tot_data(DataRepr::packets(false)), 18);
        assert_eq!(
            lookup.as_ref().unwrap().programs()[&known].tot_data(DataRepr::packets(true)),
            0
        );
        assert_eq!(
            lookup.as_ref().unwrap().programs()[&known].tot_data(DataRepr::packets(false)),
            13
        );
    }

    #[test]
    fn dropped_rates_use_counter_deltas_and_reset_on_gaps() {
        let mut traffic = InfoTraffic::default();
        let mut msg = update(1000, 10);
        msg.dropped_packets = Some(DroppedPackets::from_pcap_stats(&pcap::Stat {
            received: 10,
            dropped: 3,
            if_dropped: 2,
        }));
        traffic.refresh(&mut msg, &mut None);
        msg.dropped_packets = Some(DroppedPackets::from_pcap_stats(&pcap::Stat {
            received: 20,
            dropped: 4,
            if_dropped: 5,
        }));
        traffic.refresh(&mut msg, &mut None);
        assert_eq!(
            traffic
                .latest_dropped_packets
                .unwrap()
                .total(DataUnit::Packets, traffic.tot_data_info),
            4
        );
        assert_eq!(
            traffic
                .dropped_packets
                .unwrap()
                .total(DataUnit::Packets, traffic.tot_data_info),
            9
        );
        traffic.start_interval();
        assert_eq!(
            traffic
                .latest_dropped_packets
                .unwrap()
                .total(DataUnit::Packets, traffic.tot_data_info),
            0
        );
        assert_eq!(
            traffic
                .dropped_packets
                .unwrap()
                .total(DataUnit::Packets, traffic.tot_data_info),
            9
        );
    }
}
