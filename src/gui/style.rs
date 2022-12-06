//! Module defining the application styles: fonts, colors, containers, picklists, buttons,
//! radios, scrollbars, icons.

use iced::alignment::Horizontal;
use iced::container::{Style, StyleSheet};
use iced::{alignment, button, pick_list, Background, Color, Font, Length, Text, Vector};
use iced_style::scrollable::{Scrollbar, Scroller};
use plotters::style::RGBColor;

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

pub const HEIGHT_HEADER: u16 = 2;
pub const HEIGHT_BODY: u16 = 12;
pub const HEIGHT_FOOTER: u16 = 1;

pub const DAY_BACKGROUND: Color = Color::WHITE;
pub const NIGHT_BACKGROUND: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
pub const DAY_BUTTONS: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};
pub const NIGHT_BUTTONS: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};
pub const SPECIAL_NIGHT: Color = Color {
    r: 0.7,
    g: 0.35,
    b: 0.0,
    a: 1.0,
};
pub const SPECIAL_DAY: Color = Color {
    r: 0.0,
    g: 0.35,
    b: 0.7,
    a: 1.0,
};

pub const SPECIAL_NIGHT_RGB: RGBColor = RGBColor(189, 89, 0);
pub const SPECIAL_DAY_RGB: RGBColor = RGBColor(0, 89, 189);

pub const COLOR_CHART_MIX_DAY: f64 = 0.8;
pub const COLOR_CHART_MIX_NIGHT: f64 = 0.4;
pub const CHARTS_LINE_BORDER: u32 = 1;

#[derive(Copy, Eq, PartialEq)]
/// Used to specify the kind of style to be applied to an element
pub enum StyleType {
    Night,
    Day,
    BorderedRound,
    HeadersDay,
    HeadersNight,
    TabsActiveNight,
    TabsInactiveNight,
    TabsActiveDay,
    TabsInactiveDay,
}

impl Clone for StyleType {
    fn clone(&self) -> Self {
        *self
    }
}

/// Containers style
impl StyleSheet for StyleType {
    fn style(&self) -> Style {
        Style {
            text_color: match self {
                StyleType::Day => Some(Color::BLACK),
                StyleType::Night => Some(Color::WHITE),
                StyleType::HeadersDay => Some(Color::WHITE),
                StyleType::HeadersNight => Some(Color::BLACK),
                _ => None,
            },
            background: match self {
                StyleType::Day => Some(Background::Color(DAY_BACKGROUND)),
                StyleType::Night => Some(Background::Color(NIGHT_BACKGROUND)),
                StyleType::HeadersDay => Some(Background::Color(SPECIAL_DAY)),
                StyleType::HeadersNight => Some(Background::Color(SPECIAL_NIGHT)),
                _ => None,
            },
            border_radius: match self {
                StyleType::BorderedRound => 12.0,
                _ => 0.0,
            },
            border_width: match self {
                StyleType::Night => 0.0,
                StyleType::Day => 0.0,
                _ => BORDER_WIDTH,
            },
            border_color: Color::BLACK,
        }
    }
}

/// Picklists style
impl pick_list::StyleSheet for StyleType {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            text_color: match self {
                StyleType::Day => Color::BLACK,
                StyleType::Night => DAY_BUTTONS,
                _ => Color::BLACK,
            },
            background: Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                _ => Color::BLACK,
            }),
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                _ => Color::BLACK,
            },
            selected_text_color: match self {
                StyleType::Day => Color::BLACK,
                StyleType::Night => Color::WHITE,
                _ => Color::BLACK,
            },
            selected_background: Background::Color(match self {
                StyleType::Day => DAY_BACKGROUND,
                StyleType::Night => NIGHT_BACKGROUND,
                _ => Color::BLACK,
            }),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: match self {
                StyleType::Day => Color::BLACK,
                StyleType::Night => Color::WHITE,
                _ => Color::BLACK,
            },
            placeholder_color: Color::BLACK,
            background: Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                _ => Color::BLACK,
            }),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                _ => Color::BLACK,
            },
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: match self {
                StyleType::Day => Color::BLACK,
                StyleType::Night => Color::WHITE,
                _ => Color::BLACK,
            },
            placeholder_color: Color::BLACK,
            background: Background::Color(match self {
                StyleType::Day => DAY_BACKGROUND,
                StyleType::Night => NIGHT_BACKGROUND,
                _ => Color::BLACK,
            }),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                _ => Color::BLACK,
            },
            icon_size: 0.5,
        }
    }
}

