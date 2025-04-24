#![allow(clippy::unreadable_literal)]

//! Solarized
//! <https://ethanschoonover.com/solarized/>
use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

/// Solarized light (Day style)
pub static SOLARIZED_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xfdf6e3),      // base3
        secondary: color!(0x859900),    // green
        outgoing: color!(0x268bd2),     // blue
        starred: color!(0xb58900, 0.9), // yellow
        text_headers: color!(0xfdf6e3), // base3
        text_body: color!(0x002b36),    // base03
    });

pub static SOLARIZED_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| SOLARIZED_LIGHT_PALETTE.generate_palette_extension());

/// Solarized dark (Night style)
pub static SOLARIZED_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0x002b36),      // base03
        secondary: color!(0x859900),    // green
        outgoing: color!(0x268bd2),     // blue
        starred: color!(0xb58900),      // yellow
        text_headers: color!(0x002b36), // base03
        text_body: color!(0xeee8d5),    // base2
    });

pub static SOLARIZED_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| SOLARIZED_DARK_PALETTE.generate_palette_extension());
