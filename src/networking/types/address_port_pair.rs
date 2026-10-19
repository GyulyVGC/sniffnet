//! Module defining the `AddressPortPair` struct, which represents a network address:port pair.

use crate::Protocol;
use crate::networking::types::ipfix_exporter::IpfixExporter;
use sniffnet_packet_parser::ParsedPacket;
use std::net::IpAddr;

/// Struct representing a network address:port pair.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AddressPortPair {
    /// Network layer IPv4 or IPv6 source address.
    pub src_ip: IpAddr,
    /// Transport layer source port number (in the range 0..=65535).
    pub src_port: Option<u16>,
    /// Network layer IPv4 or IPv6 destination address.
    pub dst_ip: IpAddr,
    /// Transport layer destination port number (in the range 0..=65535).
    pub dst_port: Option<u16>,
    ///  Transport layer protocol carried through the associate address:port pair (TCP or UPD).
    pub protocol: Protocol,
    /// Exporter the flow was reported by; `None` non-IPFIX captures.
    pub exporter: Option<IpfixExporter>,
}

impl AddressPortPair {
    pub fn from_parsed_packet(parsed: &ParsedPacket) -> Self {
        let src_ip = parsed.net_info.src_ip;
        let src_port = parsed.transport_info.src_port;
        let dst_ip = parsed.net_info.dst_ip;
        let dst_port = parsed.transport_info.dst_port;
        let protocol = parsed.transport_info.protocol;

        Self {
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            protocol,
            exporter: None,
        }
    }
}

#[cfg(test)]
impl AddressPortPair {
    pub fn new(
        src_ip: IpAddr,
        src_port: Option<u16>,
        dst_ip: IpAddr,
        dst_port: Option<u16>,
        protocol: Protocol,
    ) -> Self {
        AddressPortPair {
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            protocol,
            exporter: None,
        }
    }
}
