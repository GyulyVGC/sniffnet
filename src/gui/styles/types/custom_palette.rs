use std::fmt;

use iced::Color;
use serde::{Deserialize, Serialize};

use crate::gui::styles::custom_themes::{dracula, gruvbox, nord, solarized};
use crate::gui::styles::types::palette::Palette;

/// Custom style with any relevant metadata
pub struct CustomPalette {
    /// Color scheme's palette
    pub(crate) palette: Palette,
    /// Extra colors such as the favorites star
    pub(crate) extension: PaletteExtension,
}

/// Extension color for themes.
pub struct PaletteExtension {
    /// Color of favorites star
    pub starred: Color,
    /// Badge/logo alpha
    pub chart_badge_alpha: f32,
    /// Round borders alpha
    pub round_borders_alpha: f32,
    /// Round containers alpha
    pub round_containers_alpha: f32,
}

/// Built in extra styles
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
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
        }
    }

    /// Theme is a night/dark style
    pub const fn is_nightly(self) -> bool {
        match self {
            ExtraStyles::DraculaDark
            | ExtraStyles::GruvboxDark
            | ExtraStyles::NordDark
            | ExtraStyles::SolarizedDark => true,
            ExtraStyles::DraculaLight
            | ExtraStyles::GruvboxLight
            | ExtraStyles::NordLight
            | ExtraStyles::SolarizedLight => false,
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
        }
    }
}
