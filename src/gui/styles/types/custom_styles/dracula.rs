//! Dracula theme
//! https://draculatheme.com/
use iced::color;

use super::{CustomPalette, Palette, PaletteExtension};

pub(super) fn dracula() -> CustomPalette {
    CustomPalette {
        name: "Dracula",
        palette: Palette {
            primary: color!(0x44475a),          // Current line
            secondary: color!(0xff79c6),        // Pink
            outgoing: color!(0x8be9fd),         // Cyan
            buttons: color!(0x6272a4),          // Comments
            text_headers: color!(0x44475a),     // Current line
            text_body: color!(0xf8f8f2),        // Foreground
            round_borders: color!(0xbd93f9),    // Purple
            round_containers: color!(0x44475a), // Current line
        },
        extension: PaletteExtension {
            starred: color!(0xf1fa8c),
            badge_alpha: 0.75,
            color_mixing: 0.3,
        },
    }
}
