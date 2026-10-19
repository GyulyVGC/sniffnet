use crate::gui::types::conf::deserialize_or_default;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct ConfigUpdates {
    #[serde(deserialize_with = "deserialize_or_default")]
    notify_updates: bool,
    #[serde(deserialize_with = "deserialize_or_default")]
    disable_checks: bool,
}

impl ConfigUpdates {
    pub fn notify_updates(self) -> bool {
        self.notify_updates
    }

    pub fn toggle_notify_updates(&mut self) {
        self.notify_updates = !self.notify_updates;
    }

    pub fn disable_checks(self) -> bool {
        self.disable_checks
    }

    pub fn toggle_disable_checks(&mut self) {
        self.disable_checks = !self.disable_checks;
    }
}

impl Default for ConfigUpdates {
    fn default() -> Self {
        Self {
            notify_updates: true,
            disable_checks: false,
        }
    }
}
