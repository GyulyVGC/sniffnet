use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::overlays::Overlays;
use crate::gui::components::radios::language_radios;
use crate::gui::components::tabs::get_settings_tabs;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{
    get_font, DEEP_SEA, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, INCONSOLATA_BOLD, MON_AMOUR, YETI_DAY,
    YETI_NIGHT,
};
use crate::utility::translations::{
    appearance_title_translation, deep_sea_translation, languages_title_translation,
    mon_amour_translation, notifications_title_translation, settings_translation,
    yeti_day_translation, yeti_night_translation,
};
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Language, Sniffer, StyleType};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, horizontal_space, image::Handle, vertical_space, Button, Column, Container, Image, Row,
    Text,
};
use iced::{Alignment, Length};

pub fn settings_notifications_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            &[
                Overlays::SettingsNotifications,
                Overlays::SettingsAppearance,
                Overlays::SettingsLanguage,
            ],
            &["7 ", "b ", "c "],
            &[
                Message::TickInit,
                Message::ShowModal(Overlays::SettingsAppearance),
                Message::ShowModal(Overlays::SettingsLanguage),
            ],
            Overlays::SettingsNotifications,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Length::Units(15)))
        .push(
            notifications_title_translation(sniffer.language)
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
        .push(get_settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            &[
                Overlays::SettingsNotifications,
                Overlays::SettingsAppearance,
                Overlays::SettingsLanguage,
            ],
            &["7 ", "b ", "c "],
            &[
                Message::ShowModal(Overlays::SettingsNotifications),
                Message::TickInit,
                Message::ShowModal(Overlays::SettingsLanguage),
            ],
            Overlays::SettingsAppearance,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Length::Units(15)))
        .push(
            appearance_title_translation(sniffer.language)
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
                    yeti_night_translation(sniffer.language).to_string(),
                    Night,
                ))
                .push(horizontal_space(Length::Units(33)))
                .push(get_palette_container(
                    sniffer.style,
                    YETI_DAY,
                    "Yeti Day".to_string(),
                    yeti_day_translation(sniffer.language).to_string(),
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
                    deep_sea_translation(sniffer.language).to_string(),
                    DeepSea,
                ))
                .push(horizontal_space(Length::Units(33)))
                .push(get_palette_container(
                    sniffer.style,
                    MON_AMOUR,
                    "Mon Amour".to_string(),
                    mon_amour_translation(sniffer.language).to_string(),
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

    let language_active = sniffer.language;
    let col_language_radio = language_radios(language_active, font, sniffer.style);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(get_settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            &[
                Overlays::SettingsNotifications,
                Overlays::SettingsAppearance,
                Overlays::SettingsLanguage,
            ],
            &["7 ", "b ", "c "],
            &[
                Message::ShowModal(Overlays::SettingsNotifications),
                Message::ShowModal(Overlays::SettingsAppearance),
                Message::TickInit,
            ],
            Overlays::SettingsLanguage,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Length::Units(15)))
        .push(
            languages_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Units(10)))
        .push(col_language_radio);

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

fn get_settings_header(style: StyleType, language: Language) -> Container<'static, Message> {
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                settings_translation(language)
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
