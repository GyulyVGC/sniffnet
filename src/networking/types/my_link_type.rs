use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::translations::translations_3::link_type_translation;
use crate::{Language, StyleType};
use iced::widget::Column;
use iced::{Font, Renderer};
use pcap::Linktype;

#[derive(Copy, Clone)]
pub enum MyLinkType {
    Supported(Linktype),
    Unsupported(Linktype),
    NotYetAssigned,
}

impl MyLinkType {
    pub fn is_supported(&self) -> bool {
        match self {
            MyLinkType::Supported(_) => true,
            MyLinkType::Unsupported(_) | MyLinkType::NotYetAssigned => false,
        }
    }

    pub fn from_pcap_link_type(link_type: Linktype) -> Self {
        match link_type {
            Linktype::NULL
            | Linktype::ETHERNET
            | Linktype(12)
            | Linktype::LOOP
            | Linktype::IPV4
            | Linktype::IPV6 => Self::Supported(link_type),
            _ => Self::Unsupported(link_type),
        }
    }

    pub fn full_print_on_one_line(&self, language: Language) -> String {
        match self {
            MyLinkType::Supported(l) => {
                format!(
                    "{}: {} ({})",
                    link_type_translation(language),
                    l.get_name().unwrap_or(l.0.to_string()),
                    l.get_description().unwrap_or(String::new())
                )
            }
            MyLinkType::Unsupported(l) => {
                format!(
                    "{}: {} (NOT SUPPORTED)",
                    link_type_translation(language),
                    l.get_name().unwrap_or(l.0.to_string()),
                )
            }
            MyLinkType::NotYetAssigned => String::new(),
        }
    }

    pub fn link_type_col(
        &self,
        language: Language,
        font: Font,
    ) -> Column<'static, Message, Renderer<StyleType>> {
        match self {
            MyLinkType::Supported(l) => {
                let link_info = format!(
                    "{} ({})",
                    l.get_name().unwrap_or(l.0.to_string()),
                    l.get_description().unwrap_or(String::new())
                );
                TextType::highlighted_subtitle_with_desc(
                    link_type_translation(language),
                    &link_info,
                    font,
                )
            }
            MyLinkType::Unsupported(l) => {
                let link_info = format!(
                    "{} (NOT SUPPORTED)",
                    l.get_name().unwrap_or(l.0.to_string()),
                );
                TextType::highlighted_subtitle_with_desc(
                    link_type_translation(language),
                    &link_info,
                    font,
                )
            }
            MyLinkType::NotYetAssigned => Column::new().height(0),
        }
    }
}
