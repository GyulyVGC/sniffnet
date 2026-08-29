use std::fmt::{Display, Formatter};

use pcap::Linktype;

/// `DLT_RAW`; `pcap::Linktype` exposes no associated constant for it.
const DLT_RAW: Linktype = Linktype(12);

/// Link types this crate can parse.
///
/// [`LinkType::Unsupported`] is still parsed, as Ethernet, on the assumption
/// that an unknown DLT is more often Ethernet-shaped than not; callers that
/// would rather skip such captures check [`LinkType::is_supported`] first.
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
    pub fn is_supported(self) -> bool {
        !matches!(self, LinkType::Unsupported(_))
    }

    /// The `pcap` link type this was built from.
    #[must_use]
    pub fn as_pcap(self) -> Linktype {
        match self {
            LinkType::Null(l)
            | LinkType::Ethernet(l)
            | LinkType::RawIp(l)
            | LinkType::Loop(l)
            | LinkType::IPv4(l)
            | LinkType::IPv6(l)
            | LinkType::LinuxSll(l)
            | LinkType::LinuxSll2(l)
            | LinkType::Unsupported(l) => l,
        }
    }
}

/// `libpcap`'s name for the link type, with its description in parentheses
/// when it has one, falling back to the raw DLT number.
impl Display for LinkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let link_type = self.as_pcap();
        let name = link_type
            .get_name()
            .unwrap_or_else(|_| link_type.0.to_string());
        match link_type.get_description() {
            Ok(description) => write!(f, "{name} ({description})"),
            Err(_) => write!(f, "{name}"),
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
            assert_eq!(parsed.as_pcap(), link_type);
        }
    }

    #[test]
    fn from_pcap_marks_unknown_link_types_unsupported() {
        let parsed = LinkType::from_pcap(Linktype(0x7fff));
        assert_eq!(parsed, LinkType::Unsupported(Linktype(0x7fff)));
        assert!(!parsed.is_supported());
    }

    #[test]
    fn display_includes_name_and_description() {
        assert_eq!(
            LinkType::from_pcap(Linktype::ETHERNET).to_string(),
            "EN10MB (Ethernet)"
        );
    }
}
