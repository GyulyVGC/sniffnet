#![allow(clippy::module_name_repetitions)]

use iced::Alignment;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Row, Text, Tooltip, button};

use crate::gui::styles::container::ContainerType;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::translations::translations::hide_translation;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

pub fn button_hide<'a>(message: Message, language: Language) -> Tooltip<'a, Message, StyleType> {
    Tooltip::new(
        button(
            Text::new("×")
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .size(15)
                .line_height(LineHeight::Relative(1.0)),
        )
        .padding(2)
        .height(20)
        .width(20)
        .on_press(message),
        Text::new(hide_translation(language)),
        Position::Right,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
}

pub fn button_open_file<'a>(
    old_file: String,
    file_info: FileInfo,
    language: Language,
    is_editable: bool,
    action: fn(String) -> Message,
) -> Tooltip<'a, Message, StyleType> {
    let mut tooltip_str = "";
    let mut tooltip_style = ContainerType::Standard;

    let mut button = button(
        Icon::File
            .to_text()
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
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

    Tooltip::new(button, Text::new(tooltip_str), Position::Right)
        .gap(5)
        .class(tooltip_style)
}

pub fn row_open_link_tooltip<'a>(text: &'static str) -> Row<'a, Message, StyleType> {
    Row::new()
        .align_y(Alignment::Center)
        .spacing(10)
        .push(Text::new(text))
        .push(Icon::OpenLink.to_text().size(16).class(TextType::Title))
}
