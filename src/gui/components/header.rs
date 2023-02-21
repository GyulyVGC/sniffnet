//! GUI upper header

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::my_overlay::MyOverlay;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{get_font, HEIGHT_HEADER, ICONS};
use crate::utility::translations::{quit_analysis_translation, settings_translation};
use crate::{Language, StyleType};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, horizontal_space, Container, Row, Text, Tooltip};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;

pub fn header(
    style: StyleType,
    back_button: bool,
    all_packets: u128,
    language: Language,
    last_opened_setting: MyOverlay,
) -> Container<'static, Message> {
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
                Container::new(get_button_reset(style, all_packets, language))
                    .width(FillPortion(1))
                    .align_x(Horizontal::Center)
            } else {
                Container::new(Row::new())
                    .width(FillPortion(1))
                    .align_x(Horizontal::Center)
            })
            .push(
                Container::new(
                    Row::new()
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(logo),
                )
                .width(FillPortion(6))
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .align_x(Horizontal::Center),
            )
            .push(
                Container::new(get_button_settings(style, language, last_opened_setting))
                    .width(FillPortion(1))
                    .align_x(Horizontal::Center),
            )
            .push(horizontal_space(Length::Fixed(15.0))),
    )
    .height(FillPortion(HEIGHT_HEADER))
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}

pub fn get_button_reset(
    style: StyleType,
    all_packets: u128,
    language: Language,
) -> Tooltip<'static, Message> {
    let content = button(
        Text::new('C'.to_string())
            .font(ICONS)
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(40.0))
    .width(Length::Fixed(60.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(if all_packets == 0 {
        Message::Reset
    } else {
        Message::ShowModal(MyOverlay::Quit)
    });

    Tooltip::new(
        content,
        quit_analysis_translation(language),
        Position::Right,
    )
    .font(get_font(style))
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Tooltip),
    ))
}

pub fn get_button_settings(
    style: StyleType,
    language: Language,
    open_overlay: MyOverlay,
) -> Tooltip<'static, Message> {
    let content = button(
        Text::new("a")
            .font(ICONS)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(40.0))
    .width(Length::Fixed(60.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::ShowModal(open_overlay));

    Tooltip::new(content, settings_translation(language), Position::Left)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
