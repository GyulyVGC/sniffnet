//! Module defining the `ConfigAdvancedSettings` struct, which allows to save and reload
//! the application advanced settings.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::utils::formatted_strings::get_default_report_directory;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigAdvancedSettings {
    pub scale_factor: f64,
    pub mmdb_country: String,
    pub mmdb_asn: String,
    pub style_path: String,
    pub output_path: PathBuf,
}

impl ConfigAdvancedSettings {
    pub fn load() -> Self {
        if let Ok(advanced_settings) =
            confy::load::<ConfigAdvancedSettings>("sniffnet", "advanced_settings")
        {
            advanced_settings
        } else {
            confy::store(
                "sniffnet",
                "advanced_settings",
                ConfigAdvancedSettings::default(),
            )
            .unwrap_or(());
            ConfigAdvancedSettings::default()
        }
    }

    pub fn store(self) {
        confy::store("sniffnet", "advanced_settings", self).unwrap_or(());
    }
}

impl Default for ConfigAdvancedSettings {
    fn default() -> Self {
        ConfigAdvancedSettings {
            scale_factor: 1.0,
            mmdb_country: String::new(),
            mmdb_asn: String::new(),
            style_path: String::new(),
            output_path: get_default_report_directory(),
        }
    }
}
