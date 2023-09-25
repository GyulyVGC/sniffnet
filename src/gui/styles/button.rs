//! Buttons style

#![allow(clippy::module_name_repetitions)]

use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Color, Vector};

use crate::gui::styles::style_constants::{
    get_alpha_round_borders, get_alpha_round_containers, get_starred_color, BORDER_BUTTON_RADIUS,
    BORDER_WIDTH,
};
use crate::gui::styles::types::gradient_type::{
    get_gradient_buttons, get_gradient_hovered_buttons, GradientType,
};
use crate::gui::styles::types::palette::mix_colors;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum ButtonType {
    #[default]
    Standard,
    BorderedRound,
    BorderedRoundSelected,
    TabActive,
    TabInactive,
    Starred,
    NotStarred,
    Neutral,
    Alert,
    Badge,
    Gradient(GradientType),
}

impl button::StyleSheet for StyleType {
    type Style = ButtonType;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let colors = get_colors(*self);
        button::Appearance {
            background: Some(match style {
                ButtonType::TabActive | ButtonType::BorderedRoundSelected => {
                    Background::Color(mix_colors(colors.primary, colors.buttons))
                }
                ButtonType::Starred => Background::Color(get_starred_color(*self)),
                ButtonType::BorderedRound => Background::Color(Color {
                    a: get_alpha_round_containers(*self),
                    ..colors.buttons
                }),
                ButtonType::Neutral | ButtonType::NotStarred => {
                    Background::Color(Color::TRANSPARENT)
                }
                ButtonType::Gradient(GradientType::None) | ButtonType::Badge => {
                    Background::Color(colors.secondary)
                }
                ButtonType::Gradient(gradient_type) => Background::Gradient(get_gradient_buttons(
                    &colors,
                    *gradient_type,
                    self.is_nightly(),
                )),
                _ => Background::Color(colors.buttons),
            }),
            border_radius: match style {
                ButtonType::Neutral => 0.0.into(),
                ButtonType::TabActive | ButtonType::TabInactive => [0.0, 0.0, 30.0, 30.0].into(),
                ButtonType::BorderedRound | ButtonType::BorderedRoundSelected => 12.0.into(),
                ButtonType::Starred | ButtonType::NotStarred => 100.0.into(),
                _ => BORDER_BUTTON_RADIUS.into(),
            },
            border_width: match style {
                ButtonType::TabActive
                | ButtonType::TabInactive
                | ButtonType::Starred
                | ButtonType::NotStarred
                | ButtonType::Neutral
                | ButtonType::Badge => 0.0,
                ButtonType::BorderedRound => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            shadow_offset: match style {
                ButtonType::TabActive | ButtonType::TabInactive => Vector::new(3.0, 2.0),
                _ => Vector::new(0.0, 0.0),
            },
            text_color: match style {
                ButtonType::Starred => Color::BLACK,
                ButtonType::Badge | ButtonType::Gradient(_) => colors.text_headers,
                _ => colors.text_body,
            },
            border_color: match style {
                ButtonType::Alert => Color::new(0.8, 0.15, 0.15, 1.0),
                ButtonType::BorderedRound => Color {
                    a: get_alpha_round_borders(*self),
                    ..colors.buttons
                },
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let colors = get_colors(*self);
        button::Appearance {
            shadow_offset: match style {
                ButtonType::Neutral => Vector::default(),
                ButtonType::TabActive | ButtonType::TabInactive => Vector::new(3.0, 3.0),
                _ => Vector::new(0.0, 2.0),
            },
            background: Some(match style {
                ButtonType::Starred => Background::Color(get_starred_color(*self)),
                ButtonType::Gradient(GradientType::None) => {
                    Background::Color(mix_colors(colors.primary, colors.secondary))
                }
                ButtonType::Gradient(gradient_type) => Background::Gradient(
                    get_gradient_hovered_buttons(&colors, *gradient_type, self.is_nightly()),
                ),
                _ => Background::Color(mix_colors(colors.primary, colors.buttons)),
            }),
            border_radius: match style {
                ButtonType::Neutral => 0.0.into(),
                ButtonType::TabActive | ButtonType::TabInactive => [0.0, 0.0, 30.0, 30.0].into(),
                ButtonType::BorderedRound | ButtonType::BorderedRoundSelected => 12.0.into(),
                ButtonType::Starred | ButtonType::NotStarred => 100.0.into(),
                _ => BORDER_BUTTON_RADIUS.into(),
            },
            border_width: match style {
                ButtonType::Starred
                | ButtonType::TabActive
                | ButtonType::TabInactive
                | ButtonType::BorderedRound => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: match style {
                ButtonType::Alert => Color::new(0.8, 0.15, 0.15, 1.0),
                ButtonType::BorderedRound | ButtonType::Neutral | ButtonType::NotStarred => Color {
                    a: get_alpha_round_borders(*self),
                    ..colors.buttons
                },
                _ => colors.secondary,
            },
            text_color: match style {
                ButtonType::Starred => Color::BLACK,
                ButtonType::Gradient(_) => colors.text_headers,
                _ => colors.text_body,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::StyleSheet::active(self, style)
    }
}
