//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use crate::Protocol;
use crate::networking::types::ipfix_exporter::IpfixExporter;
use sniffnet_packet_parser::ParsedPacket;
use std::net::IpAddr;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AddressPortPair {
    /// Network layer IPv4 or IPv6 source address.
    pub source: IpAddr,
    /// Transport layer source port number (in the range 0..=65535).
    pub sport: Option<u16>,
    /// Network layer IPv4 or IPv6 destination address.
    pub dest: IpAddr,
    /// Transport layer destination port number (in the range 0..=65535).
    pub dport: Option<u16>,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub protocol: Protocol,
    /// Exporter the flow was reported by; `None` non-IPFIX captures.
    pub exporter: Option<IpfixExporter>,
}

impl AddressPortPair {
    pub fn from_parsed_packet(parsed: &ParsedPacket) -> Self {
        let source = parsed.net_info.src_ip;
        let sport = parsed.transport_info.src_port;
        let dest = parsed.net_info.dst_ip;
        let dport = parsed.transport_info.dst_port;
        let protocol = match parsed.transport_info.protocol {
            sniffnet_packet_parser::Protocol::Tcp => Protocol::TCP,
            sniffnet_packet_parser::Protocol::Udp => Protocol::UDP,
            sniffnet_packet_parser::Protocol::Icmpv4
            | sniffnet_packet_parser::Protocol::Icmpv6
            // TODO IGMP!
            | sniffnet_packet_parser::Protocol::Igmp => Protocol::ICMP,
            sniffnet_packet_parser::Protocol::Arp => Protocol::ARP,
        };

        Self {
            source,
            sport,
            dest,
            dport,
            protocol,
            exporter: None,
        }
    }
}

#[cfg(test)]
impl AddressPortPair {
    pub fn new(
        source: IpAddr,
        sport: Option<u16>,
        dest: IpAddr,
        dport: Option<u16>,
        protocol: Protocol,
    ) -> Self {
        AddressPortPair {
            source,
            sport,
            dest,
            dport,
            protocol,
            exporter: None,
        }
    }
}
