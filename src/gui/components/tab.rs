//! Tab buttons to be used in the various pages just under the header

use iced::widget::text::LineHeight;
use iced::widget::{Button, Container, Row, Space, Text, button, horizontal_space};
use iced::{Alignment, Font, Length, alignment};

use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::{Language, RunningPage, StyleType};

pub fn get_settings_tabs<'a>(
    active: SettingsPage,
    font: Font,
    language: Language,
) -> Row<'a, Message, StyleType> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(2)
        .padding([0, 3]);

    for page in &SettingsPage::ALL {
        let active = page.eq(&active);
        tabs = tabs.push(new_settings_tab(*page, active, language, font));
    }
    tabs
}

pub fn get_pages_tabs<'a>(
    active: RunningPage,
    font: Font,
    font_headers: Font,
    language: Language,
    unread_notifications: usize,
) -> Row<'a, Message, StyleType> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_y(Alignment::Start)
        .spacing(2)
        .padding([0, 3]);

    for page in &RunningPage::ALL {
        let active = page.eq(&active);
        let unread = if page.eq(&RunningPage::Notifications) {
            Some(unread_notifications)
        } else {
            None
        };
        tabs = tabs.push(new_page_tab(
            *page,
            active,
            language,
            font,
            font_headers,
            unread,
        ));
    }
    tabs
}

fn new_page_tab<'a>(
    page: RunningPage,
    active: bool,
    language: Language,
    font: Font,
    font_headers: Font,
    unread: Option<usize>,
) -> Button<'a, Message, StyleType> {
    let mut content = Row::new()
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .push(horizontal_space())
        .push(
            page.icon()
                .size(15)
                .class(if active {
                    TextType::Title
                } else {
                    TextType::Standard
                })
                .align_x(alignment::Alignment::Center)
                .align_y(alignment::Alignment::Center),
        )
        .push(Space::with_width(10))
        .push(
            Text::new(page.get_tab_label(language).to_string())
                .font(font)
                .size(FONT_SIZE_SUBTITLE)
                .class(if active {
                    TextType::Title
                } else {
                    TextType::Standard
                })
                .align_x(alignment::Alignment::Center)
                .align_y(alignment::Alignment::Center),
        );

    if let Some(num) = unread {
        if num > 0 {
            content = content
                .push(Space::with_width(7))
                .push(notifications_badge(font_headers, num));
        }
    }

    content = content.push(horizontal_space());

    button(content)
        .height(if active { 35 } else { 30 })
        .padding(0)
        .width(Length::Fill)
        .class(if active {
            ButtonType::TabActive
        } else {
            ButtonType::TabInactive
        })
        .on_press(page.action())
}

fn new_settings_tab<'a>(
    page: SettingsPage,
    active: bool,
    language: Language,
    font: Font,
) -> Button<'a, Message, StyleType> {
    let content = Row::new()
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .push(horizontal_space())
        .push(
            page.icon()
                .size(15)
                .class(if active {
                    TextType::Title
                } else {
                    TextType::Standard
                })
                .align_x(alignment::Alignment::Center)
                .align_y(alignment::Alignment::Center),
        )
        .push(Space::with_width(10))
        .push(
            Text::new(page.get_tab_label(language).to_string())
                .font(font)
                .size(FONT_SIZE_SUBTITLE)
                .class(if active {
                    TextType::Title
                } else {
                    TextType::Standard
                })
                .align_x(alignment::Alignment::Center)
                .align_y(alignment::Alignment::Center),
        )
        .push(horizontal_space());

    button(content)
        .height(if active { 35 } else { 30 })
        .padding(0)
        .width(Length::Fill)
        .class(if active {
            ButtonType::TabActive
        } else {
            ButtonType::TabInactive
        })
        .on_press(page.action())
}

pub fn notifications_badge<'a>(
    font_headers: Font,
    num: usize,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Text::new(num.to_string())
            .font(font_headers)
            .size(14)
            .line_height(LineHeight::Relative(1.0)),
    )
    .align_y(Alignment::Center)
    .padding([2, 4])
    .height(20)
    .class(ContainerType::Highlighted)
}
