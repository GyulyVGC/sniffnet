#![allow(clippy::unreadable_literal)]

//! Solarized
//! <https://ethanschoonover.com/solarized/>
use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

/// Solarized light (Day style)
pub(super) fn solarized_light() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0xfdf6e3),          // base3
            secondary: color!(0x859900),        // green
            outgoing: color!(0x268bd2),         // blue
            buttons: color!(0xeee8d5),          // base2
            text_headers: color!(0xfdf6e3),     // base3
            text_body: color!(0x93a1a1),        // base1
            round_borders: color!(0xcb4b16),    // orange
            round_containers: color!(0xeee8d5), // base2
        },
        extension: PaletteExtension {
            starred: color!(0xb58900), // yellow
            badge_alpha: 0.75,
            color_mixing: 0.8,
        },
    }
}

/// Solarized dark (Night style)
pub(super) fn solarized_dark() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0x002b36),          // base03
            secondary: color!(0x859900),        // green
            outgoing: color!(0x268bd2),         // blue
            buttons: color!(0x586e75),          // base02
            text_headers: color!(0x002b36),     // base03
            text_body: color!(0x839496),        // base0
            round_borders: color!(0xcb4b16),    // orange
            round_containers: color!(0x586e75), // base2
        },
        extension: PaletteExtension {
            starred: color!(0xb58900), // yellow
            badge_alpha: 0.75,
            color_mixing: 0.3,
        },
    }
}
