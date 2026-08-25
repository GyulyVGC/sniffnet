#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn latency_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Latency",
        Language::IT => "Latenza",
        _ => "Latency",
    }
}

pub fn vlan_id_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "VLAN ID",
        _ => "VLAN ID",
    }
}
