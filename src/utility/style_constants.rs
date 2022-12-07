//! Module defining the constants used for aesthetic purposes (colors, borders...)

use crate::structs::colors::Colors;
use iced::Color;

pub const COLOR_CHART_MIX_DAY: f64 = 0.8;

pub const COLOR_CHART_MIX_NIGHT: f64 = 0.4;

// NIGHT
const PRIMARY_NIGHT: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const SECONDARY_NIGHT: Color = Color {
    r: 0.7,
    g: 0.35,
    b: 0.0,
    a: 1.0,
};
const BUTTONS_NIGHT: Color = Color {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};
pub const NIGHT_STYLE: Colors = Colors {
    primary: PRIMARY_NIGHT,
    secondary: SECONDARY_NIGHT,
    buttons: BUTTONS_NIGHT,
    incoming: SECONDARY_NIGHT,
    outgoing: SECONDARY_DAY,
    text_headers: Color::BLACK,
    text_body: Color::WHITE,
};

//DAY
const PRIMARY_DAY: Color = Color::WHITE;
const SECONDARY_DAY: Color = Color {
    r: 0.0,
    g: 0.35,
    b: 0.7,
    a: 1.0,
};
const BUTTONS_DAY: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};
pub const DAY_STYLE: Colors = Colors {
    primary: PRIMARY_DAY,
    secondary: SECONDARY_DAY,
    buttons: BUTTONS_DAY,
    incoming: SECONDARY_NIGHT,
    outgoing: SECONDARY_DAY,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
};
