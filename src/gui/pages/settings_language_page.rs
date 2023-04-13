use iced::alignment::Horizontal;
use iced::widget::{Column, Container, Row, Text};
use iced::Length::Fixed;
use iced::{Alignment, Length};
use iced_native::widget::vertical_space;

use crate::gui::components::radio::language_radios;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::style_constants::{get_font, FONT_SIZE_SUBTITLE};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::translations::translations::languages_title_translation;
use crate::{Language, Sniffer};

pub fn settings_language_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let language_active = sniffer.language;
    let col_language_radio_1 =
        language_radios(language_active, &Language::COL1, font, sniffer.style);
    let col_language_radio_2 =
        language_radios(language_active, &Language::COL2, font, sniffer.style);
    let col_language_radio_3 =
        language_radios(language_active, &Language::COL3, font, sniffer.style);
    let row_language_radio = Row::new()
        .spacing(50)
        .push(col_language_radio_1)
        .push(col_language_radio_2)
        .push(col_language_radio_3);

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                SettingsPage::Notifications,
                SettingsPage::Appearance,
                SettingsPage::Language,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::OpenSettings(SettingsPage::Notifications),
                Message::OpenSettings(SettingsPage::Appearance),
                Message::TickInit,
            ],
            SettingsPage::Language,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Fixed(15.0)))
        .push(
            languages_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Fixed(20.0)))
        .push(row_language_radio)
        .push(vertical_space(Fixed(30.0)))
        .push(
            Container::new(
                Text::new(
                    "Would you like to add support for your native language?\n\n\
        Give a look at Sniffnet issues on GitHub.",
                )
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fixed(500.0))
                .font(font),
            )
            .padding(10)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(sniffer.style, ElementType::BorderedRound),
            )),
        );

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
