//! Module defining the `ConfigSettings` struct, which allows to save and reload
//! the application default configuration.

use serde::{Deserialize, Serialize};

use crate::gui::styles::types::gradient_type::GradientType;
use crate::notifications::types::notifications::Notifications;
#[cfg(not(test))]
use crate::SNIFFNET_LOWERCASE;
use crate::{Language, StyleType};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
    const FILE_NAME: &'static str = "settings";

    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(settings) = confy::load::<ConfigSettings>(SNIFFNET_LOWERCASE, Self::FILE_NAME) {
            settings
        } else {
            confy::store(
                SNIFFNET_LOWERCASE,
                Self::FILE_NAME,
                ConfigSettings::default(),
            )
            .unwrap_or(());
            ConfigSettings::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).unwrap_or(());
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

#[cfg(test)]
mod tests {
    use crate::ConfigSettings;

    impl ConfigSettings {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<ConfigSettings>(ConfigSettings::test_path())
                .unwrap_or_else(|_| ConfigSettings::default())
        }

        pub fn store(self) {
            confy::store_path(ConfigSettings::test_path(), self).unwrap_or(());
        }
    }
}
