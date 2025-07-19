#![allow(clippy::match_same_arms)]

use iced::widget::Text;

use crate::translations::translations::network_adapter_translation;
use crate::{Language, StyleType};

// This is referred to settings (General settings)
pub fn general_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::RO => "General",
        // Language::FA => "عمومی",
        Language::ES => "Generales",
        Language::IT => "Generali",
        Language::FR => "Général",
        Language::DE => "Allgemein",
        Language::PL => "Ogólne",
        Language::RU => "Общие",
        Language::JA => "一般",
        Language::UZ => "Asosiy",
        Language::SV => "Allmänt",
        Language::VI => "Tổng quan",
        Language::ZH => "通用",
        Language::ZH_TW => "一般",
        Language::KO => "일반",
        Language::TR => "Genel",
        Language::PT => "Geral",
        Language::UK => "Загальні",
        Language::ID => "Umum",
        Language::NL => "Algemeen",
        Language::EL => "Γενικά",
        _ => "General",
    }
}

pub fn zoom_translation(language: Language) -> &'static str {
    match language {
        Language::EN
        | Language::IT
        | Language::ES
        | Language::FR
        | Language::DE
        | Language::RO
        | Language::PT
        | Language::NL
        | Language::SV => "Zoom",
        // Language::FA => "بزرگنمایی",
        Language::PL => "Powiększenie",
        Language::RU => "Масштаб интерфейса",
        Language::JA => "ズーム",
        Language::UZ => "Kattalashtirish",
        Language::VI => "Phóng to",
        Language::ZH => "缩放",
        Language::ZH_TW => "縮放",
        Language::KO => "확대",
        Language::TR => "Yakınlaştırma",
        Language::UK => "Масштабування",
        Language::ID => "Perbesar",
        Language::EL => "Εστίαση",
        _ => "Zoom",
    }
}

pub fn mmdb_files_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Database files",
        // Language::FA => "پرونده های پایگاه داده",
        Language::ES => "Archivos de la base de datos",
        Language::IT => "File di database",
        Language::FR => "Fichiers de la base de données",
        Language::DE => "Datenbank Dateien",
        Language::PL => "Pliki bazy danych",
        Language::RU => "Файлы базы данных",
        Language::RO => "Fișiere bază de date",
        Language::JA => "データベース ファイル",
        Language::UZ => "Ma'lumotlar bazasi fayllari",
        Language::SV => "Databasfiler",
        Language::VI => "Tập tin cơ sở dữ liệu",
        Language::ZH => "数据库文件",
        Language::ZH_TW => "資料庫檔案",
        Language::KO => "데이터베이스 파일",
        Language::TR => "Veri tabanı dosyaları",
        Language::PT => "Arquivos da base de dados",
        Language::UK => "Файли бази даних",
        Language::ID => "Berkas database",
        Language::NL => "Database bestanden",
        Language::EL => "Αρχεία βάσης δεδομένων",
        _ => "Database files",
    }
}

pub fn params_not_editable_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "The following parameters can't be modified during the analysis",
        // Language::FA => "مولفه های روبرو هنگام تحلیل قابل تغییر نیستند",
        Language::ES => "Los siguientes parámetros no pueden modificarse durante el análisis",
        Language::IT => "I seguenti parametri non sono modificabili durante l'analisi",
        Language::FR => "Les paramètres suivants ne peuvent pas être modifiés durant l'analyse",
        Language::DE => "Die folgenden Parameter können während der Analyse nicht verändert werden",
        Language::PL => "Następujące parametry nie mogą być modyfikowane podczas analizy",
        Language::RU => "Следующие параметры не могут быть изменены во время анализа трафика",
        Language::RO => "Următorii parametri nu sunt modificabili în timpul analizei",
        Language::JA => "以下のパラメーターは分析中は変更できません",
        Language::UZ => "Tahlil vaqtida quydagi parametrlarni o'zgartirib bo'lmaydi",
        Language::SV => "Följande parametrar kan inte ändras under analysen",
        Language::VI => "Các tham số sau không thể bị thay đổi khi đang phân tích",
        Language::ZH => "以下参数在分析过程中不能修改",
        Language::ZH_TW => "以下參數在分析期間無法修改",
        Language::KO => "분석 중 다음의 매개변수들은 수정할 수 없습니다",
        Language::TR => "Analiz sırasında bu parametrelere müdahale edilemez",
        Language::PT => "Os seguintes parâmetros não podem ser modificados durante a análise",
        Language::UK => "Наступні параметри не можна змінювати під час аналізу трафіку",
        Language::ID => "Parameter berikut tidak bisa diubah saat dianalisa",
        Language::NL => "De volgende parameters kunnen niet worden aangepast tijdens de analyse",
        Language::EL => "Οι ακόλουθες παράμετροι δεν μπορούν να τροποποιηθούν κατά τη διάρκεια της ανάλυσης",
        _ => "The following parameters can't be modified during the analysis",
    }
}

