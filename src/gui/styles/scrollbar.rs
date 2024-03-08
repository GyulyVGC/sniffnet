//! Scrollbars style

#![allow(clippy::module_name_repetitions)]

use iced::widget::container;
use iced::widget::scrollable::{Appearance, Properties};
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::{Background, Border, Color};

use crate::gui::styles::style_constants::BORDER_ROUNDED_RADIUS;
use crate::gui::styles::types::palette::mix_colors;
use crate::StyleType;

#[derive(Clone, Copy, Default)]
pub enum ScrollbarType {
    #[default]
    Standard,
}

impl ScrollbarType {
    pub fn properties() -> Properties {
        Properties::new().width(5).scroller_width(5).margin(3)
    }
}

impl iced::widget::scrollable::StyleSheet for StyleType {
    type Style = ScrollbarType;

    fn active(&self, _: &Self::Style) -> Appearance {
        let ext = self.get_extension();
        Appearance {
            container: container::Appearance::default(),
            scrollbar: Scrollbar {
                background: Some(Background::Color(Color::TRANSPARENT)),
                scroller: Scroller {
                    color: Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    border: Border {
                        radius: BORDER_ROUNDED_RADIUS.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                },
                border: Border {
                    radius: BORDER_ROUNDED_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, _: &Self::Style, is_mouse_over_scrollbar: bool) -> Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        Appearance {
            container: container::Appearance::default(),
            scrollbar: Scrollbar {
                background: Some(Background::Color(Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                })),
                scroller: Scroller {
                    color: if is_mouse_over_scrollbar {
                        colors.secondary
                    } else {
                        mix_colors(colors.secondary, ext.buttons_color)
                    },
                    border: Border {
                        radius: BORDER_ROUNDED_RADIUS.into(),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                },
                border: Border {
                    radius: BORDER_ROUNDED_RADIUS.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
            },
            gap: None,
        }
    }
}
