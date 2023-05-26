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
    /// Swedish
    SV,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub(crate) const ROW1: [Language; 4] = [Language::EN, Language::DE, Language::EL, Language::ES];
    pub(crate) const ROW2: [Language; 4] = [Language::FA, Language::FR, Language::IT, Language::KO];
    pub(crate) const ROW3: [Language; 4] = [Language::PL, Language::PT, Language::RO, Language::RU];
    pub(crate) const ROW4: [Language; 4] = [Language::SV, Language::TR, Language::UK, Language::ZH];

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
            Language::SV => "Svenska",
        }
    }
}
