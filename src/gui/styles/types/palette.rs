//! Module defining the `Colors` struct, which defines the colors in use in the GUI.

use iced::Color;
use plotters::style::RGBColor;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use super::color_remote::{color_hash, deserialize_color, serialize_color};
use crate::gui::styles::style_constants::{
    DAY_STYLE, DEEP_SEA_STYLE, MON_AMOUR_STYLE, NIGHT_STYLE,
};
use crate::StyleType;

/// Set of colors to apply to GUI
///
/// Best practices:
/// - `primary` should be a kind of neutral color
/// - `primary` and `buttons` should be similar colors
/// - `secondary` and `outgoing` should be complementary colors if possible
/// - `text_headers` should be black or white and must have a strong contrast with `secondary`
/// - `text_body` should be black or white and must have a strong contrast with `primary`
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Palette {
    /// Main color of the GUI (background, hovered buttons, active tab)
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub primary: Color,
    /// Secondary color of the GUI (incoming connections, header, footer, buttons' borders, radio selection)
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub secondary: Color,
    /// Color of outgoing connections
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub outgoing: Color,
    /// Color of active buttons (when not hovered) and inactive tabs
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub buttons: Color,
    /// Color of header and footer text
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_headers: Color,
    /// Color of body and buttons text
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_body: Color,
    /// Color of round container borders and scrollbar borders
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub round_borders: Color,
    /// Color of round containers
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub round_containers: Color,
}

pub fn get_colors(style: &StyleType) -> &Palette {
    match style {
        StyleType::Night => &NIGHT_STYLE,
        StyleType::Day => &DAY_STYLE,
        StyleType::DeepSea => &DEEP_SEA_STYLE,
        StyleType::MonAmour => &MON_AMOUR_STYLE,
        StyleType::Custom(style) => &style.palette.base,
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
        get_colors(&StyleType::Night).clone()
    }
}

impl Hash for Palette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE: Destructuring Palette here is useful in case the struct gains new fields.
        // Rust will helpfully fail to compile due to missing fields.
        let Palette {
            primary,
            secondary,
            buttons,
            outgoing,
            text_headers,
            text_body,
            round_borders,
            round_containers,
        } = self;

        color_hash(*primary, state);
        color_hash(*secondary, state);
        color_hash(*buttons, state);
        color_hash(*outgoing, state);
        color_hash(*text_headers, state);
        color_hash(*text_body, state);
        color_hash(*round_borders, state);
        color_hash(*round_containers, state);
    }
}

/// Extension colors for custom themes.
// NOTE: The purpose of this type is primarily to avoid modifying the existing [Palette].
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct PaletteExtension {
    /// Color of favorites star
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub starred: Color,
    /// Badge alpha channel
    pub badge_alpha: f32,
    /// Color mixing for charts
    pub color_mix_chart: f64,
}

impl Hash for PaletteExtension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        color_hash(self.starred, state);
        // f32::NAN is 0i32 when casted using `as`.
        let alpha: i32 = (self.badge_alpha * 1000.0).trunc() as i32;
        alpha.hash(state);
        let color_mix: i32 = (self.color_mix_chart * 1000.0).trunc() as i32;
        color_mix.hash(state)
    }
}
