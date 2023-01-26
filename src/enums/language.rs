use serde::{Deserialize, Serialize};
use std::fmt;

/// This enum defines the available languages.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Language {
    /// English (default language).
    EN,
    /// Italian.
    IT,
}

impl ::std::default::Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::EN => {
                write!(f, "English")
            }
            Language::IT => {
                write!(f, "Italiano")
            }
        }
    }
}

impl Language {
    pub(crate) const ALL: [Language; 2] = [Language::EN, Language::IT];
}
