#![allow(clippy::unreadable_literal)]

//! Original light and dark themes for Sniffnet

use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

pub static YETI_DARK_PALETTE: std::sync::LazyLock<Palette> = std::sync::LazyLock::new(|| Palette {
    primary: color!(0x282828),
    secondary: color!(0xb35900),
    outgoing: color!(0x0059b3),
    starred: color!(0xf5c127, 0.5),
    text_headers: color!(0x000000),
    text_body: color!(0xffffff),
});

pub static YETI_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| YETI_DARK_PALETTE.generate_palette_extension());

pub static YETI_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xffffff),
        secondary: color!(0x0059b3),
        outgoing: color!(0xb35900),
        starred: color!(0xd7a313, 0.9),
        text_headers: color!(0xffffff),
        text_body: color!(0x000000),
    });

pub static YETI_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| YETI_LIGHT_PALETTE.generate_palette_extension());
