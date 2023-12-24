use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::path::Path;

use serde::{de::Error as DeErrorTrait, Deserialize, Serialize};

use crate::gui::styles::custom_themes::{dracula, gruvbox, nord, solarized};
use crate::gui::styles::types::palette::Palette;

use super::color_remote::{color_hash};

impl Palette {
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

impl Hash for Palette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Palette {
            primary,
            secondary,
            outgoing,
            starred,
            text_headers,
            text_body,
        } = self;

        color_hash(*primary, state);
        color_hash(*secondary, state);
        color_hash(*outgoing, state);
        color_hash(*starred, state);
        color_hash(*text_headers, state);
        color_hash(*text_body, state);
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
    CustomToml(Palette),
}

impl ExtraStyles {
    /// [`Palette`] of the [`ExtraStyles`] variant
    pub fn to_palette(self) -> Palette {
        match self {
            ExtraStyles::DraculaLight => dracula::dracula_light(),
            ExtraStyles::DraculaDark => dracula::dracula_dark(),
            ExtraStyles::GruvboxDark => gruvbox::gruvbox_dark(),
            ExtraStyles::GruvboxLight => gruvbox::gruvbox_light(),
            ExtraStyles::NordLight => nord::nord_light(),
            ExtraStyles::NordDark => nord::nord_dark(),
            ExtraStyles::SolarizedDark => solarized::solarized_dark(),
            ExtraStyles::SolarizedLight => solarized::solarized_light(),
            ExtraStyles::CustomToml(user) => user,
        }
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
