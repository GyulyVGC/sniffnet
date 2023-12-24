//! Scrollbars style

#![allow(clippy::module_name_repetitions)]

use iced::widget::scrollable::Properties;
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::{Background, Color};

use crate::gui::styles::style_constants::{BORDER_ROUNDED_RADIUS};
use crate::gui::styles::types::palette::mix_colors;
use crate::{ StyleType};

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

impl iced::widget::scrollable::StyleSheet for StyleType {
    type Style = ScrollbarType;

    fn active(&self, _: &Self::Style) -> Scrollbar {
        let color_buttons = get_buttons_color(*self);
        Scrollbar {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: Color {
                a: get_alpha_round_borders(*self),
                ..color_buttons
            },
            scroller: Scroller {
                color: Color {
                    a: get_alpha_round_borders(*self),
                    ..color_buttons
                },
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Scrollbar {
            background: Some(Background::Color(Color {
                a: get_alpha_round_borders(*self),
                ..color_buttons
            })),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: Color {
                a: get_alpha_round_borders(*self),
                ..color_buttons
            },
            scroller: Scroller {
                color: if is_mouse_over_scrollbar {
                    colors.secondary
                } else {
                    mix_colors(colors.secondary, color_buttons)
                },
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
