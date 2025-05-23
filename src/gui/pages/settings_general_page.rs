use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{
    Column, Container, PickList, Row, Rule, Scrollable, Slider, Space, Text, Tooltip, button, vertical_space,
};
use iced::{Alignment, Font, Length, alignment};

use crate::gui::components::button::{button_open_file, row_open_link_tooltip};
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::translations::translations::language_translation;
use crate::translations::translations_2::country_translation;
use crate::translations::translations_3::{
    learn_more_translation, mmdb_files_translation, params_not_editable_translation,
    zoom_translation,
};
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
    } = sniffer.configs.lock().unwrap().settings.clone();
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
        .push(
            Scrollable::new(column_all_general_setting(sniffer, font, font_headers, color_gradient))
            .height(Length::Fill)
            .width(Length::Fill)
        );

    Container::new(content)
        .height(400)
        .width(800)
        .class(ContainerType::Modal)
}

fn column_all_general_setting(sniffer: &Sniffer, font: Font, font_headers: Font, _color_gradient: GradientType) -> Column<Message, StyleType> {
    let ConfigSettings {
        language,
        scale_factor,
        mmdb_country,
        mmdb_asn,
        ..
    } = sniffer.configs.lock().unwrap().settings.clone();

    let current_blacklist_path = sniffer
        .configs
        .lock()
        .unwrap()
        .blacklist
        .blacklist_path
        .clone()
        .unwrap_or_default();

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

    column = column.push(mmdb_section_ui(
        is_editable,
        language,
        font,
        font_headers,
        &mmdb_country,
        &mmdb_asn,
    ));

    column = column.push(Rule::horizontal(25));

    column = column.push(ip_blacklist_section_ui(
        is_editable,
        language,
        font,
        font_headers,
        &current_blacklist_path,
    ));

    column = column.push(Space::with_height(20));

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
                    "The selected language is not\nfully updated to version 1.3",
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
            Text::new(learn_more_translation(language))
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .font(font),
        )
        .push(vertical_space())
        .push(
            Tooltip::new(
                button(
                    Icon::Book
                        .to_text()
                        .align_y(Alignment::Center)
                        .align_x(Alignment::Center)
                        .size(22)
                        .line_height(LineHeight::Relative(1.0)),
                )
                .on_press(Message::OpenWebPage(WebPage::Wiki))
                .padding(2)
                .height(40)
                .width(60),
                row_open_link_tooltip("Sniffnet Wiki", font),
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

fn mmdb_section_ui<'a>(
    is_editable: bool,
    language: Language,
    font: Font,
    font_headers: Font,
    country_path: &str,
    asn_path: &str,
) -> Column<'a, Message, StyleType> {
    Column::new()
        .spacing(10)
        .align_x(Alignment::Center)
        .width(Length::Fixed(600.0))
        .push(
            Text::new(mmdb_files_translation(language))
                .font(font_headers)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        )
        .push(row_open_link_tooltip(
            learn_more_translation(language),
            font,
        ))
        .push(Space::with_height(5))
        .push(
            Row::new()
                .align_y(Alignment::Center)
                .spacing(10)
                .padding([0, 20])
                .push(
                    Text::new(format!("{}:", country_translation(language)))
                        .font(font)
                        .width(Length::Fixed(80.0)),
                )
                .push(
                    Text::new(get_path_termination_string(country_path, 35))
                        .font(font)
                        .width(Length::Fill),
                )
                .push(
                    button_open_file(
                        country_path.to_string(),
                        FileInfo::Database,
                        language,
                        font,
                        is_editable,
                        Message::CustomCountryDb,
                    )
                )
        )
        .push(Space::with_height(5))
        .push(
            Row::new()
                .align_y(Alignment::Center)
                .spacing(10)
                .padding([0, 20])
                .push(
                    Text::new("ASN:")
                        .font(font)
                        .width(Length::Fixed(80.0)),
                )
                .push(
                    Text::new(get_path_termination_string(asn_path, 35))
                        .font(font)
                        .width(Length::Fill),
                )
                .push(
                    button_open_file(
                        asn_path.to_string(),
                        FileInfo::Database,
                        language,
                        font,
                        is_editable,
                        Message::CustomAsnDb,
                    )
                )
        )
}

fn ip_blacklist_section_ui<'a>(
    is_editable: bool,
    language: Language,
    font: Font,
    font_headers: Font,
    current_blacklist_path: &str,
) -> Column<'a, Message, StyleType> {
    Column::new()
        .spacing(10)
        .align_x(Alignment::Center)
        .width(Length::Fixed(600.0))
        .push(
            Text::new("IP Blacklist")
                .font(font_headers)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        )
        .push(Space::with_height(5))
        .push(
            Row::new()
                .align_y(Alignment::Center)
                .spacing(10)
                .padding([0, 20])
                .push(
                    Text::new("txt file:")
                        .font(font)
                        .width(Length::Fixed(100.0)),
                )
                .push(
                    Text::new(get_path_termination_string(current_blacklist_path, 35))
                        .font(font)
                        .width(Length::Fill),
                )
                .push(
                    button_open_file(
                        current_blacklist_path.to_string(),
                        FileInfo::IpBlacklist,
                        language,
                        font,
                        is_editable,
                        Message::BlacklistFileSelected,
                    )
                )
        )
}
