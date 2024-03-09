//! GUI upper header

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, horizontal_space, Container, Row, Space, Text, Tooltip};
use iced::{Alignment, Font, Length};

use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{quit_analysis_translation, settings_translation};
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

pub fn header(
    font: Font,
    color_gradient: GradientType,
    back_button: bool,
    language: Language,
    last_opened_setting: SettingsPage,
) -> Container<'static, Message, StyleType> {
    let logo = Icon::Sniffnet
        .to_text()
        .vertical_alignment(Vertical::Center)
        .height(Length::Fill)
        .line_height(LineHeight::Relative(0.8))
        .size(95);

    Container::new(
        Row::new()
            .padding([0, 20])
            .align_items(Alignment::Center)
            .push(if back_button {
                Container::new(get_button_reset(font, language))
            } else {
                Container::new(Space::with_width(60))
            })
            .push(horizontal_space())
            .push(logo)
            .push(horizontal_space())
            .push(Container::new(get_button_settings(
                font,
                language,
                last_opened_setting,
            ))),
    )
    .height(90)
    .align_y(Vertical::Center)
    .style(ContainerType::Gradient(color_gradient))
}

fn get_button_reset(font: Font, language: Language) -> Tooltip<'static, Message, StyleType> {
    let content = button(
        Icon::ArrowBack
            .to_text()
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(10)
    .height(40)
    .width(60)
    .on_press(Message::ResetButtonPressed);

    Tooltip::new(
        content,
        Text::new(quit_analysis_translation(language)).font(font),
        Position::Right,
    )
    .gap(5)
    .style(ContainerType::Tooltip)
}

pub fn get_button_settings(
    font: Font,
    language: Language,
    open_overlay: SettingsPage,
) -> Tooltip<'static, Message, StyleType> {
    let content = button(
        Icon::Settings
            .to_text()
            .size(20.5)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(0)
    .height(40)
    .width(60)
    .on_press(Message::OpenSettings(open_overlay));

    Tooltip::new(
        content,
        Text::new(settings_translation(language)).font(font),
        Position::Left,
    )
    .gap(5)
    .style(ContainerType::Tooltip)
}
