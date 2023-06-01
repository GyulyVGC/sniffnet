//! Buttons style

use iced::widget::button;
use iced::widget::button::Appearance;
use iced::{Background, Color, Vector};

use crate::get_colors;
use crate::gui::styles::style_constants::{get_starred_color, BORDER_BUTTON_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::palette::mix_colors;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Button {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Button::Custom(Box::new(tuple))
    }
}

impl button::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::TabActive) => colors.primary,
                StyleTuple(_, ElementType::Starred) => get_starred_color(self.0),
                StyleTuple(_, ElementType::Badge) => colors.secondary,
                StyleTuple(_, ElementType::BorderedRound) => colors.round_containers,
                StyleTuple(_, ElementType::Neutral | ElementType::NotStarred) => Color::TRANSPARENT,
                StyleTuple(_, ElementType::BorderedRoundSelected) => {
                    mix_colors(colors.primary, colors.buttons)
                }
                _ => colors.buttons,
            })),
            border_radius: match self {
                StyleTuple(
                    _,
                    ElementType::TabActive | ElementType::TabInactive | ElementType::Neutral,
                ) => 0.0,
                StyleTuple(_, ElementType::BorderedRound | ElementType::BorderedRoundSelected) => {
                    12.0
                }
                StyleTuple(_, ElementType::Starred | ElementType::NotStarred) => 100.0,
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(
                    _,
                    ElementType::TabActive
                    | ElementType::TabInactive
                    | ElementType::Starred
                    | ElementType::NotStarred
                    | ElementType::Neutral
                    | ElementType::Badge,
                ) => 0.0,
                StyleTuple(_, ElementType::BorderedRound) => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                StyleTuple(_, ElementType::Starred) => Color::BLACK,
                StyleTuple(_, ElementType::Badge) => colors.text_headers,
                _ => colors.text_body,
            },
            border_color: match self {
                StyleTuple(_, ElementType::Alert) => Color::new(0.8, 0.15, 0.15, 1.0),
                StyleTuple(_, ElementType::BorderedRound) => colors.round_borders,
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            shadow_offset: match self.1 {
                ElementType::Neutral => Vector::default(),
                _ => Vector::new(0.0, 2.0),
            },
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::Starred) => get_starred_color(self.0),
                StyleTuple(_, ElementType::TabActive) => colors.primary,
                _ => mix_colors(colors.primary, colors.buttons),
            })),
            border_radius: match self {
                StyleTuple(
                    _,
                    ElementType::TabActive | ElementType::TabInactive | ElementType::Neutral,
                ) => 0.0,
                StyleTuple(_, ElementType::BorderedRound | ElementType::BorderedRoundSelected) => {
                    12.0
                }
                StyleTuple(_, ElementType::Starred | ElementType::NotStarred) => 100.0,
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(
                    _,
                    ElementType::Starred
                    | ElementType::TabActive
                    | ElementType::TabInactive
                    | ElementType::BorderedRound,
                ) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                StyleTuple(_, ElementType::Alert) => Color::new(0.8, 0.15, 0.15, 1.0),
                StyleTuple(
                    _,
                    ElementType::BorderedRound | ElementType::Neutral | ElementType::NotStarred,
                ) => colors.round_borders,
                _ => colors.secondary,
            },
            text_color: match self {
                StyleTuple(_, ElementType::Starred) => Color::BLACK,
                _ => colors.text_body,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        button::StyleSheet::active(self, style)
    }
}
