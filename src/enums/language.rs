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
    /// Spanish.
    ES,
    /// Polish.
    PL,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub(crate) const COL1: [Language; 3] = [Language::EN, Language::FR, Language::ES];
    pub(crate) const COL2: [Language; 2] = [Language::IT, Language::PL];
    pub fn get_radio_label(&self) -> &str {
        match self {
            Language::EN => "English",
            Language::IT => "Italiano",
            Language::FR => "Français",
            Language::ES => "Español",
            Language::PL => "Polski",
        }
    }
}
