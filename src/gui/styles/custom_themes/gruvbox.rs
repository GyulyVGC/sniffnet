#![allow(clippy::unreadable_literal)]

//! Gruvbox
//! <https://github.com/morhetz/gruvbox>

use iced::color;

use crate::gui::styles::types::custom_palette::{CustomPalette, PaletteExtension};
use crate::gui::styles::types::palette::Palette;

/// Gruvbox (night style)
pub(in crate::gui::styles) fn gruvbox_dark() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0x282828),      // bg
            secondary: color!(0xfe8019),    // orange
            outgoing: color!(0x8ec07c),     // aqua
            buttons: color!(0x7c6f64),      // bg4
            text_headers: color!(0x1d2021), // bg0_h
            text_body: color!(0xebdbb2),    // fg
        },
        extension: PaletteExtension {
            starred: color!(0xd79921, 0.8),
            chart_badge_alpha: 0.15,
            round_borders_alpha: 0.12,
            round_containers_alpha: 0.05,
            nightly: true,
        },
    }
}

/// Gruvbox (day style)
pub(in crate::gui::styles) fn gruvbox_light() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0xfbf1c7),      // bg
            secondary: color!(0xd65d0e),    // orange
            outgoing: color!(0x689d6a),     // aqua
            buttons: color!(0xd5c4a1),      // bg2
            text_headers: color!(0xf9f5d7), // bg0_h
            text_body: color!(0x282828),    // fg
        },
        extension: PaletteExtension {
            starred: color!(0xd79921, 0.8), // yellow
            chart_badge_alpha: 0.75,
            round_borders_alpha: 0.45,
            round_containers_alpha: 0.2,
            nightly: false,
        },
    }
}
