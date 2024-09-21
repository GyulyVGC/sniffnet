//! SVG style

#![allow(clippy::module_name_repetitions)]

use iced::widget::svg::{Catalog, Status, Style};

use crate::StyleType;

#[derive(Default)]
pub enum SvgType {
    AdaptColor,
    #[default]
    Standard,
}

impl SvgType {
    fn appearance(&self, style: &StyleType) -> Style {
        Style {
            color: match self {
                SvgType::AdaptColor => Some(style.get_palette().text_body),
                SvgType::Standard => None,
            },
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = SvgType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Idle | Status::Hovered => class.appearance(self),
        }
    }
}
