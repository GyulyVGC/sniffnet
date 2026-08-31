use pcap::Linktype;

/// `DLT_RAW`; `pcap::Linktype` exposes no associated constant for it.
const DLT_RAW: Linktype = Linktype(12);

/// Link types this crate can parse.
///
/// [`LinkType::Unsupported`] is still parsed, as Ethernet, on the assumption
/// that an unknown DLT is more often Ethernet-shaped than not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    Null(Linktype),
    Ethernet(Linktype),
    RawIp(Linktype),
    Loop(Linktype),
    IPv4(Linktype),
    IPv6(Linktype),
    LinuxSll(Linktype),
    LinuxSll2(Linktype),
    Unsupported(Linktype),
}

impl LinkType {
    #[must_use]
    /// Convert a `pcap::Linktype` to a `LinkType`.
    pub fn from_pcap(link_type: Linktype) -> Self {
        match link_type {
            Linktype::NULL => LinkType::Null(link_type),
            Linktype::ETHERNET => LinkType::Ethernet(link_type),
            DLT_RAW => LinkType::RawIp(link_type),
            Linktype::LOOP => LinkType::Loop(link_type),
            Linktype::IPV4 => LinkType::IPv4(link_type),
            Linktype::IPV6 => LinkType::IPv6(link_type),
            Linktype::LINUX_SLL => LinkType::LinuxSll(link_type),
            Linktype::LINUX_SLL2 => LinkType::LinuxSll2(link_type),
            _ => LinkType::Unsupported(link_type),
        }
    }

    #[must_use]
    /// Whether the link type is supported by this crate.
    pub fn is_supported(self) -> bool {
        !matches!(self, LinkType::Unsupported(_))
    }

    #[must_use]
    /// Returns a human-readable description of the link type.
    pub fn description(self) -> String {
        match self {
            LinkType::Null(l)
            | LinkType::Ethernet(l)
            | LinkType::RawIp(l)
            | LinkType::Loop(l)
            | LinkType::IPv4(l)
            | LinkType::IPv6(l)
            | LinkType::LinuxSll(l)
            | LinkType::LinuxSll2(l)
            | LinkType::Unsupported(l) => {
                format!(
                    "{}{}",
                    l.get_name().unwrap_or_else(|_| l.0.to_string()),
                    if let Ok(desc) = l.get_description() {
                        format!(" ({desc})")
                    } else {
                        String::new()
                    }
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_pcap_recognizes_every_supported_link_type() {
        for link_type in [
            Linktype::NULL,
            Linktype::ETHERNET,
            DLT_RAW,
            Linktype::LOOP,
            Linktype::IPV4,
            Linktype::IPV6,
            Linktype::LINUX_SLL,
            Linktype::LINUX_SLL2,
        ] {
            let parsed = LinkType::from_pcap(link_type);
            assert!(parsed.is_supported(), "{link_type:?} should be supported");
        }
    }

    #[test]
    fn from_pcap_marks_unknown_link_types_unsupported() {
        let parsed = LinkType::from_pcap(Linktype(0x7fff));
        assert_eq!(parsed, LinkType::Unsupported(Linktype(0x7fff)));
        assert!(!parsed.is_supported());
    }
}
