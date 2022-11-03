use iced::{pick_list, Background, Color, Vector, button, Font, Text, Length, alignment};
use iced::container::{Style, StyleSheet};
use iced_style::scrollable::{Scrollbar, Scroller};
use plotters::style::RGBColor;


pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};


pub const COURIER_PRIME: Font = Font::External {
    name: "CourierPrime",
    bytes: include_bytes!("../fonts/CourierPrime.ttf"),
};

// pub const COURIER_PRIME_BOLD: Font = Font::External {
//     name: "CourierPrimeSansBold",
//     bytes: include_bytes!("../fonts/CourierPrimeSansBold.ttf"),
// };

pub const COURIER_PRIME_BOLD_ITALIC: Font = Font::External {
    name: "CourierPrimeItalic",
    bytes: include_bytes!("../fonts/CourierPrimeBoldItalic.ttf"),
};

pub const FONT_SIZE_FOOTER: u16 = 14;
pub const FONT_SIZE_BODY: u16 = 16;
pub const FONT_SIZE_SUBTITLE: u16 = 18;
pub const FONT_SIZE_TITLE: u16 = 22;
pub const FONT_SIZE_SNIFFNET: u16 = 60;

pub const BORDER_WIDTH: f32 = 2.0;

pub const HEIGHT_HEADER: u16 = 3;
pub const HEIGHT_BODY: u16 = 15;
pub const HEIGHT_FOOTER: u16 = 1;


pub const DAY_BACKGROUND: Color = Color::WHITE;
pub const NIGHT_BACKGROUND: Color = Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 };
pub const DAY_BUTTONS: Color = Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 };
pub const NIGHT_BUTTONS: Color = Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 };
pub const SPECIAL_NIGHT: Color = Color { r: 0.7, g: 0.35, b: 0.0, a: 1.0 };
pub const SPECIAL_DAY: Color = Color { r: 0.0, g: 0.35, b: 0.7, a: 1.0 };

pub const SPECIAL_NIGHT_RGB: RGBColor = RGBColor {0: 189, 1: 89, 2: 0};
pub const SPECIAL_DAY_RGB: RGBColor = RGBColor {0: 0, 1: 89, 2: 189};

pub const COLOR_CHART_MIX_DAY: f64 = 0.8;
pub const COLOR_CHART_MIX_NIGHT: f64 = 0.4;


#[derive(Copy, Eq, PartialEq)]
pub enum Mode {
    Night,
    Day,
    BorderedRound,
    HeadersDay,
    HeadersNight
}


impl Clone for Mode {
    fn clone(&self) -> Self {
        *self
    }
}


impl StyleSheet for Mode {
    fn style(&self) -> Style {
        Style {
            text_color: match self {
                Mode::Day => Some(Color::BLACK),
                Mode::Night => Some(Color::WHITE),
                Mode::HeadersDay => Some(Color::WHITE),
                Mode::HeadersNight => Some((Color::BLACK)),
                _ => { None }
            },
            background: match self {
                Mode::Day => { Some(Background::Color(DAY_BACKGROUND))}
                Mode::Night => { Some(Background::Color(NIGHT_BACKGROUND)) }
                Mode::BorderedRound => { None }
                Mode::HeadersDay => { Some(Background::Color(SPECIAL_DAY)) }
                Mode::HeadersNight => { Some(Background::Color(SPECIAL_NIGHT)) }
            },
            border_radius: match self {
                Mode::BorderedRound => { 12.0 }
                _ => { 0.0 }
            },
            border_width: match self {
                Mode::Night => { 0.0 }
                Mode::Day => { 0.0 }
                _ => { BORDER_WIDTH }
            },
            border_color: Color::BLACK,
        }
    }
}


impl pick_list::StyleSheet for Mode {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => DAY_BUTTONS,
                _ => { Color::BLACK }
            },
            background: Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            }),
            border_width: BORDER_WIDTH,
            border_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            selected_text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
                _ => { Color::BLACK }
            },
            selected_background: Background::Color(match self {
                Mode::Day => DAY_BACKGROUND,
                Mode::Night => NIGHT_BACKGROUND,
                _ => { Color::BLACK }
            }),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
                _ => { Color::BLACK }
            },
            placeholder_color: Color::BLACK,
            background: Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            }),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
                _ => { Color::BLACK }
            },
            placeholder_color: Color::BLACK,
            background: Background::Color(match self {
                Mode::Day => DAY_BACKGROUND,
                Mode::Night => NIGHT_BACKGROUND,
                _ => { Color::BLACK }
            }),
            border_radius: 0.0,
            border_width: BORDER_WIDTH,
            border_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            icon_size: 0.5,
        }
    }
}


impl button::StyleSheet for Mode {
    fn hovered(&self) -> iced_style::button::Style {
        iced_style::button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(match self {
                Mode::Day => DAY_BACKGROUND,
                Mode::Night => NIGHT_BACKGROUND,
                _ => { Color::BLACK }
            })),
            border_radius: 12.0,
            border_width: BORDER_WIDTH,
            border_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
                _ => { Color::BLACK }
            },
        }
    }

    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            })),
            border_radius: 12.0,
            border_width: BORDER_WIDTH,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
                _ => { Color::BLACK }
            },
            border_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
        }
    }
}

impl iced_style::radio::StyleSheet for Mode {
    fn active(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            }),
            dot_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            border_width: 0.0,
            border_color: Default::default(),
            text_color: None,
        }
    }

    fn hovered(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            }),
            dot_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            border_width: BORDER_WIDTH,
            border_color: match self {
                Mode::Day => SPECIAL_DAY,
                Mode::Night => SPECIAL_NIGHT,
                _ => { Color::BLACK }
            },
            text_color: None,
        }
    }
}


impl iced_style::scrollable::StyleSheet for Mode {
    fn active(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            })),
            border_radius: 12.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: match self {
                    Mode::Day => DAY_BACKGROUND,
                    Mode::Night => NIGHT_BACKGROUND,
                    _ => { Color::BLACK }
                },
                border_radius: 12.0,
                border_width: BORDER_WIDTH,
                border_color: Color::BLACK,
            },
        }
    }

    fn hovered(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(match self {
                Mode::Day => DAY_BUTTONS,
                Mode::Night => NIGHT_BUTTONS,
                _ => { Color::BLACK }
            })),
            border_radius: 12.0,
            border_width: BORDER_WIDTH,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: match self {
                    Mode::Day => SPECIAL_DAY,
                    Mode::Night => SPECIAL_NIGHT,
                    _ => { Color::BLACK }
                },
                border_radius: 12.0,
                border_width: BORDER_WIDTH,
                border_color: Color::BLACK,
            },
        }
    }
}


pub fn icon(unicode: char) -> Text {
    Text::new(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
        .size(20)
}


pub fn icon_sun_moon(style: Mode) -> Text {
    match style {
        Mode::Night => {
            Text::new(format!("{} {} {}", font_awesome::MOON, font_awesome::ANGLE_DOUBLE_RIGHT, font_awesome::SUN))
                .font(ICONS)
                .width(Length::Units(20))
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(20)
        }
        Mode::Day => {
            Text::new(format!("{} {} {}", font_awesome::SUN, font_awesome::ANGLE_DOUBLE_RIGHT, font_awesome::MOON))
                .font(ICONS)
                .width(Length::Units(20))
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(20)
        }
        _ => { Text::new("") }
    }
}