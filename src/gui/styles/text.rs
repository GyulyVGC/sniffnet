//! Text style

use iced::widget::text::Appearance;
use iced::widget::{Column, Text};
use iced::{Color, Renderer};

use crate::gui::styles::style_constants::get_font;
use crate::gui::types::message::Message;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum TextType {
    #[default]
    Standard,
    Title,
    Subtitle,
}

/// Returns a formatted caption followed by subtitle, new line, tab, and desc
impl TextType {
    pub fn highlighted_subtitle_with_desc(
        subtitle: &str,
        desc: &str,
        style: StyleType,
    ) -> Column<'static, Message, Renderer<StyleType>> {
        let font = get_font(style);
        Column::new()
            .push(
                Text::new(format!("{subtitle}:"))
                    .style(TextType::Subtitle)
                    .font(font),
            )
            .push(Text::new(format!("   {desc}")).font(font))
    }
}

#[derive(Clone)]
pub struct TextStyleTuple(pub StyleType, pub TextType);

impl From<TextStyleTuple> for iced::theme::Text {
    fn from(tuple: TextStyleTuple) -> Self {
        iced::theme::Text::Color(highlight(tuple.0, tuple.1))
    }
}

impl iced::widget::text::StyleSheet for StyleType {
    type Style = TextType;

    fn appearance(&self, style: Self::Style) -> Appearance {
        Appearance {
            color: Some(highlight(*self, style)),
        }
    }
}

/// Returns the weighted average of two colors; color intensity is fixed to 100%
pub fn highlight(style: StyleType, element: TextType) -> Color {
    let colors = get_colors(style);
    let color = colors.secondary;
    let is_nightly = style.is_nightly();
    match element {
        TextType::Title => {
            let (p1, c) = if is_nightly { (0.6, 1.0) } else { (0.9, 0.7) };
            Color {
                r: c * (1.0 - p1) + color.r * p1,
                g: c * (1.0 - p1) + color.g * p1,
                b: c * (1.0 - p1) + color.b * p1,
                a: 1.0,
            }
        }
        TextType::Subtitle => {
            let (p1, c) = if is_nightly { (0.4, 1.0) } else { (0.6, 0.7) };
            Color {
                r: c * (1.0 - p1) + color.r * p1,
                g: c * (1.0 - p1) + color.g * p1,
                b: c * (1.0 - p1) + color.b * p1,
                a: 1.0,
            }
        }
        TextType::Standard => colors.text_body,
    }
}
