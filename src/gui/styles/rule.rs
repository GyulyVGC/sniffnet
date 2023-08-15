//! Rule style

#![allow(clippy::module_name_repetitions)]

use iced::widget::rule;
use iced::widget::rule::FillMode;
use iced::Color;

use crate::gui::styles::style_constants::get_alpha_round_borders;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum RuleType {
    #[default]
    Standard,
    PalettePrimary(StyleType),
    PaletteSecondary(StyleType),
    PaletteOutgoing(StyleType),
    PaletteButtons(StyleType),
    Incoming,
    Outgoing,
}

impl rule::StyleSheet for StyleType {
    type Style = RuleType;

    fn appearance(&self, style: &Self::Style) -> iced::widget::rule::Appearance {
        let colors = get_colors(*self);
        iced::widget::rule::Appearance {
            color: match style {
                RuleType::Incoming => colors.secondary,
                RuleType::Outgoing => colors.outgoing,
                RuleType::PalettePrimary(style) => get_colors(*style).primary,
                RuleType::PaletteSecondary(style) => get_colors(*style).secondary,
                RuleType::PaletteOutgoing(style) => get_colors(*style).outgoing,
                RuleType::PaletteButtons(style) => get_colors(*style).buttons,
                RuleType::Standard => Color {
                    a: get_alpha_round_borders(*self),
                    ..colors.buttons
                },
            },
            width: match style {
                RuleType::Incoming | RuleType::Outgoing => 5,
                RuleType::PalettePrimary(style)
                | RuleType::PaletteSecondary(style)
                | RuleType::PaletteOutgoing(style)
                | RuleType::PaletteButtons(style) => match style {
                    StyleType::Custom(_) => 25,
                    _ => 40,
                },
                RuleType::Standard => 3,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}
