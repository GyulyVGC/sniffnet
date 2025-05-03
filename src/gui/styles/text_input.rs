//! Text Input style

#![allow(clippy::module_name_repetitions)]

use iced::widget::text_input::{Catalog, Status, Style};
use iced::{Background, Border, Color};

use crate::StyleType;
use crate::gui::styles::style_constants::BORDER_WIDTH;

#[derive(Default)]
pub enum TextInputType {
    #[default]
    Standard,
    Badge,
    Error,
}

const TEXT_INPUT_BORDER_RADIUS: f32 = 5.0;

impl TextInputType {
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(match self {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                },
            }),
            border: Border {
                radius: TEXT_INPUT_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: match self {
                    TextInputType::Badge => Color::TRANSPARENT,
                    TextInputType::Standard => ext.buttons_color,
                    TextInputType::Error => ext.red_alert_color,
                },
            },
            icon: Color {
                a: if ext.is_nightly { 0.2 } else { 0.7 },
                ..colors.text_body
            },
            placeholder: self.placeholder_color(style),
            value: self.value_color(style),
            selection: self.selection_color(style),
        }
    }

    fn focused(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        let is_nightly = style.get_extension().is_nightly;
        Style {
            background: Background::Color(colors.primary),
            border: Border {
                radius: TEXT_INPUT_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: match self {
                    TextInputType::Error => ext.red_alert_color,
                    _ => colors.secondary,
                },
            },
            icon: Color {
                a: if is_nightly { 0.2 } else { 0.7 },
                ..colors.text_body
            },
            placeholder: self.placeholder_color(style),
            value: self.value_color(style),
            selection: self.selection_color(style),
        }
    }

    #[allow(clippy::unused_self)]
    fn placeholder_color(&self, style: &StyleType) -> Color {
        let color = style.get_palette().text_body;
        let is_nightly = style.get_extension().is_nightly;
        Color {
            a: if is_nightly { 0.2 } else { 0.7 },
            ..color
        }
    }

    #[allow(clippy::unused_self)]
    fn value_color(&self, style: &StyleType) -> Color {
        style.get_palette().text_body
    }

    #[allow(clippy::unused_self)]
    fn disabled_color(&self, style: &StyleType) -> Color {
        let color = style.get_palette().text_body;
        let is_nightly = style.get_extension().is_nightly;
        Color {
            a: if is_nightly { 0.2 } else { 0.7 },
            ..color
        }
    }

    #[allow(clippy::unused_self)]
    fn selection_color(&self, style: &StyleType) -> Color {
        let color = style.get_palette().text_body;
        let is_nightly = style.get_extension().is_nightly;
        Color {
            a: if is_nightly { 0.05 } else { 0.4 },
            ..color
        }
    }

    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(match self {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => ext.buttons_color,
            }),
            border: Border {
                radius: TEXT_INPUT_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: match self {
                    TextInputType::Error => ext.red_alert_color,
                    _ => colors.secondary,
                },
            },
            icon: Color {
                a: if ext.is_nightly { 0.2 } else { 0.7 },
                ..colors.text_body
            },
            placeholder: self.placeholder_color(style),
            value: self.value_color(style),
            selection: self.selection_color(style),
        }
    }

    fn disabled(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(match self {
                TextInputType::Badge => Color::TRANSPARENT,
                _ => Color {
                    a: ext.alpha_round_containers,
                    ..ext.buttons_color
                },
            }),
            border: Border {
                radius: TEXT_INPUT_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: match self {
                    TextInputType::Badge => Color::TRANSPARENT,
                    TextInputType::Standard => Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    TextInputType::Error => Color {
                        a: ext.alpha_round_borders,
                        ..ext.red_alert_color
                    },
                },
            },
            icon: Color {
                a: if ext.is_nightly { 0.2 } else { 0.7 },
                ..colors.text_body
            },
            placeholder: self.disabled_color(style),
            value: self.disabled_color(style),
            selection: self.disabled_color(style),
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = TextInputType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Disabled => class.disabled(self),
            Status::Focused => class.focused(self),
        }
    }
}
