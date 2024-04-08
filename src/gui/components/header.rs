//! GUI upper header

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, horizontal_space, Container, Row, Space, Text, Tooltip};
use iced::{Alignment, Font, Length};

use crate::configs::types::config_settings::ConfigSettings;
use crate::gui::components::tab::notifications_badge;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::translations::translations::{quit_analysis_translation, settings_translation};
use crate::translations::translations_3::thumbnail_mode_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType, SNIFFNET_TITLECASE};

pub fn header(sniffer: &Sniffer) -> Container<'static, Message, StyleType> {
    let thumbnail = sniffer.thumbnail;
    let ConfigSettings {
        style,
        language,
        color_gradient,
        ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;

    if thumbnail {
        let font_headers = style.get_extension().font_headers;
        let unread_notifications = sniffer.unread_notifications;
        return thumbnail_header(
            font,
            font_headers,
            language,
            color_gradient,
            unread_notifications,
        );
    }

    let last_opened_setting = sniffer.last_opened_setting;
    let is_running = sniffer.running_page.ne(&RunningPage::Init);

    let logo = Icon::Sniffnet
        .to_text()
        .vertical_alignment(Vertical::Center)
        .height(Length::Fill)
        .line_height(LineHeight::Relative(0.8))
        .size(90);

    Container::new(
        Row::new()
            .padding([0, 20])
            .align_items(Alignment::Center)
            .push(if is_running {
                Container::new(get_button_reset(font, language))
            } else {
                Container::new(Space::with_width(60))
            })
            .push(horizontal_space())
            .push(Container::new(Space::with_width(40)))
            .push(Space::with_width(20))
            .push(logo)
            .push(Space::with_width(20))
            .push(if is_running {
                Container::new(get_button_minimize(font, language, false))
            } else {
                Container::new(Space::with_width(40))
            })
            .push(horizontal_space())
            .push(get_button_settings(font, language, last_opened_setting)),
    )
    .height(80)
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
            .size(20)
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

pub fn get_button_minimize(
    font: Font,
    language: Language,
    thumbnail: bool,
) -> Tooltip<'static, Message, StyleType> {
    let size = if thumbnail { 20 } else { 24 };
    let button_size = if thumbnail { 30 } else { 40 };
    let icon = if thumbnail {
        Icon::ThumbnailClose
    } else {
        Icon::ThumbnailOpen
    };
    let tooltip = if thumbnail {
        ""
    } else {
        thumbnail_mode_translation(language)
    };
    let tooltip_style = if thumbnail {
        ContainerType::Standard
    } else {
        ContainerType::Tooltip
    };

    let content = button(
        icon.to_text()
            .size(size)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(0)
    .height(button_size)
    .width(button_size)
    .style(ButtonType::Thumbnail)
    .on_press(Message::ToggleThumbnail(false));

    Tooltip::new(content, Text::new(tooltip).font(font), Position::Right)
        .gap(0)
        .style(tooltip_style)
}

fn thumbnail_header(
    font: Font,
    font_headers: Font,
    language: Language,
    color_gradient: GradientType,
    unread_notifications: usize,
) -> Container<'static, Message, StyleType> {
    Container::new(
        Row::new()
            .align_items(Alignment::Center)
            .push(horizontal_space())
            .push(Space::with_width(80))
            .push(Text::new(SNIFFNET_TITLECASE).font(font_headers))
            .push(Space::with_width(10))
            .push(get_button_minimize(font, language, true))
            .push(horizontal_space())
            .push(if unread_notifications > 0 {
                Container::new(
                    notifications_badge(font, unread_notifications)
                        .style(ContainerType::HighlightedOnHeader),
                )
                .width(40)
                .align_x(Horizontal::Center)
            } else {
                Container::new(Space::with_width(40))
            }),
    )
    .height(30)
    .align_y(Vertical::Center)
    .style(ContainerType::Gradient(color_gradient))
}
