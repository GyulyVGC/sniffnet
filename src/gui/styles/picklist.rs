//! Picklists style

use std::rc::Rc;

use iced::widget::pick_list;
use iced::{Background, Color};

use crate::gui::styles::style_constants::get_alpha_round_borders;
use crate::gui::styles::types::palette::mix_colors;
use crate::{get_colors, StyleType};

#[derive(Clone, Copy)]
pub enum PicklistType {
    Standard,
}

#[derive(Clone)]
pub struct PicklistStyleTuple(pub StyleType, pub PicklistType);

impl From<PicklistStyleTuple> for iced::theme::PickList {
    fn from(tuple: PicklistStyleTuple) -> Self {
        iced::theme::PickList::Custom(Rc::new(tuple.clone()), Rc::new(tuple))
    }
}

impl iced::overlay::menu::StyleSheet for PicklistStyleTuple {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> iced::overlay::menu::Appearance {
        let colors = get_colors(self.0);
        iced::overlay::menu::Appearance {
            text_color: colors.text_body,
            background: Background::Color(colors.buttons),
            border_width: 1.0,
            border_radius: 0.0.into(),
            border_color: colors.secondary,
            selected_text_color: colors.text_body,
            selected_background: Background::Color(mix_colors(colors.buttons, colors.primary)),
        }
    }
}

impl pick_list::StyleSheet for PicklistStyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> pick_list::Appearance {
        let colors = get_colors(self.0);
        pick_list::Appearance {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(colors.buttons),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: Color {
                a: get_alpha_round_borders(self.0),
                ..colors.buttons
            },
        }
    }

    fn hovered(&self, _: &Self::Style) -> pick_list::Appearance {
        let colors = get_colors(self.0);
        pick_list::Appearance {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(mix_colors(colors.buttons, colors.primary)),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: colors.secondary,
        }
    }
}
