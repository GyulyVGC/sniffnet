//! Module defining the `Config` struct, which allows to save and reload
//! the application default configuration.

use crate::StyleType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub style: StyleType,
}
