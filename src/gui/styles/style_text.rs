// //! Text style
//
// use crate::enums::element_type::ElementType;
// use crate::get_colors;
// use crate::structs::style_tuple::StyleTuple;
// use crate::utility::style_constants::{
//     BORDER_BUTTON_RADIUS, BORDER_WIDTH, BORDER_WIDTH_TABS,
// };
// use iced::widget::{button};
// use iced::{Background, Vector};
//
// impl button::StyleSheet for StyleTuple {
//     type Style = iced_style::Theme;
//
//     fn active(&self, _: &Self::Style) -> button::Appearance {
//         let colors = get_colors(self.0);
//         button::Appearance {
//             background: Some(Background::Color(match self {
//                 StyleTuple(_, ElementType::TabActive) => colors.primary,
//                 _ => colors.buttons,
//             })),
//             border_radius: match self {
//                 StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
//                 _ => BORDER_BUTTON_RADIUS,
//             },
//             border_width: match self {
//                 StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => {
//                     BORDER_WIDTH_TABS
//                 }
//                 _ => BORDER_WIDTH,
//             },
//             shadow_offset: Vector::new(0.0, 0.0),
//             text_color: colors.text_body,
//             border_color: match self {
//                 StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => colors.buttons,
//                 _ => colors.secondary,
//             },
//         }
//     }
//
//     fn hovered(&self, _: &Self::Style) -> button::Appearance {
//         let colors = get_colors(self.0);
//         iced_style::button::Appearance {
//             shadow_offset: Vector::new(2.0, 2.0),
//             background: Some(Background::Color(colors.primary)),
//             border_radius: match self {
//                 StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
//                 _ => BORDER_BUTTON_RADIUS,
//             },
//             border_width: BORDER_WIDTH,
//             border_color: match self {
//                 StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => colors.buttons,
//                 _ => colors.secondary,
//             },
//             text_color: colors.text_body,
//         }
//     }
// }
