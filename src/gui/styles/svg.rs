//! SVG style

#![allow(clippy::module_name_repetitions)]

use iced::widget::svg::Appearance;

use crate::StyleType;

#[derive(Clone, Copy, Default)]
pub enum SvgType {
    AdaptColor,
    #[default]
    Standard,
}

impl iced::widget::svg::StyleSheet for StyleType {
    type Style = SvgType;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance {
            color: match style {
                SvgType::AdaptColor => Some(self.get_palette().text_body),
                SvgType::Standard => None,
            },
        }
    }
}
