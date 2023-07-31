use std::fmt;

use iced::Color;
use serde::{Deserialize, Serialize};

use super::palette::Palette;

/// Extension color for themes.
pub struct PaletteExtension {
    /// Color of favorites star
    pub starred: Color,
    /// Badge/logo alpha
    pub badge_alpha: f32,
    /// Traffic chart color mixing
    pub color_mixing: f64,
}

/// Custom style with any relevant metadata
// pub struct CustomPalette {
//    name: &'static str,
//    palette: Palette,
//    extension: PaletteExtension,
//}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
pub enum ExtraStyles {
    Dracula,
}

impl ExtraStyles {
    pub fn to_palette(self) -> Palette {
        match self {
            ExtraStyles::Dracula => unimplemented!(),
        }
    }

    pub fn to_ext(self) -> PaletteExtension {
        match self {
            ExtraStyles::Dracula => unimplemented!(),
        }
    }
}

impl fmt::Display for ExtraStyles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ExtraStyles::Dracula => write!(f, "Dracula"),
        }
    }
}
