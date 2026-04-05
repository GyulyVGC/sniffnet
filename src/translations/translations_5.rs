#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::CS => "Filtr provozu",
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
        Language::CS => "Zdroj provozu",
        Language::IT => "Fonte del traffico",
        Language::RO => "Sursa traficului",
        _ => "Traffic source",
    }
}

pub fn remote_notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Remote notifications",
        Language::IT => "Notifiche remote",
        Language::RO => "Notificări la distanță",
        _ => "Remote notifications",
    }
}

pub fn ip_blacklist_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IP blacklist",
        Language::IT => "Blacklist IP",
        Language::RO => "Blacklist IP-uri",
        _ => "IP blacklist",
    }
}

pub fn blacklisted_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from a blacklisted IP",
        Language::IT => "Nuovi dati scambiati da un IP in blacklist",
        Language::RO => "Noi date schimbate de la un IP în blacklist",
        _ => "New data exchanged from a blacklisted IP",
    }
}

pub fn only_show_blacklisted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show blacklisted",
        Language::IT => "Mostra solo in blacklist",
        Language::RO => "Afișează doar blacklist",
        _ => "Only show blacklisted",
    }
}

pub fn program_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Program",
        Language::IT => "Programma",
        Language::RO => "Program",
        _ => "Program",
    }
}

pub fn no_favorites_saved_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No favorites saved yet",
        Language::IT => "Nessun preferito salvato",
        Language::RO => "Niciun favorit salvat încă",
        _ => "No favorites saved yet",
    }
}
