//! SVG style

use iced::widget::svg::Appearance;

use crate::StyleType;

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
