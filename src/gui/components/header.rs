//! GUI upper header

use iced::alignment::{Horizontal, Vertical};
use iced::{Alignment, Element, Length};
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

//pub fn get_button_reset(style: StyleType) -> Element<'static, Message> {
// let button_reset = button(
//     Text::new('C'.to_string())
//         .font(ICONS)
//         .size(20)
//         .horizontal_alignment(alignment::Horizontal::Center)
//         .vertical_alignment(alignment::Vertical::Center),
// )
// .padding(10)
// .height(Length::Units(40))
// .width(Length::Units(60))
//     .style(StyleTuple(sniffer.style, ElementType::Standard).into())
// .on_press(Message::Reset);
//}