//! Custom Sniffnet color schemes.
//! Themes should be in TOML files with the following schema.
//! ```toml
//! ```

use serde::{de::Error as DeErrorTrait, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use super::palette::{Palette, PaletteExtension};
use crate::Language;

#[cfg(test)]
use super::color_remote::color_partialeq;

/// Custom color scheme data including the palette, name, and location of the toml.
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomStyle {
    /// Display name of the color scheme.
    /// This is the user facing color scheme name that may be displayed in the UI.
    /// Ex. Catppuccin Mocha
    pub name: String,
    /// Internal or path name.
    /// Ex. resources/catppuccin_mocha.toml
    /// This field isn't deserialized because the theme shouldn't provide it.
    /// However, the path is serialized into Sniffnet's config to reload the theme on launch.
    #[serde(skip)]
    pub path: String,
    /// Short description of the color scheme
    pub description: HashMap<Language, String>,
    /// Color scheme's Sniffnet palette.
    /// Should be an implementation of the scheme that is tuned to Sniffnet.
    pub palette: CustomPalette,
}

/// Base [Palette] and extension colors for [CustomStyle].
// NOTE: This is flattened for ergonomics. With flatten, both [Palette] and [PaletteExtension] can be
// defined in the TOML as a single entity rather than two separate tables. This is intentional because
// the separation between palette and its extension is an implementation detail that shouldn't be exposed
// to custom theme designers.
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomPalette {
    /// Base colors as used for the default sniffnet themes.
    #[serde(flatten)]
    pub base: Palette,
    /// Extension colors such as the yellow used for favorites.
    #[serde(flatten)]
    pub extension: PaletteExtension,
}

#[cfg(test)]
impl PartialEq for CustomPalette {
    fn eq(&self, other: &Self) -> bool {
        let Palette {
            primary,
            secondary,
            buttons,
            incoming,
            outgoing,
            text_headers,
            text_body,
            round_borders,
            round_containers,
        } = self.base;

        let PaletteExtension {
            starred,
            badge_alpha,
        } = self.extension;

        // Other
        let Palette {
            primary: primary_other,
            secondary: secondary_other,
            buttons: buttons_other,
            incoming: incoming_other,
            outgoing: outgoing_other,
            text_headers: text_headers_other,
            text_body: text_body_other,
            round_borders: round_borders_other,
            round_containers: round_containers_other,
        } = other.base;

        let PaletteExtension {
            starred: starred_other,
            badge_alpha: badge_alpha_other,
        } = other.extension;

        color_partialeq(primary, primary_other)
            && color_partialeq(secondary, secondary_other)
            && color_partialeq(buttons, buttons_other)
            && color_partialeq(incoming, incoming_other)
            && color_partialeq(outgoing, outgoing_other)
            && color_partialeq(text_headers, text_headers_other)
            && color_partialeq(text_body, text_body_other)
            && color_partialeq(round_borders, round_borders_other)
            && color_partialeq(round_containers, round_containers_other)
            && color_partialeq(starred, starred_other)
            && badge_alpha == badge_alpha_other
    }
}

/// Deserialize [CustomStyle] from a file path.
///
/// This is implemented by first deserializing a file path which in turn contains the style as TOML.
pub(super) fn deserialize_from_path<'de, D>(deserializer: D) -> Result<CustomStyle, D::Error>
where
    D: Deserializer<'de>,
{
    let path = String::deserialize(deserializer)?;
    let mut toml_reader = File::open(&path)
        .map_err(DeErrorTrait::custom)
        .map(BufReader::new)?;
    let mut style_toml = String::new();
    toml_reader
        .read_to_string(&mut style_toml)
        .map_err(DeErrorTrait::custom)?;
    toml::de::from_str::<CustomStyle>(&style_toml)
        .map_err(DeErrorTrait::custom)
        .map(|mut style| {
            style.path = path;
            style
        })
}

/// Serialize [CustomStyle]'s path.
///
/// Themes aren't serialized because they're already located somewhere else (the TOML file from which it was loaded).
/// However, the theme's path must be serialized so that Sniffnet can reload it after the program is restarted.
#[inline]
pub(super) fn serialize_to_path<S>(style: &CustomStyle, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&style.path)
}

