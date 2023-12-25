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
        let colors = self.get_palette();
        let ext = self.get_palette_extension();
        iced::widget::rule::Appearance {
            color: match style {
                RuleType::Incoming => colors.secondary,
                RuleType::Outgoing => colors.outgoing,
                RuleType::PalettePrimary(style) => style.get_palette().primary,
                RuleType::PaletteSecondary(style) => style.get_palette().secondary,
                RuleType::PaletteOutgoing(style) => style.get_palette().outgoing,
                RuleType::PaletteButtons(style) => style.get_palette_extension().buttons_color,
                RuleType::Standard => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
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
