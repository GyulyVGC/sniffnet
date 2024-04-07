use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::{button, lazy, Rule, Space};
use iced::widget::{Button, Column, Container, Row, Scrollable, Text};
use iced::{Alignment, Color, Element, Font, Length};

use crate::gui::components::button::button_open_file;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{BORDER_WIDTH, FONT_SIZE_SUBTITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::custom_palette::ExtraStyles;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::palette::Palette;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    appearance_title_translation, deep_sea_translation, mon_amour_translation,
    yeti_day_translation, yeti_night_translation,
};
use crate::translations::translations_2::color_gradients_translation;
use crate::translations::translations_3::custom_style_translation;
use crate::utils::formatted_strings::get_path_termination_string;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{ConfigSettings, Language, Sniffer, StyleType};

pub fn settings_style_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style,
        language,
        color_gradient,
        style_path,
        ..
    } = sniffer.configs.lock().unwrap().settings.clone();
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let mut content = Column::new()
        .padding([0, 0, 5, 0])
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            color_gradient,
            language,
        ))
        .push(get_settings_tabs(SettingsPage::Appearance, font, language))
        .push(Space::with_height(15))
        .push(
            appearance_title_translation(language)
                .style(TextType::Subtitle)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(Space::with_height(15))
        .push(gradients_row(font, color_gradient, language))
        .push(Space::with_height(15));

    let mut styles_col = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(
            Row::new()
                .push(get_palette_container(
                    style,
                    "Yeti Night".to_string(),
                    yeti_night_translation(language).to_string(),
                    Night,
                ))
                .push(Space::with_width(10))
                .push(get_palette_container(
                    style,
                    "Yeti Day".to_string(),
                    yeti_day_translation(language).to_string(),
                    Day,
                )),
        )
        .push(Space::with_height(10))
        .push(
            Row::new()
                .push(get_palette_container(
                    style,
                    "Deep Sea".to_string(),
                    deep_sea_translation(language).to_string(),
                    DeepSea,
                ))
                .push(Space::with_width(10))
                .push(get_palette_container(
                    style,
                    "Mon Amour".to_string(),
                    mon_amour_translation(language).to_string(),
                    MonAmour,
                )),
        )
        .push(Space::with_height(10));
    for children in get_extra_palettes(ExtraStyles::all_styles(), style) {
        styles_col = styles_col.push(children);
    }
    styles_col = styles_col
        .push(lazy((style_path.clone(), style), move |_| {
            lazy_custom_style_input(language, font, &style_path, style)
        }))
        .push(Space::with_height(10));

    let styles_scroll =
        Scrollable::new(styles_col).direction(Direction::Vertical(ScrollbarType::properties()));

    content = content.push(styles_scroll);

    Container::new(content)
        .height(400)
        .width(800)
        .style(ContainerType::Modal)
}

fn gradients_row(
    font: Font,
    color_gradient: GradientType,
    language: Language,
) -> Row<'static, Message, StyleType> {
    Row::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .push(Text::new(format!("{}:", color_gradients_translation(language))).font(font))
        .push(
            button(
                Icon::Forbidden
                    .to_text()
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(12),
            )
            .padding(0)
            .height(20.0)
            .width(if color_gradient.eq(&GradientType::None) {
                60
            } else {
                20
            })
            .on_press(Message::GradientsSelection(GradientType::None)),
        )
        .push(
            button(
                Icon::Waves
                    .to_text()
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(13),
            )
            .padding(0)
            .height(20.0)
            .width(if color_gradient.eq(&GradientType::Mild) {
                60
            } else {
                20
            })
            .on_press(Message::GradientsSelection(GradientType::Mild)),
        )
        .push(
            button(
                Icon::Lightning
                    .to_text()
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(13),
            )
            .padding(0)
            .height(20.0)
            .width(if color_gradient.eq(&GradientType::Wild) {
                60
            } else {
                20
            })
            .on_press(Message::GradientsSelection(GradientType::Wild)),
        )
}

