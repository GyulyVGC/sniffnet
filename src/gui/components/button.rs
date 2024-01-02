#![allow(clippy::module_name_repetitions)]

use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{button, Text, Tooltip};
use iced::{Font, Length, Renderer};

use crate::gui::styles::container::ContainerType;
use crate::gui::types::message::Message;
use crate::gui::types::message::Message::LoadStyle;
use crate::translations::translations::hide_translation;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

pub fn button_hide(
    message: Message,
    language: Language,
    font: Font,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    Tooltip::new(
        button(
            Text::new("×")
                .font(font)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
                .size(15),
        )
        .padding(2)
        .height(Length::Fixed(20.0))
        .width(Length::Fixed(20.0))
        .on_press(message),
        hide_translation(language),
        Position::Right,
    )
    .gap(5)
    .font(font)
    .style(ContainerType::Tooltip)
}

pub fn button_open_file(
    old_file: String,
    file_info: FileInfo,
    language: Language,
    font: Font,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let tooltip_str = file_info.action_info(language);
    Tooltip::new(
        button(
            Icon::File
                .to_text()
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
                .size(21.0),
        )
        .padding(0)
        .height(Length::Fixed(40.0))
        .width(Length::Fixed(40.0))
        .on_press(Message::OpenFile(old_file, file_info, LoadStyle)),
        tooltip_str,
        Position::Right,
    )
    .gap(5)
    .font(font)
    .style(ContainerType::Tooltip)
}
