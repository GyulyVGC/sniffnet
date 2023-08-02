//! Slider style

use iced::widget::slider::Appearance;
use iced::widget::slider::{Handle, HandleShape, Rail};

use crate::gui::styles::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::palette::mix_colors;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum SliderType {
    Standard,
}

#[derive(Clone)]
pub struct SliderStyleTuple(pub StyleType, pub SliderType);

impl From<SliderStyleTuple> for iced::theme::Slider {
    fn from(tuple: SliderStyleTuple) -> Self {
        iced::theme::Slider::Custom(Box::new(tuple))
    }
}

impl iced::widget::slider::StyleSheet for SliderStyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            rail: Rail {
                colors: (mix_colors(colors.secondary, colors.buttons), colors.buttons),
                width: 3.0,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 5.5 },
                color: mix_colors(colors.secondary, colors.buttons),
                border_width: 0.0,
                border_color: colors.secondary,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            rail: Rail {
                colors: (colors.secondary, colors.buttons),
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
        let colors = get_colors(self.0);
        Appearance {
            rail: Rail {
                colors: (colors.secondary, colors.buttons),
                width: 3.0,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: mix_colors(colors.secondary, colors.buttons),
            },
        }
    }
}
