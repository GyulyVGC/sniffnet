//! Text style

use iced::Color;

use crate::get_colors;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Text {
    fn from(tuple: StyleTuple) -> Self {
        let colors = get_colors(&tuple.0);
        iced::theme::Text::Color(highlight(tuple.1, colors))
    }
}

/// Returns the weighted average of two colors; color intensity is fixed to 100%
pub fn highlight(element: ElementType, colors: &Palette) -> Color {
    let color = colors.secondary;
    match element {
        ElementType::Title => {
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
        ElementType::Subtitle => {
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
        _ => colors.text_body,
    }
}
