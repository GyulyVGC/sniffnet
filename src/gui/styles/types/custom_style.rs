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


