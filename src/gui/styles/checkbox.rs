//! Checkbox style

use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::BORDER_WIDTH;
use iced::widget::checkbox::Appearance;
use iced::Background;

impl From<StyleTuple> for iced::theme::Checkbox {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Checkbox::Custom(Box::new(tuple))
    }
}

impl iced::widget::checkbox::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style, is_checked: bool) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(colors.buttons),
            icon_color: colors.text_body,
            border_radius: 0.0,
            border_width: if is_checked { BORDER_WIDTH } else { 0.0 },
            border_color: colors.secondary,
            text_color: None,
        }
    }

    fn hovered(&self, _: &Self::Style, _is_checked: bool) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(colors.buttons),
            icon_color: colors.text_body,
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            text_color: Some(colors.secondary),
        }
    }
}
