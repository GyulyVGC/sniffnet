use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{
    Column, Container, PickList, Row, Rule, Slider, Space, Text, Tooltip, button, vertical_space,
};
use iced::{Alignment, Font, Length, Padding};

use crate::gui::components::button::{button_open_file, row_open_link_tooltip};
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::mmdb::types::mmdb_reader::{MmdbReader, MmdbReaders};
use crate::translations::translations::language_translation;
use crate::translations::translations_2::country_translation;
use crate::translations::translations_3::{
    mmdb_files_translation, params_not_editable_translation, zoom_translation,
};
use crate::translations::translations_4::share_feedback_translation;
use crate::utils::formatted_strings::get_path_termination_string;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::utils::types::web_page::WebPage;
use crate::{ConfigSettings, Language, RunningPage, Sniffer, StyleType};

pub fn settings_general_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style,
        language,
        color_gradient,
        ..
    } = sniffer.configs.settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            color_gradient,
            language,
        ))
        .push(get_settings_tabs(SettingsPage::General, font, language))
        .push(Space::with_height(10))
        .push(column_all_general_setting(sniffer, font));

    Container::new(content)
        .height(400)
        .width(800)
        .class(ContainerType::Modal)
}

fn column_all_general_setting(sniffer: &Sniffer, font: Font) -> Column<Message, StyleType> {
    let ConfigSettings {
        language,
        scale_factor,
        mmdb_country,
        mmdb_asn,
        ..
    } = sniffer.configs.settings.clone();

    let is_editable = sniffer.running_page.eq(&RunningPage::Init);

    let mut column = Column::new()
        .align_x(Alignment::Center)
        .padding([5, 10])
        .push(row_language_scale_factor(language, font, scale_factor))
        .push(Rule::horizontal(25));

    if !is_editable {
        column = column
            .push(
                Container::new(Text::new(params_not_editable_translation(language)).font(font))
                    .padding(10.0)
                    .class(ContainerType::Badge),
            )
            .push(Space::with_height(10));
    }

    column = column.push(mmdb_settings(
        is_editable,
        language,
        font,
        &mmdb_country,
        &mmdb_asn,
        &sniffer.mmdb_readers,
    ));

    column
}

fn row_language_scale_factor<'a>(
    language: Language,
    font: Font,
    scale_factor: f64,
) -> Row<'a, Message, StyleType> {
    Row::new()
        .align_y(Alignment::Start)
        .height(100)
        .push(language_picklist(language, font))
        .push(Rule::vertical(25))
        .push(scale_factor_slider(language, font, scale_factor))
        .push(Rule::vertical(25))
        .push(need_help(language, font))
}

fn language_picklist<'a>(language: Language, font: Font) -> Container<'a, Message, StyleType> {
    let mut flag_row = Row::new()
        .align_y(Alignment::Center)
        .spacing(10)
        .push(language.get_flag());
    if !language.is_up_to_date() {
        flag_row = flag_row.push(
            Tooltip::new(
                button(
                    Text::new("!")
                        .class(TextType::Danger)
                        .font(font)
                        .align_y(Alignment::Center)
                        .align_x(Alignment::Center)
                        .size(15)
                        .line_height(LineHeight::Relative(1.0)),
                )
                .on_press(Message::OpenWebPage(WebPage::IssueLanguages))
                .padding(2)
                .height(20)
                .width(20)
                .class(ButtonType::Alert),
                row_open_link_tooltip(
                    "The selected language is not\nfully updated to version 1.4",
                    font,
                ),
                Position::FollowCursor,
            )
            .class(ContainerType::Tooltip),
        );
    }

    let content = Column::new()
        .align_x(Alignment::Center)
        .push(
            Text::new(language_translation(language))
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .font(font),
        )
        .push(vertical_space())
        .push(flag_row)
        .push(Space::with_height(10))
        .push(
            PickList::new(
                &Language::ALL[..],
                Some(language),
                Message::LanguageSelection,
            )
            .padding([2, 7])
            .font(font),
        )
        .push(vertical_space());

    Container::new(content)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}

