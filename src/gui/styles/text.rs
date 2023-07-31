//! Text style

use iced::Color;

use crate::gui::styles::types::palette::Palette;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum TextType {
    Standard,
    Title,
    Subtitle,
}

#[derive(Clone)]
pub struct TextStyleTuple(pub StyleType, pub TextType);

impl From<TextStyleTuple> for iced::theme::Text {
    fn from(tuple: TextStyleTuple) -> Self {
        let colors = get_colors(tuple.0);
        iced::theme::Text::Color(highlight(tuple.1, &colors))
    }
}

/// Returns the weighted average of two colors; color intensity is fixed to 100%
pub fn highlight(element: TextType, colors: &Palette) -> Color {
    let color = colors.secondary;
    match element {
        TextType::Title => {
            let (p1, c) = if colors.text_body.eq(&Color::BLACK) {
                (0.9, 0.7)
            } else {
                (0.6, 1.0)
            };
            Color {
                r: c * (1.0 - p1) + color.r * p1,
                g: c * (1.0 - p1) + color.g * p1,
                b: c * (1.0 - p1) + color.b * p1,
                a: 1.0,
            }
        }
        TextType::Subtitle => {
            let (p1, c) = if colors.text_body.eq(&Color::BLACK) {
                (0.6, 0.7)
            } else {
                (0.4, 1.0)
            };
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
