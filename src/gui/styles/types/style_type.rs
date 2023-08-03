use serde::{Deserialize, Serialize};

use super::custom_styles::ExtraStyles;

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
