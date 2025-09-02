use pcap::Linktype;

use crate::Language;
use crate::translations::translations_3::link_type_translation;

/// Currently supported link types
#[derive(Copy, Clone, Default)]
pub enum MyLinkType {
    Null(Linktype),
    Ethernet(Linktype),
    RawIp(Linktype),
    Loop(Linktype),
    IPv4(Linktype),
    IPv6(Linktype),
    LinuxSll(Linktype),
    LinuxSll2(Linktype),
    Unsupported(Linktype),
    #[default]
    NotYetAssigned,
}

impl MyLinkType {
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unsupported(_) | Self::NotYetAssigned)
    }

    pub fn from_pcap_link_type(link_type: Linktype) -> Self {
        match link_type {
            Linktype::NULL => Self::Null(link_type),
            Linktype::ETHERNET => Self::Ethernet(link_type),
            Linktype(12) => Self::RawIp(link_type),
            Linktype::LOOP => Self::Loop(link_type),
            Linktype::IPV4 => Self::IPv4(link_type),
            Linktype::IPV6 => Self::IPv6(link_type),
            Linktype::LINUX_SLL => Self::LinuxSll(link_type),
            Linktype::LINUX_SLL2 => Self::LinuxSll2(link_type),
            _ => Self::Unsupported(link_type),
        }
    }

    pub fn full_print_on_one_line(self, language: Language) -> String {
        match self {
            Self::Null(l)
            | Self::Ethernet(l)
            | Self::RawIp(l)
            | Self::Loop(l)
            | Self::IPv4(l)
            | Self::IPv6(l)
            | Self::LinuxSll(l)
            | Self::LinuxSll2(l)
            | Self::Unsupported(l) => {
                format!(
                    "{}: {} ({})",
                    link_type_translation(language),
                    l.get_name().unwrap_or_else(|_| l.0.to_string()),
                    l.get_description().unwrap_or_else(|_| String::new())
                )
            }
            Self::NotYetAssigned => String::new(),
        }
    }
}
