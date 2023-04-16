//! Slider style

use iced::widget::slider::Appearance;
use iced_native::widget::slider::{Handle, Rail};
use iced_native::widget::vertical_slider::HandleShape;

use crate::get_colors;
use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::gui::styles::types::palette::mix_colors;
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
            rail: Rail {
                colors: (colors.secondary, colors.buttons),
                width: 3.0,
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 5.0 },
                color: colors.primary,
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
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: mix_colors(colors.primary, colors.buttons),
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
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                color: mix_colors(colors.primary, colors.buttons),
                border_width: BORDER_WIDTH,
                border_color: colors.secondary,
            },
        }
    }
}
