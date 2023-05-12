//! Module defining the constants used for aesthetic purposes (colors, borders...)

use iced::{Color, Font};
use plotters::style::RGBColor;

use crate::gui::styles::types::palette::{to_rgb_color, Palette};
use crate::{get_colors, StyleType};

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
pub const NIGHT_STYLE: Palette = Palette {
    primary: PRIMARY_NIGHT,
    secondary: SECONDARY_NIGHT,
    buttons: BUTTONS_NIGHT,
    incoming: SECONDARY_NIGHT,
    outgoing: SECONDARY_DAY,
    text_headers: Color::BLACK,
    text_body: Color::WHITE,
    round_borders: Color {
        a: 0.3,
        ..Color::BLACK
    },
    round_containers: Color {
        a: 0.2,
        ..Color::BLACK
    },
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
pub const DAY_STYLE: Palette = Palette {
    primary: PRIMARY_DAY,
    secondary: SECONDARY_DAY,
    buttons: BUTTONS_DAY,
    incoming: SECONDARY_DAY,
    outgoing: SECONDARY_NIGHT,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
    round_borders: Color {
        a: 0.25,
        ..Color::BLACK
    },
    round_containers: Color {
        a: 0.1,
        ..Color::BLACK
    },
};

// deep sea theme
const PRIMARY_DEEP_SEA: Color = Color {
    r: 28.0 / 255.0,
    g: 49.0 / 255.0,
    b: 94.0 / 255.0,
    a: 1.0,
};
const SECONDARY_DEEP_SEA: Color = Color {
    r: 34.0 / 255.0,
    g: 124.0 / 255.0,
    b: 112.0 / 255.0,
    a: 1.0,
};
const BUTTONS_DEEP_SEA: Color = Color {
    r: 48.0 / 255.0,
    g: 71.0 / 255.0,
    b: 94.0 / 255.0,
    a: 1.0,
};
const OUTGOING_DEEP_SEA: Color = Color {
    r: 230.0 / 255.0,
    g: 226.0 / 255.0,
    b: 195.0 / 255.0,
    a: 1.0,
};
pub const DEEP_SEA_STYLE: Palette = Palette {
    primary: PRIMARY_DEEP_SEA,
    secondary: SECONDARY_DEEP_SEA,
    buttons: BUTTONS_DEEP_SEA,
    incoming: SECONDARY_DEEP_SEA,
    outgoing: OUTGOING_DEEP_SEA,
    text_headers: Color::BLACK,
    text_body: Color::WHITE,
    round_borders: Color {
        a: 0.1,
        ..SECONDARY_DEEP_SEA
    },
    round_containers: Color {
        a: 0.03,
        ..SECONDARY_DEEP_SEA
    },
};

// mon amour theme
const SECONDARY_MON_AMOUR: Color = Color {
    r: 127.0 / 255.0,
    g: 102.0 / 255.0,
    b: 157.0 / 255.0,
    a: 1.0,
};
const PRIMARY_MON_AMOUR: Color = Color {
    r: 245.0 / 255.0,
    g: 245.0 / 255.0,
    b: 220.0 / 255.0,
    a: 1.0,
};
const BUTTONS_MON_AMOUR: Color = Color {
    r: 222.0 / 255.0,
    g: 186.0 / 255.0,
    b: 206.0 / 255.0,
    a: 1.0,
};
const OUTGOING_MON_AMOUR: Color = Color {
    r: 90.0 / 255.0,
    g: 164.0 / 255.0,
    b: 105.0 / 255.0,
    a: 1.0,
};
pub const MON_AMOUR_STYLE: Palette = Palette {
    primary: PRIMARY_MON_AMOUR,
    secondary: SECONDARY_MON_AMOUR,
    buttons: BUTTONS_MON_AMOUR,
    incoming: SECONDARY_MON_AMOUR,
    outgoing: OUTGOING_MON_AMOUR,
    text_headers: Color::WHITE,
    text_body: Color::BLACK,
    round_borders: Color {
        a: 0.5,
        ..BUTTONS_MON_AMOUR
    },
    round_containers: Color {
        a: 0.3,
        ..BUTTONS_MON_AMOUR
    },
};

pub const SARASA_MONO_SC_BOLD: Font = Font::External {
    name: "sarasa-mono-sc-bold",
    bytes: include_bytes!("../../../resources/fonts/subset/sarasa-mono-sc-bold.subset.ttf"),
};

pub fn get_font(style: StyleType) -> Font {
    match to_rgb_color(get_colors(style).text_body) {
        RGBColor(255, 255, 255) => Font::Default,
        _ => SARASA_MONO_SC_BOLD,
    }
}

pub fn get_font_headers(style: StyleType) -> Font {
    match to_rgb_color(get_colors(style).text_headers) {
        RGBColor(255, 255, 255) => Font::Default,
        _ => SARASA_MONO_SC_BOLD,
    }
}

pub fn get_color_mix_chart(style: StyleType) -> f64 {
    match style {
        StyleType::Night | StyleType::DeepSea => 0.3,
        StyleType::Day | StyleType::MonAmour => 0.8,
    }
}

//font to display icons
pub const ICONS: Font = Font::External {
    name: "icons",
    bytes: include_bytes!("../../../resources/fonts/subset/icons.ttf"),
};

// palettes pictures
pub const YETI_DAY: &[u8] = include_bytes!("../../../resources/palettes/YetiDay.svg");
pub const YETI_NIGHT: &[u8] = include_bytes!("../../../resources/palettes/YetiNight.svg");
pub const DEEP_SEA: &[u8] = include_bytes!("../../../resources/palettes/DeepSea.svg");
pub const MON_AMOUR: &[u8] = include_bytes!("../../../resources/palettes/MonAmour.svg");

// font sizes
pub const FONT_SIZE_FOOTER: f32 = 18.0;
pub const FONT_SIZE_BODY: f32 = 21.0;
pub const FONT_SIZE_SUBTITLE: f32 = 23.0;
pub const FONT_SIZE_TITLE: f32 = 25.0;

// border styles
pub const BORDER_WIDTH: f32 = 2.0;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 15.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;

/// Yellow color used in favorites star
pub fn get_starred_color(style: StyleType) -> Color {
    match style {
        StyleType::Night | StyleType::DeepSea => Color {
            r: 245.0 / 255.0,
            g: 193.0 / 255.0,
            b: 39.0 / 255.0,
            a: 0.5,
        },
        StyleType::Day | StyleType::MonAmour => Color {
            r: 245.0 / 255.0,
            g: 193.0 / 255.0,
            b: 39.0 / 255.0,
            a: 0.8,
        },
    }
}

pub fn get_color_mix_filter_badge(style: StyleType) -> f32 {
    match style {
        StyleType::Night | StyleType::DeepSea => 0.2,
        StyleType::Day => 0.7,
        StyleType::MonAmour => 0.5,
    }
}