pub fn custom_style_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Custom style",
        // Language::FA => "شیوه سفارشی",
        Language::ES | Language::PT => "Estilo personalizado",
        Language::IT => "Stile personalizzato",
        Language::FR => "Style personnalisé",
        Language::DE => "Benutzerdefinierter Stil",
        Language::PL => "Niestandardowy styl",
        Language::RU => "Свой стиль",
        Language::RO => "Temă personalizată",
        Language::JA => "カスタム スタイル",
        Language::UZ => "Moslashtirilgan uslub",
        Language::SV => "Anpassad stil",
        Language::VI => "Tùy chỉnh chủ đề",
        Language::ZH => "自定义样式",
        Language::ZH_TW => "自訂佈景主題",
        Language::KO => "사용자 지정 스타일",
        Language::TR => "Kişisel görünüm",
        Language::UK => "Власний стиль",
        Language::ID => "Ubah Model",
        Language::NL => "Aangepaste stijl",
        Language::EL => "Προσαρμοσμένο στυλ",
        _ => "Custom style",
    }
}

pub fn copy_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Copy",
        // Language::FA => "رونوشت",
        Language::IT | Language::ES => "Copia",
        Language::FR | Language::RO => "Copie",
        Language::DE => "Kopieren",
        Language::PL => "Kopiuj",
        Language::RU => "Скопировать",
        Language::JA => "コピー",
        Language::UZ => "Nusxalash",
        Language::SV => "Kopia",
        Language::VI => "Sao chép",
        Language::ZH => "复制",
        Language::ZH_TW => "複製",
        Language::KO => "복사",
        Language::TR => "Kopyala",
        Language::PT => "Copiar",
        Language::UK => "Копіювати",
        Language::ID => "Salin",
        Language::NL => "Kopiëren",
        Language::EL => "Αντιγραφή",
        _ => "Copy",
    }
}

pub fn port_translation(language: Language) -> &'static str {
    match language {
        Language::EN
        | Language::FR
        | Language::DE
        | Language::PL
        | Language::RO
        | Language::UZ
        | Language::SV
        | Language::TR => "Port",
        // Language::FA => "درگاه",
        Language::ES => "Puerto",
        Language::IT | Language::PT => "Porta",
        Language::RU => "Порт",
        Language::JA => "ポート",
        Language::VI => "Cổng",
        Language::ZH => "端口",
        Language::ZH_TW => "連接埠",
        Language::KO => "포트",
        Language::UK => "Порт",
        Language::ID => "Port",
        Language::NL => "Poort",
        Language::EL => "Θύρα",
        _ => "Port",
    }
}

pub fn invalid_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Invalid filters",
        // Language::FA => "صافی نامعتبر",
        Language::ES | Language::PT => "Filtros inválidos",
        Language::IT => "Filtri non validi",
        Language::FR => "Filtres invalides",
        Language::DE => "Ungültige Filter",
        Language::PL => "Nieprawidłowe filtry",
        Language::RU => "Неверный формат фильтров",
        Language::RO => "Filtre invalide",
        Language::JA => "無効なフィルター",
        Language::UZ => "Noto'g'ri filterlar",
        Language::SV => "Ogiltiga filter",
        Language::VI => "Bộ lọc không khả dụng",
        Language::ZH => "无效的过滤器",
        Language::ZH_TW => "無效的篩選器",
        Language::KO => "잘못된 필터",
        Language::TR => "Geçersiz filtreler",
        Language::UK => "Неправильний формат фільтрів",
        Language::ID => "Filter salah",
        Language::NL => "Ongeldige filters",
        Language::EL => "Μη έγκυρα φίλτρα",
        _ => "Invalid filters",
    }
}

