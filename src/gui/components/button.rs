use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{button, Text, Tooltip};
use iced::{Font, Length, Renderer};

use crate::gui::styles::container::ContainerType;
use crate::gui::types::message::Message;
use crate::translations::translations::hide_translation;
use crate::{Language, StyleType};

#[allow(clippy::module_name_repetitions)]
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
