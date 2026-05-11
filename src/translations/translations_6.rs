#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn ipfix_collector_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX collector",
        Language::IT => "Collettore IPFIX",
        _ => "IPFIX collector",
    }
}

pub fn bind_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bind address",
        Language::IT => "Indirizzo di bind",
        _ => "Bind address",
    }
}