pub fn messages_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Messages",
        // Language::FA => "پیام ها",
        Language::ES => "Mensajes",
        Language::IT => "Messaggi",
        Language::DE => "Nachrichten",
        Language::PL => "Wiadomości",
        Language::RU => "Сообщения",
        Language::RO => "Mesaje",
        Language::JA => "メッセージ",
        Language::UZ => "Xabarlar",
        Language::SV => "Meddelanden",
        Language::VI => "Tin nhắn",
        Language::ZH => "信息",
        Language::ZH_TW => "訊息",
        Language::KO => "메시지",
        Language::TR => "Mesajlar",
        Language::PT => "Mensagens",
        Language::UK => "Повідомлення",
        Language::ID => "Pesan",
        Language::NL => "Berichten",
        Language::EL => "Μηνύματα",
        _ => "Messages",
    }
}

pub fn link_type_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::NL => "Link type",
        // Language::FA => "نوع پیوند",
        Language::ES => "Tipo de conexión",
        Language::IT => "Tipo di collegamento",
        Language::FR => "Type de connexion",
        Language::DE => "Verbindungsart",
        Language::PL => "Rodzaj połączenia", // "Typ łącza"?
        Language::RU => "Тип соединения",
        Language::RO => "Tipul conexiunii",
        Language::JA => "リンク タイプ",
        Language::UZ => "Havola turi",
        Language::SV => "Länktyp",
        Language::VI => "Loại liên kết",
        Language::ZH => "链接类型",
        Language::ZH_TW => "連線類型",
        Language::KO => "링크 유형",
        Language::TR => "Link türü",
        Language::PT => "Tipo de conexão",
        Language::UK => "Різновид зʼєднання",
        Language::ID => "Tipe koneksi",
        Language::EL => "Τύπος σύνδεσης",
        _ => "Link type",
    }
}

pub fn unsupported_link_type_translation<'a>(
    language: Language,
    adapter: &str,
) -> Text<'a, StyleType> {
    let translation = match language {
        Language::EN => {
            "The link type associated with this adapter is not supported by Sniffnet yet..."
        }
        // Language::FA => "نوع پیوند مرتبط با این مبدل هنوز توسط Sniffnet پشتیبانی نمی شود...",
        Language::ES => {
            "La conexión asociada a este adaptador aún no está soportada por Sniffnet..."
        }
        Language::IT => {
            "Il tipo di collegamento associato a questo adattatore di rete non è ancora supportato da Sniffnet..."
        }
        Language::FR => {
            "Le type de connexion associé à cet adaptateur n'est pas encore supporté par Sniffnet..."
        }
        Language::DE => {
            "Die Verbindungsart dieses Adapters wird noch nicht von Sniffnet unterstützt..."
        }
        Language::PL => {
            "Rodzaj połączenia powiązany z tym adapterem nie jest jeszcze obsługiwany przez Sniffnet..."
        }
        Language::RU => {
            "Тип соединения, связанный с этим адаптером, пока не поддерживается Sniffnet..."
        }
        Language::RO => {
            "Tipul conexiunii asociate acestui adaptor de rețea nu este încă suportat de Sniffnet..."
        }
        Language::JA => {
            "このアダプターのリンク タイプは Sniffnet ではまだサポートされていません..."
        }
        Language::UZ => {
            "Ushbu adapter bilan bog'langan havola turi hozircha Sniffnet tomonidan qo'llab quvvatlanmaydi..."
        }
        Language::SV => "Länktypen associerad med denna adapter stöds inte av Sniffnet än...",
        Language::VI => "Loại liên kết được gắn với adapter này chưa được Sniffnet hỗ trợ...",
        Language::ZH => "Sniffnet 尚不支持与此适配器关联的链接类型...",
        Language::ZH_TW => "Sniffnet 目前尚不支援與此網路介面卡相關的連線類型...",
        Language::KO => "이 어댑터와 연결된 링크 유형은 Sniffnet에서 아직 지원되지 않습니다...",
        Language::TR => {
            "Bu adaptör ile ilişkilendirilmiş link türü henüz Sniffnet tarafından desteklenmiyor..."
        }
        Language::PT => {
            "O tipo de conexão associado com este adaptador não é suportado pelo Sniffnet ainda..."
        }
        Language::UK => {
            "Різновид зʼєднання, повʼязаний з даним адаптером, ще не підтримується Sniffnet-ом..."
        }
        Language::ID => {
            "Tipe koneksi yang terhubung dengan adaptor ini belum didukung oleh Sniffnet"
        }
        Language::NL => {
            "Het linktype dat is gekoppeld aan deze adapter wordt nog niet ondersteund door Sniffnet..."
        }
        Language::EL => {
            "Ο τύπος σύνδεσης που σχετίζεται με αυτόν τον προσαρμογέα δεν υποστηρίζεται ακόμη από το Sniffnet..."
        }
        _ => "The link type associated with this adapter is not supported by Sniffnet yet...",
    };

    Text::new(format!(
        "{translation}\n\n{}: {adapter}",
        network_adapter_translation(language)
    ))
}

