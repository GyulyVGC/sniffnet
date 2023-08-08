//! Text Input style

use iced::widget::text_input;
use iced::widget::text_input::Appearance;
use iced::{Background, Color};

use crate::gui::styles::style_constants::get_alpha_round_borders;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum TextInputType {
    Standard,
    Badge,
}

#[derive(Clone)]
pub struct TextInputStyleTuple(pub StyleType, pub TextInputType);

impl From<TextInputStyleTuple> for iced::theme::TextInput {
    fn from(tuple: TextInputStyleTuple) -> Self {
        iced::theme::TextInput::Custom(Box::new(tuple))
    }
}

impl iced::widget::text_input::StyleSheet for TextInputStyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(match self.1 {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => colors.buttons,
            }),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: match self.1 {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => Color {
                    a: get_alpha_round_borders(self.0),
                    ..colors.buttons
                },
            },
            icon_color: colors.text_body,
        }
    }

    fn focused(&self, _: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(colors.primary),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: colors.secondary,
            icon_color: colors.text_body,
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        let color = get_colors(self.0).text_body;
        Color {
            a: if color.eq(&Color::BLACK) { 0.7 } else { 0.2 },
            ..color
        }
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        get_colors(self.0).text_body
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        let color = get_colors(self.0).text_body;
        Color {
            a: if color.eq(&Color::BLACK) { 0.4 } else { 0.05 },
            ..color
        }
    }

    fn hovered(&self, _: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(match self.1 {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => colors.buttons,
            }),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: colors.secondary,
            icon_color: colors.text_body,
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        text_input::StyleSheet::active(self, style)
    }
}
