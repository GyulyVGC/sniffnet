use serde::{Deserialize, Serialize};

use crate::gui::styles::types::gradient_type::GradientType;
use crate::notifications::types::notifications::Notifications;
#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::{Language, StyleType};
#[cfg(not(test))]
use crate::{SNIFFNET_LOWERCASE, location};

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

impl Settings {
    const FILE_NAME: &'static str = "settings";

    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(settings) = confy::load::<Settings>(SNIFFNET_LOWERCASE, Self::FILE_NAME) {
            settings
        } else {
            let _ = confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, Settings::default())
                .log_err(location!());
            Settings::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) -> Result<(), confy::ConfyError> {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).log_err(location!())
    }
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

#[cfg(test)]
mod tests {
    use crate::Settings;

    impl Settings {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<Settings>(Settings::test_path())
                .unwrap_or_else(|_| Settings::default())
        }

        pub fn store(self) -> Result<(), confy::ConfyError> {
            confy::store_path(Settings::test_path(), self)
        }
    }
}
