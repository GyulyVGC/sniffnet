//! GUI footer

use iced::{Alignment, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use iced::widget::{Container, Row, Text};
use plotters::prelude::RGBColor;
use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::style_type::StyleType;
use crate::gui::components::buttons::get_button_github;
use crate::structs::colors::{get_colors, to_rgb_color};
use crate::structs::style_tuple::StyleTuple;
use crate::utility::get_formatted_strings::APP_VERSION;
use crate::utility::style_constants::{COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, HEIGHT_FOOTER};

pub fn get_footer(style: StyleType) -> Container<'static, Message> {
    let font = match to_rgb_color(get_colors(style).text_body) {
        RGBColor(255, 255, 255) => COURIER_PRIME,
        _ => COURIER_PRIME_BOLD,
    };
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
        .width(Length::Fill)
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
            StyleTuple(style, ElementType::Headers),
        ))
}