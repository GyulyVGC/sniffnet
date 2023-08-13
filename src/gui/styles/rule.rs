//! Rule style

use iced::widget::rule;
use iced::widget::rule::FillMode;
use iced::Color;

use crate::gui::styles::style_constants::get_alpha_round_borders;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum RuleType {
    #[default]
    Standard,
    PalettePrimary,
    PaletteSecondary,
    PaletteOutgoing,
    PaletteButtons,
    Incoming,
    Outgoing,
}

impl rule::StyleSheet for StyleType {
    type Style = RuleType;

    fn appearance(&self, style: &Self::Style) -> iced::widget::rule::Appearance {
        let colors = get_colors(*self);
        iced::widget::rule::Appearance {
            color: match style {
                RuleType::Incoming | RuleType::PaletteSecondary => colors.secondary,
                RuleType::Outgoing | RuleType::PaletteOutgoing => colors.outgoing,
                RuleType::PalettePrimary => colors.primary,
                RuleType::PaletteButtons => colors.buttons,
                RuleType::Standard => Color {
                    a: get_alpha_round_borders(*self),
                    ..colors.buttons
                },
            },
            width: match style {
                RuleType::Incoming | RuleType::Outgoing => 5,
                RuleType::PalettePrimary
                | RuleType::PaletteSecondary
                | RuleType::PaletteOutgoing
                | RuleType::PaletteButtons => match self {
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

#[derive(Clone)]
pub struct RuleStyleTuple(pub StyleType, pub RuleType);

impl From<RuleStyleTuple> for RuleType {
    fn from(value: RuleStyleTuple) -> Self {
        value.1
    }
}

impl rule::StyleSheet for RuleStyleTuple {
    type Style = ();

    fn appearance(&self, _: &Self::Style) -> iced::widget::rule::Appearance {
        let colors = get_colors(self.0);
        iced::widget::rule::Appearance {
            color: match self.1 {
                RuleType::PaletteSecondary => colors.secondary,
                RuleType::PaletteOutgoing => colors.outgoing,
                RuleType::PalettePrimary => colors.primary,
                RuleType::PaletteButtons => colors.buttons,
                _ => Color::TRANSPARENT,
            },
            width: match self.1 {
                RuleType::PalettePrimary
                | RuleType::PaletteSecondary
                | RuleType::PaletteOutgoing
                | RuleType::PaletteButtons => match self.0 {
                    StyleType::Custom(_) => 25,
                    _ => 40,
                },
               _ => 0,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}
