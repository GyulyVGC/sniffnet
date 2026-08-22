use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::protocol::Protocol;
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
    pub(super) fn get_key(&self) -> Option<AddressPortPair> {
        let source = self.src_ip?;
        let dest = self.dst_ip?;
        let protocol = self.protocol?;

        if matches!(protocol, Protocol::TCP | Protocol::UDP)
            && (self.src_port.is_none() || self.dst_port.is_none())
        {
            return None;
        }

        if matches!(protocol, Protocol::ICMP | Protocol::ARP)
            && (self.src_port.is_some() || self.dst_port.is_some())
        {
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
