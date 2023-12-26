//! Module defining the `ConfigSettings` struct, which allows to save and reload
//! the application default configuration.

use serde::{Deserialize, Serialize};

use crate::gui::styles::types::gradient_type::GradientType;
use crate::notifications::types::notifications::Notifications;
use crate::{Language, StyleType};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigSettings {
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

impl ConfigSettings {
    pub fn load() -> Self {
        if let Ok(settings) = confy::load::<ConfigSettings>("sniffnet", "settings") {
            settings
        } else {
            println!(
                "{:?}",
                confy::load::<ConfigSettings>("sniffnet", "settings")
                    .err()
                    .unwrap()
            );
            confy::store("sniffnet", "settings", ConfigSettings::default()).unwrap_or(());
            ConfigSettings::default()
        }
    }

    pub fn store(self) {
        confy::store("sniffnet", "settings", self).unwrap_or(());
    }
}

impl Default for ConfigSettings {
    fn default() -> Self {
        ConfigSettings {
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
