//! Module defining the `ConfigSettings` struct, which allows to save and reload
//! the application default configuration.

use serde::{Deserialize, Serialize};

use crate::gui::styles::types::gradient_type::GradientType;
use crate::notifications::types::notifications::Notifications;
use crate::{Language, StyleType};

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigSettings {
    pub color_gradient: GradientType,
    pub language: Language,
    pub notifications: Notifications,
    // StyleType should be last in order to deserialize as a table properly
    pub style: StyleType,
}
