#![allow(clippy::unreadable_literal)]

//! Solarized
//! <https://ethanschoonover.com/solarized/>
use iced::color;

use crate::gui::styles::types::custom_palette::{CustomPalette, PaletteExtension};
use crate::gui::styles::types::palette::Palette;

/// Solarized light (Day style)
pub(in crate::gui::styles) fn solarized_light() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0xfdf6e3),      // base3
            secondary: color!(0x859900),    // green
            outgoing: color!(0x268bd2),     // blue
            buttons: color!(0x93a1a1),      // base1
            text_headers: color!(0xfdf6e3), // base3
            text_body: color!(0x002b36),    // base03
        },
        extension: PaletteExtension {
            starred: color!(0xb58900, 0.9), // yellow
            chart_badge_alpha: 0.75,
            round_borders_alpha: 0.35,
            round_containers_alpha: 0.15,
            nightly: true,
        },
    }
}

/// Solarized dark (Night style)
pub(in crate::gui::styles) fn solarized_dark() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0x002b36),      // base03
            secondary: color!(0x859900),    // green
            outgoing: color!(0x268bd2),     // blue
            buttons: color!(0x586e75),      // base01
            text_headers: color!(0x002b36), // base03
            text_body: color!(0xeee8d5),    // base2
        },
        extension: PaletteExtension {
            starred: color!(0xb58900), // yellow
            chart_badge_alpha: 0.25,
            round_borders_alpha: 0.15,
            round_containers_alpha: 0.08,
            nightly: true,
        },
    }
}
