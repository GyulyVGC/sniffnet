//! Custom Sniffnet color schemes.
//! Themes should be in TOML files with the following schema.
//! ```toml
//! name = "Theme's name"
//!
//! [description]
//! # Descriptions of the theme in any of Sniffnet's supported languages
//! # CODE = "DESC"
//! EN = "A fantastically dark theme!"
//! HU = ""
//! PL = ""
//!
//! # The palette is specified in RGB hexadecimal with an optional alpha
//! [palette]
//! primary = "#000000"
//! secondary = "#000000"
//! buttons = "#000000"
//! outgoing = "#000000"
//! text_headers = "#000000"
//! text_body = "#000000"
//! round_borders = "#000000"
//! round_containers = "#000000"
//! starred = "#000000"
//!
//! # floats for these two alpha values
//! badge_alpha = 0.9
//! color_mix_chart = 0.3
//! ```

use serde::{de::Error as DeErrorTrait, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{self, BufReader, Read},
    path::PathBuf,
};

use super::palette::{Palette, PaletteExtension};
use crate::Language;

/// Custom color scheme data including the palette, name, and location of the toml.
#[derive(Debug, PartialEq, Hash, Clone, Deserialize, Serialize)]
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
    pub description: BTreeMap<Language, String>,
    /// Color scheme's Sniffnet palette.
    /// Should be an implementation of the scheme that is tuned to Sniffnet.
    pub palette: CustomPalette,
}

impl CustomStyle {
    /// Deserialize [CustomStyle] from `path`.
    ///
    /// # Arguments
    /// * `path` - Path to a UTF-8 encoded file containing a custom style as TOML.
    pub fn from_file<P>(path: P) -> Result<Self, toml::de::Error>
    where
        P: Into<String>,
    {
        // Try to open the file at `path`
        let path = path.into();
        let mut toml_reader = File::open(&path)
            .map_err(DeErrorTrait::custom)
            .map(BufReader::new)?;

        // Read the ostensible TOML
        let mut style_toml = String::new();
        toml_reader
            .read_to_string(&mut style_toml)
            .map_err(DeErrorTrait::custom)?;

        // Deserialize it and store `path` into the resulting struct
        toml::de::from_str::<CustomStyle>(&style_toml).map(|mut style| {
            style.path = path;
            style
        })
    }

    /// Load [CustomStyle]s from a directory.
    ///
    /// # Errors
    /// [io::Error] is only returned if `dir` can't be read. A best effort is made to read any styles
    /// present in the directory.
    ///
    /// Styles that cannot be read are ignored.
    pub fn from_dir<P>(dir: P) -> Result<impl Iterator<Item = Self>, io::Error>
    where
        P: Into<PathBuf>,
    {
        let iter = fs::read_dir(dir.into())?.filter_map(|entry| {
            let entry = entry.ok()?.path();
            Self::from_file(entry.to_str()?).ok()
        });
        Ok(iter)
    }

    /// Return translated description or a default.
    ///
    /// Defaults to English is the language isn't implemented or "" if English is missing too.
    ///
    /// # Arguments
    /// * `language` - Description language
    pub fn description(&self, language: Language) -> &str {
        self.description
            .get(&language)
            .map(|s| s.as_str())
            .unwrap_or_else(|| {
                self.description
                    .get(&Language::EN)
                    .map(|s| s.as_str())
                    .unwrap_or_default()
            })
    }
}

/// Base [Palette] and extension colors for [CustomStyle].
// NOTE: This is flattened for ergonomics. With flatten, both [Palette] and [PaletteExtension] can be
// defined in the TOML as a single entity rather than two separate tables. This is intentional because
// the separation between palette and its extension is an implementation detail that shouldn't be exposed
// to custom theme designers.
//
// Clippy complains about deriving [Hash] with a manually written [PartialEq]. We manually implemented
// Hash for [Palette] and [PaletteExtension], so deriving Hash is convenient and the error is spurious.
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Hash, Clone, PartialEq, Deserialize, Serialize)]
pub struct CustomPalette {
    /// Base colors as used for the default sniffnet themes.
    #[serde(flatten)]
    pub base: Palette,
    /// Extension colors such as the yellow used for favorites.
    #[serde(flatten)]
    pub extension: PaletteExtension,
}

