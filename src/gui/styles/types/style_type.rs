use iced::{application, Color, Font};
use iced::application::Appearance;
use plotters::prelude::FontStyle;
use serde::{Deserialize, Serialize};

use crate::get_colors;
use crate::gui::styles::style_constants::{DAY_STYLE, DEEP_SEA_STYLE, MON_AMOUR_STYLE, NIGHT_STYLE, SARASA_MONO, SARASA_MONO_BOLD};
use crate::gui::styles::types::custom_palette::ExtraStyles;
use crate::gui::styles::types::palette::Palette;

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
        let colors = get_colors(*self);
        Appearance {
            background_color: colors.primary,
            text_color: colors.text_body,
        }
    }
}

impl StyleType {
    pub fn get_colors(self) -> Palette {
        match self {
            StyleType::Night => NIGHT_STYLE,
            StyleType::Day => DAY_STYLE,
            StyleType::DeepSea => DEEP_SEA_STYLE,
            StyleType::MonAmour => MON_AMOUR_STYLE,
            StyleType::Custom(style) => style.to_palette(),
        }
    }

    pub fn is_nightly(self) -> bool {
        let primary = get_colors(self).primary;
        primary.r + primary.g + primary.b <= 1.5
    }

    pub fn is_text_body_dark(self) -> bool {
        let text_body = get_colors(self).text_body;
        text_body.r + text_body.g + text_body.b <= 1.5
    }

    pub fn is_text_headers_dark(self) -> bool {
        let text_headers = get_colors(self).text_headers;
        text_headers.r + text_headers.g + text_headers.b <= 1.5
    }

    pub fn get_font(self) -> Font {
        if self.is_text_body_dark() {
            SARASA_MONO_BOLD
        } else {
            SARASA_MONO
        }
    }

    pub fn get_font_weight(self) -> FontStyle {
        if self.is_text_body_dark() {
            FontStyle::Bold
        } else {
            FontStyle::Normal
        }
    }

    pub fn get_font_headers(self) -> Font {
        if self.is_text_headers_dark() {
            SARASA_MONO_BOLD
        } else {
            SARASA_MONO
        }
    }

    pub fn get_alpha_chart_badge(self) -> f32 {
        if self.is_nightly() {
            0.15
        } else {
            0.75
        }
    }

    pub fn get_alpha_round_borders(self) -> f32 {
        match self {
            StyleType::Night | StyleType::DeepSea => 0.35,
            StyleType::Day => 0.45,
            StyleType::MonAmour => 0.5,
            StyleType::Custom(_) => {
                if self.is_nightly() {
                    0.3
                } else {
                    0.6
                }
            },
        }
    }

    pub fn get_alpha_round_containers(self) -> f32 {
        match self {
            StyleType::Night | StyleType::MonAmour => 0.25,
            StyleType::Day => 0.2,
            StyleType::DeepSea => 0.15,
            StyleType::Custom(_) => {
                if self.is_nightly() {
                    0.12
                } else {
                    0.24
                }
            },
        }
    }

    pub fn get_buttons_color(self) -> Color {
        match self {
            StyleType::Night => Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
            StyleType::Day => Color {
                r: 0.8,
                g: 0.8,
                b: 0.8,
                a: 1.0,
            },
            StyleType::DeepSea => Color {
                r: 48.0 / 255.0,
                g: 71.0 / 255.0,
                b: 94.0 / 255.0,
                a: 1.0,
            },
            StyleType::MonAmour => Color {
                r: 242.0 / 255.0,
                g: 190.0 / 255.0,
                b: 209.0 / 255.0,
                a: 1.0,
            },
            StyleType::Custom(_) => {
                let primary = get_colors(self).primary;
                if self.is_nightly() {
                    Color {
                        r: f32::min(primary.r + 0.15, 1.0),
                        g: f32::min(primary.g + 0.15, 1.0),
                        b: f32::min(primary.b + 0.15, 1.0),
                        a: 1.0
                    }
                } else {
                    Color {
                        r: f32::max(primary.r - 0.15, 0.0),
                        g: f32::max(primary.g - 0.15, 0.0),
                        b: f32::max(primary.b - 0.15, 0.0),
                        a: 1.0
                    }
                }
            }
        }
    }
}
