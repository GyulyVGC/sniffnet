use serde::{Deserialize, Serialize};

use super::custom_style::{deserialize_from_path, serialize_to_path, CustomStyle};

/// Used to specify the kind of style of the application
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq)]
#[serde(tag = "style", content = "path")]
pub enum StyleType {
    Night,
    Day,
    DeepSea,
    MonAmour,
    #[serde(
        serialize_with = "serialize_to_path",
        deserialize_with = "deserialize_from_path"
    )]
    Custom(CustomStyle),
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}