pub fn style_from_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select style from a file",
        // Language::FA => "انتخاب شیوه از یک پرونده",
        Language::ES => "Selecciona el estilo desde un archivo",
        Language::IT => "Seleziona lo stile da un file",
        Language::FR => "Sélectionner un style à partir d'un fichier",
        Language::DE => "Stil aus einer Datei wählen",
        Language::PL => "Wybierz styl z pliku",
        Language::RU => "Выберите тему из файла",
        Language::RO => "Selectează tema dintr-un fișier",
        Language::JA => "ファイルからスタイルを選択してください",
        Language::UZ => "Fayldan uslubni tanlang",
        Language::SV => "Välj stil från en fil",
        Language::VI => "Chọn chủ đề từ file của bạn",
        Language::ZH => "从文件中选择样式",
        Language::ZH_TW => "從檔案中選擇佈景主題",
        Language::KO => "파일에서 스타일을 선택하세요",
        Language::TR => "Dosyadan bir görünüm seç",
        Language::PT => "Selecionar estilo a partir de um arquivo",
        Language::UK => "Виберіть стиль з файлу",
        Language::ID => "Pilih model / gaya dari berkas",
        Language::NL => "Selecteer stijl vanuit een bestand",
        Language::EL => "Επιλογή στυλ από αρχείο",
        _ => "Select style from a file",
    }
}

pub fn database_from_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select database file",
        // Language::FA => "پرونده پایگاه داده را انتخاب کنید",
        Language::ES => "Selecciona un archivo de base de datos",
        Language::IT => "Seleziona file di database",
        Language::FR => "Sélection d'un fichier de base de données",
        Language::DE => "Datenbank Datei auswählen",
        Language::PL => "Wybierz plik bazy danych",
        Language::RU => "Выберите файл базы данных",
        Language::RO => "Selectează fișier bază de date",
        Language::JA => "データベース ファイルを選択してください",
        Language::UZ => "Ma'lumotlar bazasi faylini tanlang",
        Language::SV => "Välj databasfil",
        Language::VI => "Chọn tập tin cơ sở dữ liệu",
        Language::ZH => "选择数据库文件",
        Language::ZH_TW => "選擇資料庫檔案",
        Language::KO => "데이터베이스 파일 선택",
        Language::TR => "Veri tabanı dosyası seç",
        Language::PT => "Selecione um arquivo de base de dados",
        Language::UK => "Виберіть файл бази даних",
        Language::ID => "Pilih berkas database",
        Language::NL => "Selecteer database bestand",
        Language::EL => "Επιλογή αρχείου βάσης δεδομένων",
        _ => "Select database file",
    }
}

