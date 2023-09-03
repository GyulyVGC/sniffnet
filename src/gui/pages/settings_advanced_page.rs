use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::types::message::Message;
use crate::translations::translations_2::country_translation;
use crate::translations::translations_3::{
    advanced_settings_translation, info_mmdb_paths_translation, mmdb_paths_translation,
    params_not_editable_translation, restore_defaults_translation, scale_factor_translation,
};
use crate::utils::types::icon::Icon;
use crate::{ConfigAdvancedSettings, Language, Sniffer, Status, StyleType};
use iced::advanced::widget::Text;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{button, vertical_space, Column, Container, Row, Slider, TextInput, Tooltip};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

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
        ));

    if !is_editable {
        content = content.push(
            Container::new(Text::new(params_not_editable_translation(sniffer.language)).font(font))
                .padding(10.0)
                .style(ContainerType::Badge),
        );
    }

    content = content.push(mmdb_settings(
        is_editable,
        sniffer.language,
        font,
        &sniffer.advanced_settings.mmdb_country,
        &sniffer.advanced_settings.mmdb_asn,
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

fn mmdb_settings(
    is_editable: bool,
    language: Language,
    font: Font,
    country_path: &str,
    asn_path: &str,
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
                    country_translation(language),
                ))
                .push(mmdb_input(
                    is_editable,
                    font,
                    Message::CustomAsnDb,
                    asn_path,
                    "ASN",
                )),
        )
}

fn mmdb_input(
    is_editable: bool,
    font: Font,
    message: fn(String) -> Message,
    custom_path: &str,
    caption: &str,
) -> Row<'static, Message, Renderer<StyleType>> {
    let is_error = if custom_path.is_empty() {
        false
    } else {
        maxminddb::Reader::open_readfile(custom_path.clone()).is_err()
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
