use std::sync::Arc;

use iced::advanced::widget::Text;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{
    button, horizontal_space, vertical_space, Column, Container, PickList, Row, Rule, Slider,
    TextInput, Tooltip,
};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::types::message::Message;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::translations::translations::language_translation;
use crate::translations::translations_2::country_translation;
use crate::translations::translations_3::{
    mmdb_files_translation, params_not_editable_translation, zoom_translation,
};
use crate::utils::types::web_page::WebPage;
use crate::{Language, RunningPage, Sniffer, StyleType};

pub fn settings_general_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let color_gradient = sniffer.settings.color_gradient;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            color_gradient,
            language,
        ))
        .push(get_settings_tabs(SettingsPage::General, font, language))
        .push(vertical_space(Fixed(10.0)))
        .push(column_all_general_setting(sniffer, font));

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(ContainerType::Modal)
}

fn column_all_general_setting(
    sniffer: &Sniffer,
    font: Font,
) -> Column<'static, Message, Renderer<StyleType>> {
    let language = sniffer.settings.language;

    let is_editable = sniffer.running_page.eq(&RunningPage::Init);

    let mut column = Column::new()
        .align_items(Alignment::Center)
        .padding([5, 10])
        .push(row_language_scale_factor(
            language,
            font,
            sniffer.settings.scale_factor,
        ))
        .push(Rule::horizontal(25));

    if !is_editable {
        column = column
            .push(
                Container::new(Text::new(params_not_editable_translation(language)).font(font))
                    .padding(10.0)
                    .style(ContainerType::Badge),
            )
            .push(vertical_space(Fixed(10.0)));
    }

    column = column.push(mmdb_settings(
        is_editable,
        language,
        font,
        &sniffer.settings.mmdb_country,
        &sniffer.settings.mmdb_asn,
        &sniffer.country_mmdb_reader,
        &sniffer.asn_mmdb_reader,
    ));

    column
}

fn row_language_scale_factor(
    language: Language,
    font: Font,
    scale_factor: f64,
) -> Row<'static, Message, Renderer<StyleType>> {
    Row::new()
        .align_items(Alignment::Start)
        .height(Length::Fixed(90.0))
        .push(language_picklist(language, font))
        .push(Rule::vertical(25))
        .push(scale_factor_slider(language, font, scale_factor))
        .push(Rule::vertical(25))
        .push(horizontal_space(Length::FillPortion(1)))
}

fn language_picklist(
    language: Language,
    font: Font,
) -> Container<'static, Message, Renderer<StyleType>> {
    let mut flag_row = Row::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .push(language.get_flag());
    if ![Language::EN, Language::IT].contains(&language) {
        flag_row = flag_row.push(
            Tooltip::new(
                button(
                    Text::new("!")
                        .font(font)
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center)
                        .size(15),
                )
                .on_press(Message::OpenWebPage(WebPage::IssueLanguages))
                .padding(2)
                .height(Fixed(20.0))
                .width(Fixed(20.0)),
                "The selected language is not\nfully updated to version 1.3 ↗",
                Position::FollowCursor,
            )
            .font(font)
            .style(ContainerType::Tooltip),
        );
    }

    let content = Column::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .push(
            Text::new(language_translation(language))
                .style(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .font(font),
        )
        .push(flag_row)
        .push(
            PickList::new(
                &Language::ALL[..],
                Some(language),
                Message::LanguageSelection,
            )
            .padding([3, 7])
            .font(font),
        );

    Container::new(content)
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn scale_factor_slider(
    language: Language,
    font: Font,
    scale_factor: f64,
) -> Container<'static, Message, Renderer<StyleType>> {
    #[allow(clippy::cast_possible_truncation)]
    let slider_width = 150.0 / scale_factor as f32;
    Container::new(
        Column::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(
                Text::new(zoom_translation(language))
                    .style(TextType::Subtitle)
                    .size(FONT_SIZE_SUBTITLE)
                    .font(font),
            )
            .push(Text::new(format!("x{scale_factor:.2}")).font(font))
            .push(
                Slider::new(0.5..=1.5, scale_factor, Message::ChangeScaleFactor)
                    .step(0.05)
                    .width(Fixed(slider_width)),
            ),
    )
    .width(Length::FillPortion(1))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

// fn report_path_setting(
//     is_editable: bool,
//     language: Language,
//     font: Font,
//     custom_path: &str,
// ) -> Row<'static, Message, Renderer<StyleType>> {
//     let mut input = TextInput::new(&get_default_report_file_path(), custom_path)
//         .padding([0, 5])
//         .font(font)
//         .width(Length::Fixed(500.0))
//         .style(TextInputType::Standard);
//
//     if is_editable {
//         input = input.on_input(Message::CustomReport);
//     }
//
//     Row::new()
//         .push(Text::new(format!("{}:", file_path_translation(language))).font(font))
//         .push(horizontal_space(5))
//         .push(input)
// }

fn mmdb_settings(
    is_editable: bool,
    language: Language,
    font: Font,
    country_path: &str,
    asn_path: &str,
    country_reader: &Arc<MmdbReader>,
    asn_reader: &Arc<MmdbReader>,
) -> Column<'static, Message, Renderer<StyleType>> {
    Column::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .push(
            Text::new(mmdb_files_translation(language))
                .font(font)
                .style(TextType::Subtitle),
        )
        .push(
            Row::new()
                .spacing(20)
                .push(mmdb_input(
                    is_editable,
                    font,
                    Message::CustomCountryDb,
                    country_path,
                    country_reader,
                    country_translation(language),
                ))
                .push(mmdb_input(
                    is_editable,
                    font,
                    Message::CustomAsnDb,
                    asn_path,
                    asn_reader,
                    "ASN",
                )),
        )
}

fn mmdb_input(
    is_editable: bool,
    font: Font,
    message: fn(String) -> Message,
    custom_path: &str,
    mmdb_reader: &Arc<MmdbReader>,
    caption: &str,
) -> Row<'static, Message, Renderer<StyleType>> {
    let is_error = if custom_path.is_empty() {
        false
    } else {
        match **mmdb_reader {
            MmdbReader::Default(_) => true,
            MmdbReader::Custom(_) => false,
        }
    };

    let mut input = TextInput::new("-", custom_path)
        .padding([0, 5])
        .font(font)
        .width(Length::Fixed(200.0))
        .style(if is_error {
            TextInputType::Error
        } else {
            TextInputType::Standard
        });

    if is_editable {
        input = input.on_input(message);
    }

    Row::new()
        .spacing(5)
        .push(Text::new(format!("{caption}:")).font(font))
        .push(input)
}
