//! Remote type to implement [serde::Deserialize] and [serde::Serialize] for [iced::Color].
//! Check [serde's documentation](https://serde.rs/remote-derive.html) for details.
//!
//! This implementation deserializes hexadecimal RGB to float RGB.

use serde::{
    de::{Error as DeErrorTrait, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

// #aabbcc is seven bytes long
const HEX_STR_BASE_LEN: usize = 7;
// #aabbccdd is nine bytes long
const HEX_STR_ALPHA_LEN: usize = 9;

/// Serde delegate type for [iced::Color].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "iced::Color")]
pub(super) struct ColorDelegate {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<'de> Deserialize<'de> for ColorDelegate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Field should be a hex string i.e. #aabbcc
        let hex = String::deserialize(deserializer)?;

        // The string should be seven bytes long (octothorpe + six hex chars).
        // Safety: Hexadecimal is ASCII so bytes are okay here.
        let hex_len = hex.len();
        if hex_len == HEX_STR_BASE_LEN || hex_len == HEX_STR_ALPHA_LEN {
            let color = hex
                .strip_prefix('#') // Remove the octothorpe or fail
                .ok_or_else(|| {
                    DeErrorTrait::invalid_value(
                        Unexpected::Char(hex.chars().next().unwrap_or_default()),
                        &"#",
                    )
                })?
                // Iterating over bytes is safe because hex is ASCII.
                // If the hex is not ASCII or invalid hex, then the iterator will short circuit and fail on `from_str_radix`
                .bytes()
                .step_by(2) // Step by every first hex char of the two char sequence
                .zip(hex.bytes().skip(2).step_by(2)) // Step by every second hex char
                .map(|(first, second)| {
                    // Parse hex strings
                    let maybe_hex = [first, second];
                    std::str::from_utf8(&maybe_hex)
                        .map_err(|_| {
                            DeErrorTrait::invalid_value(Unexpected::Str(&hex), &"valid hexadecimal")
                        })
                        .and_then(|s| {
                            u8::from_str_radix(s, 16)
                                .map_err(DeErrorTrait::custom)
                                .map(|rgb| rgb as f32 / 255.0)
                        })
                })
                .collect::<Result<Vec<f32>, _>>()?;

            // Alpha isn't always part of the color scheme. The resulting Vec should always have at least three elements.
            Ok(Self {
                r: color[0],
                g: color[1],
                b: color[2],
                a: *color.get(3).unwrap_or(&1.0),
            })
        } else {
            Err(DeErrorTrait::invalid_length(
                hex_len,
                &&*format!("{HEX_STR_BASE_LEN} or {HEX_STR_ALPHA_LEN}"),
            ))
        }
    }
}

impl Serialize for ColorDelegate {
    // Serialize Color as a hex string.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // ColorDelegate to iced::Color
        let Self { r, g, b, a } = *self;
        let color = iced::Color { r, g, b, a };

        // iced::Color to [u8; 4]
        let color = color.into_rgba8();

        // [u8; 3] to hex string (alpha isn't serialized because it should be 0xff anyway)
        let hex_color: String = format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2]);

        // Serialize the hex string
        serializer.serialize_str(&hex_color)
    }
}

#[cfg(test)]
mod tests {
    use super::ColorDelegate;
    use iced::Color;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    // https://github.com/catppuccin/catppuccin
    const CATPPUCCIN_PINK_HEX: &str = "#f5c2e7";
    const CATPPUCCIN_PINK: ColorDelegate = ColorDelegate {
        r: 245.0 / 255.0,
        g: 194.0 / 255.0,
        b: 231.0 / 255.0,
        a: 1.0,
    };

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    struct DelegateTest {
        #[serde(flatten, with = "ColorDelegate")]
        color: Color,
    }

    const ICED_CATPPUCCIN_PINK: DelegateTest = DelegateTest {
        color: Color {
            r: 245.0 / 255.0,
            g: 194.0 / 255.0,
            b: 231.0 / 255.0,
            a: 1.0,
        },
    };

    // Invalid hex colors
    const CATPPUCCIN_PINK_NO_OCTO: &str = "%f5c2e7";
    const CATPPUCCIN_PINK_TRUNCATED: &str = "#c2e7";
    const CATPPUCCIN_PINK_TOO_LONG: &str = "#f5c2e7f5c2e7f5";
    const INVALID_COLOR: &str = "#ca🐈";

    // Test if deserializing and serializing ColorDelegate works.
    #[test]
    fn test_working_color_round_trip() {
        assert_tokens(&CATPPUCCIN_PINK, &[Token::Str(CATPPUCCIN_PINK_HEX)]);
    }

    // Test iced::Color using ColorDelegate as a delegate
    #[test]
    fn test_working_iced_color_round_trip() {
        serde_test::assert_de_tokens(&ICED_CATPPUCCIN_PINK, &[Token::Str(CATPPUCCIN_PINK_HEX)]);
    }

    // Missing octothorpe should fail.
    #[test]
    fn test_no_octothrope_color_rt() {
        assert_de_tokens_error::<ColorDelegate>(
            &[Token::Str(CATPPUCCIN_PINK_NO_OCTO)],
            "invalid value: character `%`, expected #",
        );
    }

    // A hex color that is missing components should panic.
    #[test]
    fn test_len_too_small_color_de() {
        assert_de_tokens_error::<ColorDelegate>(
            &[Token::Str(CATPPUCCIN_PINK_TRUNCATED)],
            "invalid length 5, expected 7 or 9",
        );
    }

    #[test]
    fn test_len_too_large_color_de() {
        assert_de_tokens_error::<ColorDelegate>(
            &[Token::Str(CATPPUCCIN_PINK_TOO_LONG)],
            "invalid length 15, expected 7 or 9",
        );
    }

    #[test]
    fn test_invalid_hex_color_de() {
        assert_de_tokens_error::<ColorDelegate>(
            &[Token::Str(INVALID_COLOR)],
            "invalid value: string \"#ca🐈\", expected valid hexadecimal",
        );
    }
}
