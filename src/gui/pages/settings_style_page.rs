use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::{button, horizontal_space, vertical_space, Rule};
use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text};
use iced::Length::Fixed;
use iced::{Alignment, Element, Length, Renderer};

use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::{ButtonStyleTuple, ButtonType};
use crate::gui::styles::container::{ContainerStyleTuple, ContainerType};
use crate::gui::styles::rule::{RuleStyleTuple, RuleType};
use crate::gui::styles::scrollbar::{ScrollbarStyleTuple, ScrollbarType};
use crate::gui::styles::style_constants::{get_font, BORDER_WIDTH, FONT_SIZE_SUBTITLE, ICONS};
use crate::gui::styles::text::{TextStyleTuple, TextType};
use crate::gui::styles::types::custom_palette::ExtraStyles;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::translations::translations::{
    appearance_title_translation, deep_sea_translation, mon_amour_translation,
    yeti_day_translation, yeti_night_translation,
};
use crate::translations::translations_2::color_gradients_translation;
use crate::StyleType::{Day, DeepSea, MonAmour, Night};
use crate::{Language, Sniffer, StyleType};

pub fn settings_style_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let font = get_font(sniffer.style);
    let mut content = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(
            sniffer.style,
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
                Message::TickInit,
                Message::OpenSettings(SettingsPage::Language),
            ],
            SettingsPage::Appearance,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(
            appearance_title_translation(sniffer.language)
                .style(TextType::Subtitle)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(vertical_space(Length::Fixed(15.0)))
        .push(gradients_row(
            sniffer.style,
            sniffer.color_gradient,
            sniffer.language,
        ))
        .push(vertical_space(Length::Fixed(15.0)));

    let mut styles_col = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    "Yeti Night".to_string(),
                    yeti_night_translation(sniffer.language).to_string(),
                    Night,
                ))
                .push(horizontal_space(Length::Fixed(15.0)))
                .push(get_palette_container(
                    sniffer.style,
                    "Yeti Day".to_string(),
                    yeti_day_translation(sniffer.language).to_string(),
                    Day,
                )),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Row::new()
                .push(get_palette_container(
                    sniffer.style,
                    "Deep Sea".to_string(),
                    deep_sea_translation(sniffer.language).to_string(),
                    DeepSea,
                ))
                .push(horizontal_space(Length::Fixed(15.0)))
                .push(get_palette_container(
                    sniffer.style,
                    "Mon Amour".to_string(),
                    mon_amour_translation(sniffer.language).to_string(),
                    MonAmour,
                )),
        )
        .push(vertical_space(Length::Fixed(10.0)));
    for children in get_extra_palettes(ExtraStyles::all_styles(), sniffer.style) {
        styles_col = styles_col.push(children);
    }

    let styles_scroll = Scrollable::new(styles_col)
        .direction(Direction::Vertical(ScrollbarType::properties()))
        .style(ScrollbarType::Standard);

    content = content.push(styles_scroll);

    Container::new(content)
        .height(Length::Fixed(400.0))
        .width(Length::Fixed(800.0))
        .style(ContainerType::Modal)
}

fn gradients_row(
    style: StyleType,
    color_gradient: GradientType,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    let font = get_font(style);
    Row::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .push(Text::new(format!("{}:", color_gradients_translation(language))).font(font))
        .push(
            button(
                Text::new("x")
                    .font(ICONS)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(12),
            )
            .padding(0)
            .height(20.0)
            .width(Fixed(if color_gradient.eq(&GradientType::None) {
                60.0
            } else {
                20.0
            }))
            .style(ButtonType::Gradient(GradientType::None))
            .on_press(Message::GradientsSelection(GradientType::None)),
        )
        .push(
            button(
                Text::new("y")
                    .font(ICONS)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(13),
            )
            .padding(0)
            .height(20.0)
            .width(Fixed(if color_gradient.eq(&GradientType::Mild) {
                60.0
            } else {
                20.0
            }))
            .on_press(Message::GradientsSelection(GradientType::Mild))
            .style(ButtonType::Gradient(GradientType::Mild)),
        )
        .push(
            button(
                Text::new("z")
                    .font(ICONS)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .size(13),
            )
            .padding(0)
            .height(20.0)
            .width(Fixed(if color_gradient.eq(&GradientType::Wild) {
                60.0
            } else {
                20.0
            }))
            .on_press(Message::GradientsSelection(GradientType::Wild))
            .style(ButtonType::Gradient(GradientType::Wild)),
        )
}

fn get_palette_container(
    style: StyleType,
    name: String,
    description: String,
    on_press: StyleType,
) -> Button<'static, Message, Renderer<StyleType>> {
    let font = get_font(style);

    let is_custom = matches!(on_press, StyleType::Custom(_));

    let mut content = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(name).font(font))
        .push(get_palette(on_press, is_custom));

    if !is_custom {
        content = content.push(Text::new(description).font(font));
    }

    Button::new(content)
        .height(Length::Fixed(if is_custom { 75.0 } else { 110.0 }))
        .width(Length::Fixed(380.0))
        .padding(5)
        .style(if on_press.eq(&style) {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::Style(on_press))
}

fn get_palette(
    style: StyleType,
    is_custom: bool,
) -> Container<'static, Message, Renderer<RuleStyleTuple>> {
    let height = if is_custom { 25.0 } else { 40.0 };

    Container::new(
        Row::new()
            .padding(0)
            .push(
                Row::new()
                    .padding(0)
                    .width(Length::Fixed(120.0))
                    .push(Rule::horizontal(height).style(RuleStyleTuple(style, RuleType::PalettePrimary))),
            )
            .push(
                Row::new()
                    .padding(0)
                    .width(Length::Fixed(80.0))
                    .push(Rule::horizontal(height).style(RuleStyleTuple(style, RuleType::PaletteSecondary))),
            )
            .push(
                Row::new()
                    .padding(0)
                    .width(Length::Fixed(60.0))
                    .push(Rule::horizontal(height).style(RuleStyleTuple(style,RuleType::PaletteOutgoing))),
            )
            .push(
                Row::new()
                    .padding(0)
                    .width(Length::Fixed(40.0))
                    .push(Rule::horizontal(height).style(RuleStyleTuple(style, RuleType::PaletteButtons))),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .width(300.0 + 2.0 * BORDER_WIDTH)
    .height(height + 1.7 * BORDER_WIDTH)
    .style(ContainerType::Palette)
}

// Buttons for each extra style arranged in rows of two
fn get_extra_palettes(
    styles: &[ExtraStyles],
    current_style: StyleType,
) -> Vec<Element<'static, Message, Renderer<StyleType>>> {
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
                    .push(horizontal_space(Length::Fixed(15.0)))
                    .push(second)
                    .into(),
                <Space as Into<Element<Message, Renderer<StyleType>>>>::into(vertical_space(
                    Length::Fixed(10.0),
                )),
            ]);
        } else {
            children.extend([
                Row::new().push(first).into(),
                <Space as Into<Element<Message, Renderer<StyleType>>>>::into(vertical_space(
                    Length::Fixed(10.0),
                )),
            ]);
        }
    }

    children
}
