//! Picklists style

#![allow(clippy::module_name_repetitions)]

use iced::widget::pick_list::{Catalog, Status, Style};
use iced::{Background, Border, Color};

use crate::StyleType;
use crate::gui::styles::style_constants::BORDER_WIDTH;
use crate::gui::styles::types::palette::mix_colors;

#[derive(Default)]
pub enum PicklistType {
    #[default]
    Standard,
}

const PICKLIST_BORDER_RADIUS: f32 = 8.0;

impl PicklistType {
    #[allow(clippy::unused_self)]
    fn appearance(&self, style: &StyleType) -> iced::overlay::menu::Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        iced::overlay::menu::Style {
            text_color: colors.text_body,
            background: Background::Color(ext.buttons_color),
            border: Border {
                width: BORDER_WIDTH,
                radius: PICKLIST_BORDER_RADIUS.into(),
                color: colors.secondary,
            },
            selected_text_color: colors.text_body,
            selected_background: Background::Color(mix_colors(ext.buttons_color, colors.primary)),
        }
    }
}

impl PicklistType {
    #[allow(clippy::unused_self)]
    fn active(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(ext.buttons_color),
            border: Border {
                radius: PICKLIST_BORDER_RADIUS.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        }
    }

    #[allow(clippy::unused_self)]
    fn hovered(&self, style: &StyleType) -> Style {
        let colors = style.get_palette();
        let ext = style.get_extension();
        Style {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(mix_colors(ext.buttons_color, colors.primary)),
            border: Border {
                radius: PICKLIST_BORDER_RADIUS.into(),
                width: BORDER_WIDTH,
                color: colors.secondary,
            },
        }
    }
}

impl iced::overlay::menu::Catalog for StyleType {
    type Class<'a> = PicklistType;

    fn default<'a>() -> <Self as iced::overlay::menu::Catalog>::Class<'a> {
        <Self as iced::overlay::menu::Catalog>::Class::default()
    }

    fn style(
        &self,
        class: &<Self as iced::overlay::menu::Catalog>::Class<'_>,
    ) -> iced::overlay::menu::Style {
        class.appearance(self)
    }
}

impl Catalog for StyleType {
    type Class<'a> = PicklistType;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        <Self as Catalog>::Class::default()
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        match status {
            Status::Active | Status::Opened => class.active(self),
            Status::Hovered => class.hovered(self),
        }
    }
}
