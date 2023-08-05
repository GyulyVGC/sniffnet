#![allow(clippy::unreadable_literal)]

//! Gruvbox
//! <https://github.com/morhetz/gruvbox>

use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

/// Gruvbox (night style)
pub(super) fn gruvbox_dark() -> CustomPalette {
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
        },
    }
}
