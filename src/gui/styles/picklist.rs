//! Picklists style

use crate::get_colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::BORDER_WIDTH;
use iced::widget::pick_list;
use iced::Background;
use std::rc::Rc;

impl From<StyleTuple> for iced::theme::PickList {
    fn from(tuple: StyleTuple) -> Self {
        iced::theme::PickList::Custom(Rc::new(tuple.clone()), Rc::new(tuple))
    }
}

impl iced::overlay::menu::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> iced::overlay::menu::Appearance {
        let colors = get_colors(self.0);
        iced::overlay::menu::Appearance {
            text_color: colors.text_body,
            background: Background::Color(colors.buttons),
            border_width: BORDER_WIDTH,
            border_radius: 0.0,
            border_color: colors.secondary,
            selected_text_color: colors.text_body,
            selected_background: Background::Color(colors.primary),
        }
    }
}

impl pick_list::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> pick_list::Appearance {
        let colors = get_colors(self.0);
        pick_list::Appearance {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(colors.buttons),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
        }
    }

    fn hovered(&self, _: &Self::Style) -> pick_list::Appearance {
        let colors = get_colors(self.0);
        pick_list::Appearance {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(colors.primary),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
        }
    }
}
