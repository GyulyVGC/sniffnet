use iced::{Color, Font};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::hash::{Hash, Hasher};
use serde::de::Unexpected;
use crate::gui::styles::style_constants::{SARASA_MONO, SARASA_MONO_BOLD};
use crate::gui::styles::types::color_remote::color_hash;

use super::color_remote::{deserialize_color, serialize_color};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaletteExtension {
    pub is_nightly: bool,
    #[serde(
    deserialize_with = "deserialize_font",
    serialize_with = "serialize_font"
    )]
    pub font: Font,
    #[serde(
    deserialize_with = "deserialize_font",
    serialize_with = "serialize_font"
    )]
    pub font_headers: Font,
    pub alpha_chart_badge: f32,
    pub alpha_round_borders: f32,
    pub alpha_round_containers: f32,
    #[serde(
    deserialize_with = "deserialize_color",
    serialize_with = "serialize_color"
    )]
    pub buttons_color: Color,
}

impl Hash for PaletteExtension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let PaletteExtension {
            is_nightly,
            font,
            font_headers,
            alpha_chart_badge,
            alpha_round_borders,
            alpha_round_containers,
            buttons_color,
        } = self;

        is_nightly.hash(state);
        font.hash(state);
        font_headers.hash(state);
        (997*(alpha_chart_badge + alpha_round_borders + alpha_round_containers) as u32).hash(state);
        color_hash(*buttons_color, state);
    }
}

pub(super) fn deserialize_font<'de, D>(deserializer: D) -> Result<Font, D::Error>
    where
        D: Deserializer<'de>,
{
    // Name should be SARASA_MONO or SARASA_MONO_BOLD
    let name = String::deserialize(deserializer)?;

    let regular_string = String::from("SARASA_MONO");
    let bold_string = String::from("SARASA_MONO_BOLD");

    match name {
        regular_string => Ok(SARASA_MONO),
        bold_string => Ok(SARASA_MONO_BOLD),
        _ => Err(serde::de::Error::invalid_value(Unexpected::Str(&name), &"SARASA_MONO OR SARASA_MONO_BOLD"))
    }
}

pub(super) fn serialize_font<S>(font: &Font, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    match font {
        &SARASA_MONO => serializer.serialize_str("SARASA_MONO"),
        &SARASA_MONO_BOLD => serializer.serialize_str("SARASA_MONO_BOLD"),
        _ => Err(serde::ser::Error::custom("invalid font")),
    }
}
