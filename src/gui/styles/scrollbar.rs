//! Scrollbars style

use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::Theme;
use iced::{Background, Color};

use crate::get_colors;
use crate::gui::styles::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::style_tuple::StyleTuple;

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
            background: Some(Background::Color(colors.round_borders)),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.primary,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: BORDER_WIDTH * 1.5,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.round_borders)),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: BORDER_WIDTH / 1.5,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.secondary,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: if is_mouse_over_scrollbar {
                    BORDER_WIDTH * 1.25
                } else {
                    BORDER_WIDTH * 1.75
                },
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
