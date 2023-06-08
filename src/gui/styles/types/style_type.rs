use serde::{Deserialize, Serialize};
use super::palette::Palette;

/// Used to specify the kind of style of the application
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
pub enum StyleType {
    Night,
    Day,
    DeepSea,
    MonAmour,
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}

/// Custom sniffnet color scheme.
// #[derive(Clone, Deserialize, Debug, Hash)]
pub struct CustomStyle {
    /// Display name of the color scheme.
    /// This is the user facing color scheme name that may be displayed in the UI.
    /// Ex. Catppuccin Mocha
    pub name: String,
    /// Internal or path name.
    /// Ex. catppuccin_mocha.toml
    pub path: String,
    /// Short description of the color scheme
    pub description: String,
    /// Color scheme's sniffnet palette.
    /// Should be an implementation of the scheme that is tuned to sniffnet.
    pub palette: Palette
}