/// Buttons style
impl button::StyleSheet for StyleType {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                StyleType::TabsActiveNight => NIGHT_BACKGROUND,
                StyleType::TabsInactiveNight => NIGHT_BUTTONS,
                StyleType::TabsActiveDay => DAY_BACKGROUND,
                StyleType::TabsInactiveDay => DAY_BUTTONS,
                _ => Color::BLACK,
            })),
            border_radius: match self {
                StyleType::TabsActiveNight
                | StyleType::TabsInactiveNight
                | StyleType::TabsInactiveDay
                | StyleType::TabsActiveDay => 0.0,
                _ => 12.0,
            },
            border_width: match self {
                StyleType::TabsActiveNight
                | StyleType::TabsInactiveNight
                | StyleType::TabsInactiveDay
                | StyleType::TabsActiveDay => 3.3,
                _ => BORDER_WIDTH,
            },
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                StyleType::Day | StyleType::TabsActiveDay | StyleType::TabsInactiveDay => {
                    Color::BLACK
                }
                StyleType::Night | StyleType::TabsActiveNight | StyleType::TabsInactiveNight => {
                    Color::WHITE
                }
                _ => Color::BLACK,
            },
            border_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                StyleType::TabsActiveNight | StyleType::TabsInactiveNight => NIGHT_BUTTONS,
                StyleType::TabsInactiveDay | StyleType::TabsActiveDay => DAY_BUTTONS,
                _ => Color::BLACK,
            },
        }
    }

    fn hovered(&self) -> iced_style::button::Style {
        iced_style::button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(match self {
                StyleType::Day => DAY_BACKGROUND,
                StyleType::Night => NIGHT_BACKGROUND,
                StyleType::TabsActiveNight | StyleType::TabsInactiveNight => NIGHT_BACKGROUND,
                StyleType::TabsActiveDay | StyleType::TabsInactiveDay => DAY_BACKGROUND,
                _ => Color::BLACK,
            })),
            border_radius: match self {
                StyleType::TabsActiveNight
                | StyleType::TabsInactiveNight
                | StyleType::TabsInactiveDay
                | StyleType::TabsActiveDay => 0.0,
                _ => 12.0,
            },
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                StyleType::TabsActiveNight | StyleType::TabsInactiveNight => NIGHT_BUTTONS,
                StyleType::TabsInactiveDay | StyleType::TabsActiveDay => DAY_BUTTONS,
                _ => Color::BLACK,
            },
            text_color: match self {
                StyleType::Day | StyleType::TabsActiveDay | StyleType::TabsInactiveDay => {
                    Color::BLACK
                }
                StyleType::Night | StyleType::TabsActiveNight | StyleType::TabsInactiveNight => {
                    Color::WHITE
                }
                _ => Color::BLACK,
            },
        }
    }
}

/// Radios style
impl iced_style::radio::StyleSheet for StyleType {
    fn active(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                _ => Color::BLACK,
            }),
            dot_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                _ => Color::BLACK,
            },
            border_width: 0.0,
            border_color: Default::default(),
            text_color: None,
        }
    }

    fn hovered(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                _ => Color::BLACK,
            }),
            dot_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                _ => Color::BLACK,
            },
            border_width: BORDER_WIDTH,
            border_color: match self {
                StyleType::Day => SPECIAL_DAY,
                StyleType::Night => SPECIAL_NIGHT,
                _ => Color::BLACK,
            },
            text_color: None,
        }
    }
}

/// Scrollbars style
impl iced_style::scrollable::StyleSheet for StyleType {
    fn active(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                _ => Color::BLACK,
            })),
            border_radius: 12.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: match self {
                    StyleType::Day => DAY_BACKGROUND,
                    StyleType::Night => NIGHT_BACKGROUND,
                    _ => Color::BLACK,
                },
                border_radius: 12.0,
                border_width: BORDER_WIDTH / 1.5,
                border_color: Color::BLACK,
            },
        }
    }

    fn hovered(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(match self {
                StyleType::Day => DAY_BUTTONS,
                StyleType::Night => NIGHT_BUTTONS,
                _ => Color::BLACK,
            })),
            border_radius: 12.0,
            border_width: BORDER_WIDTH / 1.5,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: match self {
                    StyleType::Day => SPECIAL_DAY,
                    StyleType::Night => SPECIAL_NIGHT,
                    _ => Color::BLACK,
                },
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

pub fn icon_sun_moon(style: StyleType) -> Text {
    //F: sun, G: moon, K: sun adjust
    match style {
        StyleType::Night => Text::new('K'.to_string())
            .font(ICONS)
            .width(Length::Units(25))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(20),
        StyleType::Day => Text::new('K'.to_string())
            .font(ICONS)
            .width(Length::Units(25))
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(20),
        _ => Text::new(""),
    }
}
