//! Checkbox style

#![allow(clippy::module_name_repetitions)]

use iced::widget::checkbox::Appearance;
use iced::Background;

use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum CheckboxType {
    #[default]
    Standard,
}

impl iced::widget::checkbox::StyleSheet for StyleType {
    type Style = CheckboxType;

    fn active(&self, _: &Self::Style, is_checked: bool) -> Appearance {
        let colors = get_colors(*self);
        Appearance {
            background: Background::Color(colors.buttons),
            icon_color: colors.text_body,
            border_radius: 0.0.into(),
            border_width: if is_checked { 1.0 } else { 0.0 },
            border_color: colors.secondary,
            text_color: None,
        }
    }

    fn hovered(&self, _: &Self::Style, _is_checked: bool) -> Appearance {
        let colors = get_colors(*self);
        Appearance {
            background: Background::Color(colors.buttons),
            icon_color: colors.text_body,
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: colors.secondary,
            text_color: None,
        }
    }
}
