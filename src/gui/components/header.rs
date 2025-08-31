//! GUI upper header

use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Button, Container, Row, Space, Text, Tooltip, button, horizontal_space};
use iced::{Alignment, Element, Font, Length};

use crate::gui::components::shared::get_release_details;
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
use crate::utils::types::icon::Icon;
use crate::{Language, SNIFFNET_TITLECASE, StyleType};

pub fn header(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let thumbnail = sniffer.thumbnail;
    let Settings {
        style,
        language,
        color_gradient,
        compact_view,
        ..
    } = sniffer.conf.settings;
    let font = style.get_extension().font;
    let font_update_notification = style.get_extension().font_headers;

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

    let last_opened_setting = sniffer.conf.last_opened_setting;
    let is_running = sniffer.running_page.is_some();
    let (logo_size, container_height) = if compact_view { (51, 45) } else { (80, 70) };

    let logo = Icon::Sniffnet
        .to_text()
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .line_height(LineHeight::Relative(0.7))
        .size(logo_size);

    let left_content: Element<Message, StyleType> = if is_running {
        get_button_reset(font, language, compact_view).into()
    } else {
        Space::with_width(10).into()
    };

    let central_content: Element<Message, StyleType> = {
        let mut row = Row::new().push(logo).spacing(10).align_y(Alignment::Center);
        if is_running {
            row = row.push(get_button_minimize(font, language, false));
        }
        row.into()
    };

    let right_content: Element<Message, StyleType> = {
        let mut row = Row::new().spacing(10);
        if compact_view {
            row = row.push(get_release_details(
                language,
                font,
                font_update_notification,
                sniffer.newer_release_available,
            ));
        }
        row.push(get_button_settings(
            font,
            language,
            last_opened_setting,
            compact_view,
        ))
        .into()
    };

    Container::new(
        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(left_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Start)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(central_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .push(
                Container::new(right_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::End)
                    .align_y(Alignment::Center),
            ),
    )
    .padding([0, 20])
    .height(container_height)
    .align_y(Alignment::Center)
    .class(ContainerType::Gradient(color_gradient))
}

fn get_button_reset<'a>(
    font: Font,
    language: Language,
    is_compact: bool,
) -> Tooltip<'a, Message, StyleType> {
    let button_factory = if is_compact {
        button_compact::<Message>
    } else {
        button_default
    };

    let padding = if is_compact { 6 } else { 10 };

    let content = button_factory(
        Icon::ArrowBack
            .to_text()
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(padding)
    .on_press(Message::ResetButtonPressed);

    Tooltip::new(
        content,
        Text::new(quit_analysis_translation(language)).font(font),
        Position::Right,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
}

pub fn get_button_settings<'a>(
    font: Font,
    language: Language,
    open_overlay: SettingsPage,
    is_compact: bool,
) -> Tooltip<'a, Message, StyleType> {
    let button_factory = if is_compact {
        button_compact::<Message>
    } else {
        button_default
    };
    let content = button_factory(
        Icon::Settings
            .to_text()
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .on_press(Message::OpenSettings(open_overlay));

    Tooltip::new(
        content,
        Text::new(settings_translation(language)).font(font),
        Position::Left,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
}

pub fn get_button_minimize<'a>(
    font: Font,
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

    Tooltip::new(content, Text::new(tooltip).font(font), Position::Right)
        .gap(0)
        .class(tooltip_style)
}

fn thumbnail_header<'a>(
    font: Font,
    font_headers: Font,
    language: Language,
    color_gradient: GradientType,
    unread_notifications: usize,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Row::new()
            .align_y(Alignment::Center)
            .push(horizontal_space())
            .push(Space::with_width(80))
            .push(Text::new(SNIFFNET_TITLECASE).font(font_headers))
            .push(Space::with_width(10))
            .push(get_button_minimize(font, language, true))
            .push(horizontal_space())
            .push(if unread_notifications > 0 {
                Container::new(
                    notifications_badge(font, unread_notifications)
                        .class(ContainerType::HighlightedOnHeader),
                )
                .width(40)
                .align_x(Alignment::Center)
            } else {
                Container::new(Space::with_width(40))
            }),
    )
    .height(30)
    .align_y(Alignment::Center)
    .class(ContainerType::Gradient(color_gradient))
}

fn button_default<'a, Message>(content: Text<'a, StyleType>) -> Button<'a, Message, StyleType> {
    button(content.size(20)).padding(0).height(40).width(60)
}

fn button_compact<'a, Message>(content: Text<'a, StyleType>) -> Button<'a, Message, StyleType> {
    button(content.size(13)).padding(0).height(26).width(39)
}
