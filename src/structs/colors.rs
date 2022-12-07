//! Module defining the `Colors` struct, which defines the colors in use in the GUI.

use crate::utility::style_constants::{DAY_STYLE, NIGHT_STYLE};
use crate::StyleType;
use iced::Color;
use plotters::style::RGBColor;

/// Set of colors to apply to GUI
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
}

pub fn get_colors(style: StyleType) -> Colors {
    match style {
        StyleType::Night => NIGHT_STYLE,
        StyleType::Day => DAY_STYLE,
    }
}

pub fn to_rgb_color(color: Color) -> RGBColor {
    RGBColor(
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
    )
}
