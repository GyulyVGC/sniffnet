use std::fmt;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

use iced::Color;
use serde::{de::Error as DeErrorTrait, Deserialize, Serialize};

use super::color_remote::{color_hash, deserialize_color};
use crate::gui::styles::custom_themes::{dracula, gruvbox, nord, solarized};
use crate::gui::styles::types::palette::Palette;

const FLOAT_PRECISION: f32 = 10000.0;

/// Custom style with any relevant metadata
// NOTE: This is flattened for ergonomics. With flatten, both [Palette] and [PaletteExtension] can be
// defined in the TOML as a single entity rather than two separate tables. This is intentional because
// the separation between palette and its extension is an implementation detail that shouldn't be exposed
// to custom theme designers.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct CustomPalette {
    /// Base colors for the theme
    #[serde(flatten)]
    pub(crate) palette: Palette,
    /// Extra colors such as the favorites star
    #[serde(flatten)]
    pub(crate) extension: PaletteExtension,
}

/// Extension color for themes.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct PaletteExtension {
    /// Color of favorites star
    #[serde(deserialize_with = "deserialize_color")]
    pub starred: Color,
    /// Badge/logo alpha
    pub chart_badge_alpha: f32,
    /// Round borders alpha
    pub round_borders_alpha: f32,
    /// Round containers alpha
    pub round_containers_alpha: f32,
    /// Nightly (dark) style
    pub nightly: bool,
}

impl CustomPalette {
    /// Deserialize [CustomPalette] from `path`.
    ///
    /// # Arguments
    /// * `path` - Path to a UTF-8 encoded file containing a custom style as TOML.
    pub fn from_file<P>(path: P) -> Result<Self, toml::de::Error>
    where
        P: AsRef<Path>,
    {
        // Try to open the file at `path`
        let mut toml_reader = File::open(path)
            .map_err(DeErrorTrait::custom)
            .map(BufReader::new)?;

        // Read the ostensible TOML
        let mut style_toml = String::new();
        toml_reader
            .read_to_string(&mut style_toml)
            .map_err(DeErrorTrait::custom)?;

        // Deserialize it and store `path` into the resulting struct
        // toml::de::from_str::<CustomPalette>(&style_toml).map(|mut style| {
        //     style.path = path;
        //     style
        // })

        toml::de::from_str(&style_toml)
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
}

impl Hash for CustomPalette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self { palette, extension } = self;

        let Palette {
            primary,
            secondary,
            outgoing,
            buttons,
            text_headers,
            text_body,
        } = palette;

        color_hash(*primary, state);
        color_hash(*secondary, state);
        color_hash(*outgoing, state);
        color_hash(*buttons, state);
        color_hash(*text_headers, state);
        color_hash(*text_body, state);

        extension.hash(state)
    }
}

impl Hash for PaletteExtension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self {
            starred,
            chart_badge_alpha,
            round_borders_alpha,
            round_containers_alpha,
            ..
        } = self;

        color_hash(*starred, state);
        ((*chart_badge_alpha * FLOAT_PRECISION).trunc() as u32).hash(state);
        ((*round_borders_alpha * FLOAT_PRECISION).trunc() as u32).hash(state);
        ((*round_containers_alpha * FLOAT_PRECISION).trunc() as u32).hash(state);
    }
}

/// Built in extra styles
#[derive(Clone, Copy, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "custom")]
pub enum ExtraStyles {
    DraculaDark,
    DraculaLight,
    GruvboxDark,
    GruvboxLight,
    NordDark,
    NordLight,
    SolarizedDark,
    SolarizedLight,
    #[serde(skip)]
    CustomToml(CustomPalette),
}

impl ExtraStyles {
    /// [`Palette`] of the [`ExtraStyles`] variant
    pub fn to_palette(self) -> Palette {
        match self {
            ExtraStyles::DraculaLight => dracula::dracula_light().palette,
            ExtraStyles::DraculaDark => dracula::dracula_dark().palette,
            ExtraStyles::GruvboxDark => gruvbox::gruvbox_dark().palette,
            ExtraStyles::GruvboxLight => gruvbox::gruvbox_light().palette,
            ExtraStyles::NordLight => nord::nord_light().palette,
            ExtraStyles::NordDark => nord::nord_dark().palette,
            ExtraStyles::SolarizedDark => solarized::solarized_dark().palette,
            ExtraStyles::SolarizedLight => solarized::solarized_light().palette,
            ExtraStyles::CustomToml(user) => user.palette,
        }
    }

    /// Extension colors for the current [`ExtraStyles`] variant
    pub fn to_ext(self) -> PaletteExtension {
        match self {
            ExtraStyles::DraculaLight => dracula::dracula_light().extension,
            ExtraStyles::DraculaDark => dracula::dracula_dark().extension,
            ExtraStyles::GruvboxDark => gruvbox::gruvbox_dark().extension,
            ExtraStyles::GruvboxLight => gruvbox::gruvbox_light().extension,
            ExtraStyles::NordLight => nord::nord_light().extension,
            ExtraStyles::NordDark => nord::nord_dark().extension,
            ExtraStyles::SolarizedDark => solarized::solarized_dark().extension,
            ExtraStyles::SolarizedLight => solarized::solarized_light().extension,
            ExtraStyles::CustomToml(user) => user.extension,
        }
    }

    /// Theme is a night/dark style
    pub fn is_nightly(self) -> bool {
        self.to_ext().nightly
    }

    /// Slice of all implemented custom styles
    pub const fn all_styles() -> &'static [Self] {
        &[
            ExtraStyles::DraculaDark,
            ExtraStyles::DraculaLight,
            ExtraStyles::GruvboxDark,
            ExtraStyles::GruvboxLight,
            ExtraStyles::NordDark,
            ExtraStyles::NordLight,
            ExtraStyles::SolarizedDark,
            ExtraStyles::SolarizedLight,
        ]
    }
}

impl fmt::Display for ExtraStyles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExtraStyles::DraculaLight => write!(f, "Dracula (Day)"),
            ExtraStyles::DraculaDark => write!(f, "Dracula (Night)"),
            ExtraStyles::GruvboxDark => write!(f, "Gruvbox (Night)"),
            ExtraStyles::GruvboxLight => write!(f, "Gruvbox (Day)"),
            ExtraStyles::NordLight => write!(f, "Nord (Day)"),
            ExtraStyles::NordDark => write!(f, "Nord (Night)"),
            ExtraStyles::SolarizedLight => write!(f, "Solarized (Day)"),
            ExtraStyles::SolarizedDark => write!(f, "Solarized (Night)"),
            // Custom style names aren't used anywhere so this shouldn't be reached
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        deserialize_from_path, serialize_to_path, CustomPalette, Palette, PaletteExtension,
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
        CustomPalette,
    );

    // Test items

    // Replace with const format when it's stable
    fn style_path() -> String {
        format!(
            "{}/resources/themes/catppuccin_mocha.toml",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    // NOTE: This has to be updated if `resources/themes/catppuccin_mocha.toml` changes
    fn catppuccin_style() -> StyleForTests {
        StyleForTests(CustomPalette {
            palette: Palette {
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
