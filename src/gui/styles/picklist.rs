//! Picklists style

use std::rc::Rc;

use iced::widget::pick_list;
use iced::Background;

use crate::get_colors;
use crate::gui::styles::types::palette::mix_colors;
use crate::gui::styles::types::style_tuple::StyleTuple;

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
            border_width: 1.0,
            border_radius: 0.0,
            border_color: colors.secondary,
            selected_text_color: colors.text_body,
            selected_background: Background::Color(mix_colors(colors.buttons, colors.primary)),
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
            border_width: 1.0,
            border_color: colors.round_borders,
        }
    }

    fn hovered(&self, _: &Self::Style) -> pick_list::Appearance {
        let colors = get_colors(self.0);
        pick_list::Appearance {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            handle_color: colors.text_body,
            background: Background::Color(mix_colors(colors.buttons, colors.primary)),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: colors.secondary,
        }
    }
}
