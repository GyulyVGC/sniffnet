//! Module defining the `Colors` struct, which defines the colors in use in the GUI.

use std::hash::{Hash, Hasher};
use std::path::Path;

use iced::Color;
use plotters::style::RGBColor;
use serde::{Deserialize, Serialize};

use super::color_remote::{deserialize_color, deserialize_color_inner, serialize_color};
use crate::gui::styles::style_constants::{RED_ALERT_COLOR_DAILY, RED_ALERT_COLOR_NIGHTLY};
use crate::gui::styles::types::color_remote::color_hash;
use crate::gui::styles::types::palette_extension::PaletteExtension;
use crate::gui::styles::types::style_type::StyleType;

/// Set of colors to apply to GUI
///
/// Best practices:
/// - `primary` should be a kind of neutral color
/// - `secondary` and `outgoing` should be complementary colors if possible
/// - `text_headers` should be black or white and must have a strong contrast with `secondary`
/// - `text_body` should be black or white and must have a strong contrast with `primary`
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Palette {
    /// Main color of the GUI (background, hovered buttons, active tab)
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub primary: Color,
    /// Secondary color of the GUI (incoming connections, header, footer, buttons' borders, radio selection)
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub secondary: Color,
    /// Color of outgoing connections
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub outgoing: Color,
    /// Color of favorites' star symbol
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub starred: Color,
    /// Color of header and footer text
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_headers: Color,
    /// Color of body and buttons text
    #[serde(
        deserialize_with = "deserialize_color",
        serialize_with = "serialize_color"
    )]
    pub text_body: Color,
}

impl Palette {
    pub fn generate_buttons_color(self) -> Color {
        let primary = self.primary;
        let is_nightly = primary.r + primary.g + primary.b <= 1.5;
        if is_nightly {
            Color {
                r: f32::min(primary.r + 0.15, 1.0),
                g: f32::min(primary.g + 0.15, 1.0),
                b: f32::min(primary.b + 0.15, 1.0),
                a: 1.0,
            }
        } else {
            Color {
                r: f32::max(primary.r - 0.15, 0.0),
                g: f32::max(primary.g - 0.15, 0.0),
                b: f32::max(primary.b - 0.15, 0.0),
                a: 1.0,
            }
        }
    }

    pub fn generate_palette_extension(self) -> PaletteExtension {
        let primary = self.primary;
        let is_nightly = primary.r + primary.g + primary.b <= 1.5;
        let alpha_chart_badge = if is_nightly { 0.3 } else { 0.5 };
        let alpha_round_borders = if is_nightly { 0.3 } else { 0.6 };
        let alpha_round_containers = if is_nightly { 0.12 } else { 0.24 };
        let buttons_color = self.generate_buttons_color();
        let red_alert_color = if is_nightly {
            RED_ALERT_COLOR_NIGHTLY
        } else {
            RED_ALERT_COLOR_DAILY
        };

        PaletteExtension {
            is_nightly,
            alpha_chart_badge,
            alpha_round_borders,
            alpha_round_containers,
            buttons_color,
            red_alert_color,
        }
    }

    /// Deserialize [`Palette`] from `path`.
    ///
    /// # Arguments
    /// * `path` - Path to a UTF-8 encoded file containing a custom style as TOML.
    pub fn from_file<P>(path: P) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        let toml_str = std::fs::read_to_string(path).ok()?;

        // manually deserialize it,
        // because using the derived impl accepts as valid everything (for config purposes)
        let toml: toml::Value = toml::from_str(&toml_str).ok()?;
        let primary = toml.get("primary").cloned()?;
        let secondary = toml.get("secondary").cloned()?;
        let outgoing = toml.get("outgoing").cloned()?;
        let starred = toml.get("starred").cloned()?;
        let text_headers = toml.get("text_headers").cloned()?;
        let text_body = toml.get("text_body").cloned()?;

        Some(Self {
            primary: deserialize_color_inner(primary)?,
            secondary: deserialize_color_inner(secondary)?,
            outgoing: deserialize_color_inner(outgoing)?,
            starred: deserialize_color_inner(starred)?,
            text_headers: deserialize_color_inner(text_headers)?,
            text_body: deserialize_color_inner(text_body)?,
        })
    }
}

impl Default for Palette {
    fn default() -> Self {
        <StyleType as std::default::Default>::default().get_palette()
    }
}

impl Hash for Palette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Palette {
            primary,
            secondary,
            outgoing,
            starred,
            text_headers,
            text_body,
        } = self;

        color_hash(*primary, state);
        color_hash(*secondary, state);
        color_hash(*outgoing, state);
        color_hash(*starred, state);
        color_hash(*text_headers, state);
        color_hash(*text_body, state);
    }
}

