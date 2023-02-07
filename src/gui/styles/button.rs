//! Buttons style

use crate::enums::element_type::ElementType;
use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{BORDER_BUTTON_RADIUS, BORDER_WIDTH, STARRED};
use iced::widget::button;
use iced::{Background, Color, Vector};

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
                StyleTuple(
                    _,
                    ElementType::TabActive | ElementType::NotStarred | ElementType::BorderedRound,
                ) => colors.primary,
                StyleTuple(_, ElementType::Starred) => STARRED,
                _ => colors.buttons,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                StyleTuple(_, ElementType::BorderedRound) => 12.0,
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(
                    _,
                    ElementType::TabActive
                    | ElementType::TabInactive
                    | ElementType::Starred
                    | ElementType::NotStarred,
                ) => 0.0,
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                StyleTuple(_, ElementType::Starred) => Color::BLACK,
                _ => colors.text_body,
            },
            border_color: match self {
                StyleTuple(_, ElementType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                StyleTuple(_, ElementType::BorderedRound) => Color::BLACK,
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        let colors = get_colors(self.0);
        button::Appearance {
            shadow_offset: Vector::new(2.0, 2.0),
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::Starred) => STARRED,
                _ => colors.primary,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                StyleTuple(_, ElementType::BorderedRound) => 12.0,
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(
                    _,
                    ElementType::Starred | ElementType::NotStarred | ElementType::TabActive,
                ) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                StyleTuple(_, ElementType::TabInactive) => colors.buttons,
                StyleTuple(_, ElementType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                _ => colors.secondary,
            },
            text_color: match self {
                StyleTuple(_, ElementType::Starred) => Color::BLACK,
                _ => colors.text_body,
            },
        }
    }
}
