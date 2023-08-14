//! Containers style

use iced::widget::container::Appearance;
use iced::{Background, Color};

use crate::gui::styles::style_constants::{
    get_alpha_chart_badge, get_alpha_round_borders, get_alpha_round_containers,
    BORDER_ROUNDED_RADIUS, BORDER_WIDTH,
};
use crate::gui::styles::types::gradient_type::{get_gradient_headers, GradientType};
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum ContainerType {
    #[default]
    Standard,
    BorderedRound,
    Tooltip,
    Badge,
    Palette,
    Neutral,
    Gradient(GradientType),
    Modal,
}

#[derive(Clone)]
pub struct ContainerStyleTuple(pub StyleType, pub ContainerType);

impl iced::widget::container::StyleSheet for StyleType {
    type Style = ContainerType;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let colors = get_colors(*self);
        Appearance {
            text_color: Some(match style {
                ContainerType::Gradient(_) => colors.text_headers,
                _ => colors.text_body,
            }),
            background: Some(match style {
                ContainerType::Gradient(GradientType::None) => Background::Color(colors.secondary),
                ContainerType::Tooltip => Background::Color(colors.buttons),
                ContainerType::BorderedRound => Background::Color(Color {
                    a: get_alpha_round_containers(*self),
                    ..colors.buttons
                }),
                ContainerType::Neutral | ContainerType::Palette => {
                    Background::Color(Color::TRANSPARENT)
                }
                ContainerType::Badge => Background::Color(Color {
                    a: get_alpha_chart_badge(*self),
                    ..colors.secondary
                }),
                ContainerType::Gradient(gradient_type) => Background::Gradient(
                    get_gradient_headers(&colors, *gradient_type, self.is_nightly()),
                ),
                ContainerType::Modal => Background::Color(colors.primary),
                _ => Background::Color(Color::TRANSPARENT),
            }),
            border_radius: match style {
                ContainerType::BorderedRound => BORDER_ROUNDED_RADIUS.into(),
                ContainerType::Modal => {
                    [0.0, 0.0, BORDER_ROUNDED_RADIUS, BORDER_ROUNDED_RADIUS].into()
                }
                ContainerType::Tooltip => 7.0.into(),
                ContainerType::Badge => 100.0.into(),
                _ => 0.0.into(),
            },
            border_width: match style {
                ContainerType::Standard
                | ContainerType::Modal
                | ContainerType::Neutral
                | ContainerType::Gradient(_) => 0.0,
                ContainerType::Tooltip => BORDER_WIDTH / 2.0,
                ContainerType::BorderedRound => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            border_color: match style {
                ContainerType::Palette => Color::BLACK,
                _ => Color {
                    a: get_alpha_round_borders(*self),
                    ..colors.buttons
                },
            },
        }
    }
}
