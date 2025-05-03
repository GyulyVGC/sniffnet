use std::hash::{Hash, Hasher};

use iced::{Color, Font};
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub red_alert_color: Color,
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
            red_alert_color,
        } = self;

        is_nightly.hash(state);
        font.hash(state);
        font_headers.hash(state);
        #[allow(clippy::cast_possible_truncation)]
        (997 * (alpha_chart_badge + alpha_round_borders + alpha_round_containers) as i32)
            .hash(state);
        color_hash(*buttons_color, state);
        color_hash(*red_alert_color, state);
    }
}

pub(super) fn deserialize_font<'de, D>(deserializer: D) -> Result<Font, D::Error>
where
    D: Deserializer<'de>,
{
    // Name should be SARASA_MONO or SARASA_MONO_BOLD
    let name = String::deserialize(deserializer)?;
    let name_str = name.as_str();

    match name_str {
        "SARASA_MONO" => Ok(SARASA_MONO),
        "SARASA_MONO_BOLD" => Ok(SARASA_MONO_BOLD),
        _ => Err(serde::de::Error::invalid_value(
            Unexpected::Str(name_str),
            &"SARASA_MONO OR SARASA_MONO_BOLD",
        )),
    }
}

pub(super) fn serialize_font<S>(font: &Font, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *font {
        SARASA_MONO => serializer.serialize_str("SARASA_MONO"),
        SARASA_MONO_BOLD => serializer.serialize_str("SARASA_MONO_BOLD"),
        _ => Err(serde::ser::Error::custom("invalid font")),
    }
}

#[cfg(test)]
mod tests {
    use iced::Color;
    use serde_test::{Token, assert_tokens};

    use crate::gui::styles::style_constants::{
        RED_ALERT_COLOR_DAILY, SARASA_MONO, SARASA_MONO_BOLD,
    };
    use crate::gui::styles::types::palette_extension::PaletteExtension;

    // Test if deserializing and serializing a PaletteExtension works.
    #[test]
    fn test_working_palette_extension_round_trip() {
        let ext = PaletteExtension {
            is_nightly: false,
            font: SARASA_MONO_BOLD,
            font_headers: SARASA_MONO,
            alpha_chart_badge: 0.5,
            alpha_round_borders: 0.25,
            alpha_round_containers: 0.1778,
            buttons_color: Color {
                r: 0.6,
                g: 0.4,
                b: 0.2,
                a: 1.0,
            },
            red_alert_color: RED_ALERT_COLOR_DAILY,
        };
        assert_tokens(
            &ext,
            &[
                Token::Struct {
                    name: "PaletteExtension",
                    len: 8,
                },
                Token::Str("is_nightly"),
                Token::Bool(false),
                Token::Str("font"),
                Token::Str("SARASA_MONO_BOLD"),
                Token::Str("font_headers"),
                Token::Str("SARASA_MONO"),
                Token::Str("alpha_chart_badge"),
                Token::F32(0.5),
                Token::Str("alpha_round_borders"),
                Token::F32(0.25),
                Token::Str("alpha_round_containers"),
                Token::F32(0.1778),
                Token::Str("buttons_color"),
                Token::Str("#996633"),
                Token::Str("red_alert_color"),
                Token::Str("#b30000"),
                Token::StructEnd,
            ],
        );
    }
}
