#![allow(clippy::match_same_arms)]

use crate::gui::styles::types::style_type::StyleType;
use crate::translations::translations_3::file_name_translation;
use crate::translations::types::language::Language;
use iced::widget::Text;

pub fn reserved_address_translation(language: Language, info: &str) -> String {
    match language {
        Language::EN => format!("Reserved address ({info})"),
        Language::IT => format!("Indirizzo riservato ({info})"),
        Language::PT => format!("Endereço reservado ({info})"),
        Language::UK => format!("Зарезервована адреса ({info})"),
        Language::ZH_TW => format!("保留的網路位址 ({info})"),
        Language::DE => format!("Reservierte Adresse ({info})"),
        _ => format!("Reserved address ({info})"),
    }
}

pub fn share_feedback_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Share your feedback",
        Language::IT => "Condividi il tuo feedback",
        Language::ZH_TW => "分享您的意見回饋",
        Language::DE => "Feedback geben",
        _ => "Share your feedback",
    }
}

// refers to bytes or packets excluded because of the filters
pub fn excluded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Excluded",
        Language::IT => "Esclusi",
        Language::ZH_TW => "已排除",
        Language::DE => "Herausgefiltert",
        _ => "Excluded",
    }
}

pub fn import_capture_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Import capture file",
        Language::IT => "Importa file di cattura",
        Language::DE => "Aufzeichnungsdatei importieren",
        _ => "Import capture file",
    }
}

pub fn select_capture_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select capture file",
        Language::IT => "Seleziona file di cattura",
        Language::DE => "Aufzeichnungsdatei auswählen",
        _ => "Select capture file",
    }
}

pub fn reading_from_pcap_translation<'a>(language: Language, file: &str) -> Text<'a, StyleType> {
    let file_name_translation = file_name_translation(language);
    Text::new(match language {
        Language::EN => format!(
            "Reading packets from file...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Are you sure the file you selected isn't empty?"
        ),
        Language::IT => format!(
            "Lettura pacchetti da file...\n\n\
                                {file_name_translation}: {file}\n\n\
                                Sei sicuro che il file che hai selezionato non sia vuoto?"
        ),
        Language::DE => format!(
            "Pakete aus Datei laden... \n\n\
                                {file_name_translation}: {file}\n\n\
                                Bist du sicher, dass die gewählte Datei nicht leer ist?"
        ),
        _ => format!(
            "Reading packets from file...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Are you sure the file you selected isn't empty?"
        ),
    })
}

pub fn data_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data threshold exceeded",
        Language::IT => "Soglia di dati superata",
        Language::DE => "Datenschwelle überschritten",
        _ => "Data threshold exceeded",
    }
}

#[allow(dead_code)]
pub fn bits_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bits threshold exceeded",
        Language::IT => "Soglia di bit superata",
        Language::DE => "Bitschwelle überschritten",
        _ => "Bits threshold exceeded",
    }
}

#[allow(dead_code)]
pub fn bits_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT | Language::DE => "Bits",
        _ => "Bits",
    }
}

#[allow(dead_code)]
pub fn pause_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE => "Pause",
        Language::IT => "Pausa",
        _ => "Pause",
    }
}

#[allow(dead_code)]
pub fn resume_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Resume",
        Language::IT => "Riprendi",
        Language::DE => "Fortsetzen",
        _ => "Resume",
    }
}
