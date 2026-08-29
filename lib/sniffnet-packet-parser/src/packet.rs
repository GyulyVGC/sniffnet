use std::net::{Ipv4Addr, Ipv6Addr};

use etherparse::{ArpPacket, EtherType, NetHeaders, TransportHeader};

use crate::addrs::NetAddrs;
use crate::arp_type::ArpType;
use crate::headers::{LinkInfo, sniffable_headers};
use crate::icmp_type::{IcmpType, IcmpTypeV4, IcmpTypeV6};
use crate::link_type::LinkType;
use crate::protocol::Protocol;

/// Fixed part of an IPv6 header, which `payload_length` excludes.
const IPV6_HEADER_LEN: u32 = 40;

/// Everything this crate reads out of a single packet.
///
/// `src_port` / `dst_port` are `None` for protocols that have no ports
/// (ICMP, IGMP, ARP). `src_mac` / `dst_mac` are `None` when the link type
/// carries no MAC — raw IP and loopback captures have none, and Linux SLL
/// carries only one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParsedPacket {
    pub addrs: NetAddrs,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Protocol,
    pub src_mac: Option<[u8; 6]>,
    pub dst_mac: Option<[u8; 6]>,
    /// Link header plus network layer, in bytes.
    pub bytes: u32,
    pub icmp_type: Option<IcmpType>,
    pub arp_type: Option<ArpType>,
}

/// Parse one raw packet captured on a `link_type` interface.
///
/// Returns `None` for anything this crate cannot key by address and protocol:
/// a malformed or truncated header, a network layer that is neither IP nor
/// ARP, an ARP packet whose protocol addresses are not IP, or an IP packet
/// whose transport header is missing (a non-first fragment, say).
#[must_use]
pub fn parse(packet: &[u8], link_type: LinkType) -> Option<ParsedPacket> {
    let sniffable = sniffable_headers(packet, link_type)?;
    let link = sniffable.link;

    match sniffable.headers.net? {
        NetHeaders::Ipv4(header, _) => Some(from_transport(
            sniffable.headers.transport?,
            NetAddrs::V4 {
                src: Ipv4Addr::from(header.source),
                dst: Ipv4Addr::from(header.destination),
            },
            link,
            u32::from(header.total_len),
        )),
        NetHeaders::Ipv6(header, _) => Some(from_transport(
            sniffable.headers.transport?,
            NetAddrs::V6 {
                src: Ipv6Addr::from(header.source),
                dst: Ipv6Addr::from(header.destination),
            },
            link,
            u32::from(header.payload_length) + IPV6_HEADER_LEN,
        )),
        NetHeaders::Arp(arp) => from_arp(&arp, link),
    }
}

fn from_transport(
    transport: TransportHeader,
    addrs: NetAddrs,
    link: LinkInfo,
    net_bytes: u32,
) -> ParsedPacket {
    let (src_port, dst_port, protocol, icmp_type) = match transport {
        TransportHeader::Tcp(header) => (
            Some(header.source_port),
            Some(header.destination_port),
            Protocol::Tcp,
            None,
        ),
        TransportHeader::Udp(header) => (
            Some(header.source_port),
            Some(header.destination_port),
            Protocol::Udp,
            None,
        ),
        TransportHeader::Icmpv4(header) => (
            None,
            None,
            Protocol::Icmpv4,
            Some(IcmpTypeV4::from_etherparse(&header.icmp_type)),
        ),
        TransportHeader::Icmpv6(header) => (
            None,
            None,
            Protocol::Icmpv6,
            Some(IcmpTypeV6::from_etherparse(&header.icmp_type)),
        ),
        TransportHeader::Igmp(_) => (None, None, Protocol::Igmp, None),
    };

    ParsedPacket {
        addrs,
        src_port,
        dst_port,
        protocol,
        src_mac: link.src_mac,
        dst_mac: link.dst_mac,
        bytes: link.bytes.saturating_add(net_bytes),
        icmp_type,
        arp_type: None,
    }
}

