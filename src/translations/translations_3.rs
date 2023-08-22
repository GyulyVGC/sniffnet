#![allow(clippy::match_same_arms)]

use crate::Language;

pub fn advanced_settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Advanced settings",
        Language::IT => "Impostazioni avanzate",
        _ => "Advanced settings",
    }
}

pub fn scale_factor_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Scale factor",
        Language::IT => "Fattore di scala",
        _ => "Scale factor",
    }
}
