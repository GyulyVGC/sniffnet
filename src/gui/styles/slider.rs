//! Slider style

#![allow(clippy::module_name_repetitions)]

use iced::widget::slider::Appearance;
use iced::widget::slider::{Handle, HandleShape, Rail};

use crate::gui::styles::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::palette::mix_colors;
use crate::{ StyleType};

#[derive(Clone, Copy, Default)]
pub enum SliderType {
    #[default]
    Standard,
}

impl iced::widget::slider::StyleSheet for StyleType {
    type Style = SliderType;

    fn active(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Appearance {
            rail: Rail {
                colors: (mix_colors(colors.secondary, color_buttons), color_buttons),
                width: 3.0,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 5.5 },
                color: mix_colors(colors.secondary, color_buttons),
                border_width: 0.0,
                border_color: colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Appearance {
            rail: Rail {
                colors: (colors.secondary, color_buttons),
                width: 3.0,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: colors.secondary,
                border_width: 0.0,
                border_color: colors.secondary,
            },
        }
    }

    fn dragging(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(*self);
        let color_buttons = get_buttons_color(*self);
        Appearance {
            rail: Rail {
                colors: (colors.secondary, color_buttons),
                width: 3.0,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: mix_colors(colors.secondary, color_buttons),
            },
        }
    }
}
