use crate::StyleType;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::utils::types::icon::Icon;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Space, Text, vertical_space};
use iced::{Alignment, Font, Length};

pub fn welcome_page<'a>(font: Font, x: u8) -> Container<'a, Message, StyleType> {
    let icon = match x {
        0..=3 | 20.. => Text::new(""),
        4 => Icon::Sniffnet1.to_text(),
        5 => Icon::Sniffnet2.to_text(),
        6 => Icon::Sniffnet3.to_text(),
        7 => Icon::Sniffnet4.to_text(),
        8..=19 => Icon::Sniffnet.to_text(),
    };

    let text = Text::new(match x {
        0..=3 | 20.. => "",
        4 => "S",
        5 => "Sn",
        6 => "Sni",
        7 => "Snif",
        8 => "Sniff",
        9 => "Sniffn",
        10 => "Sniffne",
        11..=19 => "Sniffnet",
    });

    let text_type = match x {
        0..=3 | 20.. => TextType::Welcome(0.0),
        4 => TextType::Welcome(0.125),
        5 => TextType::Welcome(0.25),
        6 => TextType::Welcome(0.375),
        7 => TextType::Welcome(0.5),
        8 => TextType::Welcome(0.625),
        9 => TextType::Welcome(0.750),
        10 => TextType::Welcome(0.875),
        11..=19 => TextType::Welcome(1.0),
    };

    let body = Column::new()
        .align_x(Alignment::Center)
        .push(vertical_space())
        .push(icon.size(200).line_height(0.9).class(text_type))
        .push(text.font(font).size(75).class(text_type))
        .push(Space::with_height(Length::FillPortion(2)));

    Container::new(body)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
