//! Scrollbars style

use iced::widget::scrollable::Properties;
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::Theme;
use iced::{Background, Color};

use crate::gui::styles::style_constants::{get_alpha_round_borders, BORDER_ROUNDED_RADIUS};
use crate::gui::styles::types::palette::mix_colors;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum ScrollbarType {
    #[default]
    Standard,
}

impl ScrollbarType {
    pub fn properties() -> Properties {
        Properties::new().width(5).scroller_width(5).margin(3)
    }
}

#[derive(Clone)]
pub struct ScrollbarStyleTuple(pub StyleType, pub ScrollbarType);

impl iced::widget::scrollable::StyleSheet for StyleType {
    type Style = ScrollbarType;

    fn active(&self, _: &Self::Style) -> Scrollbar {
        let colors = get_colors(*self);
        Scrollbar {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: Color {
                a: get_alpha_round_borders(*self),
                ..colors.buttons
            },
            scroller: Scroller {
                color: Color {
                    a: get_alpha_round_borders(*self),
                    ..colors.buttons
                },
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        let colors = get_colors(*self);
        Scrollbar {
            background: Some(Background::Color(Color {
                a: get_alpha_round_borders(*self),
                ..colors.buttons
            })),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: Color {
                a: get_alpha_round_borders(*self),
                ..colors.buttons
            },
            scroller: Scroller {
                color: if is_mouse_over_scrollbar {
                    colors.secondary
                } else {
                    mix_colors(colors.secondary, colors.buttons)
                },
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
