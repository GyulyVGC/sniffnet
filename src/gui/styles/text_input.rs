//! Text Input style

#![allow(clippy::module_name_repetitions)]

use iced::widget::text_input::Appearance;
use iced::{Background, Color};

use crate::{ StyleType};

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
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Appearance {
            background: Background::Color(match style {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => Color{a: get_alpha_round_borders(*self), ..color_buttons},
            }),
            border_radius: 0.0.into(),
            border_width: 1.5,
            border_color: match style {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => color_buttons,
                TextInputType::Error => Color::new(0.8, 0.15, 0.15, 1.0),
            },
            icon_color: colors.text_body,
        }
    }

    fn focused(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(*self);
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
        let color = get_colors(*self).text_body;
        Color {
            a: if self.is_text_body_dark() { 0.7 } else { 0.2 },
            ..color
        }
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        get_colors(*self).text_body
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        let color = get_colors(*self).text_body;
        Color {
            a: if self.is_text_body_dark() { 0.7 } else { 0.2 },
            ..color
        }
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        let color = get_colors(*self).text_body;
        Color {
            a: if self.is_text_body_dark() { 0.4 } else { 0.05 },
            ..color
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Appearance {
            background: Background::Color(match style {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => color_buttons,
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
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Appearance {
            background: Background::Color(match style {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => Color {
                    a: get_alpha_round_containers(*self),
                    ..color_buttons
                },
            }),
            border_radius: 0.0.into(),
            border_width: 1.5,
            border_color: match style {
                TextInputType::Badge => Color::TRANSPARENT,
                TextInputType::Standard => Color {
                    a: get_alpha_round_borders(*self),
                    ..color_buttons
                },
                TextInputType::Error => Color::new(0.8, 0.15, 0.15, get_alpha_round_borders(*self)),
            },
            icon_color: colors.text_body,
        }
    }
}
