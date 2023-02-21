use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::my_overlay::MyOverlay;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{
    get_font, DEEP_SEA, FONT_SIZE_SUBTITLE, MON_AMOUR, YETI_DAY, YETI_NIGHT,
};
use crate::utility::translations::{
    appearance_title_translation, deep_sea_translation, mon_amour_translation,
    yeti_day_translation, yeti_night_translation,
};
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Sniffer, StyleType};
use iced::widget::{Button, Column, Container, Image, Row, Text};
use iced::{Alignment, Length};
use iced_native::image::Handle;
use iced_native::widget::{horizontal_space, vertical_space};

pub fn settings_style_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                MyOverlay::SettingsNotifications,
                MyOverlay::SettingsAppearance,
                MyOverlay::SettingsLanguage,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::ShowModal(MyOverlay::SettingsNotifications),
                Message::TickInit,
                Message::ShowModal(MyOverlay::SettingsLanguage),
            ],
            MyOverlay::SettingsAppearance,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(
            appearance_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    YETI_NIGHT,
                    "Yeti Night".to_string(),
                    yeti_night_translation(sniffer.language).to_string(),
                    Night,
                ))
                .push(horizontal_space(Length::Fixed(33.0)))
                .push(get_palette_container(
                    sniffer.style,
                    YETI_DAY,
                    "Yeti Day".to_string(),
                    yeti_day_translation(sniffer.language).to_string(),
                    Day,
                )),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    DEEP_SEA,
                    "Deep Sea".to_string(),
                    deep_sea_translation(sniffer.language).to_string(),
                    DeepSea,
                ))
                .push(horizontal_space(Length::Fixed(33.0)))
                .push(get_palette_container(
                    sniffer.style,
                    MON_AMOUR,
                    "Mon Amour".to_string(),
                    mon_amour_translation(sniffer.language).to_string(),
                    MonAmour,
                )),
        );

    Container::new(content)
        .height(Length::Fixed(400.0))
        .width(Length::Fixed(800.0))
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
        .push(Image::new(Handle::from_memory(Vec::from(picture))).width(Length::Fixed(300.0)))
        .push(Text::new(description).font(font));

    Button::new(content)
        .height(Length::Fixed(130.0))
        .width(Length::Fixed(360.0))
        .padding(10)
        .style(StyleTuple(style, ElementType::BorderedRound).into())
        .on_press(Message::Style(on_press))
}
