//! SVG style

use crate::StyleType;
use iced::widget::svg::Appearance;

#[derive(Clone, Copy, Default)]
pub enum SvgType {
    #[default]
    Standard,
}

impl iced::widget::svg::StyleSheet for StyleType {
    type Style = SvgType;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance { color: None }
    }
}
