//! Toggler style

#![allow(clippy::module_name_repetitions)]

use iced::widget::toggler::Appearance;
use iced::Color;

use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::StyleType;

#[derive(Clone, Copy, Default)]
pub enum TogglerType {
    #[default]
    Standard,
}

impl iced::widget::toggler::StyleSheet for StyleType {
    type Style = TogglerType;

    fn active(&self, _: &Self::Style, is_active: bool) -> iced::widget::toggler::Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        let bg_color = if is_active {
            Color {
                a: ext.alpha_chart_badge,
                ..colors.secondary
            }
        } else {
            ext.buttons_color
        };
        Appearance {
            background: bg_color,
            background_border_width: BORDER_WIDTH,
            background_border_color: bg_color,
            foreground: colors.primary,
            foreground_border_width: BORDER_WIDTH,
            foreground_border_color: if is_active {
                colors.secondary
            } else {
                Color::TRANSPARENT
            },
        }
    }

    fn hovered(&self, _: &Self::Style, is_active: bool) -> iced::widget::toggler::Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        let bg_color = if is_active {
            Color {
                a: ext.alpha_chart_badge,
                ..colors.secondary
            }
        } else {
            ext.buttons_color
        };
        Appearance {
            background: bg_color,
            background_border_width: BORDER_WIDTH,
            background_border_color: colors.secondary,
            foreground: colors.primary,
            foreground_border_width: BORDER_WIDTH,
            foreground_border_color: if is_active {
                colors.secondary
            } else {
                Color::TRANSPARENT
            },
        }
    }
}
