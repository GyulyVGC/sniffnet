use iced::{pick_list, container, Background, Color, Vector, Container, Element, Row, Application, button, Font, Text, Length, alignment};
use iced::container::{Style, StyleSheet};
use iced_style::scrollable::{Scrollbar, Scroller};
use crate::app::Message;

#[derive(Copy, Eq, PartialEq)]
pub enum Mode {
    Night,
    Day
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
            },
            background: match self {
                Mode::Day => {Some(Background::Color(Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,}))}
                Mode::Night => {Some(Background::Color(Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0,}))}
            },
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default()
        }
    }
}

impl pick_list::StyleSheet for Mode {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
            },
            background: Background::Color(match self {
                Mode::Day => Color{r: 0.9, g: 0.9, b: 0.9, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            }),
            border_width: 2.0,
            border_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
            },
            selected_text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
            },
            selected_background: Background::Color(match self {
                Mode::Day => Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0,},
            })
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
            },
            placeholder_color: Color::BLACK,
            background: Background::Color(match self {
                Mode::Day => Color{r: 0.9, g: 0.9, b: 0.9, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            }),
            border_radius: 0.0,
            border_width: 2.0,
            border_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
            },
            icon_size: 0.5
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
            },
            placeholder_color: Color::BLACK,
            background: Background::Color(match self {
                Mode::Day => Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0,},
            }),
            border_radius: 0.0,
            border_width: 2.0,
            border_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.5, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.5, b: 0.5, a: 1.0,},
            },
            icon_size: 0.5
        }
    }
}

impl button::StyleSheet for Mode {

    fn hovered(&self) -> iced_style::button::Style {
        iced_style::button::Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(match self {
                Mode::Day => Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0,},
            })),
            border_radius: 12.0,
            border_width: 2.0,
            border_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.5, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.5, b: 0.5, a: 1.0,},
            },
            text_color: match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
            }
        }
    }

    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Mode::Day => Color{r: 0.9, g: 0.9, b: 0.9, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            })),
            border_radius: 12.0,
            border_width: 2.0,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color:  match self {
                Mode::Day => Color::BLACK,
                Mode::Night => Color::WHITE,
            },
            border_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
            }
        }
    }
}

impl iced_style::radio::StyleSheet for Mode {
    fn active(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(match self {
                Mode::Day => Color{r: 1.0, g: 1.0, b: 1.0, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            }),
            dot_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
            },
            border_width: 0.0,
            border_color: Default::default(),
            text_color: None
        }
    }

    fn hovered(&self) -> iced_style::radio::Style {
        iced_style::radio::Style {
            background: Background::Color(match self {
                Mode::Day => Color{r: 1.0, g: 1.0, b: 1.0, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            }),
            dot_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
            },
            border_width: 2.0,
            border_color: match self {
                Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
            },
            text_color: None
        }
    }
}

impl iced_style::scrollable::StyleSheet for Mode {

    fn active(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(match self {
                Mode::Day => Color{r: 0.9, g: 0.9, b: 0.9, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            })),
            border_radius: 12.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: match self {
                    Mode::Day => Color{r: 0.8, g: 0.8, b: 0.8, a: 1.0,},
                    Mode::Night => Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0,},
                },
                border_radius: 12.0,
                border_width: 2.0,
                border_color: Color::BLACK
            }
        }
    }

    fn hovered(&self) -> Scrollbar {
        Scrollbar {
            background: Some(Background::Color(match self {
                Mode::Day => Color{r: 0.9, g: 0.9, b: 0.9, a: 1.0,},
                Mode::Night => Color{r: 0.1, g: 0.1, b: 0.1, a: 1.0,},
            })),
            border_radius: 12.0,
            border_width: 2.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: match self {
                    Mode::Day => Color{r: 0.0, g: 0.5, b: 0.8, a: 1.0,},
                    Mode::Night => Color{r: 0.0, g: 0.8, b: 0.5, a: 1.0,},
                },
                border_radius: 12.0,
                border_width: 2.0,
                border_color: Color::BLACK
            }
        }
    }

}

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

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
    }
}

pub const CourierPrime: Font = Font::External {
    name: "CourierPrimeSans",
    bytes: include_bytes!("../fonts/CourierPrimeSans.ttf"),
};

pub const CourierPrimeItalic: Font = Font::External {
    name: "CourierPrimeSans",
    bytes: include_bytes!("../fonts/CourierPrimeSansItalic.ttf"),
};

pub const FontSizeBody: u16 = 16;
pub const FontSizeSubtitle: u16 = 18;
pub const FontSizeTitle: u16 = 22;