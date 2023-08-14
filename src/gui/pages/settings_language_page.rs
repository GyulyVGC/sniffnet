use iced::widget::vertical_space;
use iced::widget::{Column, Container, Text};
use iced::Length::Fixed;
use iced::{Alignment, Length, Renderer};

use crate::gui::components::radio::language_radios;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::translations::translations::languages_title_translation;
use crate::{Language, Sniffer, StyleType};

pub fn settings_language_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let font = get_font(sniffer.style);
    let font_headers = get_font_headers(sniffer.style);

    let language_active = sniffer.language;
    let row_language_radio_1 = language_radios(language_active, &Language::ROW1, font);
    let row_language_radio_2 = language_radios(language_active, &Language::ROW2, font);
    let row_language_radio_3 = language_radios(language_active, &Language::ROW3, font);
    let row_language_radio_4 = language_radios(language_active, &Language::ROW4, font);
    let col_language_radio_all = Column::new()
        .spacing(10)
        .push(row_language_radio_1)
        .push(row_language_radio_2)
        .push(row_language_radio_3)
        .push(row_language_radio_4);

    let mut content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            sniffer.color_gradient,
            sniffer.language,
        ))
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
            font,
            font_headers,
            sniffer.language,
        ))
        .push(vertical_space(Fixed(15.0)))
        .push(
            languages_title_translation(sniffer.language)
                .style(TextType::Subtitle)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Fixed(40.0)))
        .push(col_language_radio_all);

    if [Language::EL, Language::PT].contains(&sniffer.language) {
        content = content.push(vertical_space(Fixed(40.0))).push(
            Container::new(
                Text::new("The selected language is not fully updated to version 1.2").font(font),
            )
            .padding(10.0)
            .style(ContainerType::Badge),
        );
    }

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(ContainerType::Modal)
}
