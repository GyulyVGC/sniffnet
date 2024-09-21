//! Slider style

#![allow(clippy::module_name_repetitions)]

use iced::widget::slider::Style;
use iced::widget::slider::{Catalog, Handle, HandleShape, Rail, Status};
use iced::{Background, Border};

use crate::gui::styles::style_constants::{BORDER_ROUNDED_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::palette::mix_colors;
use crate::StyleType;

#[derive(Default)]
pub enum SliderType {
    #[default]
    Standard,
}

impl SliderType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(mix_colors(colors.secondary, ext.buttons_color)),
                    Background::Color(ext.buttons_color),
                ),
                width: 3.0,
                border: Border {
                    radius: BORDER_ROUNDED_RADIUS.into(),
                    ..Default::default()
                },
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 5.5 },
                background: Background::Color(mix_colors(colors.secondary, ext.buttons_color)),
                border_width: 0.0,
                border_color: colors.secondary,
            },
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(colors.secondary),
                    Background::Color(ext.buttons_color),
                ),
                width: 3.0,
                border: Border {
                    radius: BORDER_ROUNDED_RADIUS.into(),
                    ..Default::default()
                },
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                background: Background::Color(colors.secondary),
                border_width: 0.0,
                border_color: colors.secondary,
            },
        }
    }

    #[allow(clippy::unused_self)]
    fn dragging(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(colors.secondary),
                    Background::Color(ext.buttons_color),
                ),
                width: 3.0,
                border: Border {
                    radius: BORDER_ROUNDED_RADIUS.into(),
                    ..Default::default()
                },
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                background: Background::Color(colors.secondary),
                border_width: BORDER_WIDTH,
                border_color: mix_colors(colors.secondary, ext.buttons_color),
            },
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = SliderType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Dragged => class.dragging(self),
        }
    }
}
