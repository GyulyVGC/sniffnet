use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::components::tabs::get_tabs;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{
    get_font, DEEP_SEA, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, INCONSOLATA_BOLD, MON_AMOUR, YETI_DAY,
    YETI_NIGHT,
};
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Sniffer, StyleType};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Image, Row, Text};
use iced::{Alignment, Length};
use iced_native::image::Handle;
use iced_native::widget::{button, horizontal_space, vertical_space};

pub fn settings_notifications_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style))
        .push(get_tabs(
            &["Notifications", "Appearance", "Language"],
            &["7 ", "b ", "c "],
            &[
                Message::TickInit,
                Message::ShowModal("settings_appearance"),
                Message::ShowModal("settings_language"),
            ],
            "Notifications",
            sniffer.style,
        ))
        .push(vertical_space(Length::Units(15)))
        .push(
            Text::new("Customize your notifications")
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Units(10)));

    Container::new(content)
        .height(Length::Units(400))
        .width(Length::Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

pub fn settings_appearance_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style))
        .push(get_tabs(
            &["Notifications", "Appearance", "Language"],
            &["7 ", "b ", "c "],
            &[
                Message::ShowModal("settings_notifications"),
                Message::TickInit,
                Message::ShowModal("settings_language"),
            ],
            "Appearance",
            sniffer.style,
        ))
        .push(vertical_space(Length::Units(15)))
        .push(
            Text::new("Choose your favorite theme")
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Units(10)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    YETI_NIGHT,
                    "Yeti Night".to_string(),
                    "Sniffnet's original dark theme".to_string(),
                    Night,
                ))
                .push(horizontal_space(Length::Units(33)))
                .push(get_palette_container(
                    sniffer.style,
                    YETI_DAY,
                    "Yeti Day".to_string(),
                    "Sniffnet's original light theme".to_string(),
                    Day,
                )),
        )
        .push(vertical_space(Length::Units(10)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    DEEP_SEA,
                    "Deep Sea".to_string(),
                    "To dive into network traffic".to_string(),
                    DeepSea,
                ))
                .push(horizontal_space(Length::Units(33)))
                .push(get_palette_container(
                    sniffer.style,
                    MON_AMOUR,
                    "Mon Amour".to_string(),
                    "Lovely theme made for dreamers".to_string(),
                    MonAmour,
                )),
        );

    Container::new(content)
        .height(Length::Units(400))
        .width(Length::Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

pub fn settings_language_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style))
        .push(get_tabs(
            &["Notifications", "Appearance", "Language"],
            &["7 ", "b ", "c "],
            &[
                Message::ShowModal("settings_notifications"),
                Message::ShowModal("settings_appearance"),
                Message::TickInit,
            ],
            "Language",
            sniffer.style,
        ))
        .push(vertical_space(Length::Units(15)))
        .push(
            Text::new("Select your language")
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Units(10)));

    Container::new(content)
        .height(Length::Units(400))
        .width(Length::Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn get_palette_container(
    style: StyleType,
    picture: &[u8],
    name: String,
    description: String,
    on_press: StyleType,
) -> Button<'static, Message> {
    let font = get_font(style);
    let content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(Image::new(Handle::from_memory(Vec::from(picture))).width(Length::Units(300)))
        .push(Text::new(description).font(font));

    Button::new(content)
        .height(Length::Units(130))
        .width(Length::Units(350))
        .padding(10)
        .style(StyleTuple(style, ElementType::BorderedRound).into())
        .on_press(Message::Style(on_press))
}

fn get_settings_header(style: StyleType) -> Container<'static, Message> {
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new("Settings")
                    .font(INCONSOLATA_BOLD)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    button(
                        Text::new("x")
                            .font(INCONSOLATA_BOLD)
                            .horizontal_alignment(Horizontal::Center)
                            .size(15),
                    )
                    .padding(2)
                    .height(Length::Units(20))
                    .width(Length::Units(20))
                    .style(StyleTuple(style, ElementType::Standard).into())
                    .on_press(Message::HideModal(true)),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Length::Units(40))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}
