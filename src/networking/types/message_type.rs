//! Module defining the `MessageType` enum,
//! which unifies the message types of the protocols that carry them (ARP, ICMP, and IGMP).

use std::fmt::{Display, Formatter};

use sniffnet_packet_parser::{ArpType, IcmpType, IgmpType, ParsedPacket};

/// The message type carried by a packet, for the protocols that define one.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MessageType {
    /// The ARP message type.
    Arp(ArpType),
    /// The ICMP message type.
    Icmp(IcmpType),
    /// The IGMP message type.
    Igmp(IgmpType),
}

impl MessageType {
    /// Extracts the message type carried by a parsed packet, if any.
    pub fn from_parsed_packet(parsed: &ParsedPacket) -> Option<Self> {
        match parsed.transport_info.protocol {
            sniffnet_packet_parser::Protocol::Icmpv4 | sniffnet_packet_parser::Protocol::Icmpv6 => {
                parsed.transport_info.icmp_type.map(Self::Icmp)
            }
            sniffnet_packet_parser::Protocol::Igmp => {
                parsed.transport_info.igmp_type.map(Self::Igmp)
            }
            sniffnet_packet_parser::Protocol::Arp => parsed.net_info.arp_type.map(Self::Arp),
            _ => None,
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Arp(arp_type) => write!(f, "{arp_type}"),
            MessageType::Icmp(icmp_type) => write!(f, "{icmp_type}"),
            MessageType::Igmp(igmp_type) => write!(f, "{igmp_type}"),
        }
    }
}
