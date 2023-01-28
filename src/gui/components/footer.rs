//! GUI bottom footer

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::style_type::StyleType;
use crate::structs::colors::{get_colors, to_rgb_color};
use crate::structs::style_tuple::StyleTuple;
use crate::utility::get_formatted_strings::APP_VERSION;
use crate::utility::style_constants::{
    get_font, FONT_SIZE_FOOTER, HEIGHT_FOOTER, ICONS, INCONSOLATA, INCONSOLATA_BOLD,
};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;
use plotters::prelude::RGBColor;

pub fn get_footer(style: StyleType) -> Container<'static, Message> {
    let font = get_font(style);
    let font_footer = match to_rgb_color(get_colors(style).text_headers) {
        RGBColor(255, 255, 255) => INCONSOLATA,
        _ => INCONSOLATA_BOLD,
    };

    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(
            Text::new(format!("Sniffnet {APP_VERSION} - by Giuliano Bellini "))
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
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Headers),
        ))
}

pub fn get_button_github(style: StyleType) -> iced::widget::Tooltip<'static, Message> {
    let content = button(
        Text::new('H'.to_string())
            .font(ICONS)
            .size(24)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .height(Length::Units(35))
    .width(Length::Units(35))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenGithub);

    Tooltip::new(content, "GitHub", Position::FollowCursor)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
