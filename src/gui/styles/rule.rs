//! Rule style

use iced_native::widget::rule;
use iced_native::widget::rule::FillMode;

use crate::get_colors;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;

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
                ElementType::Incoming => colors.secondary,
                ElementType::Outgoing => colors.outgoing,
                _ => colors.round_borders,
            },
            width: match self.1 {
                ElementType::Incoming | ElementType::Outgoing => 5,
                _ => 3,
            },
            radius: 0.0,
            fill_mode: FillMode::Full,
        }
    }
}
