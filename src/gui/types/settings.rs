use serde::{Deserialize, Serialize};

use crate::gui::styles::types::gradient_type::GradientType;
use crate::notifications::types::notifications::Notifications;
use crate::{Language, StyleType};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Settings {
    pub color_gradient: GradientType,
    pub language: Language,
    pub scale_factor: f64,
    pub mmdb_country: String,
    pub mmdb_asn: String,
    pub style_path: String,
    pub notifications: Notifications,
    // StyleType should be last in order to deserialize as a table properly
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
