#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn reserved_address_translation(language: Language, info: &str) -> String {
    match language {
        Language::EN => format!("Reserved address ({info})"),
        Language::IT => format!("Indirizzo riservato ({info})"),
        Language::PT => format!("Endereço reservado ({info})"),
        Language::UK => format!("Зарезервована адреса ({info})"),
        Language::ZH_TW => format!("保留的網路位址 ({info})"),
        _ => format!("Reserved address ({info})"),
    }
}

pub fn share_feedback_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Share your feedback",
        Language::IT => "Condividi il tuo feedback",
        Language::ZH_TW => "分享您的意見回饋",
        _ => "Share your feedback",
    }
}

// refers to bytes or packets excluded because of the filters
pub fn excluded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Excluded",
        Language::IT => "Esclusi",
        Language::ZH_TW => "已排除",
        _ => "Excluded",
    }
}
