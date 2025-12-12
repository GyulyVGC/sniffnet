#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::CS => "Filtr provozu",
        Language::IT => "Filtra il traffico",
        Language::JA => "トラフィックをフィルタリング",
        _ => "Filter traffic",
    }
    .to_string()
}

// the source from which Sniffnet reads network traffic (i.e., a capture file or a network adapter)
pub fn traffic_source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Traffic source",
        Language::CS => "Zdroj provozu",
        Language::IT => "Fonte del traffico",
        Language::JA => "トラフィック元",
        _ => "Traffic source",
    }
}

pub fn remote_notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Remote notifications",
        Language::IT => "Notifiche remote",
        Language::JA => "リモート通知",
        _ => "Remote notifications",
    }
}
