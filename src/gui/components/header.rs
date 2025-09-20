//! GUI upper header

use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Container, Row, Space, Text, Tooltip, button, horizontal_space};
use iced::{Alignment, Element, Font, Length};

use crate::gui::components::footer::get_release_details;
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

pub struct ViewModeSizes {
    button_height: u16,
    button_content_size: u16,
    button_padding: u16,
    button_width: u16,
    container_height: u16,
    logo_size: u16,
}

static DEFAULT_SIZES: ViewModeSizes = ViewModeSizes {
    button_height: 40,
    button_content_size: 20,
    button_padding: 10,
    button_width: 60,
    container_height: 70,
    logo_size: 80,
};

static FOCUS_MODE_SIZES: ViewModeSizes = ViewModeSizes {
    button_height: 26,
    button_content_size: 13,
    button_padding: 6,
    button_width: 39,
    container_height: 45,
    logo_size: 51,
};

pub fn header(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let thumbnail = sniffer.thumbnail;
    let Settings {
        style,
        language,
        color_gradient,
        focus_mode,
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
    let header_sizes = if focus_mode {
        &FOCUS_MODE_SIZES
    } else {
        &DEFAULT_SIZES
    };

    let logo = Icon::Sniffnet
        .to_text()
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .line_height(LineHeight::Relative(0.7))
        .size(header_sizes.logo_size);

    let left_content: Element<Message, StyleType> = if is_running {
        get_button_reset(font, language, header_sizes).into()
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
        if focus_mode {
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
            Some(header_sizes),
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
    .height(header_sizes.container_height)
    .align_y(Alignment::Center)
    .class(ContainerType::Gradient(color_gradient))
}

fn get_button_reset<'a>(
    font: Font,
    language: Language,
    view_sizes: &ViewModeSizes,
) -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::ArrowBack
            .to_text()
            .size(view_sizes.button_content_size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(view_sizes.button_padding)
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
    view_sizes: Option<&ViewModeSizes>,
) -> Tooltip<'a, Message, StyleType> {
    let ViewModeSizes {
        button_content_size,
        button_height,
        button_width,
        ..
    } = view_sizes.unwrap_or(&DEFAULT_SIZES);

    let content = button(
        Icon::Settings
            .to_text()
            .size(*button_content_size)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .width(*button_width)
    .height(*button_height)
    .padding(0)
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
