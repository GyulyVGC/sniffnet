#![allow(clippy::unreadable_literal)]

//! Dracula theme
//! <https://draculatheme.com/>
//! Light style from: <https://github.com/AshGrowem/Dracula.min/>
use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

pub static DRACULA_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0x282a36),   // Background
        secondary: color!(0xff79c6), // Pink
        outgoing: color!(0x8be9fd),  // Cyan
        starred: color!(0xf1fa8c, 0.7),
        text_headers: color!(0x282a36), // Background
        text_body: color!(0xf8f8f2),    // Foreground
    });

pub static DRACULA_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| DRACULA_DARK_PALETTE.generate_palette_extension());

// Light Darker variant
pub static DRACULA_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xf8f8f2),
        secondary: color!(0x9f1670),
        outgoing: color!(0x005d6f),
        starred: color!(0xffb86c),
        text_headers: color!(0xf8f8f2),
        text_body: color!(0x282a36),
    });

pub static DRACULA_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| DRACULA_LIGHT_PALETTE.generate_palette_extension());
