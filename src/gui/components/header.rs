//! GUI upper header

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{HEIGHT_HEADER, ICONS};
use crate::StyleType;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Button, Container, Row, Text};
use iced::Length::FillPortion;
use iced::{alignment, Alignment, Length};

pub fn get_header(style: StyleType, back_button: bool) -> Container<'static, Message> {
    let logo = Text::new('A'.to_string())
        .font(ICONS)
        .horizontal_alignment(Horizontal::Center)
        .size(95);

    Container::new(
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(if back_button {
                Container::new(get_button_reset(style))
                    .width(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center)
            } else {
                Container::new(Row::new())
                    .width(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center)
            })
            .push(
                Container::new(
                    Row::new()
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(logo),
                )
                .width(Length::FillPortion(6))
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .align_x(Horizontal::Center),
            )
            .push(
                Container::new(get_button_style(style))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            ),
    )
    .height(FillPortion(HEIGHT_HEADER))
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}

pub fn get_button_reset(style: StyleType) -> Button<'static, Message> {
    button(
        Text::new('C'.to_string())
            .font(ICONS)
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(10)
    .height(Length::Units(40))
    .width(Length::Units(60))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::Reset)
}

pub fn get_button_style(style: StyleType) -> Button<'static, Message> {
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
}
