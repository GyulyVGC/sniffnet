//! Module defining the application styles: containers, picklists, buttons, radios, scrollbars.

use crate::enums::element_type::ElementType;
use crate::enums::style_type::StyleType;
use crate::get_colors;
use crate::structs::colors::Colors;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{
    BORDER_BUTTON_RADIUS, BORDER_ROUNDED_RADIUS, BORDER_WIDTH, BORDER_WIDTH_TABS,
};
use iced::widget::{button, container::StyleSheet, pick_list};
use iced::{Background, Color, Vector};
use iced_style::application::Appearance;
use iced_style::scrollable::{Scrollbar, Scroller};
use iced_style::Theme;
use std::rc::Rc;

// impl From<Colors> for iced_style::theme::Theme {
//     fn from(colors: Colors) -> Self {
//         iced_style::theme::Theme::Custom(colors)
//     }
// }

// impl iced::application::StyleSheet for Colors {
//     type Style = Theme;
//
//     fn appearance(&self, _: &Self::Style) -> Appearance {
//         Appearance {
//             background_color: self.primary,
//             text_color: self.text_body,
//         }
//     }
// }

// /// Containers style
// impl StyleSheet for StyleTuple {
//     fn style(&self) -> Style {
//         let colors = get_colors(self.0);
//         Style {
//             text_color: Option::Some(match self {
//                 StyleTuple(_, ElementType::Headers) => colors.text_headers,
//                 _ => colors.text_body,
//             }),
//             background: Option::Some(Background::Color(match self {
//                 StyleTuple(_, ElementType::Headers) => colors.secondary,
//                 _ => colors.primary,
//             })),
//             border_radius: match self {
//                 StyleTuple(_, ElementType::BorderedRound) => BORDER_ROUNDED_RADIUS,
//                 _ => 0.0,
//             },
//             border_width: match self {
//                 StyleTuple(_, ElementType::Standard | ElementType::Headers) => 0.0,
//                 _ => BORDER_WIDTH,
//             },
//             border_color: colors.round_borders,
//         }
//     }
// }

/// Picklists style

impl From<StyleTuple> for iced::theme::PickList {
    fn from(tuple: StyleTuple) -> Self {
        iced_style::theme::PickList::Custom(Rc::new(tuple.clone()), Rc::new(tuple))
    }
}

impl iced_style::menu::StyleSheet for StyleTuple {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced_style::menu::Appearance {
        let colors = get_colors(self.0);
        iced_style::menu::Appearance {
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
            background: Background::Color(colors.buttons),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            icon_size: 0.5,
        }
    }

    fn hovered(&self, _: &Self::Style) -> pick_list::Appearance {
        let colors = get_colors(self.0);
        pick_list::Appearance {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            background: Background::Color(colors.primary),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            icon_size: 0.5,
        }
    }
}

// /// Radios style
// impl iced_style::radio::StyleSheet for StyleTuple {
//     fn active(&self) -> iced_style::radio::Style {
//         let colors = get_colors(self.0);
//         iced_style::radio::Style {
//             background: Background::Color(colors.buttons),
//             dot_color: colors.secondary,
//             border_width: match self {
//                 StyleTuple(_, ElementType::SelectedRadio) => BORDER_WIDTH,
//                 _ => 0.0,
//             },
//             border_color: colors.secondary,
//             text_color: match self {
//                 StyleTuple(_, ElementType::SelectedRadio) => Some(colors.secondary),
//                 _ => None,
//             },
//         }
//     }
//
//     fn hovered(&self) -> iced_style::radio::Style {
//         let colors = get_colors(self.0);
//         iced_style::radio::Style {
//             background: Background::Color(colors.buttons),
//             dot_color: colors.secondary,
//             border_width: BORDER_WIDTH,
//             border_color: colors.secondary,
//             text_color: Some(colors.secondary),
//         }
//     }
// }
//
// /// Scrollbars style
// impl iced_style::scrollable::StyleSheet for StyleTuple {
//     fn active(&self) -> Scrollbar {
//         let colors = get_colors(self.0);
//         Scrollbar {
//             background: Some(Background::Color(colors.buttons)),
//             border_radius: BORDER_ROUNDED_RADIUS,
//             border_width: 0.0,
//             border_color: colors.round_borders,
//             scroller: Scroller {
//                 color: colors.primary,
//                 border_radius: BORDER_ROUNDED_RADIUS,
//                 border_width: BORDER_WIDTH / 1.5,
//                 border_color: colors.round_borders,
//             },
//         }
//     }
//
//     fn hovered(&self) -> Scrollbar {
//         let colors = get_colors(self.0);
//         Scrollbar {
//             background: Some(Background::Color(colors.buttons)),
//             border_radius: BORDER_ROUNDED_RADIUS,
//             border_width: BORDER_WIDTH / 1.5,
//             border_color: colors.round_borders,
//             scroller: Scroller {
//                 color: colors.secondary,
//                 border_radius: BORDER_ROUNDED_RADIUS,
//                 border_width: BORDER_WIDTH / 1.5,
//                 border_color: colors.round_borders,
//             },
//         }
//     }
// }