fn from_arp(arp: &ArpPacket, link: LinkInfo) -> Option<ParsedPacket> {
    let addrs = match arp.proto_addr_type {
        EtherType::IPV4 => NetAddrs::V4 {
            src: Ipv4Addr::from(<[u8; 4]>::try_from(arp.sender_protocol_addr()).ok()?),
            dst: Ipv4Addr::from(<[u8; 4]>::try_from(arp.target_protocol_addr()).ok()?),
        },
        EtherType::IPV6 => NetAddrs::V6 {
            src: Ipv6Addr::from(<[u8; 16]>::try_from(arp.sender_protocol_addr()).ok()?),
            dst: Ipv6Addr::from(<[u8; 16]>::try_from(arp.target_protocol_addr()).ok()?),
        },
        _ => return None,
    };

    Some(ParsedPacket {
        addrs,
        src_port: None,
        dst_port: None,
        protocol: Protocol::Arp,
        src_mac: link.src_mac,
        dst_mac: link.dst_mac,
        bytes: link
            .bytes
            .saturating_add(u32::try_from(arp.packet_len()).unwrap_or(u32::MAX)),
        icmp_type: None,
        arp_type: Some(ArpType::from_etherparse(arp.operation)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use etherparse::{
        ArpHardwareId, ArpOperation, IpFragOffset, IpNumber, Ipv4Header, PacketBuilder,
    };
    use pcap::Linktype;

    const SRC_MAC: [u8; 6] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    const DST_MAC: [u8; 6] = [0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb];
    const SRC_V4: [u8; 4] = [10, 0, 0, 1];
    const DST_V4: [u8; 4] = [10, 0, 0, 2];

    fn ethernet_link_type() -> LinkType {
        LinkType::from_pcap(Linktype::ETHERNET)
    }

    /// Wrap `payload` in an Ethernet II header: `dst(6) src(6) ether_type(2)`.
    fn ethernet(ether_type: EtherType, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(&DST_MAC);
        frame.extend_from_slice(&SRC_MAC);
        frame.extend_from_slice(&ether_type.0.to_be_bytes());
        frame.extend_from_slice(payload);
        frame
    }

    /// Wrap `payload` in a Linux SLL v1 header advertising an Ethernet sender.
    fn linux_sll(ether_type: EtherType, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(&0u16.to_be_bytes()); // pkttype: to us
        frame.extend_from_slice(&1u16.to_be_bytes()); // hatype: ARPHRD_ETHER
        frame.extend_from_slice(&6u16.to_be_bytes()); // halen
        frame.extend_from_slice(&SRC_MAC);
        frame.extend_from_slice(&[0, 0]); // addr is an 8-byte field
        frame.extend_from_slice(&ether_type.0.to_be_bytes());
        frame.extend_from_slice(payload);
        frame
    }

    /// Wrap `payload` in a Linux SLL2 header advertising an Ethernet sender.
    fn linux_sll2(ether_type: EtherType, payload: &[u8]) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(&ether_type.0.to_be_bytes());
        frame.extend_from_slice(&[0, 0]); // reserved
        frame.extend_from_slice(&1u32.to_be_bytes()); // if_index
        frame.extend_from_slice(&1u16.to_be_bytes()); // hatype: ARPHRD_ETHER
        frame.push(0); // pkttype: to us
        frame.push(6); // halen
        frame.extend_from_slice(&SRC_MAC);
        frame.extend_from_slice(&[0, 0]); // addr is an 8-byte field
        frame.extend_from_slice(payload);
        frame
    }

    /// An IPv4/TCP packet: 20 + 20 = 40 bytes.
    fn ipv4_tcp() -> Vec<u8> {
        let builder = PacketBuilder::ipv4(SRC_V4, DST_V4, 64).tcp(40000, 443, 0, 1024);
        let mut out = Vec::new();
        builder.write(&mut out, &[]).unwrap();
        out
    }

    #[test]
    fn ethernet_ipv4_tcp_carries_ports_macs_and_bytes() {
        let parsed = parse(
            &ethernet(EtherType::IPV4, &ipv4_tcp()),
            ethernet_link_type(),
        )
        .unwrap();
        assert_eq!(
            parsed.addrs,
            NetAddrs::V4 {
                src: Ipv4Addr::from(SRC_V4),
                dst: Ipv4Addr::from(DST_V4),
            }
        );
        assert_eq!(parsed.src_port, Some(40000));
        assert_eq!(parsed.dst_port, Some(443));
        assert_eq!(parsed.protocol, Protocol::Tcp);
        assert_eq!(parsed.src_mac, Some(SRC_MAC));
        assert_eq!(parsed.dst_mac, Some(DST_MAC));
        assert_eq!(parsed.bytes, 14 + 40);
        assert_eq!(parsed.icmp_type, None);
        assert_eq!(parsed.arp_type, None);
    }

    #[test]
    fn ethernet_ipv6_udp_counts_the_fixed_header() {
        let src = [0x20, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let dst = [0x20, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];
        let builder = PacketBuilder::ipv6(src, dst, 64).udp(53, 40000);
        let mut packet = Vec::new();
        builder.write(&mut packet, &[0xaa; 10]).unwrap();

        let parsed = parse(&ethernet(EtherType::IPV6, &packet), ethernet_link_type()).unwrap();
        assert_eq!(
            parsed.addrs,
            NetAddrs::V6 {
                src: Ipv6Addr::from(src),
                dst: Ipv6Addr::from(dst),
            }
        );
        assert_eq!(parsed.src_port, Some(53));
        assert_eq!(parsed.dst_port, Some(40000));
        assert_eq!(parsed.protocol, Protocol::Udp);
        // payload_length covers UDP header + payload; the 40-byte IPv6 header does not
        assert_eq!(parsed.bytes, 14 + 40 + 8 + 10);
    }

    #[test]
    fn ethernet_ipv4_icmp_has_no_ports_and_reports_its_type() {
        let builder = PacketBuilder::ipv4(SRC_V4, DST_V4, 64).icmpv4_echo_request(1, 2);
        let mut packet = Vec::new();
        builder.write(&mut packet, &[]).unwrap();

        let parsed = parse(&ethernet(EtherType::IPV4, &packet), ethernet_link_type()).unwrap();
        assert_eq!(parsed.protocol, Protocol::Icmpv4);
        assert_eq!(parsed.src_port, None);
        assert_eq!(parsed.dst_port, None);
        assert_eq!(parsed.icmp_type, Some(IcmpType::V4(IcmpTypeV4::Echo)));
    }

    #[test]
    fn ethernet_arp_reports_operation_and_protocol_addresses() {
        let arp = etherparse::ArpPacket::new(
            ArpHardwareId::ETHERNET,
            EtherType::IPV4,
            ArpOperation::REQUEST,
            &SRC_MAC,
            &SRC_V4,
            &DST_MAC,
            &DST_V4,
        )
        .unwrap();
        let arp_bytes = arp.to_bytes();

        let parsed = parse(&ethernet(EtherType::ARP, &arp_bytes), ethernet_link_type()).unwrap();
        assert_eq!(
            parsed.addrs,
            NetAddrs::V4 {
                src: Ipv4Addr::from(SRC_V4),
                dst: Ipv4Addr::from(DST_V4),
            }
        );
        assert_eq!(parsed.protocol, Protocol::Arp);
        assert_eq!(parsed.src_port, None);
        assert_eq!(parsed.dst_port, None);
        assert_eq!(parsed.arp_type, Some(ArpType::Request));
        assert_eq!(parsed.bytes, 14 + u32::try_from(arp_bytes.len()).unwrap());
    }

    #[test]
    fn ipv4_igmp_has_no_ports() {
        let igmp = [0x11, 0x64, 0x00, 0x00, 224, 0, 0, 1];
        let header = Ipv4Header::new(
            u16::try_from(igmp.len()).unwrap(),
            64,
            IpNumber::IGMP,
            SRC_V4,
            [224, 0, 0, 1],
        )
        .unwrap();
        let mut packet = header.to_bytes().to_vec();
        packet.extend_from_slice(&igmp);

        let parsed = parse(&ethernet(EtherType::IPV4, &packet), ethernet_link_type()).unwrap();
        assert_eq!(parsed.protocol, Protocol::Igmp);
        assert_eq!(parsed.src_port, None);
        assert_eq!(parsed.dst_port, None);
        assert_eq!(parsed.bytes, 14 + 20 + 8);
    }

    #[test]
    fn null_loopback_counts_its_four_byte_header() {
        let mut frame = 2u32.to_le_bytes().to_vec(); // AF_INET
        frame.extend_from_slice(&ipv4_tcp());

        let parsed = parse(&frame, LinkType::from_pcap(Linktype::NULL)).unwrap();
        assert_eq!(parsed.protocol, Protocol::Tcp);
        assert_eq!(parsed.src_mac, None);
        assert_eq!(parsed.dst_mac, None);
        assert_eq!(parsed.bytes, 4 + 40);
    }

    #[test]
    fn linux_sll_recovers_the_sender_mac_and_counts_its_header() {
        let frame = linux_sll(EtherType::IPV4, &ipv4_tcp());
        let parsed = parse(&frame, LinkType::from_pcap(Linktype::LINUX_SLL)).unwrap();
        assert_eq!(parsed.protocol, Protocol::Tcp);
        assert_eq!(parsed.src_mac, Some(SRC_MAC));
        // SLL carries one link address only
        assert_eq!(parsed.dst_mac, None);
        assert_eq!(parsed.bytes, 16 + 40);
    }

    #[test]
    fn linux_sll2_recovers_the_sender_mac_and_counts_its_header() {
        let frame = linux_sll2(EtherType::IPV4, &ipv4_tcp());
        let parsed = parse(&frame, LinkType::from_pcap(Linktype::LINUX_SLL2)).unwrap();
        assert_eq!(parsed.protocol, Protocol::Tcp);
        assert_eq!(parsed.src_mac, Some(SRC_MAC));
        assert_eq!(parsed.dst_mac, None);
        assert_eq!(parsed.bytes, 20 + 40);
    }

    #[test]
    fn raw_ip_has_no_link_bytes_or_macs() {
        let parsed = parse(&ipv4_tcp(), LinkType::from_pcap(Linktype(12))).unwrap();
        assert_eq!(parsed.protocol, Protocol::Tcp);
        assert_eq!(parsed.src_mac, None);
        assert_eq!(parsed.dst_mac, None);
        assert_eq!(parsed.bytes, 40);
    }

    #[test]
    fn unsupported_link_type_is_decoded_as_ethernet() {
        let link_type = LinkType::from_pcap(Linktype(0x7fff));
        assert!(!link_type.is_supported());
        let parsed = parse(&ethernet(EtherType::IPV4, &ipv4_tcp()), link_type).unwrap();
        assert_eq!(parsed.src_mac, Some(SRC_MAC));
        assert_eq!(parsed.bytes, 14 + 40);
    }

    #[test]
    fn non_first_fragment_has_no_transport_header() {
        let mut header = Ipv4Header::new(20, 64, IpNumber::TCP, SRC_V4, DST_V4).unwrap();
        header.fragment_offset = IpFragOffset::try_new(1).unwrap();
        let mut packet = header.to_bytes().to_vec();
        packet.extend_from_slice(&[0u8; 20]);

        assert_eq!(
            parse(&ethernet(EtherType::IPV4, &packet), ethernet_link_type()),
            None
        );
    }
}
