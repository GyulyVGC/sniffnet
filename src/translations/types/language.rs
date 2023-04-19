use serde::{Deserialize, Serialize};

/// This enum defines the available languages.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub enum Language {
    /// Default language
    EN, // English
    /// Additional languages (ISO Code Sorted)
    DE, // German
    ES, // Spanish
    FA, // Persian
    FR, // French
    IT, // Italian
    KO, // Korean
    PL, // Polish
    PT, // Portuguese
    RO, // Romanian
    RU, // Russian
    TR, // Turkish
    UK, // Ukrainian
    ZH, // Simplified Chinese
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub(crate) const COL1: [Language; 5] = [
        Language::EN,
        Language::DE,
        Language::ES,
        Language::FA,
        Language::FR,
    ];
    pub(crate) const COL2: [Language; 5] = [
        Language::IT,
        Language::KO,
        Language::PL,
        Language::PT,
        Language::RO,
    ];
    pub(crate) const COL3: [Language; 4] = [
        Language::RU,
        Language::TR,
        Language::UK,
        Language::ZH,
    ];

    pub fn get_radio_label(&self) -> &str {
        match self {
            Language::EN => "English",
            Language::DE => "Deutsch",
            Language::ES => "Español",
            Language::FA => "فارسی",
            Language::FR => "Français",
            Language::IT => "Italiano",
            Language::KO => "한국인",
            Language::PL => "Polski",
            Language::PT => "Português",
            Language::RO => "Română",
            Language::RU => "Русский",
            Language::TR => "Türkçe",
            Language::UK => "Українська",
            Language::ZH => "简体中文",
        }
    }
}
