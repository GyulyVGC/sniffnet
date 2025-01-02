//! Scrollbars style

#![allow(clippy::module_name_repetitions)]

use iced::widget::container;
use iced::widget::scrollable::{Catalog, Rail, Status, Style};
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::{Background, Border, Color};

use crate::gui::styles::style_constants::BORDER_ROUNDED_RADIUS;
use crate::gui::styles::types::palette::mix_colors;
use crate::StyleType;

#[derive(Default)]
pub enum ScrollbarType {
    #[default]
    Standard,
}

impl ScrollbarType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType) -> Style {
        let ext = style.get_extension();

        let rail = Rail {
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
        };

        Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType, is_mouse_over_x: bool, is_mouse_over_y: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();

        let [horizontal_rail, vertical_rail] =
            [is_mouse_over_x, is_mouse_over_y].map(|is_over| Rail {
                background: Some(Background::Color(Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                })),
                scroller: Scroller {
                    color: if is_over {
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
            });

        Style {
            container: container::Style::default(),
            vertical_rail,
            horizontal_rail,
            gap: None,
        }
    }

    pub fn properties() -> Scrollbar {
        Scrollbar::new().width(5).scroller_width(5).margin(3)
    }
}

impl Catalog for StyleType {
    type Class<'a> = ScrollbarType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
            } => class.hovered(
                self,
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
            ),
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
            } => class.hovered(
                self,
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
            ),
        }
    }
}