#[cfg(test)]
mod tests {
    use super::{
        deserialize_from_path, serialize_to_path, CustomPalette, CustomStyle, Palette,
        PaletteExtension,
    };
    use crate::translations::types::language::Language;
    use iced::Color;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use std::collections::HashMap;

    // Convenience struct for testing
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    #[serde(transparent)]
    struct StyleForTests(
        #[serde(
            deserialize_with = "deserialize_from_path",
            serialize_with = "serialize_to_path"
        )]
        CustomStyle,
    );

    // Test items

    // Replace with const format when it's stable
    fn style_path() -> String {
        format!(
            "{}/resources/themes/catppuccin_mocha.toml",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    const STYLE_DESC_ENG: &str = "Catppuccin is a colorful, medium contrast pastel theme.\nhttps://github.com/catppuccin/catppuccin";
    // Polish translation by Bartosz.
    const STYLE_DESC_PL: &str = "Catppuccin to kolorowy i pastelowy motyw o średnim kontraście.\nhttps://github.com/catppuccin/catppuccin";

    fn catppuccin_style() -> StyleForTests {
        StyleForTests(CustomStyle {
            name: "Catppuccin (Mocha)".to_owned(),
            path: style_path(),
            description: HashMap::from([
                (Language::EN, STYLE_DESC_ENG.to_owned()),
                (Language::PL, STYLE_DESC_PL.to_owned()),
            ]),
            palette: CustomPalette {
                base: Palette {
                    primary: Color {
                        r: 30.0 / 255.0,
                        g: 30.0 / 255.0,
                        b: 46.0 / 255.0,
                        a: 1.0,
                    },
                    secondary: Color {
                        r: 137.0 / 255.0,
                        g: 180.0 / 255.0,
                        b: 250.0 / 255.0,
                        a: 1.0,
                    },
                    buttons: Color {
                        r: 137.0 / 255.0,
                        g: 220.0 / 255.0,
                        b: 235.0 / 255.0,
                        a: 1.0,
                    },
                    incoming: Color {
                        r: 137.0 / 255.0,
                        g: 180.0 / 255.0,
                        b: 250.0 / 255.0,
                        a: 1.0,
                    },
                    outgoing: Color {
                        r: 245.0 / 255.0,
                        g: 194.0 / 255.0,
                        b: 231.0 / 255.0,
                        a: 1.0,
                    },
                    text_headers: Color {
                        r: 205.0 / 255.0,
                        g: 214.0 / 255.0,
                        b: 244.0 / 255.0,
                        a: 1.0,
                    },
                    text_body: Color {
                        r: 205.0 / 255.0,
                        g: 214.0 / 255.0,
                        b: 244.0 / 255.0,
                        a: 1.0,
                    },
                    round_borders: Color {
                        r: 116.0 / 255.0,
                        g: 199.0 / 255.0,
                        b: 236.0 / 255.0,
                        a: 1.0,
                    },
                    round_containers: Color {
                        r: 88.0 / 255.0,
                        g: 91.0 / 255.0,
                        b: 112.0 / 255.0,
                        a: 1.0,
                    },
                },
                extension: PaletteExtension {
                    starred: Color {
                        r: 249.0 / 255.0,
                        g: 226.0 / 255.0,
                        b: 175.0 / 255.0,
                        a: 1.0,
                    },
                    badge_alpha: 0.75,
                },
            },
        })
    }

    #[test]
    fn test_styletype_split_de() {
        let style_test = catppuccin_style();
        // This is only used for the test which requires an &'static str.
        let path: &'static str = Box::leak(style_path().into_boxed_str());
        assert_tokens(&style_test, &[Token::String(path)]);
    }
}
