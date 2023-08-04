mod dracula;
mod gruvbox;
mod solarized;

use std::fmt;

use iced::Color;
use serde::{Deserialize, Serialize};

use super::palette::Palette;

/// Custom style with any relevant metadata
pub struct CustomPalette {
    /// Displayable name of the style (i.e. "Catppuccin (Mocha)")
    name: &'static str,
    /// Color scheme's palette
    palette: Palette,
    /// Extra colors such as the favorites star
    extension: PaletteExtension,
}

/// Extension color for themes.
pub struct PaletteExtension {
    /// Color of favorites star
    pub starred: Color,
    /// Badge/logo alpha
    pub badge_alpha: f32,
    /// Traffic chart color mixing
    pub color_mixing: f64,
}

/// Built in extra styles
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
#[serde(tag = "custom")]
pub enum ExtraStyles {
    Dracula,
    Gruvbox,
    SolarizedLight,
}

impl ExtraStyles {
    /// [`Palette`] of the [`ExtraStyles`] variant
    #[inline]
    pub fn to_palette(self) -> Palette {
        match self {
            ExtraStyles::Dracula => dracula::dracula().palette,
            ExtraStyles::Gruvbox => gruvbox::gruvbox_dark().palette,
            ExtraStyles::SolarizedLight => solarized::solarized_light().palette,
        }
    }

    /// Extension colors for the current [`ExtraStyles`] variant
    #[inline]
    pub fn to_ext(self) -> PaletteExtension {
        match self {
            ExtraStyles::Dracula => dracula::dracula().extension,
            ExtraStyles::Gruvbox => gruvbox::gruvbox_dark().extension,
            ExtraStyles::SolarizedLight => solarized::solarized_light().extension,
        }
    }

    /// Theme is a night/dark style
    #[inline]
    pub const fn is_nightly(self) -> bool {
        match self {
            ExtraStyles::Dracula | ExtraStyles::Gruvbox => true,
            ExtraStyles::SolarizedLight => false,
        }
    }

    /// Slice of all implemented custom styles
    #[inline]
    pub const fn all_styles() -> &'static [Self] {
        &[
            ExtraStyles::Dracula,
            ExtraStyles::Gruvbox,
            ExtraStyles::SolarizedLight,
        ]
    }
}

impl fmt::Display for ExtraStyles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExtraStyles::Dracula => write!(f, "Dracula"),
            ExtraStyles::Gruvbox => write!(f, "Gruvbox (Dark)"),
            ExtraStyles::SolarizedLight => write!(f, "Solarized (Light)"),
        }
    }
}