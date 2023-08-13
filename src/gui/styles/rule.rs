//! Rule style

use iced::widget::rule;
use iced::widget::rule::FillMode;
use iced::Color;

use crate::gui::styles::style_constants::get_alpha_round_borders;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum RuleType {
    Standard,
    PalettePrimary,
    PaletteSecondary,
    PaletteOutgoing,
    PaletteButtons,
    Incoming,
    Outgoing,
}

#[derive(Clone)]
pub struct RuleStyleTuple(pub StyleType, pub RuleType);

// impl From<RuleStyleTuple> for iced::theme::Rule {
//     fn from(tuple: RuleStyleTuple) -> Self {
//         iced::theme::Rule::Custom(Box::new(tuple))
//     }
// }

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
