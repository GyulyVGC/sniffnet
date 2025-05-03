//! Buttons style

#![allow(clippy::module_name_repetitions)]

use iced::border::Radius;
use iced::widget::button;
use iced::widget::button::{Catalog, Status, Style};
use iced::{Background, Border, Color, Shadow, Vector};

use crate::StyleType;
use crate::gui::styles::style_constants::{BORDER_BUTTON_RADIUS, BORDER_WIDTH};
use crate::gui::styles::types::gradient_type::{
    GradientType, get_gradient_buttons, get_gradient_hovered_buttons,
};
use crate::gui::styles::types::palette::mix_colors;

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Standard,
    BorderedRound,
    BorderedRoundSelected,
    TabActive,
    TabInactive,
    Starred,
    NotStarred,
    Neutral,
    Alert,
    Gradient(GradientType),
    SortArrows,
    SortArrowActive,
    Thumbnail,
}

impl ButtonType {
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        button::Style {
            background: Some(match self {
                ButtonType::TabActive | ButtonType::BorderedRoundSelected => {
                    Background::Color(mix_colors(colors.primary, ext.buttons_color))
                }
                ButtonType::Starred => Background::Color(colors.starred),
                ButtonType::BorderedRound => Background::Color(Color {
                    a: ext.alpha_round_containers,
                    ..ext.buttons_color
                }),
                ButtonType::Neutral
                | ButtonType::Thumbnail
                | ButtonType::NotStarred
                | ButtonType::SortArrows
                | ButtonType::SortArrowActive => Background::Color(Color::TRANSPARENT),
                ButtonType::Gradient(GradientType::None) => Background::Color(colors.secondary),
                ButtonType::Gradient(gradient_type) => Background::Gradient(get_gradient_buttons(
                    &colors,
                    *gradient_type,
                    ext.is_nightly,
                    1.0,
                )),
                _ => Background::Color(ext.buttons_color),
            }),
            border: Border {
                radius: match self {
                    ButtonType::Neutral => 0.0.into(),
                    ButtonType::TabActive | ButtonType::TabInactive => Radius::new(0).bottom(30),
                    ButtonType::BorderedRound | ButtonType::BorderedRoundSelected => 12.0.into(),
                    ButtonType::Starred | ButtonType::NotStarred => 100.0.into(),
                    _ => BORDER_BUTTON_RADIUS.into(),
                },
                width: match self {
                    ButtonType::TabActive
                    | ButtonType::TabInactive
                    | ButtonType::SortArrows
                    | ButtonType::SortArrowActive
                    | ButtonType::Starred
                    | ButtonType::NotStarred
                    | ButtonType::Neutral
                    | ButtonType::Thumbnail => 0.0,
                    ButtonType::BorderedRound => BORDER_WIDTH * 2.0,
                    _ => BORDER_WIDTH,
                },
                color: match self {
                    ButtonType::Alert => ext.red_alert_color,
                    ButtonType::BorderedRound => Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    _ => colors.secondary,
                },
            },
            text_color: match self {
                ButtonType::Starred => Color::BLACK,
                ButtonType::SortArrows => Color {
                    a: if ext.is_nightly { 0.2 } else { 0.7 },
                    ..colors.text_body
                },
                ButtonType::SortArrowActive => colors.secondary,
                ButtonType::Gradient(_) => colors.text_headers,
                ButtonType::Thumbnail => mix_colors(colors.text_headers, colors.secondary),
                _ => colors.text_body,
            },
            shadow: match self {
                ButtonType::TabActive | ButtonType::TabInactive => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(3.0, 2.0),
                    blur_radius: 4.0,
                },
                _ => Shadow::default(),
            },
        }
    }

    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        button::Style {
            shadow: match self {
                ButtonType::Neutral
                | ButtonType::SortArrows
                | ButtonType::SortArrowActive
                | ButtonType::Thumbnail => Shadow::default(),
                _ => Shadow {
                    color: Color::BLACK,
                    offset: match self {
                        ButtonType::TabActive | ButtonType::TabInactive => Vector::new(3.0, 3.0),
                        _ => Vector::new(0.0, 2.0),
                    },
                    blur_radius: match self {
                        ButtonType::TabActive | ButtonType::TabInactive => 4.0,
                        _ => 2.0,
                    },
                },
            },
            background: Some(match self {
                ButtonType::Starred => Background::Color(colors.starred),
                ButtonType::SortArrows | ButtonType::SortArrowActive | ButtonType::Thumbnail => {
                    Background::Color(Color::TRANSPARENT)
                }
                ButtonType::Neutral => Background::Color(Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                }),
                ButtonType::Gradient(GradientType::None) => {
                    Background::Color(mix_colors(colors.primary, colors.secondary))
                }
                ButtonType::Gradient(gradient_type) => Background::Gradient(
                    get_gradient_hovered_buttons(&colors, *gradient_type, ext.is_nightly),
                ),
                ButtonType::BorderedRoundSelected => Background::Color(ext.buttons_color),
                _ => Background::Color(mix_colors(colors.primary, ext.buttons_color)),
            }),
            border: Border {
                radius: match self {
                    ButtonType::Neutral => 0.0.into(),
                    ButtonType::TabActive | ButtonType::TabInactive => Radius::new(0).bottom(30),
                    ButtonType::BorderedRound | ButtonType::BorderedRoundSelected => 12.0.into(),
                    ButtonType::Starred | ButtonType::NotStarred => 100.0.into(),
                    _ => BORDER_BUTTON_RADIUS.into(),
                },
                width: match self {
                    ButtonType::Starred
                    | ButtonType::TabActive
                    | ButtonType::SortArrows
                    | ButtonType::SortArrowActive
                    | ButtonType::TabInactive
                    | ButtonType::Thumbnail
                    | ButtonType::BorderedRound => 0.0,
                    _ => BORDER_WIDTH,
                },
                color: match self {
                    ButtonType::Alert => ext.red_alert_color,
                    ButtonType::BorderedRound | ButtonType::NotStarred => Color {
                        a: ext.alpha_round_borders,
                        ..ext.buttons_color
                    },
                    ButtonType::Neutral => ext.buttons_color,
                    _ => colors.secondary,
                },
            },
            text_color: match self {
                ButtonType::Starred => Color::BLACK,
                ButtonType::Gradient(_) | ButtonType::Thumbnail => colors.text_headers,
                ButtonType::SortArrowActive | ButtonType::SortArrows => colors.secondary,
                _ => colors.text_body,
            },
        }
    }

    fn disabled(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        match self {
            ButtonType::Gradient(_) => Style {
                background: Some(match self {
                    ButtonType::Gradient(GradientType::None) => Background::Color(Color {
                        a: ext.alpha_chart_badge,
                        ..colors.secondary
                    }),
                    ButtonType::Gradient(gradient_type) => {
                        Background::Gradient(get_gradient_buttons(
                            &colors,
                            *gradient_type,
                            ext.is_nightly,
                            ext.alpha_chart_badge,
                        ))
                    }
                    _ => Background::Color(ext.buttons_color),
                }),
                border: Border {
                    radius: BORDER_BUTTON_RADIUS.into(),
                    width: BORDER_WIDTH,
                    color: Color {
                        a: ext.alpha_chart_badge,
                        ..colors.secondary
                    },
                },
                text_color: Color {
                    a: ext.alpha_chart_badge,
                    ..colors.text_headers
                },
                shadow: Shadow::default(),
            },
            ButtonType::Standard => Style {
                background: Some(Background::Color(Color {
                    a: ext.alpha_chart_badge,
                    ..ext.buttons_color
                })),
                border: Border {
                    radius: BORDER_BUTTON_RADIUS.into(),
                    width: BORDER_WIDTH,
                    color: Color {
                        a: ext.alpha_chart_badge,
                        ..colors.secondary
                    },
                },
                text_color: Color {
                    a: ext.alpha_chart_badge,
                    ..colors.text_body
                },
                shadow: Shadow::default(),
            },
            _ => self.active(style),
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = ButtonType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active | Status::Pressed => class.active(self),
            Status::Hovered => class.hovered(self),
            Status::Disabled => class.disabled(self),
        }
    }
}
