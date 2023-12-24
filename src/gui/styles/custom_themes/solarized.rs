#![allow(clippy::unreadable_literal)]

//! Solarized
//! <https://ethanschoonover.com/solarized/>
use iced::color;

use crate::gui::styles::types::palette::Palette;

/// Solarized light (Day style)
pub(in crate::gui::styles) fn solarized_light() -> Palette {
    Palette {
        primary: color!(0xfdf6e3),      // base3
        secondary: color!(0x859900),    // green
        outgoing: color!(0x268bd2),     // blue
        starred: color!(0xb58900, 0.9), // yellow
        text_headers: color!(0xfdf6e3), // base3
        text_body: color!(0x002b36),    // base03
    }
}

/// Solarized dark (Night style)
pub(in crate::gui::styles) fn solarized_dark() -> Palette {
    Palette {
        primary: color!(0x002b36),      // base03
        secondary: color!(0x859900),    // green
        outgoing: color!(0x268bd2),     // blue
        starred: color!(0xb58900),      // yellow
        text_headers: color!(0x002b36), // base03
        text_body: color!(0xeee8d5),    // base2
    }
}
