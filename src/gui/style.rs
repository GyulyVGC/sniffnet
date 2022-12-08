//! Module defining the application styles: containers, picklists, buttons, radios, scrollbars.

use crate::enums::element_type::ElementType;
use crate::enums::style_type::StyleType;
use crate::get_colors;
use crate::utility::style_constants::{
    BORDER_BUTTON_RADIUS, BORDER_ROUNDED_RADIUS, BORDER_WIDTH, BORDER_WIDTH_TABS,
};
use iced::container::{Style, StyleSheet};
use iced::{button, pick_list, Background, Vector};
use iced_style::scrollable::{Scrollbar, Scroller};

/// This tuple permits to specify the correct style depending on the style type and on the element type
pub struct StyleTuple(pub StyleType, pub ElementType);

/// Containers style
impl StyleSheet for StyleTuple {
    fn style(&self) -> Style {
        let colors = get_colors(self.0);
        Style {
            text_color: Option::Some(match self {
                StyleTuple(_, ElementType::Headers) => colors.text_headers,
                _ => colors.text_body,
            }),
            background: Option::Some(Background::Color(match self {
                StyleTuple(_, ElementType::Headers) => colors.secondary,
                _ => colors.primary,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::BorderedRound) => BORDER_ROUNDED_RADIUS,
                _ => 0.0,
            },
            border_width: match self {
                StyleTuple(_, ElementType::Standard | ElementType::Headers) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: colors.round_borders,
        }
    }
}

/// Picklists style
impl pick_list::StyleSheet for StyleTuple {
    fn menu(&self) -> iced_style::menu::Style {
        let colors = get_colors(self.0);
        iced_style::menu::Style {
            text_color: colors.text_body,
            background: Background::Color(colors.buttons),
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            selected_text_color: colors.text_body,
            selected_background: Background::Color(colors.primary),
        }
    }

    fn active(&self) -> pick_list::Style {
        let colors = get_colors(self.0);
        pick_list::Style {
            text_color: colors.text_body,
            placeholder_color: colors.text_body,
            background: Background::Color(colors.buttons),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        let colors = get_colors(self.0);
        pick_list::Style {
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

/// Buttons style
impl button::StyleSheet for StyleTuple {
    fn active(&self) -> button::Style {
        let colors = get_colors(self.0);
        button::Style {
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::TabActive) => colors.primary,
                _ => colors.buttons,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => {
                    BORDER_WIDTH_TABS
                }
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: colors.text_body,
            border_color: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => colors.buttons,
                _ => colors.secondary,
            },
        }
    }

    fn hovered(&self) -> iced_style::button::Style {
        let colors = get_colors(self.0);
        iced_style::button::Style {
            shadow_offset: Vector::new(2.0, 2.0),
            background: Some(Background::Color(colors.primary)),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                _ => BORDER_BUTTON_RADIUS,
            },
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => colors.buttons,
                _ => colors.secondary,
            },
            text_color: colors.text_body,
        }
    }
}

/// Radios style
impl iced_style::radio::StyleSheet for StyleTuple {
    fn active(&self) -> iced_style::radio::Style {
        let colors = get_colors(self.0);
        iced_style::radio::Style {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: match self {
                StyleTuple(_, ElementType::SelectedRadio) => BORDER_WIDTH,
                _ => 0.0,
            },
            border_color: colors.secondary,
            text_color: match self {
                StyleTuple(_, ElementType::SelectedRadio) => Some(colors.secondary),
                _ => None,
            },
        }
    }

    fn hovered(&self) -> iced_style::radio::Style {
        let colors = get_colors(self.0);
        iced_style::radio::Style {
            background: Background::Color(colors.buttons),
            dot_color: colors.secondary,
            border_width: BORDER_WIDTH,
            border_color: colors.secondary,
            text_color: Some(colors.secondary),
        }
    }
}

/// Scrollbars style
impl iced_style::scrollable::StyleSheet for StyleTuple {
    fn active(&self) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.buttons)),
            border_radius: BORDER_ROUNDED_RADIUS,
            border_width: 0.0,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.primary,
                border_radius: BORDER_ROUNDED_RADIUS,
                border_width: BORDER_WIDTH / 1.5,
                border_color: colors.round_borders,
            },
        }
    }

    fn hovered(&self) -> Scrollbar {
        let colors = get_colors(self.0);
        Scrollbar {
            background: Some(Background::Color(colors.buttons)),
            border_radius: BORDER_ROUNDED_RADIUS,
            border_width: BORDER_WIDTH / 1.5,
            border_color: colors.round_borders,
            scroller: Scroller {
                color: colors.secondary,
                border_radius: BORDER_ROUNDED_RADIUS,
                border_width: BORDER_WIDTH / 1.5,
                border_color: colors.round_borders,
            },
        }
    }
}
