use std::hash::Hash;

use serde::{Deserialize, Serialize};

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
