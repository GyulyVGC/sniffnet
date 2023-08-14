//! Radios style

use iced::Background;

use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum RadioType {
    #[default]
    Standard,
}

#[derive(Clone)]
pub struct RadioStyleTuple(pub StyleType, pub RadioType);

impl iced::widget::radio::StyleSheet for StyleType {
    type Style = RadioType;

    fn active(&self, _: &Self::Style, is_selected: bool) -> iced::widget::radio::Appearance {
        let colors = get_colors(*self);
        iced::widget::radio::Appearance {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: if is_selected { BORDER_WIDTH } else { 0.0 },
            border_color: colors.secondary,
            text_color: None,
        }
    }

    fn hovered(&self, _: &Self::Style, _is_selected: bool) -> iced::widget::radio::Appearance {
        let colors = get_colors(*self);
        iced::widget::radio::Appearance {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            text_color: None,
        }
    }
}
