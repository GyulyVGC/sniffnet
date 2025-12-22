use iced::widget::scrollable::Direction;
use iced::widget::{Button, Column, Container, Row, Scrollable, Text};
use iced::widget::{Space, button, lazy};
use iced::{Alignment, Color, Element, Length, Padding};

use crate::StyleType::{Day, DeepSea, MonAmour, Night};
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
use crate::gui::styles::types::palette_extension::PaletteExtension;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::translations::translations::appearance_title_translation;
use crate::translations::translations_2::color_gradients_translation;
use crate::translations::translations_3::custom_style_translation;
use crate::utils::formatted_strings::get_path_termination_string;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::icon::Icon;
use crate::{Language, Sniffer, StyleType};

pub fn settings_style_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings {
        style,
        language,
        color_gradient,
        style_path,
        ..
    } = sniffer.conf.settings.clone();

    let mut content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(color_gradient, language))
        .push(get_settings_tabs(SettingsPage::Appearance, language))
        .push(Space::new().height(15))
        .push(
            appearance_title_translation(language)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(Space::new().height(15))
        .push(gradients_row(color_gradient, language))
        .push(Space::new().height(15));

    let mut styles_col = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(
            Row::new()
                .push(get_palette_container(style, "Yeti".to_string(), Night))
                .push(Space::new().width(15))
                .push(get_palette_container(style, "Yeti".to_string(), Day)),
        )
        .push(Space::new().height(15))
        .push(
            Row::new()
                .push(get_palette_container(
                    style,
                    "Deep Sea".to_string(),
                    DeepSea,
                ))
                .push(Space::new().width(15))
                .push(get_palette_container(
                    style,
                    "Mon Amour".to_string(),
                    MonAmour,
                )),
        )
        .push(Space::new().height(15));
    for children in get_extra_palettes(ExtraStyles::all_styles(), style) {
        styles_col = styles_col.push(children);
    }
    styles_col = styles_col
        .push(lazy((language, style_path.clone(), style), move |_| {
            lazy_custom_style_input(language, &style_path, style)
        }))
        .push(Space::new().height(10));

    let styles_scroll = Scrollable::with_direction(
        styles_col,
        Direction::Vertical(ScrollbarType::properties().margin(10)),
    );

    content = content.push(styles_scroll);

    Container::new(content)
        .height(400)
        .width(800)
        .class(ContainerType::Modal)
}

fn gradients_row<'a>(
    color_gradient: GradientType,
    language: Language,
) -> Row<'a, Message, StyleType> {
    Row::new()
        .align_y(Alignment::Center)
        .spacing(10)
        .push(Text::new(format!(
            "{}:",
            color_gradients_translation(language)
        )))
        .push(
            button(
                Icon::Forbidden
                    .to_text()
                    .align_y(Alignment::Center)
                    .align_x(Alignment::Center)
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
                    .align_y(Alignment::Center)
                    .align_x(Alignment::Center)
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
                    .align_y(Alignment::Center)
                    .align_x(Alignment::Center)
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

fn get_palette_container<'a>(
    style: StyleType,
    name: String,
    on_press: StyleType,
) -> Button<'a, Message, StyleType> {
    let PaletteExtension {
        buttons_color,
        is_nightly,
        ..
    } = on_press.get_extension();

    let caption = Row::new()
        .spacing(7)
        .push(Text::new(name))
        .push(if is_nightly {
            Icon::Moon.to_text().size(15)
        } else {
            Icon::Sun.to_text()
        });

    let content = Column::new()
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(5)
        .push(caption)
        .push(get_palette_rule(on_press.get_palette(), buttons_color));

    Button::new(content)
        .height(80)
        .width(350)
        .padding(Padding::ZERO.top(10))
        .class(if on_press.eq(&style) {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::Style(on_press))
}

fn get_palette_rule<'a>(
    palette: Palette,
    buttons_color: Color,
) -> Container<'a, Message, StyleType> {
    let height = 25.0;

    Container::new(
        Row::new()
            .push(
                Row::new()
                    .width(120)
                    .push(RuleType::PaletteColor(palette.primary).horizontal(height)),
            )
            .push(
                Row::new()
                    .width(80)
                    .push(RuleType::PaletteColor(palette.secondary).horizontal(height)),
            )
            .push(
                Row::new()
                    .width(60)
                    .push(RuleType::PaletteColor(palette.outgoing).horizontal(height)),
            )
            .push(
                Row::new()
                    .width(40)
                    .push(RuleType::PaletteColor(buttons_color).horizontal(height)),
            ),
    )
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .width(300.0 + 2.0 * BORDER_WIDTH)
    .height(height + 1.7 * BORDER_WIDTH)
    .class(ContainerType::Palette)
}

// Buttons for each extra style arranged in rows of two
fn get_extra_palettes<'a>(
    styles: &[ExtraStyles],
    current_style: StyleType,
) -> Vec<Element<'a, Message, StyleType>> {
    // Map each extra style into a palette container
    let mut styles = styles.iter().map(|&style| {
        let name = style.to_string();
        let style = StyleType::Custom(style);
        get_palette_container(current_style, name, style)
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
                    .push(Space::new().width(15))
                    .push(second)
                    .into(),
                <Space as Into<Element<Message, StyleType>>>::into(Space::new().height(15)),
            ]);
        } else {
            children.extend([
                Row::new().push(first).into(),
                <Space as Into<Element<Message, StyleType>>>::into(Space::new().height(15)),
            ]);
        }
    }

    children
}

fn lazy_custom_style_input<'a>(
    language: Language,
    custom_path: &str,
    style: StyleType,
) -> Button<'a, Message, StyleType> {
    let is_custom_toml_style_set = matches!(style, StyleType::Custom(ExtraStyles::CustomToml(_)));

    let custom_palette = Palette::from_file(custom_path);
    let is_error = if custom_path.is_empty() {
        false
    } else {
        custom_palette.is_err()
    };

    let button_row = Row::new()
        .align_y(Alignment::Center)
        .push(
            Text::new(get_path_termination_string(custom_path, 17)).class(if is_error {
                TextType::Danger
            } else {
                TextType::Standard
            }),
        )
        .push(button_open_file(
            custom_path.to_owned(),
            FileInfo::Style,
            language,
            true,
            Message::LoadStyle,
        ));

    let mut content = Column::new()
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .spacing(5)
        .push(Text::new(custom_style_translation(language)))
        .push(button_row);

    if is_custom_toml_style_set {
        content = content.push(get_palette_rule(
            style.get_palette(),
            style.get_extension().buttons_color,
        ));
    } else if let Ok(palette) = custom_palette {
        content = content.push(get_palette_rule(palette, palette.generate_buttons_color()));
    }

    Button::new(content)
        .height(if custom_palette.is_ok() || is_custom_toml_style_set {
            110
        } else {
            75
        })
        .width(380)
        .padding(Padding::ZERO.top(10).bottom(5))
        .class(if is_custom_toml_style_set {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::LoadStyle(custom_path.to_string()))
}
