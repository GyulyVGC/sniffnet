//! GUI upper header

use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Container, Row, Space, Text, Tooltip, button};
use iced::{Alignment, Length};

use crate::gui::components::tab::notifications_badge;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::translations::translations::{quit_analysis_translation, settings_translation};
use crate::translations::translations_3::thumbnail_mode_translation;
use crate::translations::translations_4::{pause_translation, resume_translation};
use crate::utils::types::icon::Icon;
use crate::{Language, SNIFFNET_TITLECASE, StyleType};

pub fn header(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let thumbnail = sniffer.thumbnail;
    let Settings {
        language,
        color_gradient,
        ..
    } = sniffer.conf.settings;

    if thumbnail {
        let unread_notifications = sniffer.unread_notifications;
        return thumbnail_header(
            language,
            color_gradient,
            unread_notifications,
            sniffer.frozen,
        );
    }

    let last_opened_setting = sniffer.conf.last_opened_setting;
    let is_running = sniffer.running_page.is_some();

    let logo = Icon::Sniffnet
        .to_text()
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .line_height(LineHeight::Relative(0.7))
        .size(80);

    Container::new(
        Row::new()
            .padding([0, 20])
            .align_y(Alignment::Center)
            .push(if is_running {
                Container::new(get_button_reset(language))
            } else {
                Container::new(Space::new().width(60))
            })
            .push(Space::new().width(Length::Fill))
            .push(Container::new(Space::new().width(80)))
            .push(Space::new().width(20))
            .push(logo)
            .push(Space::new().width(20))
            .push(if is_running {
                Container::new(get_button_freeze(language, sniffer.frozen, false))
            } else {
                Container::new(Space::new().width(40))
            })
            .push(if is_running {
                Container::new(get_button_minimize(language, false))
            } else {
                Container::new(Space::new().width(40))
            })
            .push(Space::new().width(Length::Fill))
            .push(get_button_settings(language, last_opened_setting)),
    )
    .height(70)
    .align_y(Alignment::Center)
    .class(ContainerType::Gradient(color_gradient))
}

fn get_button_reset<'a>(language: Language) -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::ArrowBack
            .to_text()
            .size(20)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(10)
    .height(40)
    .width(60)
    .on_press(Message::ResetButtonPressed);

    Tooltip::new(
        content,
        Text::new(quit_analysis_translation(language)),
        Position::Right,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
}

pub fn get_button_settings<'a>(
    language: Language,
    open_overlay: SettingsPage,
) -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::Settings
            .to_text()
            .size(20)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .height(40)
    .width(60)
    .on_press(Message::OpenSettings(open_overlay));

    Tooltip::new(
        content,
        Text::new(settings_translation(language)),
        Position::Left,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
}

pub fn get_button_minimize<'a>(
    language: Language,
    thumbnail: bool,
) -> Tooltip<'a, Message, StyleType> {
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
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .height(button_size)
    .width(button_size)
    .class(ButtonType::Thumbnail)
    .on_press(Message::ToggleThumbnail(false));

    Tooltip::new(content, Text::new(tooltip), Position::FollowCursor)
        .gap(0)
        .class(tooltip_style)
}

pub fn get_button_freeze<'a>(
    language: Language,
    frozen: bool,
    thumbnail: bool,
) -> Tooltip<'a, Message, StyleType> {
    let size = if thumbnail { 19 } else { 23 };
    let button_size = if thumbnail { 30 } else { 40 };
    let icon = if frozen { Icon::Resume } else { Icon::Pause };
    let tooltip = if thumbnail {
        ""
    } else if frozen {
        resume_translation(language)
    } else {
        pause_translation(language)
    };
    let tooltip_style = if thumbnail {
        ContainerType::Standard
    } else {
        ContainerType::Tooltip
    };

    let content = button(
        icon.to_text()
            .size(size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .height(button_size)
    .width(button_size)
    .class(ButtonType::Thumbnail)
    .on_press(Message::Freeze);

    Tooltip::new(content, Text::new(tooltip), Position::FollowCursor)
        .gap(0)
        .class(tooltip_style)
}

fn thumbnail_header<'a>(
    language: Language,
    color_gradient: GradientType,
    unread_notifications: usize,
    frozen: bool,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Row::new()
            .align_y(Alignment::Center)
            .push(Space::new().width(Length::Fill))
            .push(Space::new().width(110))
            .push(Text::new(SNIFFNET_TITLECASE))
            .push(Space::new().width(10))
            .push(get_button_freeze(language, frozen, true))
            .push(get_button_minimize(language, true))
            .push(Space::new().width(Length::Fill))
            .push(if unread_notifications > 0 {
                Container::new(
                    notifications_badge(unread_notifications)
                        .class(ContainerType::HighlightedOnHeader),
                )
                .width(40)
                .align_x(Alignment::Center)
            } else {
                Container::new(Space::new().width(40))
            }),
    )
    .height(30)
    .align_y(Alignment::Center)
    .class(ContainerType::Gradient(color_gradient))
}
