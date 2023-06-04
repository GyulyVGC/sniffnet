use iced::widget::{Column, Container, Text};
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
    let row_language_radio_1 =
        language_radios(language_active, &Language::ROW1, font, sniffer.style);
    let row_language_radio_2 =
        language_radios(language_active, &Language::ROW2, font, sniffer.style);
    let row_language_radio_3 =
        language_radios(language_active, &Language::ROW3, font, sniffer.style);
    let row_language_radio_4 =
        language_radios(language_active, &Language::ROW4, font, sniffer.style);
    let col_language_radio_all = Column::new()
        .spacing(10)
        .push(row_language_radio_1)
        .push(row_language_radio_2)
        .push(row_language_radio_3)
        .push(row_language_radio_4);

    let mut content = Column::new()
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
        .push(vertical_space(Fixed(40.0)))
        .push(col_language_radio_all);

    if [Language::EL, Language::FR, Language::PT].contains(&sniffer.language) {
        content = content.push(vertical_space(Fixed(40.0))).push(
            Container::new(
                Text::new("The selected language is not fully updated to version 1.2").font(font),
            )
            .padding(10.0)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(sniffer.style, ElementType::Badge),
            )),
        );
    }

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