fn get_palette_container(
    style: StyleType,
    name: String,
    description: String,
    on_press: StyleType,
) -> Button<'static, Message, StyleType> {
    let font = style.get_extension().font;

    let is_custom = matches!(on_press, StyleType::Custom(_));

    let mut content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(get_palette_rule(
            on_press.get_palette(),
            on_press.get_extension().buttons_color,
            is_custom,
        ));

    if !is_custom {
        content = content.push(Text::new(description).font(font));
    }

    Button::new(content)
        .height(if is_custom { 75 } else { 110 })
        .width(380)
        .padding(5)
        .style(if on_press.eq(&style) {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::Style(on_press))
}

fn get_palette_rule(
    palette: Palette,
    buttons_color: Color,
    is_custom: bool,
) -> Container<'static, Message, StyleType> {
    let height = if is_custom { 25 } else { 40 };

    Container::new(
        Row::new()
            .push(Row::new().width(120).push(
                Rule::horizontal(height).style(RuleType::PaletteColor(palette.primary, height)),
            ))
            .push(Row::new().width(80).push(
                Rule::horizontal(height).style(RuleType::PaletteColor(palette.secondary, height)),
            ))
            .push(Row::new().width(60).push(
                Rule::horizontal(height).style(RuleType::PaletteColor(palette.outgoing, height)),
            ))
            .push(Row::new().width(40).push(
                Rule::horizontal(height).style(RuleType::PaletteColor(buttons_color, height)),
            )),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .width(300.0 + 2.0 * BORDER_WIDTH)
    .height(f32::from(height) + 1.7 * BORDER_WIDTH)
    .style(ContainerType::Palette)
}

// Buttons for each extra style arranged in rows of two
fn get_extra_palettes(
    styles: &[ExtraStyles],
    current_style: StyleType,
) -> Vec<Element<'static, Message, StyleType>> {
    // Map each extra style into a palette container
    let mut styles = styles.iter().map(|&style| {
        let name = style.to_string();
        let description = String::new();
        let style = StyleType::Custom(style);
        get_palette_container(current_style, name, description, style)
    });

    // The best way to do this would be with itertools, but that would introduce another dependency.
    let mut children = Vec::with_capacity(styles.len());

    // This handles the case where there aren't an even number of styles.
    // [Iterator::zip] drops remainders. Itertools' `zip_longest` and the unstable array chunks API
    // are both better solutions.
    while let (Some(first), second) = (styles.next(), styles.next()) {
        // Add both styles and the vertical space if there are two styles.
        if let Some(second) = second {
            children.extend([
                Row::new()
                    .push(first)
                    .push(Space::with_width(10))
                    .push(second)
                    .into(),
                <Space as Into<Element<Message, StyleType>>>::into(Space::with_height(10)),
            ]);
        } else {
            children.extend([
                Row::new().push(first).into(),
                <Space as Into<Element<Message, StyleType>>>::into(Space::with_height(10)),
            ]);
        }
    }

    children
}

fn lazy_custom_style_input(
    language: Language,
    font: Font,
    custom_path: &str,
    style: StyleType,
) -> Button<'static, Message, StyleType> {
    let is_custom_toml_style_set = matches!(style, StyleType::Custom(ExtraStyles::CustomToml(_)));

    let custom_palette = Palette::from_file(custom_path);
    let is_error = if custom_path.is_empty() {
        false
    } else {
        custom_palette.is_err()
    };

    let button_row = Row::new()
        .align_items(Alignment::Center)
        .push(
            Text::new(get_path_termination_string(custom_path, 17))
                .font(font)
                .style(if is_error {
                    TextType::Danger
                } else {
                    TextType::Standard
                }),
        )
        .push(button_open_file(
            custom_path.to_owned(),
            FileInfo::Style,
            language,
            font,
            true,
            Message::LoadStyle,
        ));

    let mut content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(custom_style_translation(language)).font(font))
        .push(button_row);

    if is_custom_toml_style_set {
        content = content.push(get_palette_rule(
            style.get_palette(),
            style.get_extension().buttons_color,
            true,
        ));
    } else if let Ok(palette) = custom_palette {
        content = content.push(get_palette_rule(
            palette,
            palette.generate_buttons_color(),
            true,
        ));
    }

    Button::new(content)
        .height(if custom_palette.is_ok() || is_custom_toml_style_set {
            110
        } else {
            75
        })
        .width(380)
        .padding([10, 0, 5, 0])
        .style(if is_custom_toml_style_set {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::LoadStyle(custom_path.to_string()))
}
