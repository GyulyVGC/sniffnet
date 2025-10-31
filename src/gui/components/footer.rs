//! GUI bottom footer

use iced::Element;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Row, Text, button, rich_text, span};
use iced::widget::{Space, horizontal_space};
use iced::{Alignment, Font, Length, Padding};

use crate::gui::components::button::row_open_link_tooltip;
use crate::gui::components::types::my_tooltip::MyTooltip;
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

pub fn footer<'a>(
    thumbnail: bool,
    language: Language,
    color_gradient: GradientType,
    font: Font,
    font_footer: Font,
    newer_release_available: Option<bool>,
    dots_pulse: &(String, u8),
    show_tooltips: bool,
) -> Container<'a, Message, StyleType> {
    if thumbnail {
        return thumbnail_footer();
    }

    let release_details_row = get_release_details(
        language,
        font,
        font_footer,
        newer_release_available,
        &dots_pulse.0,
        show_tooltips,
    );

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
        .push(get_button_feedback(font, show_tooltips))
        .push(get_button_wiki(font, show_tooltips))
        .push(get_button_github(font, show_tooltips))
        .push(get_button_news(font, show_tooltips))
        .push(get_button_sponsor(font, show_tooltips))
        .push(
            Column::new()
                .width(Length::Fill)
                .align_x(Alignment::End)
                .push(
                    Row::new()
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .push(
                            Text::new("Made with")
                                .size(FONT_SIZE_FOOTER)
                                .font(font_footer),
                        )
                        .push(
                            Text::new("❤")
                                .size(heart_size)
                                .font(font_footer)
                                .width(25)
                                .align_x(Alignment::Center)
                                .align_y(Alignment::Center),
                        )
                        .push(Text::new("by ").size(FONT_SIZE_FOOTER).font(font_footer))
                        .push(
                            rich_text![
                                span("Giuliano Bellini")
                                    .underline(true)
                                    .link(Message::OpenWebPage(WebPage::MyGitHub)),
                            ]
                            .size(FONT_SIZE_FOOTER)
                            .font(font_footer),
                        ),
                ),
        );

    Container::new(footer_row)
        .height(45)
        .align_y(Alignment::Center)
        .class(ContainerType::Gradient(color_gradient))
}

fn get_button_feedback<'a>(font: Font, show_tooltip: bool) -> Element<'a, Message, StyleType> {
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

    MyTooltip::new(content, row_open_link_tooltip("Roadmap", font))
        .enabled(show_tooltip)
        .position(Position::Top)
        .gap(10.0)
        .style(ContainerType::Tooltip)
        .build()
}

fn get_button_wiki<'a>(font: Font, show_tooltip: bool) -> Element<'a, Message, StyleType> {
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

    MyTooltip::new(content, row_open_link_tooltip("Wiki", font))
        .enabled(show_tooltip)
        .position(Position::Top)
        .gap(7.5)
        .style(ContainerType::Tooltip)
        .build()
}

fn get_button_github<'a>(font: Font, show_tooltip: bool) -> Element<'a, Message, StyleType> {
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

    MyTooltip::new(content, row_open_link_tooltip("Github", font))
        .enabled(show_tooltip)
        .position(Position::Top)
        .gap(5.0)
        .style(ContainerType::Tooltip)
        .build()
}

fn get_button_news<'a>(font: Font, show_tooltip: bool) -> Element<'a, Message, StyleType> {
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

    MyTooltip::new(content, row_open_link_tooltip("News", font))
        .enabled(show_tooltip)
        .position(Position::Top)
        .gap(7.5)
        .style(ContainerType::Tooltip)
        .build()
}

fn get_button_sponsor<'a>(font: Font, show_tooltip: bool) -> Element<'a, Message, StyleType> {
    let content = button(
        Text::new('❤'.to_string())
            .font(font)
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

    MyTooltip::new(content, row_open_link_tooltip("Sponsor", font))
        .enabled(show_tooltip)
        .position(Position::Top)
        .gap(10.0)
        .style(ContainerType::Tooltip)
        .build()
}

fn get_release_details<'a>(
    language: Language,
    font: Font,
    font_footer: Font,
    newer_release_available: Option<bool>,
    dots: &str,
    show_tooltip: bool,
) -> Row<'a, Message, StyleType> {
    let mut ret_val = Row::new()
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .width(Length::Fill)
        .push(
            Text::new(format!("{SNIFFNET_TITLECASE} {APP_VERSION}"))
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );
    if let Some(boolean_response) = newer_release_available {
        if boolean_response {
            // a newer release is available on GitHub
            let icon_size = match dots.len() {
                2 => 16.0,
                3 => 18.0,
                _ => 14.0,
            };
            let button = button(
                Icon::Update
                    .to_text()
                    .size(icon_size)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .line_height(LineHeight::Relative(0.8)),
            )
            .padding(0)
            .height(35)
            .width(35)
            .class(ButtonType::Alert)
            .on_press(Message::OpenWebPage(WebPage::WebsiteDownload));

            let tooltip = MyTooltip::new(
                button,
                row_open_link_tooltip(new_version_available_translation(language), font),
            )
            .enabled(show_tooltip)
            .position(Position::Top)
            .gap(7.5)
            .style(ContainerType::Tooltip)
            .build();

            ret_val = ret_val.push(Space::with_width(10)).push(tooltip);
        } else {
            // this is the latest release
            ret_val = ret_val.push(Text::new(" ✔").size(FONT_SIZE_SUBTITLE).font(font_footer));
        }
    }
    ret_val
}

fn thumbnail_footer<'a>() -> Container<'a, Message, StyleType> {
    Container::new(horizontal_space()).height(0)
}
