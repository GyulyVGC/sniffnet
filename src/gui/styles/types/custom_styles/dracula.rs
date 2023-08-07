#![allow(clippy::unreadable_literal)]

//! Dracula theme
//! <https://draculatheme.com/>
//! Light style from: https://github.com/AshGrowem/Dracula.min/
use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

pub(super) fn dracula_dark() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0x282a36),      // Background
            secondary: color!(0xff79c6),    // Pink
            outgoing: color!(0x8be9fd),     // Cyan
            buttons: color!(0x6272a4),      // Comments
            text_headers: color!(0x282a36), // Background
            text_body: color!(0xf8f8f2),    // Foreground
        },
        extension: PaletteExtension {
            starred: color!(0xf1fa8c, 0.7),
            round_borders_alpha: 0.1,
            round_containers_alpha: 0.04,
            chart_badge_alpha: 0.15,
        },
    }
}

pub(super) fn dracula_light() -> CustomPalette {
    CustomPalette {
        palette: Palette {
            primary: color!(0xf1f2ff),
            secondary: color!(0xff79c6),
            outgoing: color!(0x8be9fd),
            buttons: color!(0xcbcee5),
            text_headers: color!(0xf1f2ff),
            text_body: color!(0x282a36),
        },
        extension: PaletteExtension {
            starred: color!(0xf1fa8c, 0.7),
            chart_badge_alpha: 0.25,
            round_borders_alpha: 0.15,
            round_containers_alpha: 0.05,
        },
    }
}
