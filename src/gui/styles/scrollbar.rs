//! Scrollbars style

use iced::widget::scrollable::Properties;
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::Theme;
use iced::{Background, Color};

use crate::gui::styles::style_constants::BORDER_ROUNDED_RADIUS;
use crate::gui::styles::types::palette::mix_colors;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum ScrollbarType {
    Standard,
}

impl ScrollbarType {
    pub fn properties() -> Properties {
        Properties::new().width(5).scroller_width(5).margin(3)
    }
}

#[derive(Clone)]
pub struct ScrollbarStyleTuple(pub StyleType, pub ScrollbarType);

impl From<ScrollbarStyleTuple> for iced::theme::Scrollable {
    fn from(tuple: ScrollbarStyleTuple) -> Self {
        iced::theme::Scrollable::Custom(Box::new(tuple))
    }
}

impl iced::widget::scrollable::StyleSheet for ScrollbarStyleTuple {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.round_borders,
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.round_borders)),
            border_radius: BORDER_ROUNDED_RADIUS.into(),
            border_width: 0.0,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: if is_mouse_over_scrollbar {
                    colors.secondary
                } else {
                    mix_colors(colors.secondary, colors.buttons)
                },
                border_radius: BORDER_ROUNDED_RADIUS.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
