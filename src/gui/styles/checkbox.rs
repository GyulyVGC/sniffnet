//! Checkbox style

use iced::widget::checkbox::Appearance;
use iced::Background;

use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum CheckboxType {
    Standard,
}

#[derive(Clone)]
pub struct CheckboxStyleTuple(pub StyleType, pub CheckboxType);

impl From<CheckboxStyleTuple> for iced::theme::Checkbox {
    fn from(tuple: CheckboxStyleTuple) -> Self {
        iced::theme::Checkbox::Custom(Box::new(tuple))
    }
}

impl iced::widget::checkbox::StyleSheet for CheckboxStyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style, is_checked: bool) -> Appearance {
        let colors = get_colors(self.0);
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
        let colors = get_colors(self.0);
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
