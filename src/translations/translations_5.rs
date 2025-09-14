#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::IT => "Filtra il traffico",
        Language::RO => "Filtrează traficul",
        _ => "Filter traffic",
    }
    .to_string()
}

// the source from which Sniffnet reads network traffic (i.e., a capture file or a network adapter)
pub fn traffic_source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Traffic source",
        Language::IT => "Fonte del traffico",
        Language::RO => "Sursa traficului",
        _ => "Traffic source",
    }
}
