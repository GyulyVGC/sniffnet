// WARNING: this file is imported in build.rs

/// Enum representing the possible observed values of protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Protocol {
    /// Transmission Control Protocol
    TCP,
    /// User Datagram Protocol
    UDP,
    /// Internet Control Message Protocol
    ICMP,
    /// Address Resolution Protocol
    ARP,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Protocol {
    pub const ALL: [Protocol; 4] = [Protocol::TCP, Protocol::UDP, Protocol::ICMP, Protocol::ARP];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_display() {
        for protocol in Protocol::ALL {
            match protocol {
                Protocol::TCP => assert_eq!(protocol.to_string(), "TCP"),
                Protocol::UDP => assert_eq!(protocol.to_string(), "UDP"),
                Protocol::ICMP => assert_eq!(protocol.to_string(), "ICMP"),
                Protocol::ARP => assert_eq!(protocol.to_string(), "ARP"),
            }
        }
    }

    #[test]
    fn test_all_protocols_collection() {
        assert_eq!(Protocol::ALL.len(), 4);
        assert_eq!(Protocol::ALL.get(0).unwrap(), &Protocol::TCP);
        assert_eq!(Protocol::ALL.get(1).unwrap(), &Protocol::UDP);
        assert_eq!(Protocol::ALL.get(2).unwrap(), &Protocol::ICMP);
        assert_eq!(Protocol::ALL.get(3).unwrap(), &Protocol::ARP);
    }
}
