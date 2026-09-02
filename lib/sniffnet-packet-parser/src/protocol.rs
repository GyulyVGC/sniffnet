use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// `repr` makes the derived `Hash` write a fixed-width discriminant:
// `SERVICES` is built by the host in build.rs, but looked up by the target at runtime
#[repr(u8)]
/// The protocol of the network packet.
pub enum Protocol {
    Tcp,
    Udp,
    Icmpv4,
    Icmpv6,
    Igmp,
    Arp,
}

impl Protocol {
    #[must_use]
    /// IANA IP protocol number, or `None` for ARP (which is not carried over IP).
    pub fn number(self) -> Option<u8> {
        match self {
            Protocol::Tcp => Some(6),
            Protocol::Udp => Some(17),
            Protocol::Icmpv4 => Some(1),
            Protocol::Icmpv6 => Some(58),
            Protocol::Igmp => Some(2),
            Protocol::Arp => None,
        }
    }

    #[must_use]
    /// Get the `Protocol` given its IANA IP protocol number.
    pub fn from_number(number: u8) -> Option<Self> {
        match number {
            6 => Some(Protocol::Tcp),
            17 => Some(Protocol::Udp),
            1 => Some(Protocol::Icmpv4),
            58 => Some(Protocol::Icmpv6),
            2 => Some(Protocol::Igmp),
            _ => None,
        }
    }

    #[must_use]
    /// Whether the protocol is portless.
    pub fn is_portless(self) -> bool {
        !matches!(self, Protocol::Tcp | Protocol::Udp)
    }

    #[must_use]
    /// Whether the protocol is `ICMPv4` or `ICMPv6`.
    pub fn is_icmp(self) -> bool {
        matches!(self, Protocol::Icmpv4 | Protocol::Icmpv6)
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Protocol::Tcp => "TCP",
                Protocol::Udp => "UDP",
                Protocol::Icmpv4 => "ICMPv4",
                Protocol::Icmpv6 => "ICMPv6",
                Protocol::Igmp => "IGMP",
                Protocol::Arp => "ARP",
            }
        )
    }
}
