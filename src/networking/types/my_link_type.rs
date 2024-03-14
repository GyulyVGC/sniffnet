use iced::widget::Column;
use iced::Font;
use pcap::Linktype;

use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::translations::translations_3::link_type_translation;
use crate::{Language, StyleType};

/// Currently supported link types
#[derive(Copy, Clone, Default)]
pub enum MyLinkType {
    Null(Linktype),
    Ethernet(Linktype),
    RawIp(Linktype),
    Loop(Linktype),
    IPv4(Linktype),
    IPv6(Linktype),
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
            | Self::Unsupported(l) => {
                format!(
                    "{}: {} ({})",
                    link_type_translation(language),
                    l.get_name().unwrap_or_else(|_| l.0.to_string()),
                    l.get_description().unwrap_or(String::new())
                )
            }
            Self::NotYetAssigned => String::new(),
        }
    }

    pub fn link_type_col(
        self,
        language: Language,
        font: Font,
    ) -> Column<'static, Message, StyleType> {
        match self {
            Self::Null(l)
            | Self::Ethernet(l)
            | Self::RawIp(l)
            | Self::Loop(l)
            | Self::IPv4(l)
            | Self::IPv6(l)
            | Self::Unsupported(l) => {
                let link_info = format!(
                    "{} ({})",
                    l.get_name().unwrap_or_else(|_| l.0.to_string()),
                    l.get_description().unwrap_or(String::new())
                );
                TextType::highlighted_subtitle_with_desc(
                    link_type_translation(language),
                    &link_info,
                    font,
                )
            }
            Self::NotYetAssigned => Column::new().height(0),
        }
    }
}
