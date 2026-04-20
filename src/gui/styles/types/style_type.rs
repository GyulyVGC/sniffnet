use crate::gui::styles::custom_themes::a11y::{
    A11Y_DARK_PALETTE, A11Y_DARK_PALETTE_EXTENSION, A11Y_LIGHT_PALETTE,
    A11Y_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::dracula::{
    DRACULA_DARK_PALETTE, DRACULA_DARK_PALETTE_EXTENSION, DRACULA_LIGHT_PALETTE,
    DRACULA_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::gruvbox::{
    GRUVBOX_DARK_PALETTE, GRUVBOX_DARK_PALETTE_EXTENSION, GRUVBOX_LIGHT_PALETTE,
    GRUVBOX_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::nord::{
    NORD_DARK_PALETTE, NORD_DARK_PALETTE_EXTENSION, NORD_LIGHT_PALETTE,
    NORD_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::solarized::{
    SOLARIZED_DARK_PALETTE, SOLARIZED_DARK_PALETTE_EXTENSION, SOLARIZED_LIGHT_PALETTE,
    SOLARIZED_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::custom_themes::yeti::{
    YETI_DARK_PALETTE, YETI_DARK_PALETTE_EXTENSION, YETI_LIGHT_PALETTE,
    YETI_LIGHT_PALETTE_EXTENSION,
};
use crate::gui::styles::types::custom_palette::CustomPalette;
use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;
use iced::theme::{Base, Mode, Style};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Used to specify the kind of style of the application
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq, Default)]
#[serde(tag = "style", content = "attributes")]
#[allow(clippy::large_enum_variant)]
pub enum StyleType {
    #[default]
    A11yDark,
    A11yLight,
    DraculaDark,
    DraculaLight,
    GruvboxDark,
    GruvboxLight,
    NordDark,
    NordLight,
    SolarizedDark,
    SolarizedLight,
    YetiDark,
    YetiLight,
    Custom(CustomPalette),
}

impl Base for StyleType {
    fn default(preference: Mode) -> Self {
        match preference {
            Mode::Light => Self::A11yLight,
            _ => Self::A11yDark,
        }
    }

    fn mode(&self) -> Mode {
        if self.get_extension().is_nightly {
            Mode::Dark
        } else {
            Mode::Light
        }
    }

    fn base(&self) -> Style {
        let colors = self.get_palette();
        Style {
            background_color: colors.primary,
            text_color: colors.text_body,
        }
    }

    fn palette(&self) -> Option<iced::theme::Palette> {
        None
    }

    fn name(&self) -> &str {
        match self {
            Self::A11yDark => "A11y Dark",
            Self::A11yLight => "A11y Light",
            Self::DraculaDark => "Dracula Dark",
            Self::DraculaLight => "Dracula Light",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::GruvboxLight => "Gruvbox Light",
            Self::NordDark => "Nord Dark",
            Self::NordLight => "Nord Light",
            Self::SolarizedDark => "Solarized Dark",
            Self::SolarizedLight => "Solarized Light",
            Self::YetiDark => "Yeti Dark",
            Self::YetiLight => "Yeti Light",
            Self::Custom(_) => "Custom",
        }
    }
}

impl StyleType {
    /// [`Palette`] of the [`StyleType`] variant
    pub fn get_palette(self) -> Palette {
        match self {
            Self::A11yDark => *A11Y_DARK_PALETTE,
            Self::A11yLight => *A11Y_LIGHT_PALETTE,
            Self::DraculaDark => *DRACULA_DARK_PALETTE,
            Self::DraculaLight => *DRACULA_LIGHT_PALETTE,
            Self::GruvboxDark => *GRUVBOX_DARK_PALETTE,
            Self::GruvboxLight => *GRUVBOX_LIGHT_PALETTE,
            Self::NordDark => *NORD_DARK_PALETTE,
            Self::NordLight => *NORD_LIGHT_PALETTE,
            Self::SolarizedDark => *SOLARIZED_DARK_PALETTE,
            Self::SolarizedLight => *SOLARIZED_LIGHT_PALETTE,
            Self::YetiDark => *YETI_DARK_PALETTE,
            Self::YetiLight => *YETI_LIGHT_PALETTE,
            Self::Custom(custom_palette) => custom_palette.palette,
        }
    }

    /// [`PaletteExtension`] of the [`StyleType`] variant
    pub fn get_extension(self) -> PaletteExtension {
        match self {
            Self::A11yDark => *A11Y_DARK_PALETTE_EXTENSION,
            Self::A11yLight => *A11Y_LIGHT_PALETTE_EXTENSION,
            Self::DraculaDark => *DRACULA_DARK_PALETTE_EXTENSION,
            Self::DraculaLight => *DRACULA_LIGHT_PALETTE_EXTENSION,
            Self::GruvboxDark => *GRUVBOX_DARK_PALETTE_EXTENSION,
            Self::GruvboxLight => *GRUVBOX_LIGHT_PALETTE_EXTENSION,
            Self::NordDark => *NORD_DARK_PALETTE_EXTENSION,
            Self::NordLight => *NORD_LIGHT_PALETTE_EXTENSION,
            Self::SolarizedDark => *SOLARIZED_DARK_PALETTE_EXTENSION,
            Self::SolarizedLight => *SOLARIZED_LIGHT_PALETTE_EXTENSION,
            Self::YetiDark => *YETI_DARK_PALETTE_EXTENSION,
            Self::YetiLight => *YETI_LIGHT_PALETTE_EXTENSION,
            Self::Custom(custom_palette) => custom_palette.extension,
        }
    }

    /// Slice of all implemented custom styles
    pub const fn all_styles() -> &'static [Self] {
        &[
            Self::A11yDark,
            Self::A11yLight,
            Self::DraculaDark,
            Self::DraculaLight,
            Self::GruvboxDark,
            Self::GruvboxLight,
            Self::NordDark,
            Self::NordLight,
            Self::SolarizedDark,
            Self::SolarizedLight,
            Self::YetiDark,
            Self::YetiLight,
        ]
    }
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::A11yLight | Self::A11yDark => write!(f, "A11y"),
            Self::DraculaLight | Self::DraculaDark => write!(f, "Dracula"),
            Self::GruvboxDark | Self::GruvboxLight => write!(f, "Gruvbox"),
            Self::NordLight | Self::NordDark => write!(f, "Nord"),
            Self::SolarizedLight | Self::SolarizedDark => write!(f, "Solarized"),
            Self::YetiLight | Self::YetiDark => write!(f, "Yeti"),
            Self::Custom(_) => write!(f, "Custom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use iced::{Color, color};
    use serde_test::{Token, assert_tokens};

    use crate::StyleType;
    use crate::gui::styles::types::custom_palette::CustomPalette;
    use crate::gui::styles::types::palette::Palette;

    // test if deserializing and serializing a StyleType works n.1
    // simple case: one of the default themes
    #[test]
    fn test_working_style_type_round_trip_1() {
        let style = StyleType::A11yDark;
        assert_tokens(
            &style,
            &[
                Token::Struct {
                    name: "StyleType",
                    len: 1,
                },
                Token::Str("style"),
                Token::UnitVariant {
                    name: "StyleType",
                    variant: "A11yDark",
                },
                Token::StructEnd,
            ],
        );
    }

    // test if deserializing and serializing a StyleType works n.2
    // complex case: a custom theme from a TOML file
    #[test]
    fn test_working_style_type_round_trip_2() {
        let palette = Palette {
            primary: color!(0x22, 0x22, 0x22),
            secondary: color!(0xa6, 0xd1, 0x89),
            outgoing: color!(0xf4, 0xb8, 0xe4),
            starred: color!(0xe5, 0xc8, 0x90, 0.6666667),
            text_headers: color!(0x23, 0x26, 0x34),
            text_body: color!(0xc6, 0xd0, 0xf5),
        };
        let mut extension = palette.generate_palette_extension();
        // reassigned only because of floating precision errors otherwise
        extension.buttons_color = Color {
            r: 0.28235295,
            g: 0.28235295,
            b: 0.28235295,
            a: 1.0,
        };
        let custom_palette = CustomPalette { palette, extension };
        let style = StyleType::Custom(custom_palette);

        assert_tokens(
            &style,
            &[
                Token::Struct {
                    name: "StyleType",
                    len: 2,
                },
                Token::Str("style"),
                Token::UnitVariant {
                    name: "StyleType",
                    variant: "Custom",
                },
                Token::Str("attributes"),
                Token::Map { len: None },
                Token::Str("primary"),
                Token::Str("#222222"),
                Token::Str("secondary"),
                Token::Str("#a6d189"),
                Token::Str("outgoing"),
                Token::Str("#f4b8e4"),
                Token::Str("starred"),
                Token::Str("#e5c890aa"),
                Token::Str("text_headers"),
                Token::Str("#232634"),
                Token::Str("text_body"),
                Token::Str("#c6d0f5"),
                Token::Str("is_nightly"),
                Token::Bool(true),
                Token::Str("alpha_chart_badge"),
                Token::F32(0.3),
                Token::Str("alpha_round_borders"),
                Token::F32(0.3),
                Token::Str("alpha_round_containers"),
                Token::F32(0.12),
                Token::Str("buttons_color"),
                Token::Str("#484848"),
                Token::Str("red_alert_color"),
                Token::Str("#ff6666"),
                Token::MapEnd,
                Token::StructEnd,
            ],
        );
    }
}
