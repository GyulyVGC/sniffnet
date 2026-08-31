use crate::Protocol;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::ipfix_exporter::IpfixExporter;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::utils::types::timestamp::Timestamp;
use std::net::IpAddr;

/// Decoded fields from a single data record
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct FlowRecord {
    pub(super) src_ip: Option<IpAddr>,
    pub(super) dst_ip: Option<IpAddr>,
    pub(super) src_port: Option<u16>,
    pub(super) dst_port: Option<u16>,
    pub(super) protocol: Option<Protocol>,
    pub(super) bytes_delta: Option<u128>,
    pub(super) packets_delta: Option<u128>,
    pub(super) bytes_total: Option<u128>,
    pub(super) packets_total: Option<u128>,
    pub(super) src_mac: Option<[u8; 6]>,
    pub(super) dst_mac: Option<[u8; 6]>,
    pub(super) direction: Option<TrafficDirection>,
    pub(super) flow_start: Option<Timestamp>,
    pub(super) flow_end: Option<Timestamp>,
    /// Set only when the exporter sends a biflow (same flow in the opposite direction)
    pub(super) reverse: Option<ReverseCounters>,
}

impl FlowRecord {
    /// Return the key for this record, if the record is valid
    pub(super) fn get_key(&self, exporter: IpfixExporter) -> Option<AddressPortPair> {
        let source = self.src_ip?;
        let dest = self.dst_ip?;
        // TODO: ARP not supported yet
        let protocol = self.protocol?;

        if !protocol.is_portless() && (self.src_port.is_none() || self.dst_port.is_none()) {
            return None;
        }

        if protocol.is_portless() && (self.src_port.is_some() || self.dst_port.is_some()) {
            return None;
        }

        let sport = self.src_port;
        let dport = self.dst_port;

        Some(AddressPortPair {
            source,
            sport,
            dest,
            dport,
            protocol,
            exporter: Some(exporter),
        })
    }

    /// The other half of an RFC 5103 biflow, as a full-fledged record
    pub(super) fn get_reverse_record(&self) -> Option<FlowRecord> {
        let reverse = self.reverse?;
        Some(FlowRecord {
            src_ip: self.dst_ip,
            dst_ip: self.src_ip,
            src_port: self.dst_port,
            dst_port: self.src_port,
            protocol: self.protocol,
            bytes_delta: reverse.bytes_delta,
            packets_delta: reverse.packets_delta,
            bytes_total: reverse.bytes_total,
            packets_total: reverse.packets_total,
            src_mac: self.dst_mac,
            dst_mac: self.src_mac,
            direction: self.direction.map(TrafficDirection::opposite),
            flow_start: self.flow_start,
            flow_end: self.flow_end,
            reverse: None,
        })
    }
}

/// The counters of an RFC 5103 biflow's reverse direction
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ReverseCounters {
    pub(super) bytes_delta: Option<u128>,
    pub(super) packets_delta: Option<u128>,
    pub(super) bytes_total: Option<u128>,
    pub(super) packets_total: Option<u128>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn exporter() -> IpfixExporter {
        IpfixExporter {
            addr: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 9)),
            observation_domain_id: 0,
        }
    }

    #[test]
    fn test_get_key() {
        let record = FlowRecord {
            src_ip: Some("1.1.1.1".parse().unwrap()),
            dst_ip: Some("2.2.2.2".parse().unwrap()),
            src_port: Some(1234),
            dst_port: Some(5678),
            protocol: Some(Protocol::Tcp),
            ..Default::default()
        };

        let key = record.get_key(exporter());
        assert_eq!(
            key,
            Some(AddressPortPair {
                source: "1.1.1.1".parse().unwrap(),
                sport: Some(1234),
                dest: "2.2.2.2".parse().unwrap(),
                dport: Some(5678),
                protocol: Protocol::Tcp,
                exporter: Some(exporter()),
            })
        );

        let record_2 = FlowRecord {
            protocol: Some(Protocol::Icmpv4),
            ..record
        };
        let key = record_2.get_key(exporter());
        assert!(key.is_none());

        let record_3 = FlowRecord {
            protocol: Some(Protocol::Arp),
            ..record
        };
        let key = record_3.get_key(exporter());
        assert!(key.is_none());

        let record_4 = FlowRecord {
            src_port: None,
            ..record
        };
        let key = record_4.get_key(exporter());
        assert!(key.is_none());

        let record_5 = FlowRecord {
            protocol: Some(Protocol::Udp),
            ..record
        };
        let key = record_5.get_key(exporter());
        assert!(key.is_some());

        let record_6 = FlowRecord {
            protocol: None,
            ..record
        };
        let key = record_6.get_key(exporter());
        assert!(key.is_none());

        let record_7 = FlowRecord {
            dst_ip: None,
            ..record
        };
        let key = record_7.get_key(exporter());
        assert!(key.is_none());

        let record_8 = FlowRecord {
            protocol: Some(Protocol::Icmpv6),
            src_port: None,
            dst_port: None,
            ..record
        };
        let key = record_8.get_key(exporter());
        assert_eq!(
            key,
            Some(AddressPortPair {
                source: "1.1.1.1".parse().unwrap(),
                sport: None,
                dest: "2.2.2.2".parse().unwrap(),
                dport: None,
                protocol: Protocol::Icmpv6,
                exporter: Some(exporter()),
            })
        );
    }

    #[test]
    fn test_get_reverse_record() {
        let record = FlowRecord {
            src_ip: Some("1.1.1.1".parse().unwrap()),
            dst_ip: Some("2.2.2.2".parse().unwrap()),
            src_port: Some(1234),
            dst_port: Some(5678),
            protocol: Some(Protocol::Tcp),
            bytes_delta: Some(100),
            packets_delta: Some(10),
            bytes_total: Some(1000),
            packets_total: Some(100),
            src_mac: Some([0, 1, 2, 3, 4, 5]),
            dst_mac: Some([5, 4, 3, 2, 1, 0]),
            direction: Some(TrafficDirection::Incoming),
            flow_start: Some(Timestamp::new(10, 0)),
            flow_end: Some(Timestamp::new(20, 0)),
            reverse: Some(ReverseCounters {
                bytes_delta: Some(200),
                packets_delta: Some(20),
                bytes_total: Some(2000),
                packets_total: Some(200),
            }),
        };

        let reverse_record = record.get_reverse_record().unwrap();
        assert_eq!(
            reverse_record,
            FlowRecord {
                src_ip: Some("2.2.2.2".parse().unwrap()),
                dst_ip: Some("1.1.1.1".parse().unwrap()),
                src_port: Some(5678),
                dst_port: Some(1234),
                protocol: Some(Protocol::Tcp),
                bytes_delta: Some(200),
                packets_delta: Some(20),
                bytes_total: Some(2000),
                packets_total: Some(200),
                src_mac: Some([5, 4, 3, 2, 1, 0]),
                dst_mac: Some([0, 1, 2, 3, 4, 5]),
                direction: Some(TrafficDirection::Outgoing),
                flow_start: Some(Timestamp::new(10, 0)),
                flow_end: Some(Timestamp::new(20, 0)),
                reverse: None,
            }
        );

        let record_no_reverse = FlowRecord {
            reverse: None,
            ..record
        };
        let reverse_record = record_no_reverse.get_reverse_record();
        assert!(reverse_record.is_none());
    }
}
