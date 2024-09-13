use std::fmt;

use iced::widget::svg::Handle;
use iced::widget::Svg;
use serde::{Deserialize, Serialize};

use crate::countries::flags_pictures::{
    CN, DE, ES, FI, FLAGS_WIDTH_BIG, FR, GB, GR, IT, JP, KR, PL, PT, RO, RU, SE, TR, UA, UZ, VN,
};
use crate::StyleType;

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
    // /// Persian
    // FA,
    /// Swedish
    SV,
    /// Finnish
    FI,
    /// Japanese
    JA,
    /// Uzbek
    UZ,
    /// Vietnamese
    VI,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

impl Language {
    pub const ALL: [Language; 19] = [
        Language::EN,
        Language::DE,
        Language::EL,
        Language::ES,
        Language::FI,
        Language::FR,
        Language::IT,
        Language::JA,
        Language::KO,
        Language::PL,
        Language::PT,
        Language::RO,
        Language::RU,
        Language::SV,
        Language::TR,
        Language::UK,
        Language::UZ,
        Language::VI,
        Language::ZH,
    ];

    pub fn get_flag(self) -> Svg<StyleType> {
        Svg::new(Handle::from_memory(Vec::from(match self {
            Language::ZH => CN,
            Language::DE => DE,
            Language::ES => ES,
            Language::FR => FR,
            Language::EN => GB,
            Language::IT => IT,
            Language::KO => KR,
            Language::PL => PL,
            Language::PT => PT,
            Language::RO => RO,
            Language::RU => RU,
            Language::TR => TR,
            Language::UK => UA,
            Language::EL => GR,
            // Language::FA => IR,
            Language::SV => SE,
            Language::FI => FI,
            Language::JA => JP,
            Language::UZ => UZ,
            Language::VI => VN,
        })))
        .width(FLAGS_WIDTH_BIG)
    }

    pub fn is_up_to_date(self) -> bool {
        matches!(
            self,
            Language::FR
                | Language::EN
                | Language::IT
                | Language::DE
                | Language::PL
                | Language::RU
                | Language::RO
                | Language::JA
                | Language::UZ
                | Language::SV
                | Language::VI
                | Language::ZH
                | Language::KO
                | Language::TR
        )
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lang_str = match self {
            Language::EN => "English",
            Language::IT => "Italiano",
            Language::FR => "Français",
            Language::ES => "Español",
            Language::PL => "Polski",
            Language::DE => "Deutsch",
            Language::UK => "Українська",
            Language::ZH => "简体中文",
            Language::RO => "Română",
            Language::KO => "한국어",
            Language::TR => "Türkçe",
            Language::RU => "Русский",
            Language::PT => "Português",
            Language::EL => "Ελληνικά",
            // Language::FA => "فارسی",
            Language::SV => "Svenska",
            Language::FI => "Suomi",
            Language::JA => "日本語",
            Language::UZ => "O'zbekcha",
            Language::VI => "Tiếng Việt",
        };
        write!(f, "{self:?} - {lang_str}")
    }
}
