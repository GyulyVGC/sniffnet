//! Slider style

use iced::widget::slider::Appearance;
use iced::BorderRadius;
use iced_widget::slider::{Handle, HandleShape, Rail};

use crate::gui::styles::style_constants::BORDER_WIDTH;
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
                colors: (colors.secondary, colors.buttons),
                width: 3.0,
                border_radius: BorderRadius::default(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 5.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
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
                border_radius: BorderRadius::default(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
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
                border_radius: BorderRadius::default(),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
        }
    }
}
