//! Containers style

use iced::widget::container::Appearance;
use iced::Theme;
use iced::{Background, Color};

use crate::gui::styles::style_constants::{
    get_color_mix_filter_badge, BORDER_ROUNDED_RADIUS, BORDER_WIDTH,
};
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
                ContainerStyleTuple(_, ContainerType::Headers) => colors.text_headers,
                _ => colors.text_body,
            }),
            background: Some(Background::Color(match self {
                ContainerStyleTuple(_, ContainerType::Headers) => colors.secondary,
                ContainerStyleTuple(_, ContainerType::Tooltip) => colors.buttons,
                ContainerStyleTuple(_, ContainerType::BorderedRound) => colors.round_containers,
                ContainerStyleTuple(_, ContainerType::Neutral | ContainerType::Palette) => {
                    Color::TRANSPARENT
                }
                ContainerStyleTuple(_, ContainerType::Badge) => Color {
                    a: get_color_mix_filter_badge(self.0),
                    ..colors.secondary
                },
                _ => colors.primary,
            })),
            border_radius: match self {
                ContainerStyleTuple(_, ContainerType::BorderedRound | ContainerType::Alert) => {
                    BORDER_ROUNDED_RADIUS.into()
                }
                ContainerStyleTuple(_, ContainerType::Tooltip) => 7.0.into(),
                ContainerStyleTuple(_, ContainerType::Badge) => 100.0.into(),
                _ => 0.0.into(),
            },
            border_width: match self {
                ContainerStyleTuple(
                    _,
                    ContainerType::Standard | ContainerType::Headers | ContainerType::Neutral,
                ) => 0.0,
                ContainerStyleTuple(_, ContainerType::Tooltip) => BORDER_WIDTH / 2.0,
                ContainerStyleTuple(_, ContainerType::BorderedRound) => BORDER_WIDTH * 2.0,
                _ => BORDER_WIDTH,
            },
            border_color: match self {
                ContainerStyleTuple(_, ContainerType::Alert) => Color::new(1.0, 0.0, 0.0, 1.0),
                ContainerStyleTuple(_, ContainerType::Palette) => Color::BLACK,
                _ => colors.round_borders,
            },
        }
    }
}