pub fn filter_by_host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Filter by network host",
        // Language::FA => "صافی بر اساس میزبان شبکه",
        Language::ES => "Filtra por host de red",
        Language::IT => "Filtra per host di rete",
        Language::FR => "Filtrer par réseau hôte",
        Language::DE => "Nach Netzwerk-Host filtern",
        Language::PL => "Filtruj według hosta sieciowego",
        Language::RU => "Фильтр по сетевому хосту",
        Language::RO => "Filtrează după host-ul de rețea",
        Language::JA => "ネットワーク ホストでフィルター",
        Language::UZ => "Tarmoq host bo'yicha filterlash",
        Language::SV => "Filtrera efter nätverksvärd",
        Language::VI => "Lọc bởi máy chủ mạng",
        Language::ZH => "按网络主机筛选",
        Language::ZH_TW => "依網路主機篩選",
        Language::KO => "네트워크 호스트로 필터링",
        Language::TR => "Ağ sunucusuna göre filtrele",
        Language::PT => "Filtrar por host de rede",
        Language::UK => "Фільтр за хостом мережі",
        Language::ID => "Filter berdasarkan jaringan asal",
        Language::NL => "Filteren op netwerk host",
        Language::EL => "Φίλτρο ανά διακομιστή δικτύου",
        _ => "Filter by network host",
    }
}

pub fn service_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR | Language::DE | Language::SV => "Service",
        // Language::FA => "خدمت",
        Language::ES => "Servicio",
        Language::IT => "Servizio",
        Language::PL => "Usługa",
        Language::RU => "Сервис",
        Language::RO => "Serviciu",
        Language::JA => "サービス",
        Language::UZ => "Xizmat",
        Language::VI => "Dịch vụ",
        Language::ZH => "服务",
        Language::ZH_TW => "服務",
        Language::KO => "서비스",
        Language::TR => "Servis",
        Language::PT => "Serviço",
        Language::UK => "Сервіс",
        Language::ID => "Layanan",
        Language::NL => "Dienst",
        Language::EL => "Υπηρεσία",
        _ => "Service",
    }
}

pub fn export_capture_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Export capture file",
        // Language::FA => "خروجی گرفتن پرونده تسخیری",
        Language::IT => "Esporta file di cattura",
        Language::FR => "Exporter le fichier de capture",
        Language::DE => "Aufzeichnungsdatei exportieren",
        Language::PL => "Eksportuj plik przechwytywania",
        Language::RU => "Экспорт файла захвата",
        Language::RO => "Export fișier captură",
        Language::JA => "キャプチャ ファイルをエクスポート",
        Language::UZ => "Cap faylni export qilish",
        Language::SV => "Exportera inspelningsfil",
        Language::VI => "Xuất tập tin đã bắt",
        Language::ZH => "导出捕获文件",
        Language::ZH_TW => "匯出擷取的檔案",
        Language::KO => "캡처 파일 내보내기",
        Language::TR => "Yakalanan dosyayı dışa aktar",
        Language::PT => "Exportar arquivo capturado",
        Language::UK => "Експорт файлу захоплення",
        Language::ID => "Ekspor data tangkapan",
        Language::ES => "Exportar archivo de captura",
        Language::NL => "Exporteer capture bestand",
        Language::EL => "Εξαγωγή αρχείου καταγραφής",
        _ => "Export capture file",
    }
}

// (a filesystem directory)
pub fn directory_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Directory",
        // Language::FA => "پوشه",
        Language::IT => "Cartella",
        Language::FR => "Répertoire",
        Language::DE => "Ordner",
        Language::PL | Language::UZ | Language::SV => "Katalog",
        Language::RU => "Директория",
        Language::RO => "Director",
        Language::JA => "ディレクトリー",
        Language::VI => "Thư mục",
        Language::ZH => "目录",
        Language::ZH_TW => "目錄",
        Language::KO => "디렉토리",
        Language::TR => "Klasör",
        Language::PT => "Diretório",
        Language::UK => "Тека",
        Language::ID => "Direktori",
        Language::ES => "Directorio",
        Language::NL => "Map",
        Language::EL => "Κατάλογος",
        _ => "Directory",
    }
}

