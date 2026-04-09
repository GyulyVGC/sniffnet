//! Remote implementation of [`serde::Deserialize`] and [`serde::Serialize`] for [`iced::Color`].
//!
//! This implementation deserializes hexadecimal RGB(A) as string to float RGB(A) and back.
//! NOTE: The alpha channel is optional and defaults to #ff or 1.0.
//! `#ffffffff` deserializes to `1.0`, `1.0`, `1.0`, `1.0`.
//! `1.0`, `1.0`, `1.0`, `1.0` serializes to #ffffffff

use std::hash::{Hash, Hasher};

use iced::Color;
use serde::{Deserialize, Deserializer, Serializer};

// #aabbcc is seven bytes long
const HEX_STR_BASE_LEN: usize = 7;
// #aabbccdd is nine bytes long
const HEX_STR_ALPHA_LEN: usize = 9;

#[allow(clippy::unnecessary_wraps)]
pub(super) fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(deserialize_color_inner(deserializer).unwrap_or(Color::BLACK))
}

pub(super) fn deserialize_color_inner<'de, D>(deserializer: D) -> Option<Color>
where
    D: Deserializer<'de>,
{
    let hex = String::deserialize(deserializer).ok()?;

    let hex_len = hex.len();
    if hex_len == HEX_STR_BASE_LEN || hex_len == HEX_STR_ALPHA_LEN {
        let digits_str = hex.strip_prefix('#')?;

        let r_str = digits_str.get(0..2)?;
        let g_str = digits_str.get(2..4)?;
        let b_str = digits_str.get(4..6)?;
        let a_str = digits_str.get(6..8).unwrap_or("ff");

        let r = u8::from_str_radix(r_str, 16).ok()?;
        let g = u8::from_str_radix(g_str, 16).ok()?;
        let b = u8::from_str_radix(b_str, 16).ok()?;
        let a = u8::from_str_radix(a_str, 16).ok()?;

        Some(Color {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: f32::from(a) / 255.0,
        })
    } else {
        None
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
    use serde_test::{Token, assert_de_tokens, assert_tokens};

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

    // test if deserializing and serializing a color works.
    #[test]
    fn test_working_color_round_trip() {
        assert_tokens(
            &CATPPUCCIN_PINK_DELEGATE,
            &[Token::Str(CATPPUCCIN_PINK_HEX)],
        );
    }

    // test if deserializing and serializing a color with an alpha channel works.
    #[test]
    fn test_working_color_with_alpha_round_trip() {
        assert_tokens(
            &CATPPUCCIN_PINK_ALPHA_DELEGATE,
            &[Token::Str(CATPPUCCIN_PINK_HEX_ALPHA)],
        );
    }

    // missing octothorpe should fail -> use Color::BLACK
    #[test]
    fn test_no_octothrope_color_rt() {
        assert_de_tokens(
            &DelegateTest {
                color: Color::BLACK,
            },
            &[Token::Str(CATPPUCCIN_PINK_NO_OCTO)],
        );
    }

    // a hex color that is missing components should fail -> use Color::BLACK
    #[test]
    fn test_len_too_small_color_de() {
        assert_de_tokens(
            &DelegateTest {
                color: Color::BLACK,
            },
            &[Token::Str(CATPPUCCIN_PINK_TRUNCATED)],
        );
    }

    // a hex string that is too long should fail -> use Color::BLACK
    #[test]
    fn test_len_too_large_color_de() {
        assert_de_tokens(
            &DelegateTest {
                color: Color::BLACK,
            },
            &[Token::Str(CATPPUCCIN_PINK_TOO_LONG)],
        );
    }

    // invalid hexadecimal should fail -> use Color::BLACK
    #[test]
    fn test_invalid_hex_color_de() {
        assert_de_tokens(
            &DelegateTest {
                color: Color::BLACK,
            },
            &[Token::Str(INVALID_COLOR)],
        );
    }

    // not string should fail -> use Color::BLACK
    #[test]
    fn test_no_string_color_de() {
        assert_de_tokens(
            &DelegateTest {
                color: Color::BLACK,
            },
            &[Token::I8(12)],
        );
    }
}
