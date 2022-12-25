//! Tab buttons to be used in the various pages just under the header

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{get_font, FONT_SIZE_SUBTITLE};
use crate::StyleType;
use iced::widget::{button, Button, Row, Text};
use iced::{alignment, Alignment, Length};

pub fn get_tabs(
    labels: &[&str],
    actions: &[Message],
    active: &str,
    style: StyleType,
) -> Row<'static, Message> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_items(Alignment::Center);

    for (i, label) in labels.iter().enumerate() {
        let active = label.eq(&active);
        tabs = tabs.push(new_tab(
            (*label).to_string(),
            actions.get(i).unwrap().clone(),
            active,
            style,
        ));
    }
    tabs
}

fn new_tab(
    label: String,
    action: Message,
    active: bool,
    style: StyleType,
) -> Button<'static, Message> {
    button(
        Text::new(label)
            .font(get_font(style))
            .size(FONT_SIZE_SUBTITLE)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .height(Length::Units(30))
    .width(Length::FillPortion(1))
    .style(
        StyleTuple(
            style,
            if active {
                ElementType::TabActive
            } else {
                ElementType::TabInactive
            },
        )
        .into(),
    )
    .on_press(action)
}
