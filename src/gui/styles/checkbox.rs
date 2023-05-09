//! Checkbox style

use iced::widget::checkbox::Appearance;
use iced::Background;

use crate::get_colors;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;

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
            border_radius: 0.0,
            border_width: 1.0,
            border_color: colors.secondary,
            text_color: match self.1 {
                ElementType::Badge => None,
                _ => Some(colors.secondary),
            },
        }
    }
}
