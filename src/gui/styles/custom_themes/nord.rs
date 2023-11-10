#![allow(clippy::unreadable_literal)]

//! Nord theme
//! <https://www.nordtheme.com/docs/colors-and-palettes>
use iced::color;

use crate::gui::styles::types::custom_palette::{CustomPalette, PaletteExtension};
use crate::gui::styles::types::palette::Palette;

pub(in crate::gui::styles) fn nord_dark() -> CustomPalette {
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
            nightly: true,
        },
    }
}

pub(in crate::gui::styles) fn nord_light() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0xeceff4),      // nord6
            secondary: color!(0x05e81ac),   // nord10
            outgoing: color!(0xb48ead),     // nord15
            buttons: color!(0x8FBCBB),      // nord7
            text_headers: color!(0xeceff4), // nord6
            text_body: color!(0x2e3440),    // nord0
        },
        extension: PaletteExtension {
            starred: color!(0xebcb8b), // nord13
            chart_badge_alpha: 0.6,
            round_borders_alpha: 0.35,
            round_containers_alpha: 0.15,
            nightly: false,
        },
    }
}
