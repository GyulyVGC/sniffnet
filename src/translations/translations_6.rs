#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn latency_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Latency",
        Language::IT => "Latenza",
        _ => "Latency",
    }
}
