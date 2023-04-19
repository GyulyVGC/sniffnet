//! GUI upper header

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;

use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::style_constants::{get_font, ICONS};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::translations::translations::{quit_analysis_translation, settings_translation};
use crate::{Language, StyleType};

pub fn header(
    style: StyleType,
    back_button: bool,
    language: Language,
    last_opened_setting: SettingsPage,
) -> Container<'static, Message> {
    let logo = Text::new('A'.to_string())
        .font(ICONS)
        .horizontal_alignment(Horizontal::Center)
        .size(100);

    Container::new(
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(if back_button {
                Container::new(get_button_reset(style, language))
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
            ),
    )
    .height(Length::Fixed(95.0))
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}

fn get_button_reset(style: StyleType, language: Language) -> Tooltip<'static, Message> {
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
    .on_press(Message::ResetButtonPressed);

    Tooltip::new(
        content,
        quit_analysis_translation(language),
        Position::Right,
    )
    .font(get_font(style, language))
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Tooltip),
    ))
}

pub fn get_button_settings(
    style: StyleType,
    language: Language,
    open_overlay: SettingsPage,
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
    .on_press(Message::OpenSettings(open_overlay));

    Tooltip::new(content, settings_translation(language), Position::Left)
        .font(get_font(style, language))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
