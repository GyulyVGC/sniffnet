#![allow(clippy::match_same_arms)]

use iced::widget::Text;

use crate::translations::translations::network_adapter_translation;
use crate::{Language, StyleType};

// This is referred to settings (General settings)
pub fn general_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::RO => "General",
        Language::ES => "Generales",
        Language::IT => "Generali",
        Language::FR => "Général",
        Language::DE => "Allgemein",
        Language::PL => "Ogólne",
        Language::RU => "Общие",
        Language::JA => "一般",
        _ => "General",
    }
}

pub fn zoom_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT | Language::ES | Language::FR | Language::DE | Language::RO => {
            "Zoom"
        }
        Language::PL => "Powiększenie",
        Language::RU => "Масштаб интерфейса",
        Language::JA => "ズーム",
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
        Language::RU => "Файлы базы данных",
        Language::RO => "Fișiere bază de date",
        Language::JA => "データベース ファイル",
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
        Language::RU => "Следующие параметры не могут быть изменены во время анализа трафика",
        Language::RO => "Următorii parametri nu sunt modificabili în timpul analizei",
        Language::JA => "以下のパラメーターは分析中は変更できません",
        _ => "The following parameters can't be modified during the analysis",
    }
}

pub fn custom_style_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Custom style",
        Language::ES => "Estilo personalizado",
        Language::IT => "Stile personalizzato",
        Language::FR => "Style personnalisé",
        Language::DE => "Benutzerdefinierter Stil",
        Language::PL => "Niestandardowy styl",
        Language::RU => "Свой стиль",
        Language::RO => "Temă personalizată",
        Language::JA => "カスタム スタイル",
        _ => "Custom style",
    }
}

pub fn copy_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Copy",
        Language::IT | Language::ES => "Copia",
        Language::FR | Language::RO => "Copie",
        Language::DE => "Kopieren",
        Language::PL => "Kopiuj",
        Language::RU => "Скопировать",
        Language::JA => "コピー",
        _ => "Copy",
    }
}

pub fn port_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR | Language::DE | Language::PL | Language::RO => "Port",
        Language::ES => "Puerto",
        Language::IT => "Porta",
        Language::RU => "Порт",
        Language::JA => "ポート",
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
        Language::RU => "Неверный формат фильтров",
        Language::RO => "Filtre invalide",
        Language::JA => "無効なフィルター",
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
        Language::RU => "Сообщения",
        Language::RO => "Mesaje",
        Language::JA => "メッセージ",
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
        Language::RU => "Тип соединения",
        Language::RO => "Tipul conexiunii",
        Language::JA => "リンク タイプ",
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
        Language::RU => "Тип соединения, связанный с этим адаптером, пока не поддерживается Sniffnet...",
        Language::RO => "Tipul conexiunii asociate acestui adaptor de rețea nu este încă suportat de Sniffnet...",
        Language::JA => "このアダプターのリンク タイプは Sniffnet ではまだサポートされていません...",
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
        Language::RU => "Выберите тему из файла",
        Language::RO => "Selectează tema dintr-un fișier",
        Language::JA => "ファイルからスタイルを選択してください",
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
        Language::RU => "Выберите файл базы данных",
        Language::RO => "Selectează fișier bază de date",
        Language::JA => "データベース ファイルを選択してください",
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
        Language::RU => "Фильтр по сетевому хосту",
        Language::RO => "Filtrează după host-ul de rețea",
        Language::JA => "ネットワーク ホストでフィルター",
        _ => "Filter by network host",
    }
}

pub fn service_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR | Language::DE => "Service",
        Language::ES => "Servicio",
        Language::IT => "Servizio",
        Language::PL => "Usługa",
        Language::RU => "Сервис",
        Language::RO => "Serviciu",
        Language::JA => "サービス",
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
        Language::RU => "Экспорт файла захвата",
        Language::RO => "Export fișier captură",
        Language::JA => "キャプチャ ファイルをエクスポート",
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
        Language::RU => "Директория",
        Language::RO => "Director",
        Language::JA => "ディレクトリー",
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
        Language::RU => "Выберите директорию назначения",
        Language::RO => "Selectează directorul destinație",
        Language::JA => "宛先のディレクトリーを選択する",
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
        Language::RU => "Имя файла",
        Language::RO => "Nume fișier",
        Language::JA => "ファイル ネーム",
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
        Language::RU => "Режим миниатюры",
        Language::RO => "Mod thumbnail",
        Language::JA => "サムネイル モード",
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
        Language::RU => "Хотите узнать больше?",
        Language::RO => "Vrei să înveți mai multe?",
        Language::JA => "もっと知りたいですか？",
        _ => "Do you want to learn more?",
    }
}
