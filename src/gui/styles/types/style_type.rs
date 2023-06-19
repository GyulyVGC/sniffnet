use serde::{Deserialize, Serialize};

use super::custom_style::CustomStyle;

/// Used to specify the kind of style of the application
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Hash, PartialEq)]
pub enum StyleType {
    Night,
    Day,
    DeepSea,
    MonAmour,
    // Custom(CustomStyle)
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}
