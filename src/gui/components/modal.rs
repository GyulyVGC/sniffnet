use iced::alignment::{Alignment, Horizontal, Vertical};
use iced::widget::{
    button, center, container, horizontal_space, mouse_area, opaque, stack, Column, Container, Row,
    Space, Text,
};
use iced::{Color, Element, Font, Length};

use crate::gui::components::button::button_hide;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_TITLE;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    ask_clear_all_translation, ask_quit_translation, clear_all_translation,
    quit_analysis_translation, yes_translation,
};
use crate::{Language, StyleType};

pub fn get_exit_overlay(
    color_gradient: GradientType,
    font: Font,
    font_headers: Font,
    language: Language,
) -> Container<'static, Message, StyleType> {
    let row_buttons = confirm_button_row(language, font, Message::Reset);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            font,
            font_headers,
            color_gradient,
            language,
            quit_analysis_translation(language),
        ))
        .push(Space::with_height(20))
        .push(
            ask_quit_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(row_buttons);

    Container::new(content)
        .height(160)
        .width(450)
        .class(ContainerType::Modal)
}

pub fn get_clear_all_overlay(
    color_gradient: GradientType,
    font: Font,
    font_headers: Font,
    language: Language,
) -> Container<'static, Message, StyleType> {
    let row_buttons = confirm_button_row(language, font, Message::ClearAllNotifications);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_modal_header(
            font,
            font_headers,
            color_gradient,
            language,
            clear_all_translation(language),
        ))
        .push(Space::with_height(20))
        .push(
            ask_clear_all_translation(language)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
        .push(row_buttons);

    Container::new(content)
        .height(160)
        .width(450)
        .class(ContainerType::Modal)
}

fn get_modal_header(
    font: Font,
    font_headers: Font,
    color_gradient: GradientType,
    language: Language,
    title: &'static str,
) -> Container<'static, Message, StyleType> {
    Container::new(
        Row::new()
            .push(horizontal_space())
            .push(
                Text::new(title)
                    .font(font_headers)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(button_hide(Message::HideModal, language, font))
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(40)
    .width(Length::Fill)
    .class(ContainerType::Gradient(color_gradient))
}

fn confirm_button_row(
    language: Language,
    font: Font,
    message: Message,
) -> Row<'static, Message, StyleType> {
    Row::new()
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .push(
            button(
                yes_translation(language)
                    .font(font)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center),
            )
            .padding(5)
            .height(40)
            .width(80)
            .class(ButtonType::Alert)
            .on_press(message),
        )
}

pub fn new_modal<'a, Message, Theme, Renderer>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    on_blur: Message,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
