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

    fn appearance(&self, _: &Self::Style) -> Appearance {
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
            StyleType::Custom(style) => style.to_palette(),
        }
    }

    pub fn get_palette_extension(self) -> PaletteExtension {
        match self {
            StyleType::Night => NIGHT_PALETTE_EXTENSION,
            StyleType::Day => DAY_PALETTE_EXTENSION,
            StyleType::DeepSea => DEEP_SEA_PALETTE_EXTENSION,
            StyleType::MonAmour => MON_AMOUR_PALETTE_EXTENSION,
            StyleType::Custom(style) => style.to_palette_extension(),
        }
    }

    pub fn get_font_weight(self) -> FontStyle {
        if self.get_palette_extension().font.eq(&SARASA_MONO_BOLD) {
            FontStyle::Bold
        } else {
            FontStyle::Normal
        }
    }
}
