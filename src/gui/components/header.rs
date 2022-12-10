//! GUI upper header

use iced::alignment::{Horizontal, Vertical};
use iced::{Alignment, Length};
use iced::widget::{button, Container, Row, Text};
use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::components::buttons::get_button_style;
use crate::structs::style_tuple::StyleTuple;
use crate::StyleType;
use crate::utility::style_constants::{HEIGHT_HEADER, ICONS};

pub fn get_header(style: StyleType) -> Container<'static, Message> {
    let logo = Text::new('A'.to_string())
        .font(ICONS)
        .horizontal_alignment(Horizontal::Center)
        .size(95);

    Container::new(
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Container::new(Row::new())
                    .width(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            )
            .push(
                Container::new(
                    Row::new()
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(logo),
                )
                    .width(Length::FillPortion(6))
                    .height(Length::Fill)
                    .align_y(Vertical::Center)
                    .align_x(Horizontal::Center),
            )
            .push(
                Container::new(get_button_style(style))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            ),
    )
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
            StyleTuple(style, ElementType::Headers),
        ))
}