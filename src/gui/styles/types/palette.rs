//! Module defining the `Colors` struct, which defines the colors in use in the GUI.

use iced::Color;
use plotters::style::RGBColor;

use crate::gui::styles::style_constants::{
    DAY_STYLE, DEEP_SEA_STYLE, MON_AMOUR_STYLE, NIGHT_STYLE,
};
use crate::StyleType;

/// Set of colors to apply to GUI
///
/// Best practices:
/// - `primary` should be a kind of neutral color
/// - `primary` and `buttons` should be similar colors
/// - `secondary` and one of `incoming` or `outgoing` should be the same color
/// - `incoming` and `outgoing` should be complementary colors if possible
/// - `text_headers` should be black or white and must have a strong contrast with `secondary`
/// - `text_body` should be black or white and must have a strong contrast with `primary`
pub struct Palette {
    /// Main color of the GUI (background, hovered buttons, active tab)
    pub primary: Color,
    /// Secondary color of the GUI (header, footer, buttons' borders, radio selection)
    pub secondary: Color,
    /// Color of active buttons (when not hovered) and inactive tabs
    pub buttons: Color,
    /// Color of incoming connections
    pub incoming: Color,
    /// Color of outgoing connections
    pub outgoing: Color,
    /// Color of header and footer text
    pub text_headers: Color,
    /// Color of body and buttons text
    pub text_body: Color,
    /// Color of round container borders and scrollbar borders
    pub round_borders: Color,
    /// Color of round containers
    pub round_containers: Color,
}

pub fn get_colors(style: StyleType) -> Palette {
    match style {
        StyleType::Night => NIGHT_STYLE,
        StyleType::Day => DAY_STYLE,
        StyleType::DeepSea => DEEP_SEA_STYLE,
        StyleType::MonAmour => MON_AMOUR_STYLE,
    }
}

pub fn to_rgb_color(color: Color) -> RGBColor {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    if color.r <= 1.0
        && color.r >= 0.0
        && color.g <= 1.0
        && color.g >= 0.0
        && color.b <= 1.0
        && color.b >= 0.0
    {
        RGBColor(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        )
    } else {
        RGBColor(0, 0, 0) // Black
    }
}

/// Returns the average of two colors; color intensity is fixed to 100%
pub fn mix_colors(color_1: Color, color_2: Color) -> Color {
    Color {
        r: (color_1.r + color_2.r) / 2.0,
        g: (color_1.g + color_2.g) / 2.0,
        b: (color_1.b + color_2.b) / 2.0,
        a: 1.0,
    }
}

impl Default for Palette {
    fn default() -> Self {
        get_colors(StyleType::Night)
    }
}
