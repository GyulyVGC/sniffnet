#![allow(clippy::unreadable_literal)]

//! Themes optimized for Accessibility
//! <https://github.com/GyulyVGC/sniffnet/pull/785>

use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

pub static A11Y_DARK_PALETTE: std::sync::LazyLock<Palette> = std::sync::LazyLock::new(|| Palette {
    primary: color!(0x0f1e3c),
    secondary: color!(0xFCB608),
    outgoing: color!(0x0BB1FC),
    starred: color!(0xFBFF0F),
    text_headers: color!(0x081020),
    text_body: color!(0xdddddd),
});

pub static A11Y_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| A11Y_DARK_PALETTE.generate_palette_extension());

pub static A11Y_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xFFFFFF),
        secondary: color!(0xB30000),
        outgoing: color!(0x004ECC),
        starred: color!(0xC25E00),
        text_headers: color!(0xFFFFFF),
        text_body: color!(0x081020),
    });

pub static A11Y_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| A11Y_LIGHT_PALETTE.generate_palette_extension());
