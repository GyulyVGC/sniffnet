//! Rule style

#![allow(clippy::module_name_repetitions)]

use crate::StyleType;
use crate::gui::types::message::Message;
use iced::widget::rule::{Catalog, FillMode, Style};
use iced::widget::{Container, rule};
use iced::{Alignment, Color, Length};

#[derive(Default)]
pub enum RuleType {
    #[default]
    Standard,
    PaletteColor(Color),
    Incoming,
    Outgoing,
    Dropped,
}

impl RuleType {
    fn appearance(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            color: match self {
                RuleType::Incoming => colors.secondary,
                RuleType::Outgoing => colors.outgoing,
                RuleType::PaletteColor(color) => *color,
                RuleType::Dropped => ext.buttons_color,
                RuleType::Standard => Color {
                    a: ext.alpha_round_borders,
                    ..ext.buttons_color
                },
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
            snap: true,
        }
    }

    fn thickness(&self) -> u32 {
        match self {
            RuleType::Standard => 3,
            RuleType::PaletteColor(_) => 25,
            RuleType::Dropped | RuleType::Incoming | RuleType::Outgoing => 5,
        }
    }

    pub fn horizontal<'a>(self, height: impl Into<Length>) -> Container<'a, Message, StyleType> {
        let rule = rule::horizontal(self.thickness()).class(self);
        Container::new(rule)
            .height(height)
            .align_y(Alignment::Center)
    }

    pub fn vertical<'a>(self, width: impl Into<Length>) -> Container<'a, Message, StyleType> {
        let rule = rule::vertical(self.thickness()).class(self);
        Container::new(rule).width(width).align_x(Alignment::Center)
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
