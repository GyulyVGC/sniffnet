#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn reserved_address_translation(language: Language, info: &str) -> String {
    match language {
        Language::EN => format!("Reserved address ({info})"),
        Language::IT => format!("Indirizzo riservato ({info})"),
        _ => format!("Reserved address ({info})"),
    }
}
