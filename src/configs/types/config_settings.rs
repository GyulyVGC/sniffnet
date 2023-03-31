//! Module defining the `ConfigSettings` struct, which allows to save and reload
//! the application default configuration.

use crate::notifications::types::notifications::Notifications;
use crate::{Language, StyleType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigSettings {
    pub style: StyleType,
    pub language: Language,
    pub notifications: Notifications,
}
