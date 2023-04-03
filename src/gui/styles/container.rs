//! Containers style

use iced::widget::container::Appearance;
use iced::Theme;
use iced::{Background, Color};

use crate::get_colors;
use crate::gui::styles::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;

impl From<StyleTuple> for iced::theme::Container {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Container::Custom(Box::new(tuple))
    }
}

impl iced::widget::container::StyleSheet for StyleTuple {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            text_color: Some(match self {
                StyleTuple(_, ElementType::Headers) => colors.text_headers,
                _ => colors.text_body,
            }),
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::Headers) => colors.secondary,
                StyleTuple(_, ElementType::Tooltip) => colors.buttons,
                StyleTuple(_, ElementType::BorderedRound) => colors.round_containers,
                _ => colors.primary,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::BorderedRound | ElementType::Alert) => {
                    BORDER_ROUNDED_RADIUS
                }
                StyleTuple(_, ElementType::Tooltip) => 7.0,
                _ => 0.0,
            },
            border_width: match self {
                StyleTuple(_, ElementType::Standard | ElementType::Headers) => 0.0,
                StyleTuple(_, ElementType::Tooltip) => BORDER_WIDTH / 2.0,
                StyleTuple(_, ElementType::BorderedRound) => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                StyleTuple(_, ElementType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                _ => colors.round_borders,
            },
        }
    }
}
