use std::sync::Arc;

use iced::advanced::widget::Text;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{button, vertical_space, Column, Container, Row, Slider, TextInput, Tooltip};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::types::message::Message;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::translations::translations_2::country_translation;
use crate::translations::translations_3::{
    advanced_settings_translation, info_mmdb_paths_translation, mmdb_paths_translation,
    params_not_editable_translation, scale_factor_translation,
};
use crate::{Language, RunningPage, Sniffer, StyleType};

pub fn settings_advanced_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let font = get_font(sniffer.style);
    let font_headers = get_font_headers(sniffer.style);

    let is_editable = sniffer.running_page.eq(&RunningPage::Init);

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
            SettingsPage::Advanced,
            font,
            sniffer.language,
        ))
        .push(vertical_space(Fixed(15.0)))
        .push(title_row(sniffer.language, font))
        .push(vertical_space(Fixed(10.0)))
        .push(scale_factor_slider(
            sniffer.language,
            font,
            sniffer.advanced_settings.scale_factor,
        ))
        .push(vertical_space(Fixed(10.0)));

    if !is_editable {
        content = content
            .push(
                Container::new(
                    Text::new(params_not_editable_translation(sniffer.language)).font(font),
                )
                .padding(10.0)
                .style(ContainerType::Badge),
            )
            .push(vertical_space(Fixed(10.0)));
    }

    content = content
        // .push(report_path_setting(
        //     is_editable,
        //     sniffer.language,
        //     font,
        //     &sniffer.advanced_settings.output_path,
        // ))
        .push(mmdb_settings(
            is_editable,
            sniffer.language,
            font,
            &sniffer.advanced_settings.mmdb_country,
            &sniffer.advanced_settings.mmdb_asn,
            &sniffer.country_mmdb_reader,
            &sniffer.asn_mmdb_reader,
        ));

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(ContainerType::Modal)
}

fn title_row(language: Language, font: Font) -> Row<'static, Message, Renderer<StyleType>> {
    Row::new().spacing(10).align_items(Alignment::Center).push(
        Text::new(advanced_settings_translation(language))
            .style(TextType::Title)
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    )
}

fn scale_factor_slider(
    language: Language,
    font: Font,
    scale_factor: f64,
) -> Container<'static, Message, Renderer<StyleType>> {
    Container::new(
        Column::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(
                Text::new(format!(
                    "{}: x{scale_factor:.2}",
                    scale_factor_translation(language)
                ))
                .font(font),
            )
            .push(
                Slider::new(0.5..=1.5, scale_factor, Message::ChangeScaleFactor)
                    .step(0.05)
                    .width(Fixed(150.0)),
            ),
    )
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
            Row::new()
                .spacing(10)
                .push(
                    Text::new(mmdb_paths_translation(language))
                        .font(font)
                        .style(TextType::Subtitle),
                )
                .push(
                    Tooltip::new(
                        button(
                            Text::new("i")
                                .font(font)
                                .vertical_alignment(Vertical::Center)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Fixed(20.0))
                        .width(Fixed(20.0)),
                        info_mmdb_paths_translation(language),
                        Position::Top,
                    )
                    .font(font)
                    .style(ContainerType::Tooltip),
                ),
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
