//! GUI bottom footer

use std::sync::{Arc, Mutex};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::horizontal_space;
use iced::widget::tooltip::Position;
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::{Alignment, Font, Length, Renderer};

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
use crate::Language;

pub fn footer(
    language: Language,
    color_gradient: GradientType,
    font: Font,
    font_footer: Font,
    newer_release_available: &Arc<Mutex<Result<bool, String>>>,
) -> Container<'static, Message, Renderer<StyleType>> {
    let release_details_row =
        get_release_details(language, font, font_footer, newer_release_available);

    let footer_row = Row::new()
        .spacing(10)
        .width(Length::Fill)
        .padding([0, 20])
        .align_items(Alignment::Center)
        .push(release_details_row)
        .push(get_button_website(font))
        .push(get_button_github(font))
        .push(get_button_sponsor(font))
        .push(
            Text::new("Made with ❤ by Giuliano Bellini")
                .width(Length::FillPortion(1))
                .horizontal_alignment(Horizontal::Right)
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );

    Container::new(footer_row)
        .height(Length::Fixed(45.0))
        .width(Length::Fill)
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(ContainerType::Gradient(color_gradient))
}

fn get_button_website(font: Font) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let content = button(
        Icon::Globe
            .to_text()
            .size(17)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fixed(30.0))
    .on_press(Message::OpenWebPage(WebPage::Website));

    Tooltip::new(content, "Website", Position::Top)
        .font(font)
        .style(ContainerType::Tooltip)
}

fn get_button_github(font: Font) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let content = button(
        Icon::GitHub
            .to_text()
            .size(26)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Fixed(40.0))
    .width(Length::Fixed(40.0))
    .on_press(Message::OpenWebPage(WebPage::Repo));

    Tooltip::new(content, "GitHub", Position::Top)
        .font(font)
        .style(ContainerType::Tooltip)
}

fn get_button_sponsor(font: Font) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let content = button(
        Text::new('❤'.to_string())
            .size(23)
            .style(TextType::Sponsor)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding([2, 0, 0, 0])
    .height(Length::Fixed(30.0))
    .width(Length::Fixed(30.0))
    .on_press(Message::OpenWebPage(WebPage::Sponsor));

    Tooltip::new(content, "Sponsor", Position::Top)
        .font(font)
        .style(ContainerType::Tooltip)
}

fn get_release_details(
    language: Language,
    font: Font,
    font_footer: Font,
    newer_release_available: &Arc<Mutex<Result<bool, String>>>,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Row::new()
        .align_items(Alignment::Center)
        .height(Length::Fill)
        .width(Length::FillPortion(1))
        .push(
            Text::new(format!("Version {APP_VERSION}"))
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );
    if let Ok(boolean_response) = *newer_release_available.lock().unwrap() {
        if boolean_response {
            // a newer release is available on GitHub
            let button = button(
                Text::new('!'.to_string())
                    .style(TextType::Danger)
                    .size(28)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            )
            .padding(0)
            .height(Length::Fixed(35.0))
            .width(Length::Fixed(35.0))
            .style(ButtonType::Alert)
            .on_press(Message::OpenWebPage(WebPage::WebsiteDownload));
            let tooltip = Tooltip::new(
                button,
                new_version_available_translation(language),
                Position::Top,
            )
            .font(font)
            .style(ContainerType::Tooltip);
            ret_val = ret_val
                .push(horizontal_space(Length::Fixed(10.0)))
                .push(tooltip);
        } else {
            // this is the latest release
            ret_val = ret_val.push(Text::new(" ✔").size(FONT_SIZE_SUBTITLE).font(font_footer));
        }
    }
    ret_val
}
