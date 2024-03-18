use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use iced::widget::{Container, Row};

/// Computes the body of the thumbnail view
pub fn thumbnail_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    Container::new(Row::new())
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
}
