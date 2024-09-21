//! Checkbox style

#![allow(clippy::module_name_repetitions)]

use iced::widget::checkbox::{Catalog, Status, Style};
use iced::{Background, Border};

use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::StyleType;

#[derive(Default)]
pub enum CheckboxType {
    #[default]
    Standard,
}

const CHECKBOX_BORDER_RADIUS: f32 = 5.0;

impl CheckboxType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType, is_checked: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(ext.buttons_color),
            icon_color: colors.text_body,
            border: Border {
                radius: CHECKBOX_BORDER_RADIUS.into(),
                width: if is_checked { BORDER_WIDTH } else { 0.0 },
                color: colors.secondary,
            },
            text_color: None,
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType, _is_checked: bool) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            background: Background::Color(ext.buttons_color),
            icon_color: colors.text_body,
            border: Border {
                radius: CHECKBOX_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: colors.secondary,
            },
            text_color: None,
        }
    }
}

impl Catalog for StyleType {
    type Class<'a> = CheckboxType;

    fn default<'a>() -> Self::Class<'a> {
        Self::Class::default()
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active { is_checked } | Status::Disabled { is_checked } => {
                class.active(self, is_checked)
            }
            Status::Hovered { is_checked } => class.hovered(self, is_checked),
        }
    }
}
