#![allow(clippy::unreadable_literal)]

//! Gruvbox
//! <https://github.com/morhetz/gruvbox>

use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

/// Gruvbox (night style)
pub static GRUVBOX_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0x282828),   // bg
        secondary: color!(0xfe8019), // orange
        outgoing: color!(0x8ec07c),  // aqua
        starred: color!(0xd79921, 0.8),
        text_headers: color!(0x1d2021), // bg0_h
        text_body: color!(0xebdbb2),    // fg
    });

pub static GRUVBOX_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| GRUVBOX_DARK_PALETTE.generate_palette_extension());

/// Gruvbox (day style)
pub static GRUVBOX_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xfbf1c7),      // bg
        secondary: color!(0xd65d0e),    // orange
        outgoing: color!(0x689d6a),     // aqua
        starred: color!(0xd79921, 0.9), // yellow
        text_headers: color!(0xf9f5d7), // bg0_h
        text_body: color!(0x282828),    // fg
    });

pub static GRUVBOX_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| GRUVBOX_LIGHT_PALETTE.generate_palette_extension());
