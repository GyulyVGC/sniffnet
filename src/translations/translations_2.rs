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

pub fn dropped_packets_translation(
    language: Language,
    dropped: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Dropped packets:\n   {dropped} ({percentage} of the total)"),
        Language::IT => format!("Pacchetti mancati:\n   {dropped} ({percentage} del totale)"),
        _ => format!("Dropped packets:\n   {dropped} ({percentage} of the total)"),
    })
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
