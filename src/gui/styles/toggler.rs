//! Toggler style

#![allow(clippy::module_name_repetitions)]

use iced::widget::toggler::Appearance;
use iced::Color;

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
        Appearance {
            background: if is_active {
                Color {
                    a: ext.alpha_chart_badge,
                    ..colors.secondary
                }
            } else {
                ext.buttons_color
            },
            background_border: None,
            foreground: colors.primary,
            foreground_border: Some(ext.buttons_color),
        }
    }

    fn hovered(&self, _: &Self::Style, is_active: bool) -> iced::widget::toggler::Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        Appearance {
            background: if is_active {
                Color {
                    a: ext.alpha_chart_badge,
                    ..colors.secondary
                }
            } else {
                ext.buttons_color
            },
            background_border: Some(colors.secondary),
            foreground: colors.primary,
            foreground_border: Some(ext.buttons_color),
        }
    }
}
