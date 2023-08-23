//! Module defining the `ConfigAdvancedSettings` struct, which allows to save and reload
//! the application advanced settings.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct ConfigAdvancedSettings {
    pub scale_factor: f64,
}

impl Default for ConfigAdvancedSettings {
    fn default() -> Self {
        ConfigAdvancedSettings { scale_factor: 1.0 }
    }
}
