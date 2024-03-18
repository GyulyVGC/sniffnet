#![allow(clippy::module_name_repetitions)]

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, Row, Text, Tooltip};
use iced::{Alignment, Font};

use crate::gui::styles::container::ContainerType;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::translations::translations::hide_translation;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

pub fn button_hide(
    message: Message,
    language: Language,
    font: Font,
) -> Tooltip<'static, Message, StyleType> {
    Tooltip::new(
        button(
            Text::new("×")
                .font(font)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
                .size(15)
                .line_height(LineHeight::Relative(1.0)),
        )
        .padding(2)
        .height(20)
        .width(20)
        .on_press(message),
        Text::new(hide_translation(language)).font(font),
        Position::Right,
    )
    .gap(5)
    .style(ContainerType::Tooltip)
}

pub fn button_open_file(
    old_file: String,
    file_info: FileInfo,
    language: Language,
    font: Font,
    is_editable: bool,
    action: fn(String) -> Message,
) -> Tooltip<'static, Message, StyleType> {
    let mut tooltip_str = "";
    let mut tooltip_style = ContainerType::Standard;

    let mut button = button(
        Icon::File
            .to_text()
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .size(16.0),
    )
    .padding(0)
    .height(25)
    .width(40);

    if is_editable {
        tooltip_str = file_info.action_info(language);
        tooltip_style = ContainerType::Tooltip;
        button = button.on_press(Message::OpenFile(old_file, file_info, action));
    }

    Tooltip::new(button, Text::new(tooltip_str).font(font), Position::Right)
        .gap(5)
        .style(tooltip_style)
}

pub fn row_open_link_tooltip(text: &'static str, font: Font) -> Row<'static, Message, StyleType> {
    Row::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .push(Text::new(text).font(font))
        .push(Icon::OpenLink.to_text().size(16).style(TextType::Title))
}
