//! Toggler style

#![allow(clippy::module_name_repetitions)]

use iced::widget::toggler::{Catalog, Status, Style};
use iced::{Background, Color};

use crate::StyleType;
use crate::gui::styles::style_constants::BORDER_WIDTH;

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
            background: Background::Color(bg_color),
            background_border_width: BORDER_WIDTH,
            background_border_color: bg_color,
            foreground: Background::Color(colors.primary),
            foreground_border_width: BORDER_WIDTH,
            foreground_border_color: if is_active {
                colors.secondary
            } else {
                Color::TRANSPARENT
            },
            text_color: None,
            border_radius: None,
            padding_ratio: 0.0,
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
            background: Background::Color(bg_color),
            background_border_width: BORDER_WIDTH,
            background_border_color: colors.secondary,
            foreground: Background::Color(colors.primary),
            foreground_border_width: BORDER_WIDTH,
            foreground_border_color: if is_active {
                colors.secondary
            } else {
                Color::TRANSPARENT
            },
            text_color: None,
            border_radius: None,
            padding_ratio: 0.0,
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
            Status::Active { is_toggled } | Status::Disabled { is_toggled } => {
                class.active(self, is_toggled)
            }
            Status::Hovered { is_toggled } => class.hovered(self, is_toggled),
        }
    }
}
