//! Remote implementation of [`serde::Deserialize`] and [`serde::Serialize`] for [`iced::Color`].
//!
//! This implementation deserializes hexadecimal RGB(A) as string to float RGB(A) and back.
//! NOTE: The alpha channel is optional and defaults to #ff or 1.0.
//! `#ffffffff` deserializes to `1.0`, `1.0`, `1.0`, `1.0`.
//! `1.0`, `1.0`, `1.0`, `1.0` serializes to #ffffffff

use std::hash::{Hash, Hasher};

use iced::Color;
use serde::{
    de::{Error as DeErrorTrait, Unexpected},
    Deserialize, Deserializer, Serializer,
};

// #aabbcc is seven bytes long
const HEX_STR_BASE_LEN: usize = 7;
// #aabbccdd is nine bytes long
const HEX_STR_ALPHA_LEN: usize = 9;

pub(super) fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
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
            // TODO: This can be cleaned up when `iter_array_chunks` is stabilized (https://github.com/rust-lang/rust/issues/100450)
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
                            .map(|rgb| f32::from(rgb) / 255.0)
                    })
            })
            .collect::<Result<Vec<f32>, _>>()?;

        // Alpha isn't always part of the color scheme. The resulting Vec should always have at least three elements.
        // Accessing the first three elements without [slice::get] is okay because I checked the length of the hex string earlier.
        Ok(Color {
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

/// Hash delegate for [`iced::Color`] that hashes RGBA in lieu of floats.
#[inline]
pub(super) fn color_hash<H: Hasher>(color: Color, state: &mut H) {
    // Hash isn't implemented for floats, so I hash the color as RGBA instead.
    let color = color.into_rgba8();
    color.hash(state);
}

/// Serialize [`iced::Color`] as a hex string.
#[inline]
pub(super) fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // iced::Color to [u8; 4]
    let color = color.into_rgba8();

    // [u8; 4] to hex string, precluding the alpha if it's 0xff.
    let hex_color = if color[3] == 255 {
        format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2])
    } else {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            color[0], color[1], color[2], color[3]
        )
    };

    // Serialize the hex string
    serializer.serialize_str(&hex_color)
}

#[cfg(test)]
mod tests {
    use iced::Color;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_de_tokens_error, assert_tokens, Token};

    use super::{deserialize_color, serialize_color};

    // https://github.com/catppuccin/catppuccin
    const CATPPUCCIN_PINK_HEX: &str = "#f5c2e7";
    const CATPPUCCIN_PINK: Color = Color {
        r: 245.0 / 255.0,
        g: 194.0 / 255.0,
        b: 231.0 / 255.0,
        a: 1.0,
    };

    const CATPPUCCIN_PINK_HEX_ALPHA: &str = "#f5c2e780";
    const CATPPUCCIN_PINK_ALPHA: Color = Color {
        r: 245.0 / 255.0,
        g: 194.0 / 255.0,
        b: 231.0 / 255.0,
        a: 128.0 / 255.0,
    };

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    #[serde(transparent)]
    struct DelegateTest {
        #[serde(
            flatten,
            deserialize_with = "deserialize_color",
            serialize_with = "serialize_color"
        )]
        color: Color,
    }

    const CATPPUCCIN_PINK_DELEGATE: DelegateTest = DelegateTest {
        color: CATPPUCCIN_PINK,
    };

    const CATPPUCCIN_PINK_ALPHA_DELEGATE: DelegateTest = DelegateTest {
        color: CATPPUCCIN_PINK_ALPHA,
    };

    // Invalid hex colors
    const CATPPUCCIN_PINK_NO_OCTO: &str = "%f5c2e7";
    const CATPPUCCIN_PINK_TRUNCATED: &str = "#c2e7";
    const CATPPUCCIN_PINK_TOO_LONG: &str = "#f5c2e7f5c2e7f5";
    const INVALID_COLOR: &str = "#cal✘";

    // Test if deserializing and serializing a color works.
    #[test]
    fn test_working_color_round_trip() {
        assert_tokens(
            &CATPPUCCIN_PINK_DELEGATE,
            &[Token::Str(CATPPUCCIN_PINK_HEX)],
        );
    }

    // Test if deserializing and serializing a color with an alpha channel works.
    #[test]
    fn test_working_color_with_alpha_round_trip() {
        assert_tokens(
            &CATPPUCCIN_PINK_ALPHA_DELEGATE,
            &[Token::Str(CATPPUCCIN_PINK_HEX_ALPHA)],
        );
    }

    // Missing octothorpe should fail.
    #[test]
    fn test_no_octothrope_color_rt() {
        assert_de_tokens_error::<DelegateTest>(
            &[Token::Str(CATPPUCCIN_PINK_NO_OCTO)],
            "invalid value: character `%`, expected #",
        );
    }

    // A hex color that is missing components should panic.
    #[test]
    fn test_len_too_small_color_de() {
        assert_de_tokens_error::<DelegateTest>(
            &[Token::Str(CATPPUCCIN_PINK_TRUNCATED)],
            "invalid length 5, expected 7 or 9",
        );
    }

    // A hex string that is too long shouldn't deserialize
    #[test]
    fn test_len_too_large_color_de() {
        assert_de_tokens_error::<DelegateTest>(
            &[Token::Str(CATPPUCCIN_PINK_TOO_LONG)],
            "invalid length 15, expected 7 or 9",
        );
    }

    // Invalid hexadecimal should panic
    #[test]
    fn test_invalid_hex_color_de() {
        assert_de_tokens_error::<DelegateTest>(
            &[Token::Str(INVALID_COLOR)],
            "invalid value: string \"#cal✘\", expected valid hexadecimal",
        );
    }
}
