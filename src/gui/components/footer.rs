//! GUI bottom footer

use std::sync::{Arc, Mutex};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::{Alignment, Color, Font, Length};
use iced_native::widget::horizontal_space;
use iced_native::widget::tooltip::Position;

use crate::gui::styles::style_constants::{
    get_font, get_font_headers, FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, ICONS,
};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::translations::translations_2::new_version_available_translation;
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::types::web_page::WebPage;
use crate::Language;

pub fn footer(
    language: Language,
    style: StyleType,
    newer_release_available: &Arc<Mutex<Result<bool, String>>>,
) -> Container<'static, Message> {
    let font_footer = get_font_headers(style);

    let release_details_row =
        get_release_details(language, style, font_footer, newer_release_available);

    let footer_row = Row::new()
        .spacing(10)
        .width(Length::Fill)
        .padding([0, 20])
        .align_items(Alignment::Center)
        .push(release_details_row)
        .push(get_button_website(style))
        .push(get_button_github(style))
        .push(get_button_sponsor(style))
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
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Headers),
        ))
}

fn get_button_website(style: StyleType) -> Tooltip<'static, Message> {
    let content = button(
        Text::new('c'.to_string())
            .font(ICONS)
            .size(18)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fixed(30.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenWebPage(WebPage::Website));

    Tooltip::new(content, "Website", Position::Top)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}

fn get_button_github(style: StyleType) -> Tooltip<'static, Message> {
    let content = button(
        Text::new('H'.to_string())
            .font(ICONS)
            .size(28)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Fixed(40.0))
    .width(Length::Fixed(40.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenWebPage(WebPage::Repo));

    Tooltip::new(content, "GitHub", Position::Top)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}

fn get_button_sponsor(style: StyleType) -> Tooltip<'static, Message> {
    let content = button(
        Text::new('❤'.to_string())
            .size(28)
            .style(iced::theme::Text::Color(Color::from_rgb(1.0, 0.3, 0.5)))
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Fixed(30.0))
    .width(Length::Fixed(30.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenWebPage(WebPage::Sponsor));

    Tooltip::new(content, "Sponsor", Position::Top)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}

fn get_release_details(
    language: Language,
    style: StyleType,
    font_footer: Font,
    newer_release_available: &Arc<Mutex<Result<bool, String>>>,
) -> Row<'static, Message> {
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
                Text::new('T'.to_string())
                    .style(iced::theme::Text::Color(Color::from_rgb(0.8, 0.15, 0.15)))
                    .font(ICONS)
                    .size(22)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            )
            .padding(5)
            .height(Length::Fixed(35.0))
            .width(Length::Fixed(35.0))
            .style(StyleTuple(style, ElementType::Alert).into())
            .on_press(Message::OpenWebPage(WebPage::WebsiteDownload));
            let tooltip = Tooltip::new(
                button,
                new_version_available_translation(language),
                Position::Top,
            )
            .font(get_font(style))
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(style, ElementType::Tooltip),
            ));
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
