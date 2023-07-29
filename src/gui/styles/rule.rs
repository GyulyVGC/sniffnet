//! Rule style

use crate::get_colors;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use iced_widget::rule;
use iced_widget::rule::FillMode;

impl From<StyleTuple> for iced::theme::Rule {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::Rule::Custom(Box::new(tuple))
    }
}

impl rule::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> iced::widget::rule::Appearance {
        let colors = get_colors(self.0);
        iced::widget::rule::Appearance {
            color: match self.1 {
                ElementType::Incoming | ElementType::PaletteSecondary => colors.secondary,
                ElementType::Outgoing | ElementType::PaletteOutgoing => colors.outgoing,
                ElementType::PalettePrimary => colors.primary,
                ElementType::PaletteButtons => colors.buttons,
                _ => colors.round_borders,
            },
            width: match self.1 {
                ElementType::Incoming | ElementType::Outgoing => 5,
                ElementType::PalettePrimary
                | ElementType::PaletteSecondary
                | ElementType::PaletteOutgoing
                | ElementType::PaletteButtons => 50,
                _ => 3,
            },
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}
