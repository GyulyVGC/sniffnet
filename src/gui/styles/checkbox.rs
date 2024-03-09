//! Checkbox style

#![allow(clippy::module_name_repetitions)]

use iced::widget::checkbox::Appearance;
use iced::{Background, Border};

use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::StyleType;

#[derive(Clone, Copy, Default)]
pub enum CheckboxType {
    #[default]
    Standard,
}

const CHECKBOX_BORDER_RADIUS: f32 = 5.0;

impl iced::widget::checkbox::StyleSheet for StyleType {
    type Style = CheckboxType;

    fn active(&self, _: &Self::Style, is_checked: bool) -> Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        Appearance {
            background: Background::Color(ext.buttons_color),
            icon_color: colors.text_body,
            border: Border {
                radius: CHECKBOX_BORDER_RADIUS.into(),
                width: if is_checked { BORDER_WIDTH } else { 0.0 },
                color: colors.secondary,
            },
            text_color: None,
        }
    }

    fn hovered(&self, _: &Self::Style, _is_checked: bool) -> Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        Appearance {
            background: Background::Color(ext.buttons_color),
            icon_color: colors.text_body,
            border: Border {
                radius: CHECKBOX_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: colors.secondary,
            },
            text_color: None,
        }
    }
}
