use serde::{Deserialize, Serialize};

/// Used to specify the kind of style of the application
#[derive(Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum StyleType {
    Night,
    Day,
    Try,
    Almond,
    Red,
}

impl Clone for StyleType {
    fn clone(&self) -> Self {
        *self
    }
}

impl ::std::default::Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}
