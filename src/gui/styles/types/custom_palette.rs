use std::fmt;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::gui::styles::custom_themes::dracula::{
    DRACULA_DARK_PALETTE, DRACULA_DARK_PALETTE_EXTENSION, DRACULA_LIGHT_PALETTE,
    DRACULA_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::gruvbox::{
    GRUVBOX_DARK_PALETTE, GRUVBOX_DARK_PALETTE_EXTENSION, GRUVBOX_LIGHT_PALETTE,
    GRUVBOX_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::nord::{
    NORD_DARK_PALETTE, NORD_DARK_PALETTE_EXTENSION, NORD_LIGHT_PALETTE,
    NORD_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::solarized::{
    SOLARIZED_DARK_PALETTE, SOLARIZED_DARK_PALETTE_EXTENSION, SOLARIZED_LIGHT_PALETTE,
    SOLARIZED_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct CustomPalette {
    #[serde(flatten)]
    pub(crate) palette: Palette,
    #[serde(flatten)]
    pub(crate) extension: PaletteExtension,
}

impl CustomPalette {
    pub fn from_palette(palette: Palette) -> Self {
        Self {
            palette,
            extension: palette.generate_palette_extension(),
        }
    }
}

/// Built in extra styles
#[derive(Clone, Copy, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "custom", content = "attributes")]
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
    pub fn get_palette(self) -> Palette {
        match self {
            ExtraStyles::DraculaDark => *DRACULA_DARK_PALETTE,
            ExtraStyles::DraculaLight => *DRACULA_LIGHT_PALETTE,
            ExtraStyles::GruvboxDark => *GRUVBOX_DARK_PALETTE,
            ExtraStyles::GruvboxLight => *GRUVBOX_LIGHT_PALETTE,
            ExtraStyles::NordDark => *NORD_DARK_PALETTE,
            ExtraStyles::NordLight => *NORD_LIGHT_PALETTE,
            ExtraStyles::SolarizedDark => *SOLARIZED_DARK_PALETTE,
            ExtraStyles::SolarizedLight => *SOLARIZED_LIGHT_PALETTE,
            ExtraStyles::CustomToml(custom_palette) => custom_palette.palette,
        }
    }

    /// [`PaletteExtension`] of the [`ExtraStyles`] variant
    pub fn get_extension(self) -> PaletteExtension {
        match self {
            ExtraStyles::DraculaDark => *DRACULA_DARK_PALETTE_EXTENSION,
            ExtraStyles::DraculaLight => *DRACULA_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::GruvboxDark => *GRUVBOX_DARK_PALETTE_EXTENSION,
            ExtraStyles::GruvboxLight => *GRUVBOX_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::NordDark => *NORD_DARK_PALETTE_EXTENSION,
            ExtraStyles::NordLight => *NORD_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::SolarizedDark => *SOLARIZED_DARK_PALETTE_EXTENSION,
            ExtraStyles::SolarizedLight => *SOLARIZED_LIGHT_PALETTE_EXTENSION,
            ExtraStyles::CustomToml(custom_palette) => custom_palette.extension,
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
