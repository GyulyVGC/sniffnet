//! GUI bottom footer

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::style_type::StyleType;
use crate::structs::colors::{get_colors, to_rgb_color};
use crate::structs::style_tuple::StyleTuple;
use crate::utility::get_formatted_strings::APP_VERSION;
use crate::utility::style_constants::{
    get_font, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, HEIGHT_FOOTER,
    ICONS,
};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Button, Container, Row, Text};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use plotters::prelude::RGBColor;

pub fn get_footer(style: StyleType) -> Container<'static, Message> {
    let font = get_font(style);
    let font_footer = match to_rgb_color(get_colors(style).text_headers) {
        RGBColor(255, 255, 255) => COURIER_PRIME_ITALIC,
        _ => COURIER_PRIME_BOLD_ITALIC,
    };

    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(
            Text::new(format!("Sniffnet {} - by Giuliano Bellini ", APP_VERSION))
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        )
        .push(get_button_github(style))
        .push(Text::new("  ").font(font));

    Container::new(footer_row)
        .height(FillPortion(HEIGHT_FOOTER))
        .width(Length::Fill)
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
            StyleTuple(style, ElementType::Headers),
        ))
}

pub fn get_button_github(style: StyleType) -> Button<'static, Message> {
    button(
        Text::new('H'.to_string())
            .font(ICONS)
            .size(24)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Units(35))
    .width(Length::Units(35))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenGithub)
}
