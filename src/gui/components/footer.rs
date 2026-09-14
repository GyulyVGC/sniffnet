//! GUI bottom footer

use crate::gui::components::button::row_open_link_tooltip;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{FONT_SIZE_BODY, FONT_SIZE_FOOTER, TOOLTIP_DELAY};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::update_status::UpdateStatus;
use crate::translations::translations_6::update_status_translation;
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::types::icon::Icon;
use crate::utils::types::web_page::WebPage;
use crate::{Language, SNIFFNET_TITLECASE};
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Row, Text, Tooltip, button, rich_text, span};
use iced::{Alignment, Length, Padding};

pub fn footer<'a>(
    thumbnail: bool,
    language: Language,
    color_gradient: GradientType,
    update_status: &UpdateStatus,
    dots_pulse: &(String, u8),
    expanded_view: bool,
) -> Option<Container<'a, Message, StyleType>> {
    if thumbnail || expanded_view {
        return None;
    }

    let release_details_row =
        get_release_details(language, update_status, dots_pulse, expanded_view);

    let heart_size = match dots_pulse.1 {
        1 => 17.0,
        2 => 20.0,
        _ => 14.0,
    };

    let footer_row = Row::new()
        .spacing(10)
        .padding([0, 20])
        .align_y(Alignment::Center)
        .push(release_details_row)
        .push(get_button_roadmap())
        .push(get_button_wiki())
        .push(get_button_github())
        .push(get_button_news())
        .push(get_button_sponsor())
        .push(
            Column::new()
                .width(Length::Fill)
                .align_x(Alignment::End)
                .push(
                    Row::new()
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .push(Text::new("Made with").size(FONT_SIZE_FOOTER))
                        .push(
                            Text::new("❤")
                                .size(heart_size)
                                .width(25)
                                .align_x(Alignment::Center)
                                .align_y(Alignment::Center),
                        )
                        .push(Text::new("by ").size(FONT_SIZE_FOOTER))
                        .push(
                            Tooltip::new(
                                rich_text![span("Giuliano Bellini").underline(true).link(())]
                                    .on_link_click(|()| Message::OpenWebPage(WebPage::MyGitHub))
                                    .size(FONT_SIZE_FOOTER),
                                row_open_link_tooltip(""),
                                Position::FollowCursor,
                            )
                            .class(ContainerType::Tooltip)
                            .delay(TOOLTIP_DELAY),
                        ),
                ),
        );

    Some(
        Container::new(footer_row)
            .height(45)
            .align_y(Alignment::Center)
            .class(ContainerType::Gradient(color_gradient)),
    )
}

fn get_button_roadmap<'a>() -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::Roadmap
            .to_text()
            .size(15)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(Padding::ZERO.top(2))
    .height(30)
    .width(30)
    .on_press(Message::OpenWebPage(WebPage::Roadmap));

    Tooltip::new(content, row_open_link_tooltip("Roadmap"), Position::Top)
        .gap(10)
        .class(ContainerType::Tooltip)
        .delay(TOOLTIP_DELAY)
}

fn get_button_wiki<'a>() -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::Book
            .to_text()
            .size(19)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(Padding::ZERO.top(1))
    .height(35)
    .width(35)
    .on_press(Message::OpenWebPage(WebPage::Wiki));

    Tooltip::new(content, row_open_link_tooltip("Wiki"), Position::Top)
        .gap(7.5)
        .class(ContainerType::Tooltip)
        .delay(TOOLTIP_DELAY)
}

fn get_button_github<'a>() -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::GitHub
            .to_text()
            .size(26)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .height(40)
    .width(40)
    .on_press(Message::OpenWebPage(WebPage::Repo));

    Tooltip::new(content, row_open_link_tooltip("GitHub"), Position::Top)
        .gap(5)
        .class(ContainerType::Tooltip)
        .delay(TOOLTIP_DELAY)
}

fn get_button_news<'a>() -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::News
            .to_text()
            .size(16)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .height(35)
    .width(35)
    .on_press(Message::OpenWebPage(WebPage::WebsiteNews));

    Tooltip::new(content, row_open_link_tooltip("News"), Position::Top)
        .gap(7.5)
        .class(ContainerType::Tooltip)
        .delay(TOOLTIP_DELAY)
}

fn get_button_sponsor<'a>() -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Text::new('❤'.to_string())
            .size(23)
            .class(TextType::Sponsor)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(Padding::ZERO.top(2))
    .height(30)
    .width(30)
    .on_press(Message::OpenWebPage(WebPage::WebsiteSponsor));

    Tooltip::new(content, row_open_link_tooltip("Sponsor"), Position::Top)
        .gap(10)
        .class(ContainerType::Tooltip)
        .delay(TOOLTIP_DELAY)
}

pub(super) fn get_release_details<'a>(
    language: Language,
    update_status: &UpdateStatus,
    dots_pulse: &(String, u8),
    expanded_view: bool,
) -> Row<'a, Message, StyleType> {
    let mut ret_val = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .width(if expanded_view {
            Length::Shrink
        } else {
            Length::Fill
        })
        .push(
            Text::new(format!("{SNIFFNET_TITLECASE} {APP_VERSION}")).size(if expanded_view {
                FONT_SIZE_BODY
            } else {
                FONT_SIZE_FOOTER
            }),
        );

    let mut button = button(
        update_status
            .icon(dots_pulse.0.len())
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(0)
    .height(35)
    .width(35)
    .on_press(Message::ShowModal(MyModal::UpdateStatus(true)));

    if matches!(update_status, UpdateStatus::UpdateAvailable(_)) {
        button = button.class(ButtonType::Alert);
    }

    let tooltip = Tooltip::new(
        button,
        Text::new(update_status_translation(language)).size(FONT_SIZE_FOOTER),
        if expanded_view {
            Position::FollowCursor
        } else {
            Position::Right
        },
    )
    .gap(5)
    .class(ContainerType::Tooltip)
    .delay(TOOLTIP_DELAY);
    ret_val = ret_val.push(tooltip);

    ret_val
}
