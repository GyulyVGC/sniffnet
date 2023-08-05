#![allow(clippy::unreadable_literal)]

//! Nord theme
//! https://www.nordtheme.com/docs/colors-and-palettes
use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

pub(super) fn nord() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0x2e3440),          // nord0
            secondary: color!(0x88c0d0),        // nord8
            outgoing: color!(0x81a1c1),         // nord9
            buttons: color!(0x3b4252),          // nord1
            text_headers: color!(0x4c566a),     // nord3
            text_body: color!(0xd8dee9),        // nord4
            round_borders: color!(0xe5e9f0),    // nord5
            round_containers: color!(0x434c5e), // nord2
        },
        extension: PaletteExtension {
            starred: color!(0xebcb8b), // nord13
            badge_alpha: 0.75,
            color_mixing: 0.3,
        },
    }
}
