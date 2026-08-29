//! Link-layer handling: strip whatever the DLT puts in front of the network
//! header, and report what that link layer carried.

use etherparse::{EtherType, LaxPacketHeaders, LinkHeader};

use crate::link_type::LinkType;

/// Ethernet II header length.
const ETHERNET_HEADER_LEN: u32 = 14;
/// BSD loopback (`DLT_NULL` / `DLT_LOOP`) header: a 4-byte address family.
const NULL_HEADER_LEN: usize = 4;
/// Linux SLL header: `pkttype(2) hatype(2) halen(2) addr(8) protocol(2)`.
const SLL_HEADER_LEN: usize = 16;
/// Linux SLL2 header: `protocol(2) reserved(2) if_index(4) hatype(2)
/// pkttype(1) halen(1) addr(8)`.
const SLL2_HEADER_LEN: usize = 20;
/// `ARPHRD_ETHER`: the SLL address field holds a MAC.
const ARPHRD_ETHER: u16 = 1;

/// A packet's headers from the network layer down, plus what its link layer
/// carried.
#[derive(Debug, Clone)]
pub struct Sniffable<'a> {
    pub headers: LaxPacketHeaders<'a>,
    pub link: LinkInfo,
}

/// What a packet's link layer contributed.
///
/// `bytes` counts the link header actually present on the wire for this DLT,
/// so a caller adding it to the network layer's own length gets the packet's
/// full size. DLTs that carry no link header (raw IP) report zero.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LinkInfo {
    pub bytes: u32,
    pub src_mac: Option<[u8; 6]>,
    pub dst_mac: Option<[u8; 6]>,
}

/// Parse `packet` according to `link_type`, stripping the link layer.
///
/// Returns `None` when the link layer is malformed or the network layer
/// cannot be reached.
#[must_use]
pub fn sniffable_headers(packet: &[u8], link_type: LinkType) -> Option<Sniffable<'_>> {
    match link_type {
        LinkType::Null(_) | LinkType::Loop(_) => from_null(packet),
        LinkType::LinuxSll(_) => from_linux_sll(packet, true),
        LinkType::LinuxSll2(_) => from_linux_sll(packet, false),
        LinkType::RawIp(_) | LinkType::IPv4(_) | LinkType::IPv6(_) => {
            let headers = LaxPacketHeaders::from_ip(packet).ok()?;
            Some(Sniffable {
                headers,
                link: LinkInfo::default(),
            })
        }
        LinkType::Ethernet(_) | LinkType::Unsupported(_) => {
            let headers = LaxPacketHeaders::from_ethernet(packet).ok()?;
            let link = link_info_from_header(headers.link.as_ref());
            Some(Sniffable { headers, link })
        }
    }
}

/// Link info for a header etherparse parsed itself.
fn link_info_from_header(link_header: Option<&LinkHeader>) -> LinkInfo {
    match link_header {
        Some(LinkHeader::Ethernet2(header)) => LinkInfo {
            bytes: ETHERNET_HEADER_LEN,
            src_mac: Some(header.source),
            dst_mac: Some(header.destination),
        },
        _ => LinkInfo::default(),
    }
}

fn from_null(packet: &[u8]) -> Option<Sniffable<'_>> {
    if packet.len() <= NULL_HEADER_LEN {
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
        let h = &packet[..NULL_HEADER_LEN];
        let b = [h[0], h[1], h[2], h[3]];
        // check both big endian and little endian representations
        // as some OS'es use native endianness and others use big endian
        matches(u32::from_le_bytes(b)) || matches(u32::from_be_bytes(b))
    };

    if !is_valid_af_inet {
        return None;
    }

    let headers = LaxPacketHeaders::from_ip(&packet[NULL_HEADER_LEN..]).ok()?;
    Some(Sniffable {
        headers,
        link: LinkInfo {
            bytes: u32::try_from(NULL_HEADER_LEN).unwrap_or(u32::MAX),
            src_mac: None,
            dst_mac: None,
        },
    })
}

