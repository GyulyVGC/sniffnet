#![allow(clippy::unreadable_literal)]

//! Nord theme
//! <https://www.nordtheme.com/docs/colors-and-palettes>
use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

pub(super) fn nord() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0x2e3440),      // nord0
            secondary: color!(0x88c0d0),    // nord8
            outgoing: color!(0xB48EAD),     // nord15
            buttons: color!(0x4C566A),      // nord3
            text_headers: color!(0x2e3440), // nord0
            text_body: color!(0xd8dee9),    // nord4
        },
        extension: PaletteExtension {
            starred: color!(0xebcb8b), // nord13
            chart_badge_alpha: 0.2,
            round_borders_alpha: 0.35,
            round_containers_alpha: 0.15,
        },
    }
}