pub fn select_directory_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select destination directory",
        // Language::FA => "انتخاب پوشه مقصد",
        Language::IT => "Seleziona cartella di destinazione",
        Language::FR => "Sélectionner le répertoire de destination",
        Language::DE => "Zielordner wählen",
        Language::PL => "Wybierz katalog docelowy", // "Wybierz folder docelowy"?
        Language::RU => "Выберите директорию назначения",
        Language::RO => "Selectează directorul destinație",
        Language::JA => "宛先のディレクトリーを選択する",
        Language::UZ => "Manzil katalogni tanlang",
        Language::SV => "Välj målkatalog",
        Language::VI => "Chọn thư mục đích đến",
        Language::ZH => "选择目标目录",
        Language::ZH_TW => "選擇目的目錄",
        Language::KO => "대상 디렉토리 선택",
        Language::TR => "Hedef klasörü seç",
        Language::PT => "Selecionar diretório de destino",
        Language::UK => "Виберіть теку призначення",
        Language::ID => "Pilih direktori tujuan",
        Language::ES => "Selecciona el directorio de destino",
        Language::NL => "Selecteer doelmap",
        Language::EL => "Επιλογή καταλόγου προορισμού",
        _ => "Select destination directory",
    }
}

pub fn file_name_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "File name",
        // Language::FA => "نام پرونده",
        Language::IT => "Nome del file",
        Language::FR => "Nom du fichier",
        Language::DE => "Dateiname",
        Language::PL => "Nazwa pliku",
        Language::RU => "Имя файла",
        Language::RO => "Nume fișier",
        Language::JA => "ファイル ネーム",
        Language::UZ => "Fayl nomi",
        Language::SV => "Filnamn",
        Language::VI => "Tên file",
        Language::ZH => "文件名",
        Language::ZH_TW => "檔案名稱",
        Language::KO => "파일 이름",
        Language::TR => "Dosya adı",
        Language::PT => "Nome do arquivo",
        Language::UK => "Назва файлу",
        Language::ID => "Nama berkas",
        Language::ES => "Nombre del archivo",
        Language::NL => "Bestandsnaam",
        Language::EL => "Όνομα αρχείου",
        _ => "File name",
    }
}

pub fn thumbnail_mode_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Thumbnail mode",
        // Language::FA => "حالت تصویر بندانگشتی",
        Language::IT => "Modalità miniatura",
        Language::FR => "Mode miniature",
        Language::DE => "Bild-in-Bild Modus",
        Language::PL => "Tryb miniatury",
        Language::RU => "Режим миниатюры",
        Language::RO => "Mod thumbnail",
        Language::JA => "サムネイル モード",
        Language::UZ => "Eskiz rejim",
        Language::SV => "Miniatyrläge",
        Language::VI => "Chế độ thu nhỏ",
        Language::ZH => "缩略图模式",
        Language::ZH_TW => "縮圖模式",
        Language::KO => "썸네일 모드",
        Language::TR => "Küçük resim modu",
        Language::PT | Language::ES => "Modo miniatura",
        Language::UK => "Режим мініатюри",
        Language::ID => "Mode gambar kecil",
        Language::NL => "Miniatuur modus",
        Language::EL => "Λειτουργία μικρογραφιών",
        _ => "Thumbnail mode",
    }
}

// pub fn learn_more_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Do you want to learn more?",
//         // Language::FA => "آیا می خواهید بیشتر یاد بگیرید؟",
//         Language::IT => "Vuoi saperne di più?",
//         Language::FR => "Voulez-vous en savoir davantage?",
//         Language::DE => "Mehr erfahren",
//         Language::PL => "Chcesz dowiedzieć się więcej?",
//         Language::RU => "Хотите узнать больше?",
//         Language::RO => "Vrei să înveți mai multe?",
//         Language::JA => "もっと知りたいですか？",
//         Language::UZ => "Ko'proq bilishni hohlaysizmi?",
//         Language::SV => "Vill du veta mer?",
//         Language::VI => "Bạn có muốn tìm hiểu thêm?",
//         Language::ZH => "想知道更多吗？",
//         Language::ZH_TW => "想了解更多嗎？",
//         Language::KO => "더 자세히 알고 싶으십니까?",
//         Language::TR => "Daha fazlasını öğrenmek ister misin?",
//         Language::PT => "Quer aprender mais?",
//         Language::UK => "Бажаєте дізнатись більше?",
//         Language::ID => "Apakah kamu mau belajar lebih lanjut?",
//         Language::ES => "¿Quieres aprender más?",
//         Language::NL => "Wil je meer leren?",
//         _ => "Do you want to learn more?",
//     }
// }
