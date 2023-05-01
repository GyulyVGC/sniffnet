#![allow(clippy::match_same_arms)]

use crate::Language;
use iced::widget::Text;

pub fn new_version_available_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "A newer version is available on GitHub",
        Language::IT => "Una versione più recente è disponibile su GitHub",
        Language::RU => "Новая версия доступна на GitHub",
        Language::EL => "Μια νεότερη έκδοση είναι διαθέσιμη στο GitHub",
        Language::FA => "یک نسخه جدیدتر روی GitHub موجود است",
        _ => "A newer version is available on GitHub",
    }
}

pub fn inspect_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Inspect",
        Language::IT => "Ispeziona",
        Language::FR => "Inspecter",
        Language::ES => "Inspeccionar",
        Language::PL => "Sprawdź",
        Language::DE => "Überprüfen",
        Language::RU => "Инспектировать",
        _ => "Inspect",
    }
}

pub fn connection_details_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Connection details",
        Language::IT => "Dettagli della connessione",
        _ => "Connection details",
    }
}

pub fn dropped_packets_translation(language: Language, dropped: &str) -> String {
    match language {
        Language::EN => format!("Dropped packets:\n   {dropped}"),
        Language::IT => format!("Pacchetti mancati:\n   {dropped}"),
        _ => format!("Dropped packets:\n   {dropped}"),
    }
}

pub fn of_total_translation(language: Language, percentage: String) -> String {
    match language {
        Language::EN => format!(" ({percentage} of the total)"),
        Language::IT => format!(" ({percentage} del totale)"),
        _ => format!(" ({percentage} of the total)"),
    }
}

pub fn data_representation_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Data representation:"),
        Language::IT => format!("Rappresentazione dei dati:"),
        _ => format!("Data representation:"),
    })
}

pub fn host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Network host",
        Language::IT => "Host di rete",
        _ => "Network host",
    }
}

pub fn only_top_30_hosts_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the top 30 hosts are displayed here",
        Language::IT => "Solo i maggiori 30 host sono mostrati qui",
        _ => "Only the top 30 hosts are displayed here",
    }
}

pub fn local_translation(language: Language) -> String {
    match language {
        Language::EN => "Local",
        Language::IT => "Locale",
        _ => "Local",
    }
    .to_string()
}

pub fn unknown_translation(language: Language) -> String {
    match language {
        Language::EN => "Unknown",
        Language::IT => "Sconosciuto",
        _ => "Unknown",
    }
    .to_string()
}
