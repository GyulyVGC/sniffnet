use std::fmt::{Display, Formatter};

/// Transport protocol observed in a packet.
///
/// ICMP is kept split by IP version: the two carry different message type
/// registries and different IANA protocol numbers. Callers that treat them as
/// one protocol collapse the two variants at their own boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmpv4,
    Icmpv6,
    Igmp,
    Arp,
}

impl Protocol {
    /// IANA IP protocol number, or `None` for ARP, which is not carried over IP.
    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_matches_iana_assignments() {
        assert_eq!(Protocol::Tcp.number(), Some(6));
        assert_eq!(Protocol::Udp.number(), Some(17));
        assert_eq!(Protocol::Icmpv4.number(), Some(1));
        assert_eq!(Protocol::Icmpv6.number(), Some(58));
        assert_eq!(Protocol::Igmp.number(), Some(2));
        assert_eq!(Protocol::Arp.number(), None);
    }
}
