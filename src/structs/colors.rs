//! Module defining the `Colors` struct, which defines the colors in use in the GUI.

use crate::utility::style_constants::{ALMOND_STYLE, DAY_STYLE, NIGHT_STYLE, RED_STYLE, TRY_STYLE};
use crate::StyleType;
use iced::Color;
use plotters::style::RGBColor;

/// Set of colors to apply to GUI
///
/// Best practices:
/// - primary should be a kind of neutral color
/// - primary and buttons should be similar colors
/// - secondary and one of incoming or outgoing should be the same color
/// - incoming and outgoing should be complementary colors if possible
/// - text_headers should be black or white and must have a strong contrast with secondary
/// - text_body should be black or white and must have a strong contrast with primary
pub struct Colors {
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
}

pub fn get_colors(style: StyleType) -> Colors {
    match style {
        StyleType::Night => NIGHT_STYLE,
        StyleType::Day => DAY_STYLE,
        StyleType::Try => TRY_STYLE,
        StyleType::Almond => ALMOND_STYLE,
        StyleType::Red => RED_STYLE,
    }
}

pub fn to_rgb_color(color: Color) -> RGBColor {
    RGBColor(
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    )
}