/// Deserialize [CustomStyle] from a file path.
///
/// This is implemented by first deserializing a file path which in turn contains the style as TOML.
#[inline]
pub(super) fn deserialize_from_path<'de, D>(deserializer: D) -> Result<CustomStyle, D::Error>
where
    D: Deserializer<'de>,
{
    let path = String::deserialize(deserializer)?;
    CustomStyle::from_file(path).map_err(DeErrorTrait::custom)
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
    use crate::{translations::types::language::Language, StyleType};
    use iced::color;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use std::collections::BTreeMap;

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

    const STYLE_DESC_ENG: &str = "Soothing pastel theme for the high-spirited!";
    // Hungarian translation by Emi.
    const STYLE_DESC_HU: &str = "Catpuccin egy színes, közepes kontrasztú, pasztell téma.\nhttps://github.com/catppuccin/catppuccin";
    // Polish translation by Bartosz.
    const STYLE_DESC_PL: &str = "Kojący pastelowy motyw dla porywczych";

    // NOTE: This has to be updated if `resources/themes/catppuccin_mocha.toml` changes
    fn catppuccin_style() -> StyleForTests {
        StyleForTests(CustomStyle {
            name: "Catppuccin (Mocha)".to_owned(),
            path: style_path(),
            description: BTreeMap::from([
                (Language::EN, STYLE_DESC_ENG.to_owned()),
                // (Language::HU, STYLE_DESC_HU.to_owned()),
                (Language::PL, STYLE_DESC_PL.to_owned()),
            ]),
            palette: CustomPalette {
                base: Palette {
                    primary: color!(30, 30, 46),
                    secondary: color!(137, 180, 250),
                    buttons: color!(49, 50, 68),
                    outgoing: color!(245, 194, 231),
                    text_headers: color!(17, 17, 27),
                    text_body: color!(205, 214, 244),
                    round_borders: color!(180, 190, 254),
                    round_containers: color!(24, 24, 37),
                },
                extension: PaletteExtension {
                    starred: color!(249, 226, 175),
                    badge_alpha: 0.75,
                    color_mix_chart: 0.3,
                },
            },
        })
    }

    // Test that split deserialization works for `CustomStyle`.
    // This is different than testing that `StyleType` properly deserializes.
    #[test]
    fn test_customstyle_split_de() {
        let style_test = catppuccin_style();
        // This is only used for the test which requires an &'static str.
        let path: &'static str = Box::leak(style_path().into_boxed_str());
        assert_tokens(&style_test, &[Token::String(path)]);
    }

    // Ensure that StyleType itself still deserializes properly
    #[test]
    fn test_styletype_unit_split_de() {
        // Unit variant without a struct
        assert_tokens(
            &StyleType::DeepSea,
            &[
                Token::Struct {
                    name: "StyleType",
                    len: 1,
                },
                Token::Str("style"),
                Token::Str("DeepSea"),
                Token::StructEnd,
            ],
        );
    }

    // Test that StyleType::Custom successfully deserializes.
    // Originally, StyleType::Custom did not ser/de correctly because of how TOML
    // handles enums.
    #[test]
    fn test_styletype_custom_split_de() {
        // CustomStyle
        // This is only used for the test so leaking it is fine.
        let path = &*Box::leak(style_path().into_boxed_str());
        assert_tokens(
            &StyleType::Custom(catppuccin_style().0),
            &[
                Token::Struct {
                    name: "StyleType",
                    len: 2,
                },
                Token::Str("style"),
                Token::Str("Custom"),
                Token::Str("path"),
                Token::Str(path),
                Token::StructEnd,
            ],
        );
    }
}
