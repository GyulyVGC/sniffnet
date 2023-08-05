//! Containers style

use iced::widget::container::Appearance;
use iced::Theme;
use iced::{Background, Color};

use crate::gui::styles::style_constants::{
    get_alpha_chart_badge, get_alpha_round_borders, get_alpha_round_containers,
    BORDER_ROUNDED_RADIUS, BORDER_WIDTH,
};
use crate::gui::styles::types::gradient_type::{get_gradient_headers, GradientType};
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum ContainerType {
    Standard,
    Headers,
    BorderedRound,
    BorderedRoundSelected,
    Tooltip,
    Badge,
    Palette,
    Neutral,
    Alert,
    Gradient(GradientType),
    Modal,
}

#[derive(Clone)]
pub struct ContainerStyleTuple(pub StyleType, pub ContainerType);

impl From<ContainerStyleTuple> for iced::theme::Container {
    fn from(tuple: ContainerStyleTuple) -> Self {
        iced::theme::Container::Custom(Box::new(tuple))
    }
}

impl iced::widget::container::StyleSheet for ContainerStyleTuple {
    type Style = Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        let colors = get_colors(self.0);
        Appearance {
            text_color: Some(match self {
                ContainerStyleTuple(_, ContainerType::Gradient(_)) => colors.text_headers,
                _ => colors.text_body,
            }),
            background: Some(match self {
                ContainerStyleTuple(_, ContainerType::Gradient(GradientType::None)) => {
                    Background::Color(colors.secondary)
                }
                ContainerStyleTuple(_, ContainerType::Tooltip) => Background::Color(colors.buttons),
                ContainerStyleTuple(_, ContainerType::BorderedRound) => Background::Color(Color {
                    a: get_alpha_round_containers(self.0),
                    ..colors.buttons
                }),
                ContainerStyleTuple(_, ContainerType::Neutral | ContainerType::Palette) => {
                    Background::Color(Color::TRANSPARENT)
                }
                ContainerStyleTuple(_, ContainerType::Badge) => Background::Color(Color {
                    a: get_alpha_chart_badge(self.0),
                    ..colors.secondary
                }),
                ContainerStyleTuple(_, ContainerType::Gradient(gradient_type)) => {
                    Background::Gradient(get_gradient_headers(
                        &colors,
                        *gradient_type,
                        self.0.is_nightly(),
                    ))
                }
                _ => Background::Color(colors.primary),
            }),
            border_radius: match self {
                ContainerStyleTuple(_, ContainerType::BorderedRound | ContainerType::Alert) => {
                    BORDER_ROUNDED_RADIUS.into()
                }
                ContainerStyleTuple(_, ContainerType::Modal) => {
                    [0.0, 0.0, BORDER_ROUNDED_RADIUS, BORDER_ROUNDED_RADIUS].into()
                }
                ContainerStyleTuple(_, ContainerType::Tooltip) => 7.0.into(),
                ContainerStyleTuple(_, ContainerType::Badge) => 100.0.into(),
                _ => 0.0.into(),
            },
            border_width: match self {
                ContainerStyleTuple(
                    _,
                    ContainerType::Standard
                    | ContainerType::Modal
                    | ContainerType::Neutral
                    | ContainerType::Gradient(_),
                ) => 0.0,
                ContainerStyleTuple(_, ContainerType::Tooltip) => BORDER_WIDTH / 2.0,
                ContainerStyleTuple(_, ContainerType::BorderedRound) => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                ContainerStyleTuple(_, ContainerType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                ContainerStyleTuple(_, ContainerType::Palette) => Color::BLACK,
                _ => Color {
                    a: get_alpha_round_borders(self.0),
                    ..colors.buttons
                },
            },
        }
    }
}
