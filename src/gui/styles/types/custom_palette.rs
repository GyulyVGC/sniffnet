use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::path::Path;

use iced::Color;
use serde::{de::Error as DeErrorTrait, Deserialize, Serialize};

use crate::gui::styles::custom_themes::{dracula, gruvbox, nord, solarized};
use crate::gui::styles::types::palette::Palette;

use super::color_remote::{color_hash, deserialize_color, serialize_color};

const FLOAT_PRECISION: f32 = 10000.0;

/// Custom style with any relevant metadata
// NOTE: This is flattened for ergonomics. With flatten, both [Palette] and [PaletteExtension] can be
// defined in the TOML as a single entity rather than two separate tables. This is intentional because
// the separation between palette and its extension is an implementation detail that shouldn't be exposed
// to custom theme designers.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct CustomPalette {
    /// Base colors for the theme
    #[serde(flatten)]
    pub(crate) palette: Palette,
    /// Extra colors such as the favorites star
    #[serde(flatten)]
    pub(crate) extension: PaletteExtension,
}

/// Extension color for themes.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct PaletteExtension {
    /// Color of favorites star
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
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
    /// Deserialize [`CustomPalette`] from `path`.
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

        toml::de::from_str(&style_toml)
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

        extension.hash(state);
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
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        ((*chart_badge_alpha * FLOAT_PRECISION).trunc() as u32).hash(state);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        ((*round_borders_alpha * FLOAT_PRECISION).trunc() as u32).hash(state);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
            ExtraStyles::CustomToml(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use iced::color;

    use super::{CustomPalette, Palette, PaletteExtension};

    fn style_path(name: &str) -> String {
        format!(
            "{}/resources/themes/{}.toml",
            env!("CARGO_MANIFEST_DIR"),
            name
        )
    }

    // NOTE: This has to be updated if `resources/themes/catppuccin.toml` changes
    fn catppuccin_style() -> CustomPalette {
        CustomPalette {
            palette: Palette {
                primary: color!(0x30, 0x34, 0x46),
                secondary: color!(0xa6, 0xd1, 0x89),
                buttons: color!(0x41, 0x45, 0x59),
                outgoing: color!(0xf4, 0xb8, 0xe4),
                text_headers: color!(0x23, 0x26, 0x34),
                text_body: color!(0xc6, 0xd0, 0xf5),
            },
            extension: PaletteExtension {
                starred: color!(0xe5, 0xc8, 0x90, 0.6666667),
                round_borders_alpha: 0.4,
                round_containers_alpha: 0.25,
                chart_badge_alpha: 0.2,
                nightly: true,
            },
        }
    }

    #[test]
    fn custompalette_from_file_de() -> Result<(), toml::de::Error> {
        let style = catppuccin_style();
        let style_de = CustomPalette::from_file(style_path("catppuccin"))?;

        assert_eq!(style, style_de);
        Ok(())
    }
}