// TODO: do this with etherparse once they support Linux SLL2
fn from_linux_sll(packet: &[u8], is_v1: bool) -> Option<Sniffable<'_>> {
    let header_len = if is_v1 {
        SLL_HEADER_LEN
    } else {
        SLL2_HEADER_LEN
    };
    if packet.len() <= header_len {
        return None;
    }

    let protocol_type = u16::from_be_bytes(if is_v1 {
        [packet[14], packet[15]]
    } else {
        [packet[0], packet[1]]
    });

    // SLL carries a single link address, which is the sender's for incoming
    // packets. There is no counterpart for the other end, so `dst_mac` stays
    // `None` on these captures.
    let (arp_hardware_type, address_len, address) = if is_v1 {
        (
            u16::from_be_bytes([packet[2], packet[3]]),
            u16::from_be_bytes([packet[4], packet[5]]),
            &packet[6..14],
        )
    } else {
        (
            u16::from_be_bytes([packet[8], packet[9]]),
            u16::from(packet[11]),
            &packet[12..20],
        )
    };
    let src_mac = (arp_hardware_type == ARPHRD_ETHER && address_len == 6)
        .then(|| <[u8; 6]>::try_from(&address[..6]).ok())
        .flatten();

    Some(Sniffable {
        headers: LaxPacketHeaders::from_ether_type(EtherType(protocol_type), &packet[header_len..]),
        link: LinkInfo {
            bytes: u32::try_from(header_len).unwrap_or(u32::MAX),
            src_mac,
            dst_mac: None,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use etherparse::{IpNumber, Ipv4Header};
    use pcap::Linktype;

    /// A bare IPv4 header, enough for the network layer to be reachable.
    fn ipv4() -> Vec<u8> {
        Ipv4Header::new(0, 64, IpNumber::TCP, [10, 0, 0, 1], [10, 0, 0, 2])
            .unwrap()
            .to_bytes()
            .to_vec()
    }

    #[test]
    fn null_rejects_a_prefix_that_is_not_an_af_inet_family() {
        let mut frame = 7u32.to_le_bytes().to_vec();
        frame.extend_from_slice(&ipv4());
        assert!(sniffable_headers(&frame, LinkType::from_pcap(Linktype::NULL)).is_none());
    }

    #[test]
    fn null_accepts_both_endianness_conventions() {
        for family in [2u32, 24, 28, 30] {
            for frame in [family.to_le_bytes(), family.to_be_bytes()] {
                let mut packet = frame.to_vec();
                packet.extend_from_slice(&ipv4());
                assert!(
                    sniffable_headers(&packet, LinkType::from_pcap(Linktype::NULL)).is_some(),
                    "family {family} should be accepted"
                );
            }
        }
    }

    #[test]
    fn a_packet_no_longer_than_its_link_header_is_rejected() {
        for (link_type, header_len) in [
            (Linktype::NULL, NULL_HEADER_LEN),
            (Linktype::LINUX_SLL, SLL_HEADER_LEN),
            (Linktype::LINUX_SLL2, SLL2_HEADER_LEN),
        ] {
            let frame = vec![0u8; header_len];
            assert!(
                sniffable_headers(&frame, LinkType::from_pcap(link_type)).is_none(),
                "{link_type:?} should reject a packet of exactly {header_len} bytes"
            );
        }
    }

    #[test]
    fn linux_sll_reports_no_mac_for_a_non_ethernet_sender() {
        let mut frame = Vec::new();
        frame.extend_from_slice(&0u16.to_be_bytes()); // pkttype
        frame.extend_from_slice(&824u16.to_be_bytes()); // hatype: ARPHRD_NETLINK
        frame.extend_from_slice(&0u16.to_be_bytes()); // halen
        frame.extend_from_slice(&[0u8; 8]); // addr
        frame.extend_from_slice(&EtherType::IPV4.0.to_be_bytes());
        frame.extend_from_slice(&ipv4());

        let sniffable =
            sniffable_headers(&frame, LinkType::from_pcap(Linktype::LINUX_SLL)).unwrap();
        assert_eq!(sniffable.link.src_mac, None);
        assert_eq!(sniffable.link.dst_mac, None);
        assert_eq!(sniffable.link.bytes, 16);
    }
}
