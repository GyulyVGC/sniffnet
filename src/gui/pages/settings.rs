use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::components::tabs::get_tabs;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{
    get_font, COURIER_PRIME_BOLD, DEEP_SEA, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, MON_AMOUR,
    YETI_DAY, YETI_NIGHT,
};
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Sniffer, StyleType};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Image, Row, Text};
use iced::{Alignment, Length};
use iced_native::image::Handle;
use iced_native::widget::{horizontal_space, vertical_space};

pub fn settings_appearance_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(
            Container::new(
                Text::new("Settings")
                    .font(COURIER_PRIME_BOLD)
                    .size(FONT_SIZE_TITLE)
                    .horizontal_alignment(Horizontal::Center),
            )
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .height(Length::Units(40))
            .width(Length::Fill)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(sniffer.style, ElementType::Headers),
            )),
        )
        .push(get_tabs(
            &["Notifications", "Appearance", "Language"],
            &["A ", "A ", "A "],
            &[Message::TickInit, Message::TickInit, Message::TickInit],
            "Appearance",
            sniffer.style,
        ))
        .push(vertical_space(Length::Units(10)))
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
                    Message::Style(Night),
                ))
                .push(horizontal_space(Length::Units(40)))
                .push(get_palette_container(
                    sniffer.style,
                    YETI_DAY,
                    "Yeti Day".to_string(),
                    "Sniffnet's original light theme".to_string(),
                    Message::Style(Day),
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
                    Message::Style(DeepSea),
                ))
                .push(horizontal_space(Length::Units(40)))
                .push(get_palette_container(
                    sniffer.style,
                    MON_AMOUR,
                    "Mon Amour".to_string(),
                    "Lovely theme made for dreamers".to_string(),
                    Message::Style(MonAmour),
                )),
        );

    Container::new(content)
        .height(Length::Units(400))
        .width(Length::Units(800))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::BorderedRound),
        ))
}

fn get_palette_container(
    style: StyleType,
    picture: &[u8],
    name: String,
    description: String,
    on_press: Message,
) -> Button<'static, Message> {
    let font = get_font(style);
    let content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(Image::new(Handle::from_memory(Vec::from(picture))).width(Length::Units(300)))
        .push(Text::new(description).font(font));

    Button::new(
        Container::new(content)
            .align_x(Horizontal::Center)
            .padding(10)
            .height(Length::Units(130))
            .width(Length::Units(350))
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(style, ElementType::BorderedRound),
            )),
    )
    .padding(0)
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(on_press)
}
