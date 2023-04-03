#![allow(clippy::match_same_arms)]
use crate::Language;

pub fn new_version_available_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "A newer version is available on GitHub",
        Language::IT => "Una versione più recente è disponibile su GitHub",
        _ => "A newer version is available on GitHub",
    }
}
