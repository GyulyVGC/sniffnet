#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::IT => "Filtra il traffico",
        _ => "Filter traffic",
    }
    .to_string()
}
