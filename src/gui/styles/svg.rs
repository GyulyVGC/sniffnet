//! SVG style

#![allow(clippy::module_name_repetitions)]

use iced::widget::svg::{Catalog, Status, Style};

use crate::StyleType;

#[derive(Default)]
pub enum SvgType {
    #[default]
    Standard,
}

impl SvgType {
    fn appearance(&self, _style: &StyleType) -> Style {
        Style {
            color: match self {
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
