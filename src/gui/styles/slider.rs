//! Slider style

use iced::widget::slider::{Appearance, Rail};
use iced_native::widget::slider::Handle;
use iced_native::widget::vertical_slider::HandleShape;

use crate::get_colors;
use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Slider {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Slider::Custom(Box::new(tuple))
    }
}

impl iced::widget::slider::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            handle: Handle {
                shape: HandleShape::Circle { radius: 7.0 },
                color: colors.primary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
            rail: Rail {
                colors: (colors.secondary, colors.primary),
                width: 2.,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            handle: Handle {
                shape: HandleShape::Circle { radius: 7.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
            rail: Rail {
                colors: (colors.secondary, colors.secondary),
                width: 2.,
            },
        }
    }

    fn dragging(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            handle: Handle {
                shape: HandleShape::Circle { radius: 7.0 },
                color: colors.secondary,
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
            rail: Rail {
                colors: (colors.secondary, colors.secondary),
                width: 2.,
            },
        }
    }
}
