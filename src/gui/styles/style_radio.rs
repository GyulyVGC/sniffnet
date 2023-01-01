//! Radios style

use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::BORDER_WIDTH;
use iced::Background;
use iced::Theme;

impl From<StyleTuple> for iced::theme::Radio {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Radio::Custom(Box::new(tuple))
    }
}

impl iced::widget::radio::StyleSheet for StyleTuple {
    type Style = Theme;

    fn active(&self, _: &Self::Style, is_selected: bool) -> iced::widget::radio::Appearance {
        let colors = get_colors(self.0);
        iced::widget::radio::Appearance {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: if is_selected { BORDER_WIDTH } else { 0.0 },
            border_color: colors.secondary,
            text_color: None,
        }
    }

    fn hovered(&self, _: &Self::Style, _is_selected: bool) -> iced::widget::radio::Appearance {
        let colors = get_colors(self.0);
        iced::widget::radio::Appearance {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            text_color: Some(colors.secondary),
        }
    }
}
