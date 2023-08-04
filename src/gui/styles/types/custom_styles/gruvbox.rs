#![allow(clippy::unreadable_literal)]

//! Gruvbox
//! <https://github.com/morhetz/gruvbox>

use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

pub(super) fn gruvbox_dark() -> CustomPalette {
    CustomPalette {
        name: "Gruvbox (Dark)",
        palette: Palette {
            primary: color!(0x282828),          // bg
            secondary: color!(0xfe8019),        // orange
            outgoing: color!(0x8ec07c),         // aqua
            buttons: color!(0x928374),          // gray
            text_headers: color!(0x1d2021),     // bg0_h
            text_body: color!(0xebdbb2),        // fg
            round_borders: color!(0x98971a),    // green
            round_containers: color!(0x504945), // bg2
        },
        extension: PaletteExtension {
            starred: color!(0xd79921),
            badge_alpha: 0.75,
            color_mixing: 0.3,
        },
    }
}
