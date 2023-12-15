#![allow(clippy::match_same_arms)]

use crate::Language;

// This is referred to settings (General settings)
pub fn general_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "General",
        Language::IT => "Generali",
        _ => "General",
    }
}

pub fn zoom_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT => "Zoom",
        _ => "Zoom",
    }
}

pub fn mmdb_files_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Database files (MMDB format)",
        Language::IT => "File di database (formato MMDB)",
        _ => "Database files (MMDB format)",
    }
}

pub fn params_not_editable_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "The following parameters can't be modified during the analysis",
        Language::IT => "I seguenti parametri non sono modificabili durante l'analisi",
        _ => "The following parameters can't be modified during the analysis",
    }
}

// pub fn file_path_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "File path",
//         Language::IT => "Percorso del file",
//         _ => "File path",
//     }
// }

pub fn custom_style_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Custom style",
        Language::IT => "Stile personalizzato",
        _ => "Custom style",
    }
}

pub fn copy_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Copy",
        Language::IT => "Copia",
        _ => "Copy",
    }
}

pub fn port_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Port",
        Language::IT => "Porta",
        _ => "Port",
    }
}
