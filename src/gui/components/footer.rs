//! GUI bottom footer

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::style_type::StyleType;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::get_formatted_strings::APP_VERSION;
use crate::utility::style_constants::{get_font, get_font_headers, FONT_SIZE_FOOTER, ICONS};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::{Alignment, Length};
use iced_native::widget::horizontal_space;
use iced_native::widget::tooltip::Position;

pub fn footer(style: StyleType) -> Container<'static, Message> {
    let font_footer = get_font_headers(style);

    let footer_row = Row::new()
        .width(Length::Fill)
        .padding([0, 20])
        .align_items(Alignment::Center)
        .push(
            Text::new(format!("Version {APP_VERSION}                  "))
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        )
        .push(horizontal_space(Length::FillPortion(1)))
        .push(get_button_github(style))
        .push(horizontal_space(Length::FillPortion(1)))
        .push(
            Text::new("Made with ❤ by Giuliano Bellini")
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

pub fn get_button_github(style: StyleType) -> Tooltip<'static, Message> {
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
    .on_press(Message::OpenGithub);

    Tooltip::new(content, "GitHub", Position::Top)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
