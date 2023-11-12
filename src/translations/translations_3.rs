#![allow(clippy::match_same_arms)]

use crate::Language;

pub fn advanced_settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Advanced settings",
        Language::IT => "Impostazioni avanzate",
        _ => "Advanced settings",
    }
}

pub fn scale_factor_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Scale factor",
        Language::IT => "Fattore di scala",
        _ => "Scale factor",
    }
}

pub fn restore_defaults_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Restore defaults",
        Language::IT => "Ripristina valori predefiniti",
        _ => "Restore defaults",
    }
}

pub fn mmdb_paths_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Database file paths (MMDB format)",
        Language::IT => "Percorsi dei file di database (formato MMDB)",
        _ => "Database file paths (MMDB format)",
    }
}

pub fn info_mmdb_paths_translation(language: Language) -> &'static str {
    match language {
        Language::EN => {
            "You can specify database files different from the default ones. \n\
                            This is useful if you own the commercial version of such databases."
        }
        Language::IT => {
            "Puoi specificare file di database diversi da quelli predefiniti. \n\
                            Ciò è utile se possiedi la versione commerciale di tali database."
        }
        _ => {
            "You can specify database files different from the default ones. \n\
                            This is useful if you own the commercial version of such databases."
        }
    }
}

pub fn params_not_editable_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "The following parameters can't be modified during the analysis",
        Language::IT => "I seguenti parametri non sono modificabili durante l'analisi",
        _ => "The following parameters can't be modified during the analysis",
    }
}

pub fn file_path_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "File path",
        Language::IT => "Percorso del file",
        _ => "File path",
    }
}

pub fn custom_style_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Custom style",
        Language::IT => "Stile personalizzato",
        _ => "Custom style",
    }
}
