//! Toggler style

#![allow(clippy::module_name_repetitions)]

use iced::widget::toggler::{Catalog, Status, Style};
use iced::Color;

use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::StyleType;

#[derive(Default)]
pub enum TogglerType {
    #[default]
    Standard,
}

impl TogglerType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType, is_active: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        let bg_color = if is_active {
            Color {
                a: ext.alpha_chart_badge,
                ..colors.secondary
            }
        } else {
            ext.buttons_color
        };
        Style {
            background: bg_color,
            background_border_width: BORDER_WIDTH,
            background_border_color: bg_color,
            foreground: colors.primary,
            foreground_border_width: BORDER_WIDTH,
            foreground_border_color: if is_active {
                colors.secondary
            } else {
                Color::TRANSPARENT
            },
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType, is_active: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        let bg_color = if is_active {
            Color {
                a: ext.alpha_chart_badge,
                ..colors.secondary
            }
        } else {
            ext.buttons_color
        };
        Style {
            background: bg_color,
            background_border_width: BORDER_WIDTH,
            background_border_color: colors.secondary,
            foreground: colors.primary,
            foreground_border_width: BORDER_WIDTH,
            foreground_border_color: if is_active {
                colors.secondary
            } else {
                Color::TRANSPARENT
            },
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = TogglerType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active { is_toggled } => class.active(self, is_toggled),
            Status::Hovered { is_toggled } => class.hovered(self, is_toggled),
            Status::Disabled => class.active(self, false),
        }
    }
}