fn scale_factor_slider<'a>(
    language: Language,
    font: Font,
    scale_factor: f64,
) -> Container<'a, Message, StyleType> {
    #[allow(clippy::cast_possible_truncation)]
    let slider_width = 130.0 / scale_factor as f32;
    let slider_val = scale_factor.log(3.0);
    Container::new(
        Column::new()
            .align_x(Alignment::Center)
            .push(
                Text::new(zoom_translation(language))
                    .class(TextType::Subtitle)
                    .size(FONT_SIZE_SUBTITLE)
                    .font(font),
            )
            .push(vertical_space())
            .push(Text::new(format!("{:.0}%", scale_factor * 100.0)).font(font))
            .push(Space::with_height(5))
            .push(
                Slider::new(-1.0..=1.0, slider_val, Message::ChangeScaleFactor)
                    .step(0.01)
                    .width(slider_width),
            )
            .push(vertical_space()),
    )
    .width(Length::Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
}

fn need_help<'a>(language: Language, font: Font) -> Container<'a, Message, StyleType> {
    let content = Column::new()
        .align_x(Alignment::Center)
        .push(
            Text::new(share_feedback_translation(language))
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .font(font),
        )
        .push(vertical_space())
        .push(
            Tooltip::new(
                button(
                    Icon::Feedback
                        .to_text()
                        .align_y(Alignment::Center)
                        .align_x(Alignment::Center)
                        .size(20)
                        .line_height(LineHeight::Relative(1.0)),
                )
                .on_press(Message::OpenWebPage(WebPage::Issues))
                .padding(Padding::new(2.0).top(5))
                .height(40)
                .width(60),
                row_open_link_tooltip("GitHub Issues", font),
                Position::Right,
            )
            .gap(5)
            .class(ContainerType::Tooltip),
        )
        .push(vertical_space());

    Container::new(content)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}

fn mmdb_settings<'a>(
    is_editable: bool,
    language: Language,
    font: Font,
    country_path: &str,
    asn_path: &str,
    mmdb_readers: &MmdbReaders,
) -> Column<'a, Message, StyleType> {
    Column::new()
        .spacing(5)
        .align_x(Alignment::Center)
        .push(
            Text::new(mmdb_files_translation(language))
                .font(font)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(mmdb_selection_row(
            is_editable,
            font,
            Message::CustomCountryDb,
            country_path,
            &mmdb_readers.country,
            country_translation(language),
            language,
        ))
        .push(mmdb_selection_row(
            is_editable,
            font,
            Message::CustomAsnDb,
            asn_path,
            &mmdb_readers.asn,
            "ASN",
            language,
        ))
}

fn mmdb_selection_row<'a>(
    is_editable: bool,
    font: Font,
    message: fn(String) -> Message,
    custom_path: &str,
    mmdb_reader: &MmdbReader,
    caption: &str,
    language: Language,
) -> Row<'a, Message, StyleType> {
    let is_error = if custom_path.is_empty() {
        false
    } else {
        match *mmdb_reader {
            MmdbReader::Default(_) | MmdbReader::Empty => true,
            MmdbReader::Custom(_) => false,
        }
    };

    Row::new()
        .align_y(Alignment::Center)
        .push(Text::new(format!("{caption}: ")).font(font))
        .push(
            Text::new(get_path_termination_string(custom_path, 25))
                .font(font)
                .class(if is_error {
                    TextType::Danger
                } else {
                    TextType::Standard
                }),
        )
        .push(if custom_path.is_empty() {
            button_open_file(
                custom_path.to_owned(),
                FileInfo::Database,
                language,
                font,
                is_editable,
                message,
            )
        } else {
            button_clear_mmdb(message, font, is_editable)
        })
}

fn button_clear_mmdb<'a>(
    message: fn(String) -> Message,
    font: Font,
    is_editable: bool,
) -> Tooltip<'a, Message, StyleType> {
    let mut button = button(
        Text::new("×")
            .font(font)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .size(15)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(2)
    .height(20)
    .width(20);

    if is_editable {
        button = button.on_press(message(String::new()));
    }

    Tooltip::new(button, "", Position::Right)
}
