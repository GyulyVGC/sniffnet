use iced::application;
use iced::application::Appearance;
use plotters::prelude::FontStyle;
use serde::{Deserialize, Serialize};

use crate::gui::styles::style_constants::{
    DAY_PALETTE, DAY_PALETTE_EXTENSION, DEEP_SEA_PALETTE, DEEP_SEA_PALETTE_EXTENSION,
    MON_AMOUR_PALETTE, MON_AMOUR_PALETTE_EXTENSION, NIGHT_PALETTE, NIGHT_PALETTE_EXTENSION,
    SARASA_MONO_BOLD,
};
use crate::gui::styles::types::custom_palette::ExtraStyles;
use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

/// Used to specify the kind of style of the application
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
#[serde(tag = "style", content = "name")]
pub enum StyleType {
    Night,
    Day,
    DeepSea,
    MonAmour,
    Custom(ExtraStyles),
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}

impl application::StyleSheet for StyleType {
    type Style = ();

    fn appearance(&self, (): &Self::Style) -> Appearance {
        let colors = self.get_palette();
        Appearance {
            background_color: colors.primary,
            text_color: colors.text_body,
        }
    }
}

impl StyleType {
    pub fn get_palette(self) -> Palette {
        match self {
            StyleType::Night => NIGHT_PALETTE,
            StyleType::Day => DAY_PALETTE,
            StyleType::DeepSea => DEEP_SEA_PALETTE,
            StyleType::MonAmour => MON_AMOUR_PALETTE,
            StyleType::Custom(style) => style.get_palette(),
        }
    }

    pub fn get_extension(self) -> PaletteExtension {
        match self {
            StyleType::Night => NIGHT_PALETTE_EXTENSION,
            StyleType::Day => DAY_PALETTE_EXTENSION,
            StyleType::DeepSea => DEEP_SEA_PALETTE_EXTENSION,
            StyleType::MonAmour => MON_AMOUR_PALETTE_EXTENSION,
            StyleType::Custom(style) => style.get_extension(),
        }
    }

    pub fn get_font_weight(self) -> FontStyle {
        if self.get_extension().font.eq(&SARASA_MONO_BOLD) {
            FontStyle::Bold
        } else {
            FontStyle::Normal
        }
    }
}

#[cfg(test)]
mod tests {
    use iced::{color, Color};
    use serde_test::{assert_tokens, Token};

    use crate::gui::styles::types::custom_palette::{CustomPalette, ExtraStyles};
    use crate::gui::styles::types::palette::Palette;
    use crate::StyleType;

    // test if deserializing and serializing a StyleType works n.1
    // simple case: one of the default themes
    #[test]
    fn test_working_style_type_round_trip_1() {
        let style = StyleType::Night;
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
                    variant: "Night",
                },
                Token::StructEnd,
            ],
        );
    }

    // test if deserializing and serializing a StyleType works n.2
    // medium case: one predefined additional themes
    #[test]
    fn test_working_style_type_round_trip_2() {
        let style = StyleType::Custom(ExtraStyles::DraculaLight);
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
                Token::Str("name"),
                Token::Struct {
                    name: "ExtraStyles",
                    len: 1,
                },
                Token::Str("custom"),
                Token::UnitVariant {
                    name: "ExtraStyles",
                    variant: "DraculaLight",
                },
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }

    // test if deserializing and serializing a StyleType works n.3
    // complex case: a custom theme from a TOML file
    #[test]
    fn test_working_style_type_round_trip_3() {
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
        let style = StyleType::Custom(ExtraStyles::CustomToml(custom_palette));

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
                Token::Str("name"),
                Token::Struct {
                    name: "ExtraStyles",
                    len: 2,
                },
                Token::Str("custom"),
                Token::UnitVariant {
                    name: "ExtraStyles",
                    variant: "CustomToml",
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
                Token::Str("font"),
                Token::Str("SARASA_MONO"),
                Token::Str("font_headers"),
                Token::Str("SARASA_MONO_BOLD"),
                Token::Str("alpha_chart_badge"),
                Token::F32(0.15),
                Token::Str("alpha_round_borders"),
                Token::F32(0.3),
                Token::Str("alpha_round_containers"),
                Token::F32(0.12),
                Token::Str("buttons_color"),
                Token::Str("#484848"),
                Token::MapEnd,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
