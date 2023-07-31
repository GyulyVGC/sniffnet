//! Rule style

use crate::{get_colors, StyleType};
use iced_widget::rule;
use iced_widget::rule::FillMode;

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
pub struct RuleTuple(pub StyleType, pub RuleType);

impl From<RuleTuple> for iced::theme::Rule {
    fn from(tuple: RuleTuple) -> Self {
        iced::theme::Rule::Custom(Box::new(tuple))
    }
}

impl rule::StyleSheet for RuleTuple {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> iced::widget::rule::Appearance {
        let colors = get_colors(self.0);
        iced::widget::rule::Appearance {
            color: match self.1 {
                RuleType::Incoming | RuleType::PaletteSecondary => colors.secondary,
                RuleType::Outgoing | RuleType::PaletteOutgoing => colors.outgoing,
                RuleType::PalettePrimary => colors.primary,
                RuleType::PaletteButtons => colors.buttons,
                RuleType::Standard => colors.round_borders,
            },
            width: match self.1 {
                RuleType::Incoming | RuleType::Outgoing => 5,
                RuleType::PalettePrimary
                | RuleType::PaletteSecondary
                | RuleType::PaletteOutgoing
                | RuleType::PaletteButtons => 50,
                RuleType::Standard => 3,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}
