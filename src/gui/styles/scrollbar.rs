//! Scrollbars style

use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::Background;
use iced::Theme;

impl From<StyleTuple> for iced::theme::Scrollable {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Scrollable::Custom(Box::new(tuple))
    }
}

impl iced::widget::scrollable::StyleSheet for StyleTuple {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.buttons)),
            border_radius: BORDER_ROUNDED_RADIUS,
            border_width: 0.0,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.primary,
                border_radius: BORDER_ROUNDED_RADIUS,
                border_width: BORDER_WIDTH / 1.5,
                border_color: colors.round_borders,
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.buttons)),
            border_radius: BORDER_ROUNDED_RADIUS,
            border_width: BORDER_WIDTH / 1.5,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.secondary,
                border_radius: BORDER_ROUNDED_RADIUS,
                border_width: BORDER_WIDTH / 1.5,
                border_color: colors.round_borders,
            },
        }
    }

    fn dragging(&self, _: &Self::Style) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.buttons)),
            border_radius: BORDER_ROUNDED_RADIUS,
            border_width: BORDER_WIDTH / 1.5,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.secondary,
                border_radius: BORDER_ROUNDED_RADIUS,
                border_width: BORDER_WIDTH / 1.5,
                border_color: colors.round_borders,
            },
        }
    }
}
