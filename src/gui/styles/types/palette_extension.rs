use std::hash::{Hash, Hasher};

use iced::Color;
use serde::{Deserialize, Serialize};

use super::color_remote::{deserialize_color, serialize_color};
use crate::gui::styles::types::color_remote::color_hash;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::conf::deserialize_or_default;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PaletteExtension {
    #[serde(deserialize_with = "deserialize_or_default")]
    pub is_nightly: bool,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub alpha_chart_badge: f32,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub alpha_round_borders: f32,
    #[serde(deserialize_with = "deserialize_or_default")]
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

impl Default for PaletteExtension {
    fn default() -> Self {
        <StyleType as std::default::Default>::default().get_extension()
    }
}

impl Hash for PaletteExtension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let PaletteExtension {
            is_nightly,
            alpha_chart_badge,
            alpha_round_borders,
            alpha_round_containers,
            buttons_color,
            red_alert_color,
        } = self;

        is_nightly.hash(state);
        #[allow(clippy::cast_possible_truncation)]
        (997 * (alpha_chart_badge + alpha_round_borders + alpha_round_containers) as i32)
            .hash(state);
        color_hash(*buttons_color, state);
        color_hash(*red_alert_color, state);
    }
}

#[cfg(test)]
mod tests {
    use iced::Color;
    use serde_test::{Token, assert_tokens};

    use crate::gui::styles::style_constants::RED_ALERT_COLOR_DAILY;
    use crate::gui::styles::types::palette_extension::PaletteExtension;

    // Test if deserializing and serializing a PaletteExtension works.
    #[test]
    fn test_working_palette_extension_round_trip() {
        let ext = PaletteExtension {
            is_nightly: false,
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
                    len: 6,
                },
                Token::Str("is_nightly"),
                Token::Bool(false),
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
