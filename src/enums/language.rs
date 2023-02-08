use serde::{Deserialize, Serialize};

/// This enum defines the available languages.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Language {
    /// English (default language).
    EN,
    /// Italian.
    IT,
    /// French.
    FR,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub(crate) const ALL: [Language; 3] = [Language::EN, Language::IT, Language::FR];

    pub fn get_radio_label(&self) -> &str {
        match self {
            Language::EN => "English",
            Language::IT => "Italiano",
            Language::FR => "Français",
        }
    }
}
