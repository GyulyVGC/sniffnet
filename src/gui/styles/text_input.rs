//! Text Input style

use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use iced::widget::text_input::Appearance;
use iced::{Background, Color};

impl From<StyleTuple> for iced::theme::TextInput {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::TextInput::Custom(Box::new(tuple))
    }
}

impl iced::widget::text_input::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(colors.buttons),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: colors.round_borders,
        }
    }

    fn focused(&self, _: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(colors.primary),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: colors.secondary,
        }
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        let color = get_colors(self.0).text_body;
        Color {
            a: if color.eq(&Color::BLACK) { 0.7 } else { 0.2 },
            ..color
        }
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        get_colors(self.0).text_body
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        let color = get_colors(self.0).text_body;
        Color {
            a: if color.eq(&Color::BLACK) { 0.4 } else { 0.05 },
            ..color
        }
    }

    fn hovered(&self, _: &Self::Style) -> iced::widget::text_input::Appearance {
        let colors = get_colors(self.0);
        Appearance {
            background: Background::Color(colors.buttons),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: colors.secondary,
        }
    }
}
