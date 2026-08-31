use std::net::IpAddr;

use etherparse::{ArpHardwareId, EtherType, LinkHeader, NetHeaders, TransportHeader};

use crate::arp_type::ArpType;
use crate::headers::{LinkInfo, NetInfo, TransportInfo, get_sniffable_headers};
use crate::icmp_type::{IcmpTypeV4, IcmpTypeV6};
use crate::link_type::LinkType;
use crate::protocol::Protocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Parsed network packet with info extracted from its headers.
pub struct ParsedPacket {
    /// Info extracted from the data link layer header.
    pub link_info: LinkInfo,
    /// Info extracted from the network layer header.
    pub net_info: NetInfo,
    /// Info extracted from the transport layer header.
    pub transport_info: TransportInfo,
}

impl ParsedPacket {
    #[must_use]
    /// Parse one raw network packet from its link type and wire bytes.
    pub fn from_bytes(bytes: &[u8], link_type: LinkType) -> Option<ParsedPacket> {
        let headers = get_sniffable_headers(bytes, link_type)?;

        let link_info = analyze_link_header(headers.link);

        let is_arp = matches!(&headers.net, Some(NetHeaders::Arp(_)));

        let net_info = analyze_net_header(headers.net)?;

        let transport_info = if is_arp {
            Some(TransportInfo {
                src_port: None,
                dst_port: None,
                protocol: Protocol::Arp,
                icmp_type: None,
            })
        } else {
            analyze_transport_header(headers.transport)
        }?;

        Some(ParsedPacket {
            link_info,
            net_info,
            transport_info,
        })
    }

    #[must_use]
    /// Returns the total number of bytes in the packet.
    pub fn bytes_count(&self) -> usize {
        self.link_info.bytes.saturating_add(self.net_info.bytes)
    }
}

/// This function extracts info from the data link layer header passed as parameter.
fn analyze_link_header(link_header: Option<LinkHeader>) -> LinkInfo {
    match link_header {
        Some(LinkHeader::Ethernet2(header)) => {
            let src_mac = Some(header.source);
            let dst_mac = Some(header.destination);
            let bytes = 14;
            LinkInfo {
                src_mac,
                dst_mac,
                bytes,
            }
        }
        Some(LinkHeader::LinuxSll(header)) => {
            let src_mac: Option<[u8; 6]> = if header.sender_address_valid_length == 6
                && header.arp_hrd_type == ArpHardwareId::ETHERNET
                && let Ok(sender) = header.sender_address[0..6].try_into()
            {
                Some(sender)
            } else {
                None
            };
            let dst_mac = None;
            let bytes = 16;
            LinkInfo {
                src_mac,
                dst_mac,
                bytes,
            }
        }
        None => LinkInfo {
            src_mac: None,
            dst_mac: None,
            bytes: 0,
        },
    }
}

/// This function extracts info from the network layer header passed as parameter.
/// Returns `None` if packet has to be skipped.
fn analyze_net_header(network_header: Option<NetHeaders>) -> Option<NetInfo> {
    match network_header {
        Some(NetHeaders::Ipv4(ipv4header, _)) => {
            let src_ip = IpAddr::from(ipv4header.source);
            let dst_ip = IpAddr::from(ipv4header.destination);
            let bytes = usize::from(ipv4header.total_len);
            Some(NetInfo {
                src_ip,
                dst_ip,
                arp_type: None,
                bytes,
            })
        }
        Some(NetHeaders::Ipv6(ipv6header, _)) => {
            let src_ip = IpAddr::from(ipv6header.source);
            let dst_ip = IpAddr::from(ipv6header.destination);
            let bytes = usize::from(ipv6header.payload_length.saturating_add(40));
            Some(NetInfo {
                src_ip,
                dst_ip,
                arp_type: None,
                bytes,
            })
        }
        Some(NetHeaders::Arp(arp_packet)) => {
            let (src_ip, dst_ip) = match arp_packet.proto_addr_type {
                EtherType::IPV4 => {
                    let src_ip =
                        match TryInto::<[u8; 4]>::try_into(arp_packet.sender_protocol_addr()) {
                            Ok(source) => IpAddr::from(source),
                            Err(_) => return None,
                        };
                    let dst_ip =
                        match TryInto::<[u8; 4]>::try_into(arp_packet.target_protocol_addr()) {
                            Ok(destination) => IpAddr::from(destination),
                            Err(_) => return None,
                        };
                    (src_ip, dst_ip)
                }
                EtherType::IPV6 => {
                    let src_ip =
                        match TryInto::<[u8; 16]>::try_into(arp_packet.sender_protocol_addr()) {
                            Ok(source) => IpAddr::from(source),
                            Err(_) => return None,
                        };
                    let dst_ip =
                        match TryInto::<[u8; 16]>::try_into(arp_packet.target_protocol_addr()) {
                            Ok(destination) => IpAddr::from(destination),
                            Err(_) => return None,
                        };
                    (src_ip, dst_ip)
                }
                _ => return None,
            };
            let bytes = arp_packet.packet_len();
            let arp_type = ArpType::from_etherparse(arp_packet.operation);
            Some(NetInfo {
                src_ip,
                dst_ip,
                arp_type: Some(arp_type),
                bytes,
            })
        }
        None => None,
    }
}

/// This function extracts info from the transport layer header passed as parameter.
/// Returns `None` if packet has to be skipped.
fn analyze_transport_header(transport_header: Option<TransportHeader>) -> Option<TransportInfo> {
    match transport_header {
        Some(TransportHeader::Udp(udp_header)) => {
            let src_port = Some(udp_header.source_port);
            let dst_port = Some(udp_header.destination_port);
            let protocol = Protocol::Udp;
            Some(TransportInfo {
                src_port,
                dst_port,
                protocol,
                icmp_type: None,
            })
        }
        Some(TransportHeader::Tcp(tcp_header)) => {
            let src_port = Some(tcp_header.source_port);
            let dst_port = Some(tcp_header.destination_port);
            let protocol = Protocol::Tcp;
            Some(TransportInfo {
                src_port,
                dst_port,
                protocol,
                icmp_type: None,
            })
        }
        Some(TransportHeader::Icmpv4(icmpv4_header)) => {
            let src_port = None;
            let dst_port = None;
            let protocol = Protocol::Icmpv4;
            let icmp_type = IcmpTypeV4::from_etherparse(&icmpv4_header.icmp_type);
            Some(TransportInfo {
                src_port,
                dst_port,
                protocol,
                icmp_type: Some(icmp_type),
            })
        }
        Some(TransportHeader::Icmpv6(icmpv6_header)) => {
            let src_port = None;
            let dst_port = None;
            let protocol = Protocol::Icmpv6;
            let icmp_type = IcmpTypeV6::from_etherparse(&icmpv6_header.icmp_type);
            Some(TransportInfo {
                src_port,
                dst_port,
                protocol,
                icmp_type: Some(icmp_type),
            })
        }
        Some(TransportHeader::Igmp(_)) => {
            #[allow(clippy::match_same_arms)]
            // TODO!
            None
        }
        None => None,
    }
}
