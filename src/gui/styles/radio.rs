// //! Radios style
//
// #![allow(clippy::module_name_repetitions)]
//
// use iced::Background;
//
// use crate::gui::styles::style_constants::BORDER_WIDTH;
// use crate::StyleType;
//
// #[derive(Default)]
// pub enum RadioType {
//     #[default]
//     Standard,
// }
//
// impl iced::widget::radio::StyleSheet for StyleType {
//     type Style = RadioType;
//
//     fn active(&self, _: &Self::Style, is_selected: bool) -> iced::widget::radio::Appearance {
//         let colors = self.get_palette();
//         let ext = self.get_extension();
//         iced::widget::radio::Appearance {
//             background: Background::Color(ext.buttons_color),
//             dot_color: colors.secondary,
//             border_width: if is_selected { BORDER_WIDTH } else { 0.0 },
//             border_color: colors.secondary,
//             text_color: None,
//         }
//     }
//
//     fn hovered(&self, _: &Self::Style, _is_selected: bool) -> iced::widget::radio::Appearance {
//         let colors = self.get_palette();
//         let ext = self.get_extension();
//         iced::widget::radio::Appearance {
//             background: Background::Color(ext.buttons_color),
//             dot_color: colors.secondary,
//             border_width: BORDER_WIDTH,
//             border_color: colors.secondary,
//             text_color: None,
//         }
//     }
// }