pub fn to_rgb_color(color: Color) -> RGBColor {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if color.r <= 1.0
        && color.r >= 0.0
        && color.g <= 1.0
        && color.g >= 0.0
        && color.b <= 1.0
        && color.b >= 0.0
    {
        RGBColor(
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        )
    } else {
        RGBColor(0, 0, 0) // Black
    }
}

/// Returns the average of two colors; color intensity is fixed to 100%
pub fn mix_colors(color_1: Color, color_2: Color) -> Color {
    Color {
        r: f32::midpoint(color_1.r, color_2.r),
        g: f32::midpoint(color_1.g, color_2.g),
        b: f32::midpoint(color_1.b, color_2.b),
        a: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use iced::Color;
    use iced::color;

    use crate::gui::styles::style_constants::{RED_ALERT_COLOR_DAILY, RED_ALERT_COLOR_NIGHTLY};
    use crate::gui::styles::types::palette_extension::PaletteExtension;

    use super::Palette;

    fn style_path(name: &str) -> String {
        format!(
            "{}/resources/themes/{}.toml",
            env!("CARGO_MANIFEST_DIR"),
            name
        )
    }

    // NOTE: This has to be updated if `resources/themes/catppuccin.toml` changes
    fn catppuccin_style() -> Palette {
        Palette {
            primary: color!(0x30, 0x34, 0x46),
            secondary: color!(0xa6, 0xd1, 0x89),
            outgoing: color!(0xf4, 0xb8, 0xe4),
            starred: color!(0xe5, 0xc8, 0x90, 0.6666667),
            text_headers: color!(0x23, 0x26, 0x34),
            text_body: color!(0xc6, 0xd0, 0xf5),
        }
    }

    #[test]
    fn custompalette_from_file_de() {
        let style = catppuccin_style();
        let style_de = Palette::from_file(style_path("catppuccin")).unwrap();
        assert_eq!(style, style_de);
    }

    #[test]
    fn test_generate_palette_extension_dark() {
        let palette = Palette {
            primary: Color {
                r: 1.0,
                g: 0.4,
                b: 0.05,
                a: 1.0,
            },
            secondary: Color {
                r: 0.7,
                g: 0.9,
                b: 0.5,
                a: 1.0,
            },
            outgoing: Color {
                r: 0.9,
                g: 0.5,
                b: 0.7,
                a: 1.0,
            },
            starred: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 0.5,
            },
            text_headers: Color {
                r: 0.0,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            text_body: Color {
                r: 0.5,
                g: 0.5,
                b: 0.51,
                a: 1.0,
            },
        };

        assert_eq!(
            palette.generate_palette_extension(),
            PaletteExtension {
                is_nightly: true,
                alpha_chart_badge: 0.3,
                alpha_round_borders: 0.3,
                alpha_round_containers: 0.12,
                buttons_color: Color {
                    r: 1.0,
                    g: 0.55,
                    b: 0.2,
                    a: 1.0
                },
                red_alert_color: RED_ALERT_COLOR_NIGHTLY,
            }
        )
    }

    #[test]
    fn test_generate_palette_extension_light() {
        let palette = Palette {
            primary: Color {
                r: 1.0,
                g: 0.9,
                b: 0.05,
                a: 1.0,
            },
            secondary: Color {
                r: 0.7,
                g: 0.9,
                b: 0.5,
                a: 1.0,
            },
            outgoing: Color {
                r: 0.9,
                g: 0.5,
                b: 0.7,
                a: 1.0,
            },
            starred: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 0.5,
            },
            text_headers: Color {
                r: 0.7,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            text_body: Color {
                r: 1.0,
                g: 0.9,
                b: 0.4,
                a: 1.0,
            },
        };

        assert_eq!(
            palette.generate_palette_extension(),
            PaletteExtension {
                is_nightly: false,
                alpha_chart_badge: 0.5,
                alpha_round_borders: 0.6,
                alpha_round_containers: 0.24,
                buttons_color: Color {
                    r: 0.85,
                    g: 0.75,
                    b: 0.0,
                    a: 1.0
                },
                red_alert_color: RED_ALERT_COLOR_DAILY,
            }
        )
    }

    #[test]
    fn test_palette_from_not_existing_file_fails() {
        let result = Palette::from_file("non_existent_file.toml");
        assert!(result.is_none());
    }

    #[test]
    fn test_palette_from_invalid_file_fails() {
        let result = Palette::from_file("Cargo.toml");
        assert!(result.is_none());
    }
}
