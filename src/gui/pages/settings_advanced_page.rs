use std::path::PathBuf;
use std::sync::Arc;

use iced::advanced::widget::Text;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{
    button, horizontal_space, vertical_space, Column, Container, Row, Slider, TextInput, Tooltip,
};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::styles::types::custom_palette::CustomPalette;
use crate::gui::types::message::Message;
use crate::mmdb::types::mmdb_reader::MmdbReader;
use crate::translations::translations_2::country_translation;
use crate::translations::translations_3::{
    advanced_settings_translation, custom_style_translation, file_path_translation,
    info_mmdb_paths_translation, mmdb_paths_translation, params_not_editable_translation,
    restore_defaults_translation, scale_factor_translation,
};
use crate::utils::formatted_strings::get_default_report_directory;
use crate::utils::types::icon::Icon;
use crate::{ConfigAdvancedSettings, Language, Sniffer, Status, StyleType};

pub fn settings_advanced_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let font = get_font(sniffer.style);
    let font_headers = get_font_headers(sniffer.style);

    let is_editable = sniffer.status_pair.0.lock().unwrap().eq(&Status::Init);

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
        .push(title_row(
            sniffer.language,
            font,
            &sniffer.advanced_settings,
        ))
        .push(vertical_space(Fixed(5.0)))
        .push(scale_factor_slider(
            sniffer.language,
            font,
            sniffer.advanced_settings.scale_factor,
        ))
        .push(custom_style_settings(
            sniffer.language,
            font,
            &sniffer.advanced_settings.style_path,
        ));

    if !is_editable {
        content = content.push(
            Container::new(Text::new(params_not_editable_translation(sniffer.language)).font(font))
                .padding(10.0)
                .style(ContainerType::Badge),
        );
    }

    content = content
        .push(report_path_setting(
            is_editable,
            sniffer.language,
            font,
            sniffer.advanced_settings.output_path.clone(),
        ))
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

fn title_row(
    language: Language,
    font: Font,
    advanced_settings: &ConfigAdvancedSettings,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Row::new().spacing(10).align_items(Alignment::Center).push(
        Text::new(advanced_settings_translation(language))
            .style(TextType::Title)
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    );

    if advanced_settings.ne(&ConfigAdvancedSettings::default()) {
        ret_val = ret_val.push(
            Tooltip::new(
                button(
                    Icon::Restore
                        .to_text()
                        .vertical_alignment(Vertical::Center)
                        .horizontal_alignment(Horizontal::Center)
                        .size(17),
                )
                .padding(2)
                .height(Fixed(25.0))
                .width(Fixed(25.0))
                .on_press(Message::RestoreDefaults),
                restore_defaults_translation(language),
                Position::Right,
            )
            .font(font)
            .style(ContainerType::Tooltip),
        );
    }

    ret_val
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
    .padding(5)
    .width(Length::FillPortion(1))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

fn report_path_setting(
    is_editable: bool,
    language: Language,
    font: Font,
    mut path: PathBuf,
) -> Row<'static, Message, Renderer<StyleType>> {
    let default_directory = &get_default_report_directory().to_string_lossy().to_string();
    path.pop();
    let custom_directory = &path.to_string_lossy().to_string();
    // to be updated.........!!!
    let is_error = !custom_directory.is_empty();

    let mut input = TextInput::new(default_directory, custom_directory)
        .padding([0, 5])
        .font(font)
        .width(Length::Fixed(200.0))
        .style(if is_error {
            TextInputType::Error
        } else {
            TextInputType::Standard
        });

    if is_editable {
        input = input.on_input(Message::CustomReportDirectory);
    }

    Row::new()
        .push(Text::new(format!("{}:", file_path_translation(language))).font(font))
        .push(horizontal_space(5))
        .push(input)
        .push(Text::new("/sniffnet.pcap").font(font))
}

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

fn custom_style_settings(
    language: Language,
    font: Font,
    custom_path: &PathBuf,
) -> Row<'static, Message, Renderer<StyleType>> {
    let path_str = &custom_path.to_string_lossy().to_string();

    let is_error = if path_str.is_empty() {
        false
    } else {
        CustomPalette::from_file(custom_path).is_err()
    };

    let input = TextInput::new("-", path_str)
        .on_input(Message::LoadStyle)
        .on_submit(Message::LoadStyle(path_str.clone()))
        .padding([0, 5])
        .font(font)
        .width(Length::Fixed(200.0))
        .style(if is_error {
            TextInputType::Error
        } else {
            TextInputType::Standard
        });

    Row::new()
        .spacing(5)
        .push(Text::new(format!("{}:", custom_style_translation(language))).font(font))
        .push(input)
}
