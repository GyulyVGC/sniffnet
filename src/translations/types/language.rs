use serde::{Deserialize, Serialize};

/// This enum defines the available languages.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
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
    /// German,
    DE,
    /// Ukrainian
    UK,
    /// Simplified Chinese
    ZH,
    /// Romanian
    RO,
    /// Korean
    KO,
    /// Portuguese
    PT,
    /// Turkish
    TR,
    /// Russian
    RU,
    /// Greek
    EL,
    /// Persian
    FA,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub(crate) const COL1: [Language; 5] = [
        Language::EN,
        Language::ES,
        Language::IT,
        Language::PT,
        Language::TR,
    ];
    pub(crate) const COL2: [Language; 5] = [
        Language::DE,
        Language::FA,
        Language::KO,
        Language::RO,
        Language::UK,
    ];
    pub(crate) const COL3: [Language; 5] = [
        Language::EL,
        Language::FR,
        Language::PL,
        Language::RU,
        Language::ZH,
    ];

    pub fn get_radio_label(&self) -> &str {
        match self {
            Language::EN => "English",
            Language::IT => "Italiano",
            Language::FR => "Français",
            Language::ES => "Español",
            Language::PL => "Polski",
            Language::DE => "Deutsch",
            Language::UK => "Українська",
            Language::ZH => "简体中文",
            Language::RO => "Română",
            Language::KO => "한국인",
            Language::TR => "Türkçe",
            Language::RU => "Русский",
            Language::PT => "Português",
            Language::EL => "Ελληνικά",
            Language::FA => "فارسی",
        }
    }
}
