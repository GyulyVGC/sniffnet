//! Module defining the constants used for aesthetic purposes (colors, borders...)

use iced::{Color, Font};

use crate::structs::colors::Colors;

pub const COLOR_CHART_MIX_DAY: f64 = 0.8;

pub const COLOR_CHART_MIX_NIGHT: f64 = 0.4;

// night theme
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

// day theme
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

// font sizes
pub const FONT_SIZE_FOOTER: u16 = 14;
pub const FONT_SIZE_BODY: u16 = 16;
pub const FONT_SIZE_SUBTITLE: u16 = 19;
pub const FONT_SIZE_TITLE: u16 = 22;

// border styles
pub const BORDER_WIDTH: f32 = 2.0;
pub const BORDER_WIDTH_TABS: f32 = 3.3;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 15.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;

// body proportions
pub const HEIGHT_HEADER: u16 = 2;
pub const HEIGHT_BODY: u16 = 12;
pub const HEIGHT_FOOTER: u16 = 1;
