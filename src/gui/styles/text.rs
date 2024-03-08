//! Text style

#![allow(clippy::module_name_repetitions)]

use iced::widget::text::Appearance;
use iced::widget::{Column, Text};
use iced::{Color, Font};

use crate::gui::types::message::Message;
use crate::StyleType;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum TextType {
    #[default]
    Standard,
    Incoming,
    Outgoing,
    Title,
    Subtitle,
    Danger,
    Sponsor,
    Starred,
}

/// Returns a formatted caption followed by subtitle, new line, tab, and desc
impl TextType {
    pub fn highlighted_subtitle_with_desc(
        subtitle: &str,
        desc: &str,
        font: Font,
    ) -> Column<'static, Message, StyleType> {
        Column::new()
            .push(
                Text::new(format!("{subtitle}:"))
                    .style(TextType::Subtitle)
                    .font(font),
            )
            .push(Text::new(format!("   {desc}")).font(font))
    }
}

impl iced::widget::text::StyleSheet for StyleType {
    type Style = TextType;

    fn appearance(&self, style: Self::Style) -> Appearance {
        Appearance {
            color: if style == TextType::Standard {
                None
            } else {
                Some(highlight(*self, style))
            },
        }
    }
}

pub fn highlight(style: StyleType, element: TextType) -> Color {
    let colors = style.get_palette();
    let secondary = colors.secondary;
    let is_nightly = style.get_extension().is_nightly;
    match element {
        TextType::Title => {
            let (p1, c) = if is_nightly { (0.6, 1.0) } else { (0.9, 0.7) };
            Color {
                r: c * (1.0 - p1) + secondary.r * p1,
                g: c * (1.0 - p1) + secondary.g * p1,
                b: c * (1.0 - p1) + secondary.b * p1,
                a: 1.0,
            }
        }
        TextType::Subtitle => {
            let (p1, c) = if is_nightly { (0.4, 1.0) } else { (0.6, 0.7) };
            Color {
                r: c * (1.0 - p1) + secondary.r * p1,
                g: c * (1.0 - p1) + secondary.g * p1,
                b: c * (1.0 - p1) + secondary.b * p1,
                a: 1.0,
            }
        }
        TextType::Incoming => colors.secondary,
        TextType::Outgoing => colors.outgoing,
        TextType::Danger => Color::from_rgb(0.8, 0.15, 0.15),
        TextType::Sponsor => Color::from_rgb(1.0, 0.3, 0.5),
        TextType::Standard => colors.text_body,
        TextType::Starred => colors.starred,
    }
}
