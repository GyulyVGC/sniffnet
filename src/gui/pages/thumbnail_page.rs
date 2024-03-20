use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use iced::widget::{Column, Container, Space};
use iced::Length;

/// Computes the body of the thumbnail view
pub fn thumbnail_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let content = Column::new()
        .push(Container::new(sniffer.traffic_chart.view()).height(Length::FillPortion(3)))
        .push(Space::with_height(Length::FillPortion(3)));

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
}
