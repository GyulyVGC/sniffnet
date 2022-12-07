//! Module defining the application styles: fonts, colors, containers, picklists, buttons,
//! radios, scrollbars, icons.

use crate::get_colors;
use iced::alignment::Horizontal;
use iced::container::{Style, StyleSheet};
use iced::{alignment, button, pick_list, Background, Color, Font, Length, Text, Vector};
use iced_style::scrollable::{Scrollbar, Scroller};

/// Application version number (to be displayed in gui footer)
pub const APP_VERSION: &str = "v1.0.1";

// gui Text fonts
pub const COURIER_PRIME: Font = Font::External {
    name: "CourierPrime",
    bytes: include_bytes!("../../fonts/CourierPrime.ttf"),
};
pub const COURIER_PRIME_BOLD: Font = Font::External {
    name: "CourierPrimeBold",
    bytes: include_bytes!("../../fonts/CourierPrimeBold.ttf"),
};
pub const COURIER_PRIME_ITALIC: Font = Font::External {
    name: "CourierPrimeItalic",
    bytes: include_bytes!("../../fonts/CourierPrimeItalic.ttf"),
};
pub const COURIER_PRIME_BOLD_ITALIC: Font = Font::External {
    name: "CourierPrimeBoldItalic",
    bytes: include_bytes!("../../fonts/CourierPrimeBoldItalic.ttf"),
};

// gui charts fonts
pub const NOTOSANS: Font = Font::External {
    name: "Notosans",
    bytes: include_bytes!("../../fonts/notosans-regular.ttf"),
};
pub const NOTOSANS_BOLD: Font = Font::External {
    name: "NotosansBold",
    bytes: include_bytes!("../../fonts/notosans-bold.ttf"),
};

//font to display icons
pub const ICONS: Font = Font::External {
    name: "icons",
    bytes: include_bytes!("../../fonts/icons.ttf"),
};

pub const FONT_SIZE_FOOTER: u16 = 14;
pub const FONT_SIZE_BODY: u16 = 16;
pub const FONT_SIZE_SUBTITLE: u16 = 19;
pub const FONT_SIZE_TITLE: u16 = 22;

pub const BORDER_WIDTH: f32 = 2.0;

pub const CHARTS_LINE_BORDER: u32 = 1;

pub const HEIGHT_HEADER: u16 = 2;
pub const HEIGHT_BODY: u16 = 12;
pub const HEIGHT_FOOTER: u16 = 1;

/// Used to specify the kind of `iced` element to be able to choose the appropriate style for it
#[derive(Copy, Eq, PartialEq)]
pub enum ElementType {
    Standard,
    Headers,
    BorderedRound,
    TabActive,
    TabInactive,
}

impl Clone for ElementType {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Eq, PartialEq)]
/// Used to specify the kind of style to be applied to an element
pub enum StyleType {
    Night,
    Day,
}

pub struct StyleTuple(pub StyleType, pub ElementType);

impl Clone for StyleType {
    fn clone(&self) -> Self {
        *self
    }
}

/// Containers style
impl StyleSheet for StyleTuple {
    fn style(&self) -> Style {
        Style {
            text_color: Option::Some(match self {
                StyleTuple(style, ElementType::Headers) => get_colors(*style).text_headers,
                StyleTuple(style, _) => get_colors(*style).text_body,
            }),
            background: Option::Some(Background::Color(match self {
                StyleTuple(style, ElementType::Headers) => get_colors(*style).secondary,
                StyleTuple(style, _) => get_colors(*style).primary,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::BorderedRound) => 12.0,
                StyleTuple(_, _) => 0.0,
            },
            border_width: match self {
                StyleTuple(_, ElementType::Standard | ElementType::Headers) => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: Color::BLACK,
        }
    }
}

/// Picklists style
impl pick_list::StyleSheet for StyleTuple {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            text_color: get_colors(self.0).text_body,
            background: Background::Color(get_colors(self.0).buttons),
            border_width: BORDER_WIDTH,
            border_color: get_colors(self.0).secondary,
            selected_text_color: get_colors(self.0).text_body,
            selected_background: Background::Color(get_colors(self.0).primary),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: get_colors(self.0).text_body,
            placeholder_color: Color::BLACK,
            background: Background::Color(get_colors(self.0).buttons),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: get_colors(self.0).secondary,
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: get_colors(self.0).text_body,
            placeholder_color: Color::BLACK,
            background: Background::Color(get_colors(self.0).primary),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: get_colors(self.0).secondary,
            icon_size: 0.5,
        }
    }
}

/// Buttons style
impl button::StyleSheet for StyleTuple {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                StyleTuple(_, ElementType::TabActive) => get_colors(self.0).primary,
                _ => get_colors(self.0).buttons,
            })),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                _ => 12.0,
            },
            border_width: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 3.3,
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                StyleTuple(StyleType::Day, _) => Color::BLACK,
                StyleTuple(StyleType::Night, _) => Color::WHITE,
            },
            border_color: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => {
                    get_colors(self.0).buttons
                }
                _ => get_colors(self.0).secondary,
            },
        }
    }

    fn hovered(&self) -> iced_style::button::Style {
        iced_style::button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(get_colors(self.0).primary)),
            border_radius: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => 0.0,
                _ => 12.0,
            },
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleTuple(_, ElementType::TabActive | ElementType::TabInactive) => {
                    get_colors(self.0).buttons
                }
                _ => get_colors(self.0).secondary,
            },
            text_color: match self {
                StyleTuple(StyleType::Day, _) => Color::BLACK,
                StyleTuple(StyleType::Night, _) => Color::WHITE,
            },
        }
    }
}

/// Radios style
impl iced_style::radio::StyleSheet for StyleTuple {
    fn active(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(get_colors(self.0).buttons),
            dot_color: get_colors(self.0).secondary,
            border_width: 0.0,
            border_color: Default::default(),
            text_color: None,
        }
    }

    fn hovered(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(get_colors(self.0).buttons),
            dot_color: get_colors(self.0).secondary,
            border_width: BORDER_WIDTH,
            border_color: get_colors(self.0).secondary,
            text_color: None,
        }
    }
}

/// Scrollbars style
impl iced_style::scrollable::StyleSheet for StyleTuple {
    fn active(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(get_colors(self.0).buttons)),
            border_radius: 12.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: get_colors(self.0).primary,
                border_radius: 12.0,
                border_width: BORDER_WIDTH / 1.5,
                border_color: Color::BLACK,
            },
        }
    }

    fn hovered(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(get_colors(self.0).buttons)),
            border_radius: 12.0,
            border_width: BORDER_WIDTH / 1.5,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: get_colors(self.0).secondary,
                border_radius: 12.0,
                border_width: BORDER_WIDTH / 1.5,
                border_color: Color::BLACK,
            },
        }
    }
}

/// It returns a glyph featuring Sniffnet's logo
pub fn logo_glyph() -> Text {
    Text::new('A'.to_string())
        .font(ICONS)
        .horizontal_alignment(Horizontal::Center)
}

pub fn icon_sun_moon() -> Text {
    //F: sun, G: moon, K: sun adjust
    Text::new('K'.to_string())
        .font(ICONS)
        .width(Length::Units(25))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}
