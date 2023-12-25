//! Text Input style

#![allow(clippy::module_name_repetitions)]

use iced::widget::text_input::Appearance;
use iced::{Background, Color};

use crate::StyleType;

#[derive(Clone, Copy, Default)]
pub enum TextInputType {
    #[default]
    Standard,
    Badge,
    Error,
}

impl iced::widget::text_input::StyleSheet for StyleType {
    type Style = TextInputType;

    fn active(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = self.get_palette();
        let ext = self.get_palette_extension();
        Appearance {
            background: Background::Color(match style {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                },
            }),
            border_radius: 0.0.into(),
            border_width: 1.5,
            border_color: match style {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => ext.buttons_color,
                TextInputType::Error => Color::new(0.8, 0.15, 0.15, 1.0),
            },
            icon_color: colors.text_body,
        }
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = self.get_palette();
        Appearance {
            background: Background::Color(colors.primary),
            border_radius: 0.0.into(),
            border_width: 1.5,
            border_color: match style {
                TextInputType::Error => Color::new(0.8, 0.15, 0.15, 1.0),
                _ => colors.secondary,
            },
            icon_color: colors.text_body,
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        let color = self.get_palette().text_body;
        let is_nightly = self.get_palette_extension().is_nightly;
        Color {
            a: if is_nightly { 0.7 } else { 0.2 },
            ..color
        }
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        self.get_palette().text_body
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        let color = self.get_palette().text_body;
        let is_nightly = self.get_palette_extension().is_nightly;
        Color {
            a: if is_nightly { 0.7 } else { 0.2 },
            ..color
        }
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        let color = self.get_palette().text_body;
        let is_nightly = self.get_palette_extension().is_nightly;
        Color {
            a: if is_nightly { 0.4 } else { 0.05 },
            ..color
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = self.get_palette();
        let ext = self.get_palette_extension();
        Appearance {
            background: Background::Color(match style {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => ext.buttons_color,
            }),
            border_radius: 0.0.into(),
            border_width: 1.5,
            border_color: match style {
                TextInputType::Error => Color::new(0.8, 0.15, 0.15, 1.0),
                _ => colors.secondary,
            },
            icon_color: colors.text_body,
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        let colors = self.get_palette();
        let ext = self.get_palette_extension();
        Appearance {
            background: Background::Color(match style {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => Color {
                    a: ext.alpha_round_containers,
                    ..ext.buttons_color
                },
            }),
            border_radius: 0.0.into(),
            border_width: 1.5,
            border_color: match style {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                },
                TextInputType::Error => Color::new(0.8, 0.15, 0.15, ext.alpha_round_borders),
            },
            icon_color: colors.text_body,
        }
    }
}
