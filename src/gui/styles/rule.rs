//! Rule style

#![allow(clippy::module_name_repetitions)]

use iced::widget::rule;
use iced::widget::rule::FillMode;
use iced::Color;

use crate::StyleType;

#[derive(Clone, Copy, Default)]
pub enum RuleType {
    #[default]
    Standard,
    PaletteColor(Color, u16),
    Incoming,
    Outgoing,
}

impl rule::StyleSheet for StyleType {
    type Style = RuleType;

    fn appearance(&self, style: &Self::Style) -> iced::widget::rule::Appearance {
        let colors = self.get_palette();
        let ext = self.get_extension();
        iced::widget::rule::Appearance {
            color: match style {
                RuleType::Incoming => colors.secondary,
                RuleType::Outgoing => colors.outgoing,
                RuleType::PaletteColor(color, _) => *color,
                RuleType::Standard => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                },
            },
            width: match style {
                RuleType::Incoming | RuleType::Outgoing => 5,
                RuleType::PaletteColor(_, width) => *width,
                RuleType::Standard => 3,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}
