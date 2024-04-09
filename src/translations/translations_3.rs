#![allow(clippy::match_same_arms)]

use iced::widget::Text;

use crate::translations::translations::network_adapter_translation;
use crate::{Language, StyleType};

// This is referred to settings (General settings)
pub fn general_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "General",
        Language::ES => "Generales",
        Language::IT => "Generali",
        Language::FR => "Général",
        Language::DE => "Allgemein",
        Language::PL => "Ogólne",
        _ => "General",
    }
}

pub fn zoom_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT | Language::ES | Language::FR | Language::DE => "Zoom",
        Language::PL => "Powiększenie",
        _ => "Zoom",
    }
}

pub fn mmdb_files_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Database files",
        Language::ES => "Archivos de la base de datos",
        Language::IT => "File di database",
        Language::FR => "Fichiers de la base de données",
        Language::DE => "Datenbank Dateien",
        Language::PL => "Pliki bazy danych",
        _ => "Database files",
    }
}

pub fn params_not_editable_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "The following parameters can't be modified during the analysis",
        Language::ES => "Los siguientes parámetros no pueden modificarse durante el análisis",
        Language::IT => "I seguenti parametri non sono modificabili durante l'analisi",
        Language::FR => "Les paramètres suivants ne peuvent pas être modifiés durant l'analyse",
        Language::DE => "Die folgenden Paramter können während der Analyse nicht verändert werden",
        Language::PL => "Następujące parametry nie mogą być modyfikowane podczas analizy",
        _ => "The following parameters can't be modified during the analysis",
    }
}

// pub fn file_path_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "File path",
//         Language::IT => "Percorso del file",
//         Language::FR => "Chemin du fichier",
//         Language::DE => "Dateipfad",
//         Language::PL => "Ścieżka pliku",
//         _ => "File path",
//     }
// }

pub fn custom_style_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Custom style",
        Language::ES => "Estilo personalizado",
        Language::IT => "Stile personalizzato",
        Language::FR => "Style personnalisé",
        Language::DE => "Benutzerdefinierter Stil",
        Language::PL => "Niestandardowy styl",
        _ => "Custom style",
    }
}

pub fn copy_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Copy",
        Language::IT | Language::ES => "Copia",
        Language::FR => "Copie",
        Language::DE => "Kopieren",
        Language::PL => "Kopiuj",
        _ => "Copy",
    }
}

pub fn port_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR | Language::DE => "Port",
        Language::EN | Language::PL => "Port",
        Language::ES => "Puerto",
        Language::IT => "Porta",
        _ => "Port",
    }
}

pub fn invalid_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Invalid filters",
        Language::ES => "Filtros inválidos",
        Language::IT => "Filtri non validi",
        Language::FR => "Filtres invalides",
        Language::DE => "Ungültige Filter",
        Language::PL => "Nieprawidłowe filtry",
        _ => "Invalid filters",
    }
}

pub fn messages_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Messages",
        Language::ES => "Mensajes",
        Language::IT => "Messaggi",
        Language::DE => "Nachrichten",
        Language::PL => "Wiadomości",
        _ => "Messages",
    }
}

pub fn link_type_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Link type",
        Language::ES => "Tipo de conexión",
        Language::IT => "Tipo di collegamento",
        Language::FR => "Type de connexion",
        Language::DE => "Verbindungsart",
        Language::PL => "Rodzaj połączenia", // "Typ łącza"?
        _ => "Link type",
    }
}

pub fn unsupported_link_type_translation(
    language: Language,
    adapter: &str,
) -> Text<'static, StyleType> {
    let mut string = match language {
        Language::EN => "The link type associated with this adapter is not supported by Sniffnet yet...",
        Language::ES => "La conexión asociada con este adaptador aún no esta implementada en Sniffnet...",
        Language::IT => "Il tipo di collegamento associato a questo adattatore di rete non è ancora supportato da Sniffnet...",
        Language::FR => "Le type de connexion associé à cet adaptateur n'est pas encore supporté par Sniffnet...",
        Language::DE => "Die Verbindungsart dieses Adapters wird noch nicht von Sniffnet unterstützt",
        Language::PL => "Rodzaj połączenia powiązany z tym adapterem nie jest jeszcze obsługiwany przez Sniffnet...",
        _ => "The link type associated with this adapter is not supported by Sniffnet yet...",
    }.to_string();

    let network_adapter_translation = network_adapter_translation(language);
    string.push_str(&format!("\n\n{network_adapter_translation}: {adapter}"));
    Text::new(string)
}

pub fn style_from_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select style from a file",
        Language::ES => "Selecciona el estilo desde un archivo",
        Language::IT => "Seleziona lo stile da un file",
        Language::FR => "Sélectionner un style à partir d'un fichier",
        Language::DE => "Stil aus einer Datei wählen",
        Language::PL => "Wybierz styl z pliku",
        _ => "Select style from a file",
    }
}

pub fn database_from_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select database file",
        Language::ES => "Selecciona un archivo de base de datos",
        Language::IT => "Seleziona file di database",
        Language::FR => "Sélection d'un fichier de base de données",
        Language::DE => "Datenbank Datei auswählen",
        Language::PL => "Wybierz plik bazy danych",
        _ => "Select database file",
    }
}

pub fn filter_by_host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Filter by network host",
        Language::ES => "Filtra por host de red",
        Language::IT => "Filtra per host di rete",
        Language::FR => "Filtrer par réseau hôte",
        Language::DE => "Nach Netzwerk-Host filtern",
        Language::PL => "Filtruj według hosta sieciowego",
        _ => "Filter by network host",
    }
}

pub fn service_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR | Language::DE => "Service",
        Language::ES => "Servicio",
        Language::IT => "Servizio",
        Language::PL => "Usługa",
        _ => "Service",
    }
}

pub fn export_capture_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Export capture file",
        Language::IT => "Esporta file di cattura",
        Language::FR => "Exporter le fichier de capture",
        Language::DE => "Aufzeichnungsdatei exportieren",
        Language::PL => "Eksportuj plik przechwytywania",
        _ => "Export capture file",
    }
}

// (a filesystem directory)
pub fn directory_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Directory",
        Language::IT => "Cartella",
        Language::FR => "Répertoire",
        Language::DE => "Ordner",
        Language::PL => "Katalog", // Katalog usually refers to Linux based directory while "Folder" is more common between Windows users (e.g. windows explorer refers to directories as "Folders")
        _ => "Directory",
    }
}

pub fn select_directory_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select destination directory",
        Language::IT => "Seleziona cartella di destinazione",
        Language::FR => "Sélectionner le répertoire de destination",
        Language::DE => "Zielorder wählen",
        Language::PL => "Wybierz katalog docelowy", // "Wybierz folder docelowy"?
        _ => "Select destination directory",
    }
}

pub fn file_name_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "File name",
        Language::IT => "Nome del file",
        Language::FR => "Nom du fichier",
        Language::DE => "Dateiname",
        Language::PL => "Nazwa pliku",
        _ => "File name",
    }
}

pub fn thumbnail_mode_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Thumbnail mode",
        Language::IT => "Modalità miniatura",
        Language::FR => "Mode miniature",
        Language::DE => "Bild-in-Bild Modus",
        Language::PL => "Tryb miniatury",
        _ => "Thumbnail mode",
    }
}

pub fn learn_more_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Do you want to learn more?",
        Language::IT => "Vuoi saperne di più?",
        Language::FR => "Voulez-vous en savoir davantage?",
        Language::DE => "Mehr erfahren",
        Language::PL => "Chcesz dowiedzieć się więcej?",
        _ => "Do you want to learn more?",
    }
}
