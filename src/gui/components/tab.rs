//! Tab buttons to be used in the various pages just under the header

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::overlay::Overlay;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{get_font, FONT_SIZE_SUBTITLE, ICONS};
use crate::{Language, RunningPage, StyleType};
use iced::widget::{button, horizontal_space, Button, Row, Text};
use iced::{alignment, Alignment, Length};

pub fn get_settings_tabs(
    labels: [Overlay; 3],
    icons: &[&str],
    actions: &[Message],
    active: Overlay,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_items(Alignment::Center);

    for (i, label) in labels.iter().enumerate() {
        let active = label.eq(&active);
        tabs = tabs.push(new_tab(
            (*label).get_tab_label(language).to_string(),
            (*icons.get(i).unwrap()).to_string(),
            actions.get(i).unwrap().clone(),
            active,
            style,
        ));
    }
    tabs
}

pub fn get_pages_tabs(
    labels: [RunningPage; 2],
    icons: &[&str],
    actions: &[Message],
    active: RunningPage,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
    let mut tabs = Row::new()
        .width(Length::Fill)
        .align_items(Alignment::Center);

    for (i, label) in labels.iter().enumerate() {
        let active = label.eq(&active);
        tabs = tabs.push(new_tab(
            (*label).get_tab_label(language).to_string(),
            (*icons.get(i).unwrap()).to_string(),
            actions.get(i).unwrap().clone(),
            active,
            style,
        ));
    }
    tabs
}

fn new_tab(
    label: String,
    icon: String,
    action: Message,
    active: bool,
    style: StyleType,
) -> Button<'static, Message> {
    let content = Row::new()
        .align_items(Alignment::Center)
        .push(horizontal_space(Length::FillPortion(1)))
        .push(
            Text::new(icon)
                .font(ICONS)
                .size(15)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .push(
            Text::new(label)
                .font(get_font(style))
                .size(FONT_SIZE_SUBTITLE)
                .horizontal_alignment(alignment::Horizontal::Center)
                .vertical_alignment(alignment::Vertical::Center),
        )
        .push(horizontal_space(Length::FillPortion(1)));

    button(content)
        .height(Length::Units(35))
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
