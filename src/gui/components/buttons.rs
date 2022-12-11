//! GUI buttons

use iced::alignment::Horizontal;
use iced::{alignment, Element, Font, Length};
use iced::widget::{button, Text};
use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::StyleType;
use crate::utility::style_constants::{FONT_SIZE_TITLE, ICONS};

pub fn get_button_style(style: StyleType) -> Element<'static, Message> {
    button(
        Text::new('K'.to_string())
            .font(ICONS)
            .width(Length::Units(25))
            .horizontal_alignment(Horizontal::Center)
            .size(20),
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(60))
        .style(StyleTuple(style, ElementType::Standard).into())
        .on_press(Message::Style)
        .into()
}

pub fn get_button_start(style: StyleType, font: Font) -> Element<'static, Message> {
    button(
        Text::new("Run!")
            .font(font)
            .size(FONT_SIZE_TITLE)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
        .padding(10)
        .height(Length::Units(80))
        .width(Length::Units(160))
        .style(StyleTuple(style, ElementType::Standard).into())
        .on_press(Message::Start)
        .into()
}

pub fn get_button_github(style: StyleType) -> Element<'static, Message> {
    button(
        Text::new('H'.to_string())
            .font(ICONS)
            .size(24)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .height(Length::Units(35))
        .width(Length::Units(35))
        .style(StyleTuple(style, ElementType::Standard).into())
        .on_press(Message::OpenGithub)
        .into()
}
