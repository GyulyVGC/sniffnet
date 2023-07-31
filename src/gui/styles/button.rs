//! Buttons style

use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Color, Vector};

use crate::gui::styles::style_constants::{get_starred_color, BORDER_BUTTON_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::palette::mix_colors;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum ButtonType {
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
}

#[derive(Clone)]
pub struct ButtonStyleTuple(pub StyleType, pub ButtonType);

impl From<ButtonStyleTuple> for iced::theme::Button {
    fn from(tuple: ButtonStyleTuple) -> Self {
        iced::theme::Button::Custom(Box::new(tuple))
    }
}

impl button::StyleSheet for ButtonStyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            background: Some(Background::Color(match self {
                ButtonStyleTuple(_, ButtonType::TabActive) => colors.primary,
                ButtonStyleTuple(_, ButtonType::Starred) => get_starred_color(self.0),
                ButtonStyleTuple(_, ButtonType::Badge) => colors.secondary,
                ButtonStyleTuple(_, ButtonType::BorderedRound) => colors.round_containers,
                ButtonStyleTuple(_, ButtonType::Neutral | ButtonType::NotStarred) => {
                    Color::TRANSPARENT
                }
                ButtonStyleTuple(_, ButtonType::BorderedRoundSelected) => {
                    mix_colors(colors.primary, colors.buttons)
                }
                _ => colors.buttons,
            })),
            border_radius: match self {
                ButtonStyleTuple(
                    _,
                    ButtonType::TabActive | ButtonType::TabInactive | ButtonType::Neutral,
                ) => 0.0.into(),
                ButtonStyleTuple(
                    _,
                    ButtonType::BorderedRound | ButtonType::BorderedRoundSelected,
                ) => 12.0.into(),
                ButtonStyleTuple(_, ButtonType::Starred | ButtonType::NotStarred) => 100.0.into(),
                _ => BORDER_BUTTON_RADIUS.into(),
            },
            border_width: match self {
                ButtonStyleTuple(
                    _,
                    ButtonType::TabActive
                    | ButtonType::TabInactive
                    | ButtonType::Starred
                    | ButtonType::NotStarred
                    | ButtonType::Neutral
                    | ButtonType::Badge,
                ) => 0.0,
                ButtonStyleTuple(_, ButtonType::BorderedRound) => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                ButtonStyleTuple(_, ButtonType::Starred) => Color::BLACK,
                ButtonStyleTuple(_, ButtonType::Badge) => colors.text_headers,
                _ => colors.text_body,
            },
            border_color: match self {
                ButtonStyleTuple(_, ButtonType::Alert) => Color::new(0.8, 0.15, 0.15, 1.0),
                ButtonStyleTuple(_, ButtonType::BorderedRound) => colors.round_borders,
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            shadow_offset: match self.1 {
                ButtonType::Neutral => Vector::default(),
                _ => Vector::new(0.0, 2.0),
            },
            background: Some(Background::Color(match self {
                ButtonStyleTuple(_, ButtonType::Starred) => get_starred_color(self.0),
                ButtonStyleTuple(_, ButtonType::TabActive) => colors.primary,
                _ => mix_colors(colors.primary, colors.buttons),
            })),
            border_radius: match self {
                ButtonStyleTuple(
                    _,
                    ButtonType::TabActive | ButtonType::TabInactive | ButtonType::Neutral,
                ) => 0.0.into(),
                ButtonStyleTuple(
                    _,
                    ButtonType::BorderedRound | ButtonType::BorderedRoundSelected,
                ) => 12.0.into(),
                ButtonStyleTuple(_, ButtonType::Starred | ButtonType::NotStarred) => 100.0.into(),
                _ => BORDER_BUTTON_RADIUS.into(),
            },
            border_width: match self {
                ButtonStyleTuple(
                    _,
                    ButtonType::Starred
                    | ButtonType::TabActive
                    | ButtonType::TabInactive
                    | ButtonType::BorderedRound,
                ) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                ButtonStyleTuple(_, ButtonType::Alert) => Color::new(0.8, 0.15, 0.15, 1.0),
                ButtonStyleTuple(
                    _,
                    ButtonType::BorderedRound | ButtonType::Neutral | ButtonType::NotStarred,
                ) => colors.round_borders,
                _ => colors.secondary,
            },
            text_color: match self {
                ButtonStyleTuple(_, ButtonType::Starred) => Color::BLACK,
                _ => colors.text_body,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::StyleSheet::active(self, style)
    }
}
