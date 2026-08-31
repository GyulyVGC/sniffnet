use crate::link_type::LinkType;
use crate::{ArpType, IcmpType, Protocol};
use etherparse::{EtherType, LaxPacketHeaders};
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LinkInfo {
    pub src_mac: Option<[u8; 6]>,
    pub dst_mac: Option<[u8; 6]>,
    pub(crate) bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetInfo {
    pub src_ip: IpAddr,
    pub dst_ip: IpAddr,
    pub arp_type: Option<ArpType>,
    pub(crate) bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransportInfo {
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub protocol: Protocol,
    pub icmp_type: Option<IcmpType>,
}

#[must_use]
pub(crate) fn get_sniffable_headers(
    packet: &[u8],
    link_type: LinkType,
) -> Option<LaxPacketHeaders<'_>> {
    match link_type {
        LinkType::Ethernet(_) | LinkType::Unsupported(_) => {
            LaxPacketHeaders::from_ethernet(packet).ok()
        }
        LinkType::RawIp(_) | LinkType::IPv4(_) | LinkType::IPv6(_) => {
            LaxPacketHeaders::from_ip(packet).ok()
        }
        LinkType::LinuxSll(_) => from_linux_sll(packet, true),
        LinkType::LinuxSll2(_) => from_linux_sll(packet, false),
        LinkType::Null(_) | LinkType::Loop(_) => from_null(packet),
    }
}

fn from_null(packet: &[u8]) -> Option<LaxPacketHeaders<'_>> {
    if packet.len() <= 4 {
        return None;
    }

    let is_valid_af_inet = {
        // based on https://wiki.wireshark.org/NullLoopback.md (2023-12-31)
        fn matches(value: u32) -> bool {
            match value {
                // 2 = IPv4 on all platforms
                // 24, 28, or 30 = IPv6 depending on platform
                2 | 24 | 28 | 30 => true,
                _ => false,
            }
        }
        let h = &packet[..4];
        let b = [h[0], h[1], h[2], h[3]];
        // check both big endian and little endian representations
        // as some OS'es use native endianness and others use big endian
        matches(u32::from_le_bytes(b)) || matches(u32::from_be_bytes(b))
    };

    if is_valid_af_inet {
        LaxPacketHeaders::from_ip(&packet[4..]).ok()
    } else {
        None
    }
}

// TODO: do this with etherparse once they support Linux SLL2
fn from_linux_sll(packet: &[u8], is_v1: bool) -> Option<LaxPacketHeaders<'_>> {
    let header_len = if is_v1 { 16 } else { 20 };
    if packet.len() <= header_len {
        return None;
    }

    let protocol_type = u16::from_be_bytes(if is_v1 {
        [packet[14], packet[15]]
    } else {
        [packet[0], packet[1]]
    });
    let payload = &packet[header_len..];

    Some(LaxPacketHeaders::from_ether_type(
        EtherType(protocol_type),
        payload,
    ))
}
