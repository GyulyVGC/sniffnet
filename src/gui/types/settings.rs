use serde::{Deserialize, Serialize};

use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::conf::deserialize_or_default;
use crate::notifications::types::notifications::Notifications;
use crate::{Language, StyleType};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct Settings {
    #[serde(deserialize_with = "deserialize_or_default")]
    pub color_gradient: GradientType,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub style_path: String,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub language: Language,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub scale_factor: f32,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub mmdb_country: String,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub mmdb_asn: String,
    // ---------------------------------------------------------------------------------------------
    #[serde(deserialize_with = "deserialize_or_default")]
    pub notifications: Notifications,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub style: StyleType,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            color_gradient: GradientType::default(),
            language: Language::default(),
            scale_factor: 1.0,
            mmdb_country: String::new(),
            mmdb_asn: String::new(),
            style_path: String::new(),
            notifications: Notifications::default(),
            style: StyleType::default(),
        }
    }
}
