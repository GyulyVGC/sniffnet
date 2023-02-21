use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::enums::my_overlay::MyOverlay;
use crate::gui::components::radio::language_radios;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{get_font, FONT_SIZE_SUBTITLE};
use crate::utility::translations::languages_title_translation;
use crate::{Language, Sniffer};
use iced::widget::{Column, Container, Row, Text};
use iced::Length::Fixed;
use iced::{Alignment, Length};
use iced_native::widget::vertical_space;

pub fn settings_language_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let language_active = sniffer.language;
    let col_language_radio_1 =
        language_radios(language_active, &Language::COL1, font, sniffer.style);
    let col_language_radio_2 =
        language_radios(language_active, &Language::COL2, font, sniffer.style);
    let row_language_radio = Row::new()
        .spacing(50)
        .push(col_language_radio_1)
        .push(col_language_radio_2);

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
                Message::ShowModal(MyOverlay::SettingsAppearance),
                Message::TickInit,
            ],
            MyOverlay::SettingsLanguage,
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
        .push(Container::new(Text::new("Support for more languages will come with the next releases.\n\n\
        If you want to help me translating the app in your native language, give a look at Sniffnet issues on GitHub.")
            .width(Length::Fixed(600.0))
            .font(font)).padding(10).style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::BorderedRound),
        )));

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
