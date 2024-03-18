//! GUI bottom footer

use std::sync::{Arc, Mutex};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::widget::{horizontal_space, Space};
use iced::{Alignment, Font, Length};

use crate::gui::components::button::row_open_link_tooltip;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::translations::translations_2::new_version_available_translation;
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::types::icon::Icon;
use crate::utils::types::web_page::WebPage;
use crate::{Language, SNIFFNET_TITLECASE};

pub fn footer(
    thumbnail: bool,
    language: Language,
    color_gradient: GradientType,
    font: Font,
    font_footer: Font,
    newer_release_available: &Arc<Mutex<Option<bool>>>,
) -> Container<'static, Message, StyleType> {
    if thumbnail {
        return thumbnail_footer();
    }

    let release_details_row =
        get_release_details(language, font, font_footer, newer_release_available);

    let footer_row = Row::new()
        .spacing(10)
        .padding([0, 20])
        .align_items(Alignment::Center)
        .push(release_details_row)
        .push(get_button_website(font))
        .push(get_button_github(font))
        .push(get_button_sponsor(font))
        .push(
            Text::new("Made with ❤ by Giuliano Bellini")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Right)
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );

    Container::new(footer_row)
        .height(45)
        .align_y(Vertical::Center)
        .style(ContainerType::Gradient(color_gradient))
}

fn get_button_website(font: Font) -> Tooltip<'static, Message, StyleType> {
    let content = button(
        Icon::Globe
            .to_text()
            .size(17)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .height(30)
    .width(30)
    .on_press(Message::OpenWebPage(WebPage::Website));

    Tooltip::new(
        content,
        row_open_link_tooltip("Website", font),
        Position::Top,
    )
    .style(ContainerType::Tooltip)
}

fn get_button_github(font: Font) -> Tooltip<'static, Message, StyleType> {
    let content = button(
        Icon::GitHub
            .to_text()
            .size(26)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .height(40)
    .width(40)
    .on_press(Message::OpenWebPage(WebPage::Repo));

    Tooltip::new(
        content,
        row_open_link_tooltip("GitHub", font),
        Position::Top,
    )
    .style(ContainerType::Tooltip)
}

fn get_button_sponsor(font: Font) -> Tooltip<'static, Message, StyleType> {
    let content = button(
        Text::new('❤'.to_string())
            .font(font)
            .size(23)
            .style(TextType::Sponsor)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding([2, 0, 0, 0])
    .height(30)
    .width(30)
    .on_press(Message::OpenWebPage(WebPage::Sponsor));

    Tooltip::new(
        content,
        row_open_link_tooltip("Sponsor", font),
        Position::Top,
    )
    .style(ContainerType::Tooltip)
}

fn get_release_details(
    language: Language,
    font: Font,
    font_footer: Font,
    newer_release_available: &Arc<Mutex<Option<bool>>>,
) -> Row<'static, Message, StyleType> {
    let mut ret_val = Row::new()
        .align_items(Alignment::Center)
        .height(Length::Fill)
        .width(Length::Fill)
        .push(
            Text::new(format!("{SNIFFNET_TITLECASE} {APP_VERSION}"))
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );
    if let Some(boolean_response) = *newer_release_available.lock().unwrap() {
        if boolean_response {
            // a newer release is available on GitHub
            let button = button(
                Text::new('!'.to_string())
                    .style(TextType::Danger)
                    .size(28)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
                    .line_height(LineHeight::Relative(0.8)),
            )
            .padding(0)
            .height(35)
            .width(35)
            .style(ButtonType::Alert)
            .on_press(Message::OpenWebPage(WebPage::WebsiteDownload));
            let tooltip = Tooltip::new(
                button,
                row_open_link_tooltip(new_version_available_translation(language), font),
                Position::Top,
            )
            .style(ContainerType::Tooltip);
            ret_val = ret_val.push(Space::with_width(10)).push(tooltip);
        } else {
            // this is the latest release
            ret_val = ret_val.push(Text::new(" ✔").size(FONT_SIZE_SUBTITLE).font(font_footer));
        }
    }
    ret_val
}

fn thumbnail_footer() -> Container<'static, Message, StyleType> {
    Container::new(horizontal_space()).height(0)
}
