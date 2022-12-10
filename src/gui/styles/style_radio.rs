//! Radios style

use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::BORDER_WIDTH;
use iced::Background;
use iced_style::Theme;

impl From<StyleTuple> for iced::theme::Radio {
    fn from(tuple: StyleTuple) -> Self {
        iced_style::theme::Radio::Custom(Box::new(tuple))
    }
}

impl iced_style::radio::StyleSheet for StyleTuple {
    type Style = Theme;

    fn active(&self, _: &Self::Style, is_selected: bool) -> iced_style::radio::Appearance {
        let colors = get_colors(self.0);
        iced_style::radio::Appearance {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: if is_selected { BORDER_WIDTH } else { 0.0 },
            border_color: colors.secondary,
            text_color: if is_selected {
                Some(colors.secondary)
            } else {
                None
            },
        }
    }

    fn hovered(&self, _: &Self::Style, _is_selected: bool) -> iced_style::radio::Appearance {
        let colors = get_colors(self.0);
        iced_style::radio::Appearance {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            text_color: Some(colors.secondary),
        }
    }
}
