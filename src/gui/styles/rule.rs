//! Rule style

#![allow(clippy::module_name_repetitions)]

use iced::widget::rule::{Catalog, FillMode, Style};
use iced::Color;

use crate::StyleType;

#[derive(Default)]
pub enum RuleType {
    #[default]
    Standard,
    PaletteColor(Color, u16),
    Incoming,
    Outgoing,
}

impl RuleType {
    fn appearance(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            color: match self {
                RuleType::Incoming => colors.secondary,
                RuleType::Outgoing => colors.outgoing,
                RuleType::PaletteColor(color, _) => *color,
                RuleType::Standard => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                },
            },
            width: match self {
                RuleType::Incoming | RuleType::Outgoing => 5,
                RuleType::PaletteColor(_, width) => *width,
                RuleType::Standard => 3,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = RuleType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class.appearance(self)
    }
}
