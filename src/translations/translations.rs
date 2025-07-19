// EXTRA NEEDED CHARACTERS: Б

use iced::widget::Text;

use crate::StyleType;
use crate::translations::types::language::Language;

pub fn choose_adapters_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
        Language::FR => "Sélectionnez une carte réseau à inspecter",
        Language::ES => "Seleccione el adaptador de red que desea inspeccionar",
        Language::PL => "Wybierz adapter sieciowy do inspekcji",
        Language::DE => "Wähle einen Netzwerkadapter zum überwachen aus",
        Language::UK => "Виберіть мережевий адаптер для перевірки",
        Language::ZH => "选择需要监控的网络适配器",
        Language::ZH_TW => "選取要檢視的網路介面卡",
        Language::RO => "Selectați adaptor de rețea pentru a inspecta",
        Language::KO => "검사할 네트워크 어댑터 선택",
        Language::TR => "İncelemek için bir ağ adaptörü seçiniz",
        Language::RU => "Выберите сетевой адаптер для инспекции",
        Language::PT => "Selecione o adaptador de rede a inspecionar",
        Language::EL => "Επιλέξτε τον προσαρμογέα δικτύου για ανάλυση",
        // Language::FA => "مبدل شبکه را برای بازرسی انتخاب کنید",
        Language::SV => "Välj nätverksadapter att inspektera",
        Language::FI => "Valitse tarkasteltava verkkosovitin",
        Language::JA => "使用するネットワーク アダプターを選択してください",
        Language::UZ => "Tekshirish uchun tarmoq adapterini tanlang",
        Language::VI => "Hãy chọn network adapter để quan sát",
        Language::ID => "Pilih Adapter Jaringan yang ingin dicek",
        Language::NL => "Selecteer netwerkadapter om te inspecteren",
    })
}

// pub fn application_protocol_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Application protocol",
//         Language::IT => "Protocollo applicativo",
//         Language::FR => "Protocole applicatif",
//         Language::ES => "Protocolo de aplicación",
//         Language::PL => "Protokół aplikacji",
//         Language::DE => "Anwendungs-Protokoll",
//         Language::UK => "Протокол застосування",
//         Language::ZH => "目标应用层协议",
//         Language::ZH_TW => "應用程式通訊協定",
//         Language::RO => "Protocol aplicație",
//         Language::KO => "어플리케이션 프로토콜",
//         Language::TR => "Uygulama protokolü",
//         Language::RU => "Прикладной протокол",
//         Language::PT => "Protocolo de aplicação",
//         Language::EL => "Πρωτόκολλο εφαρμογής",
//         // Language::FA => "پیوندنامهٔ درخواست",
//         Language::SV => "Applikationsprotokoll",
//         Language::FI => "Sovellusprotokolla",
//         Language::JA => "アプリケーション プロトコル",
//         Language::UZ => "Ilova protokoli",
//         Language::ID => "Protokol Aplikasi",
//         Language::NL => "Toepassingsprotocol",
//     }
// }

pub fn select_filters_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
        Language::FR => "Sélectionnez les filtres à appliquer sur le traffic réseau",
        Language::ES => "Seleccionar los filtros que se aplicarán al tráfico de red",
        Language::PL => "Wybierz filtry, które mają być zastosowane na ruchu sieciowym",
        Language::DE => "Wähle die Filter, die auf den Netzwerkverkehr angewendet werden sollen",
        Language::UK => "Виберіть фільтри, які мають бути застосовані до мережевого руху",
        Language::ZH => "选择需要监控的目标",
        Language::ZH_TW => "選取要套用於網路流量的篩選器",
        Language::RO => "Selectați filtre pentru traficul de rețea",
        Language::KO => "네트워크 트레픽에 적용할 필터 선택",
        Language::TR => "Ağ trafiğine uygulanacak filtreleri seçiniz",
        Language::RU => "Выберите фильтры для применения к сетевому трафику",
        Language::PT => "Selecione os filtros a serem aplicados no tráfego de rede",
        Language::EL => "Επιλέξτε τα φίλτρα που θα εφαρμοστούν στην κίνηση του δικτύου",
        // Language::FA => "صافی ها را جهت اعمال بر آمد و شد شبکه انتخاب کنید",
        Language::SV => "Välj filtren som ska appliceras på nätverkstrafiken",
        Language::FI => "Valitse suodattimet verkkoliikenteelle",
        Language::JA => "トラフィックに適用するフィルターを選択してください",
        Language::UZ => "Tarmoq trafigiga qo'llaniladigan filtrlarni tanlang",
        Language::VI => "Hãy chọn bộ lọc cho lưu lượng mạng",
        Language::ID => "Pilih filter yang ingin dipasang dilalulintas jaringan",
        Language::NL => "Selecteer filters om toe te passen op netwerkverkeer",
    })
}

pub fn start_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE | Language::RO | Language::KO | Language::NL => "Start!",
        Language::IT => "Avvia!",
        Language::FR => "Commencer!",
        Language::ES => "¡Empieza!",
        Language::PL => "Rozpocznij!",
        Language::UK => "Почати!",
        Language::ZH => "开始!",
        Language::TR => "Başla!",
        Language::RU => "Начать!",
        Language::PT => "Começar!",
        Language::EL => "Ξεκίνα!",
        // Language::FA => "شروع!",
        Language::SV => "Starta!",
        Language::FI => "Aloita!",
        Language::JA | Language::ZH_TW => "開始！",
        Language::UZ => "Boshlash!",
        Language::VI => "Bắt đầu!",
        Language::ID => "Mulai!",
    }
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Address",
        Language::IT => "Indirizzo",
        Language::FR | Language::DE => "Adresse",
        Language::ES => "Dirección",
        Language::PL | Language::TR | Language::NL => "Adres",
        Language::UK => "Адреса",
        Language::ZH => "网络地址",
        Language::ZH_TW => "網路位址",
        Language::RO => "Adresă",
        Language::KO => "주소",
        Language::RU => "Адрес",
        Language::PT => "Endereço",
        Language::EL => "Διεύθυνση",
        // Language::FA => "نشانی",
        Language::SV => "Adress",
        Language::FI => "Osoite",
        Language::JA => "アドレス",
        Language::UZ => "Manzil",
        Language::VI => "Địa chỉ",
        Language::ID => "Alamat",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Addresses",
        Language::IT => "Indirizzi",
        Language::FR => "Adresses",
        Language::ES => "Direcciones",
        Language::PL => "Adresy",
        Language::DE | Language::NL => "Adressen",
        Language::UK => "Адреси",
        Language::ZH => "网络地址",
        Language::ZH_TW => "網路位址",
        Language::RO => "Adrese",
        Language::KO => "주소",
        Language::TR => "Adresler",
        Language::RU => "Адреса",
        Language::PT => "Endereços",
        Language::EL => "Διευθύνσεις",
        // Language::FA => "نشانی ها",
        Language::SV => "Adresser",
        Language::FI => "Osoitteet",
        Language::JA => "アドレス",
        Language::UZ => "Manzillar",
        Language::VI => "Danh sách địa chỉ",
        Language::ID => "Alamat",
    }
}

pub fn ip_version_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IP version",
        Language::IT => "Versione IP",
        Language::FR => "Version IP",
        Language::ES => "Versión IP",
        Language::PL => "Wersja IP",
        Language::DE => "IP Version",
        Language::UK => "Версія IP",
        Language::ZH => "目标IP协议版本",
        Language::ZH_TW => "IP 版本",
        Language::RO => "Versiune IP",
        Language::KO => "IP 버전",
        Language::TR => "IP versiyonu",
        Language::RU => "Версия IP",
        Language::PT => "Versão de IP",
        Language::EL => "Έκδοση IP",
        // Language::FA => "نسخهٔ IP",
        Language::SV => "IP-version",
        Language::FI => "IP-versio",
        Language::JA => "IP バージョン",
        Language::UZ => "IP versiyasi",
        Language::VI => "Phiên bản IP",
        Language::ID => "Versi IP",
        Language::NL => "IP versie",
    }
}

// pub fn transport_protocol_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Transport protocol",
//         Language::IT => "Protocollo di trasporto",
//         Language::FR => "Protocole de transport",
//         Language::ES | Language::PT => "Protocolo de transporte",
//         Language::PL => "Protokół transportowy",
//         Language::DE => "Netzwerkprotokoll",
//         Language::UK => "Транспортний протокол",
//         Language::ZH => "目标传输协议",
//         Language::ZH_TW => "傳輸通訊協定",
//         Language::RO => "Protocol de transport",
//         Language::KO => "전송 프로토콜",
//         Language::TR => "İletişim protokolü",
//         Language::RU => "Транспортный протокол",
//         Language::EL => "Πρωτόκολλο μεταφοράς",
//         // Language::FA => "پیوندنامهٔ ترابرد",
//         Language::SV => "Transportprotokoll",
//         Language::FI => "Kuljetusprotokolla",
//         Language::JA => "トランスポート プロトコル",
//         Language::UZ => "Transport protokoli",
//         Language::ID => "Protokol berjalan",
//         Language::NL => "Transportprotocol",
//     }
// }

pub fn protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::RO | Language::NL => "Protocol",
        Language::IT => "Protocollo",
        Language::FR => "Protocole",
        Language::ES | Language::PT => "Protocolo",
        Language::PL => "Protokół",
        Language::DE | Language::SV => "Protokoll",
        Language::UK | Language::RU => "Протокол",
        Language::ZH => "协议",
        Language::ZH_TW => "通訊協定",
        Language::KO => "프로토콜",
        Language::TR => "Protokolü",
        Language::EL => "Πρωτόκολλο",
        // Language::FA => "پیوندنامهٔ",
        Language::FI => "Protokolla",
        Language::JA => "プロトコル",
        Language::UZ => "Protokoli",
        Language::VI => "Phương thức",
        Language::ID => "Protokol",
    }
}

pub fn traffic_rate_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Traffic rate",
        Language::IT => "Intensità del traffico",
        Language::FR => "Fréquence du traffic",
        Language::ES => "Tasa de tráfico",
        Language::PL => "Prędkość ruchu",
        Language::DE => "Daten Frequenz",
        Language::UK => "Швидкість руху",
        Language::ZH => "网络速率图",
        Language::ZH_TW => "流量速率",
        Language::RO => "Rata de trafic",
        Language::KO => "트레픽 속도",
        Language::TR => "Trafik oranı",
        Language::RU => "Cкорость трафика",
        Language::PT => "Taxa de tráfego",
        Language::EL => "Ρυθμός κίνησης",
        // Language::FA => "نرخ آمد و شد",
        Language::SV => "Datafrekvens",
        Language::FI => "Liikennemäärä",
        Language::JA => "トラフィック レート",
        Language::UZ => "Trafik tezligi",
        Language::VI => "Lưu lượng truy cập",
        Language::ID => "Tingkat lalulintas",
        Language::NL => "Verkeerssnelheid",
    })
}

// pub fn relevant_connections_translation(language: Language) -> Text<StyleType> {
//     Text::new(match language {
//         Language::EN => "Relevant connections:",
//         Language::IT => "Connessioni rilevanti:",
//         Language::FR => "Connexions pertinentes:",
//         Language::ES => "Conexiones Relevantes:",
//         Language::PL => "Istotne połączenia:",
//         Language::DE => "Relevante Verbindungen:",
//         Language::UK => "Важливі підключення:",
//         Language::ZH => "连接详情:",
//         Language::ZH_TW => "相關連線：",
//         Language::RO => "Conexiuni relevante:",
//         Language::KO => "관련 연결:",
//         Language::TR => "İlgili bağlantılar:",
//         Language::RU => "Важные подключения:",
//         Language::PT => "Conexões relevantes:",
//         Language::EL => "Σχετικές συνδέσεις:",
//         Language::FA => "پیوند های خویشاوند:",
//         Language::SE => "Relevanta anslutningar:",
//         Language::UZ => "Tegishli ulanishlar:",
//         Language::ID => "Koneksi yang berkaitan",
//         Language::NL => "Relevante verbindingen:",
//     })
// }

pub fn settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Settings",
        Language::IT => "Impostazioni",
        Language::FR => "Paramètres",
        Language::ES => "Ajustes",
        Language::PL => "Ustawienia",
        Language::DE => "Einstellungen",
        Language::UK => "Налаштування",
        Language::ZH => "设置",
        Language::RO => "Setări",
        Language::KO => "설정",
        Language::TR => "Ayarlar",
        Language::RU => "Настройки",
        Language::PT => "Configurações",
        Language::EL => "Ρυθμίσεις",
        // Language::FA => "پیکربندی",
        Language::SV => "Inställningar",
        Language::FI => "Asetukset",
        Language::JA | Language::ZH_TW => "設定",
        Language::UZ => "Sozlamalar",
        Language::VI => "Cài đặt",
        Language::ID => "Pengaturan",
        Language::NL => "Instellingen",
    }
}

pub fn yes_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Yes",
        Language::IT => "Sì",
        Language::FR => "Oui",
        Language::ES => "Sí",
        Language::PL => "Tak",
        Language::DE | Language::SV | Language::NL => "Ja",
        Language::UK => "Так",
        Language::ZH | Language::ZH_TW => "是",
        Language::RO => "Da",
        Language::KO => "네",
        Language::TR => "Evet",
        Language::RU => "Да",
        Language::PT => "Sim",
        Language::EL => "Ναι",
        // Language::FA => "بله",
        Language::FI => "Kyllä",
        Language::JA => "はい",
        Language::UZ => "Ha",
        Language::VI => "Chấp nhận",
        Language::ID => "Ya",
    })
}

pub fn ask_quit_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Are you sure you want to quit this analysis?",
        Language::IT => "Sei sicuro di voler interrompere questa analisi?",
        Language::FR => "Êtes-vous sûr de vouloir quitter l'application ?",
        Language::ES => "¿Estás seguro de que quieres dejar este análisis?",
        Language::PL => "Jesteś pewien, że chcesz zakończyć analizę?",
        Language::DE => "Bist du sicher, dass du diese Analyse beenden willst?",
        Language::UK => "Чи справді хочете закінчити аналіз?",
        Language::ZH => "您确定退出当前监控吗?",
        Language::ZH_TW => "您確定要結束目前的分析嗎？",
        Language::RO => "Sunteți sigur că doriți să renunțați la această analiză?",
        Language::KO => "정말로 분석을 종료하겠습니까?",
        Language::TR => "Bu analizden çıkmak istediğine emin misin?",
        Language::RU => "Вы уверены, что хотите выйти из текущего анализа?",
        Language::PT => "Tem a certeza que deseja sair desta análise?",
        Language::EL => "Είστε βέβαιοι ότι θέλετε να τερματίσετε την ανάλυση;",
        // Language::FA => "آیا مطمئن هستید می خواهید از این تحلیل خارج شوید؟",
        Language::SV => "Är du säker på att du vill avsluta analysen?",
        Language::FI => "Haluatko varmasti lopettaa analyysin?",
        Language::JA => "分析を終了しますか？",
        Language::UZ => "Tahlildan chiqishga ishonchingiz komilmi?",
        Language::VI => "Bạn có chắc là muốn thoát phiên phân tích này?",
        Language::ID => "Apa kamu yakin untuk berhenti analisa?",
        Language::NL => "Weet je zeker dat je deze analyse wilt afsluiten?",
    })
}

pub fn quit_analysis_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Quit analysis",
        Language::IT => "Interrompi analisi",
        Language::FR => "Quitter l'analyse",
        Language::ES => "Quitar el análisis",
        Language::PL => "Zakończ analize",
        Language::DE => "Analyse beenden",
        Language::UK => "Закінчити аналіз",
        Language::ZH => "退出监控",
        Language::ZH_TW => "結束分析",
        Language::RO => "Renunță la analiză",
        Language::KO => "분석종료",
        Language::TR => "Analizden çık",
        Language::RU => "Закончить анализ",
        Language::PT => "Sair da análise",
        Language::EL => "Έξοδος ανάλυσης",
        // Language::FA => "خروج از تحلیل",
        Language::SV => "Avsluta analys",
        Language::FI => "Lopeta analyysi",
        Language::JA => "分析の終了",
        Language::UZ => "Tahlildan chiqish",
        Language::VI => "Thoát phiên phân tích",
        Language::ID => "Berhenti analisa",
        Language::NL => "Analyse afsluiten",
    }
}

pub fn ask_clear_all_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Are you sure you want to clear notifications?",
        Language::IT => "Sei sicuro di voler eliminare le notifiche?",
        Language::FR => "Êtes-vous sûr de vouloir effacer les notifications ?",
        Language::ES => "¿Seguro que quieres borrar las notificaciones?",
        Language::PL => "Czy na pewno chcesz wyczyścić powiadomienia?",
        Language::DE => "Bist du sicher, dass du alle Benachrichtigungen löschen willst?",
        Language::UK => "Чи справді хочете видалити всі повідомлення?",
        Language::ZH => "确定清除所有通知?",
        Language::ZH_TW => "您確定要清除所有通知嗎？",
        Language::RO => "Sigur doriți să ștergeți notificările?",
        Language::KO => "알림을 삭제하시겠습니까?",
        Language::TR => "Bildirimleri temizlemek istediğine emin misin?",
        Language::RU => "Вы уверены, что хотите удлить все уведомления?",
        Language::PT => "Tem a certeza que deseja eliminar as notificações?",
        Language::EL => "Είστε βέβαιοι ότι θέλετε να εκκαθαρίσετε τις ειδοποιήσεις;",
        // Language::FA => "آیا مطمئن هستید می خواهید اعلان ها را پاک کنید؟",
        Language::SV => "Är du säker på att du vill radera notifikationerna?",
        Language::FI => "Haluatko varmasti tyhjentää ilmoitukset?",
        Language::JA => "すべての通知を削除します。よろしいですか？",
        Language::UZ => "Haqiqatan ham bildirishnomalarni tozalamoqchimisiz?",
        Language::VI => "Bạn có chắc là muốn xóa các thông báo?",
        Language::ID => "Apa kamu yakin untuk membersihkan notifikasi?",
        Language::NL => "Weet je zeker dat je alle meldingen wilt wissen?",
    })
}

pub fn clear_all_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Clear all",
        Language::IT => "Elimina tutto",
        Language::FR => "Tout effacer",
        Language::ES => "Borrar todo",
        Language::PL => "Wyczyść wszystko",
        Language::DE => "Alle löschen",
        Language::UK => "Видалити все",
        Language::ZH | Language::ZH_TW => "清除所有",
        Language::RO => "Ștergeți tot",
        Language::KO => "모두 지우기",
        Language::TR => "Hepsini temizle",
        Language::RU => "Очистить всё",
        Language::PT => "Limpar tudo",
        Language::EL => "Εκκαθάριση όλων",
        // Language::FA => "پاک کردن همه",
        Language::SV => "Radera alla",
        Language::FI => "Tyhjennä kaikki",
        Language::JA => "すべて削除",
        Language::UZ => "Barchasini tozalash",
        Language::VI => "Xóa tất cả",
        Language::ID => "Bersihkan semua",
        Language::NL => "Alles wissen",
    }
}

pub fn hide_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Hide",
        Language::IT => "Nascondi",
        Language::FR => "Masquer",
        Language::ES => "Ocultar",
        Language::PL => "Ukryj",
        Language::DE => "Verstecken",
        Language::UK => "Заховати",
        Language::ZH => "隐藏",
        Language::ZH_TW => "隱藏",
        Language::RO => "Ascundeți",
        Language::KO => "숨기기",
        Language::TR => "Gizle",
        Language::RU => "Скрыть",
        Language::PT => "Esconder",
        Language::EL => "Κλείσιμο",
        // Language::FA => "پنهان کردن",
        Language::SV => "Göm",
        Language::FI => "Piilota",
        Language::JA => "隠す",
        Language::UZ => "Yashirish",
        Language::VI => "Ẩn",
        Language::ID => "Sembunyikan",
        Language::NL => "Verbergen",
    }
}

pub fn network_adapter_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::VI => "Network adapter",
        Language::IT => "Adattatore di rete",
        Language::FR => "Carte réseau",
        Language::ES => "Adaptador de red",
        Language::PL => "Adapter sieciowy",
        Language::DE => "Netzwerkadapter",
        Language::UK => "Мережевий адаптер",
        Language::ZH => "网络适配器",
        Language::ZH_TW => "網路介面卡",
        Language::RO => "Adaptor de rețea",
        Language::KO => "네트워크 어뎁터",
        Language::TR => "Ağ adaptörü",
        Language::RU => "Сетевой интерфейс",
        Language::PT => "Adaptador de rede",
        Language::EL => "Προσαρμογέας δικτύου",
        // Language::FA => "مبدل شبکه",
        Language::SV => "Nätverksadapter",
        Language::FI => "Verkkosovitin",
        Language::JA => "ネットワーク アダプター",
        Language::UZ => "Tarmoq adapteri",
        Language::ID => "Adapter jaringan",
        Language::NL => "Netwerkadapter",
    }
}

#[allow(clippy::too_many_lines)]
pub fn no_addresses_translation<'a>(language: Language, adapter: &str) -> Text<'a, StyleType> {
    let network_adapter_translation = network_adapter_translation(language);
    Text::new(match language {
        Language::EN => format!(
            "No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 If you are sure you are connected to the internet, try choosing a different adapter."
        ),
        Language::IT => format!(
            "Non è osservabile alcun traffico perché l'adattatore di rete selezionato non ha indirizzi attivi...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Se sei sicuro di essere connesso ad internet, prova a scegliere un adattatore diverso."
        ),
        Language::FR => format!(
            "Aucun trafic ne peut être observé, car la carte réseau que vous avez saisie n'a pas d'adresse...\n\n\
                                {network_adapter_translation} : {adapter}\n\n\
                                Si vous êtes sûr d'être connecté à internet, essayez une autre carte."
        ),
        Language::ES => format!(
            "No se puede observar ningún tráfico porque el adaptador seleccionado no tiene direcciones activas...\n\n\
                                 {network_adapter_translation} : {adapter}\n\n\
                                 Si estás seguro de que estás conectado a Internet, prueba a elegir otro adaptador."
        ),
        Language::PL => format!(
            "Nie można zaobserwować żadnego ruchu, ponieważ wybrany adapter nie ma aktywnych adresów...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Jeśli jesteś pewien, że jesteś podłączony do internetu, spróbuj wybrać inny adapter."
        ),
        Language::DE => format!(
            "Es kann kein Netzwerkverkehr beobachtet werden, weil der Adapter keine aktiven Adressen hat...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Wenn du dir sicher bist, dass du mit dem Internet verbunden bist, probier einen anderen Adapter auszuwählen."
        ),
        Language::UK => format!(
            "Не зафіксовано жодного мережевого руху, тому що вибраний адаптер не має активних адрес... \n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Якщо ви впевнені, що підключені до інтернету, спробуйте вибрати інший адаптер."
        ),
        Language::ZH => format!(
            "您选择的网络适配器当前无活动网络...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                如果您确信您已成功连接互联网, 请尝试选择其他网络适配器."
        ),
        Language::ZH_TW => format!(
            "無法觀察到任何流量，因為您選取的網路介面卡沒有有效的位址...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                如果您確定已連線至網際網路，請嘗試選取其他網路介面卡。"
        ),
        Language::RO => format!(
            "Niciun trafic nu poate fi observat deoarece adaptorul selectat nu are adrese active...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Dacă sunteți sigur că sunteți conectat la internet, încercați să alegeți un alt adaptor."
        ),
        Language::KO => format!(
            "선택한 어댑터에 유효한 주소가 없기 때문에 트래픽을 확인할 수 없습니다...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                인터넷이 연결되어있다면 다른 어댑터로 시도해보세요."
        ),
        Language::TR => format!(
            "Seçtiğiniz adaptör aktif bir adrese sahip olmadığı için hiç bir trafik izlenemez...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Eğer gerçekten internete bağlı olduğunuza eminseniz, başka bir adaptör seçmeyi deneyiniz."
        ),
        Language::RU => format!(
            "Наблюдение за трафиком не возможно, потому что Вы выбрали интерфейс без активного адреса...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Если Вы уверены, что подключены к Интернету, попробуйте выбрать другой интерфейс."
        ),
        Language::PT => format!(
            "Não é possível observar tráfego porque o adaptador que selecionou não tem endereços ativos...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Se tiver a certeza que está ligado à internet, tente escolher um adaptador diferente."
        ),
        Language::EL => format!(
            "Δεν μπορεί να ανιχνευθεί κίνηση επειδή ο προσαρμογέας που επέλεξες δεν έχει ενεργές διευθύνσεις...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Αν είσαι σίγουρος ότι είσαι συνδεδεμένος στο διαδίκτυο, δοκίμασε αν επιλέξεις έναν διαφορετικό προσαρμογέα."
        ),
        // Language::FA => format!("هیچ آمد و شدی قابل مشاهده نیست چون مبدلی که انتخاب کرده اید هیچ نشانی فعالی ندارد...\n\n\
        //                         مبدل شبکه: {adapter}\n\n\
        //                         اگر مطمئن هستید به اینترنت وصل هستید، سعی کنید مبدل متفاوتی را انتخاب کنید."),
        Language::SV => format!(
            "Det går inte att observa någon trafik eftersom den valda adaptern inte har några aktiva adresser ...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Om du är säker att du är ansluten till internet, testa att välja en annan adapter."
        ),
        Language::FI => format!(
            "Liikennettä ei voitu havainnoida, koska valitulla sovittimella ei ole aktiivista osoitetta...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Jos olet varma että sinulla on internet-yhteys, kokeile valita toinen verkkosovitin."
        ),
        Language::JA => format!(
            "選択されたアダプターが有効なアドレスを持っていないため、トラフィックを観測できていません...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                インターネットに接続しているか確認し、別のネットワーク アダプターを試してください。"
        ),
        Language::UZ => format!(
            "Trafik kuzatilmaydi, chunki siz tanlagan adapterda faol manzillar yo'q...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Internetga ulanganingizga ishonchingiz komil bo'lsa, boshqa adapterni tanlashga harakat qiling"
        ),
        Language::VI => format!(
            "Không thể quan sát lưu lượng nào vì adapter mà bạn chọn không địa chỉ hoạt động...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Nếu bạn đã chắc chắn kết nối với internet, hãy thử chọn network adapter khác."
        ),
        Language::ID => format!(
            "Tidak ada sinyal yang bisa dilihat karena adapter yang kamu pilih tidak memiliki alamat yang aktif...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Jika kamu yakin kamu terhubung ke internet, coba untuk memilih adapter lainnya."
        ),
        Language::NL => format!(
            "Er kan geen verkeer worden waargenomen omdat de geselecteerde adapter geen actieve adressen heeft...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Als je zeker weet dat je verbonden bent met het internet, probeer dan een andere adapter te kiezen."
        ),
    })
}

#[allow(clippy::too_many_lines)]
pub fn waiting_translation<'a>(language: Language, adapter: &str) -> Text<'a, StyleType> {
    let network_adapter_translation = network_adapter_translation(language);
    Text::new(match language {
        Language::EN => format!(
            "No traffic has been observed yet. Waiting for network packets...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Are you sure you are connected to the internet and you have selected the correct adapter?"
        ),
        Language::IT => format!(
            "Nessun tipo di traffico è stato osservato finora. Attendo pacchetti di rete...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Sei sicuro di esser connesso ad internet e di aver selezionato l'adattatore corretto?"
        ),
        Language::FR => format!(
            "Aucun trafic n'a été capturé pour le moment. En attente de paquets...\n\n\
                                {network_adapter_translation} : {adapter}\n\n\
                                Êtes-vous sûr d'être connecté à internet et d'avoir selectionné la bonne carte réseau ?"
        ),
        Language::ES => format!(
            "Aún no se ha captado tráfico. Esperando paquetes...\n\n\
                                 {network_adapter_translation} : {adapter}\n\n\
                                 ¿Está seguro de que está conectado a Internet y ha seleccionado la tarjeta de red correcta?"
        ),
        Language::PL => format!(
            "Nie zaobserowano żadnego ruchu sieciowego. Oczekiwanie na pakiety...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Czy na pewno jesteś podłączony do internetu i wybrałeś właściwy adapter?"
        ),
        Language::DE => format!(
            "Noch kein Netzwerkverkehr beobachtet. Warten auf Pakete...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Bist du sicher, dass du mit dem Internet verbunden bist und den richtigen Adapter ausgewählt hast?"
        ),
        Language::UK => format!(
            "Не зафіксовано жодного мережевого руху. Очікування пакетів...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Чи ви дійсно підключені до інтернету і вибрали відповідний мережевий адаптер?"
        ),
        Language::ZH => format!(
            "暂无流量数据. 等待网络活动中......\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 您确信您已成功连接到互联网, 并选择了当前正在使用的的网络适配器吗?"
        ),
        Language::ZH_TW => format!(
            "尚未觀察到任何流量。正在等待網路封包...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 您確定已連線至網際網路，並已選取正確的網路介面卡嗎？"
        ),
        Language::RO => format!(
            "Nu a fost observat încă trafic. Se așteaptă pachetele de rețea...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Ești sigur că ești conectat la internet și ai selectat adaptorul corect?"
        ),
        Language::KO => format!(
            "아직 트래픽이 관찰되지 않았습니다. 네트워크 패킷 대기 중...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                인터넷에 연결되어 있고 올바른 어댑터를 선택하셨습니까?"
        ),
        Language::TR => format!(
            "Henüz bir trafik algılanamadı. Ağ paketleri için bekleniyor...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 İnternete bağlı olduğunuza ve doğru adaptörü seçtiğinize emin misiniz?"
        ),
        Language::RU => format!(
            "Трафик не обнаружен. Ожидаем сетевые пакеты...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Вы уверены, что подключены к Интернету и выбрали правильный интерфейс?"
        ),
        Language::PT => format!(
            "Ainda não foi observado tráfego. Aguardando por pacotes...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Tem a certeza de que está ligado à internet e selecionou o adaptador correto?"
        ),
        Language::EL => format!(
            "Δεν έχει παρατηρηθεί κίνηση μέχρι στιγμής. Ανέμενε για πακέτα δικτύου...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Είσαι σίγουρος ότι είσαι συνδεδεμένος στο διαδίκτυο και ότι έχεις επιλέξει τον σωστό προσαρμογέα;"
        ),
        // Language::FA => format!("هنوز هیچ آمد و شدی مشاهده نشده است. در حال انتظار برای بسته های شبکه...\n\n
        //                         مبدل شبکه: {adapter}\n\n
        //                         آیا مطمئن هستید به اینترنت وصل هستید و مبدل درست را انتخاب کرده اید؟"),
        Language::SV => format!(
            "Ingen trafik har observerats ännu. Väntar på paket ...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Är du säker på att du är ansluten till internet och att du har valt rätt adapter?"
        ),
        Language::FI => format!(
            "Ei vielä havaittua liikennettä. Odotetaan verkkopaketteja...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Onhan sinulla varmasti internet-yhteys ja olet valinnut oikean verkkosovittimen."
        ),
        Language::JA => format!(
            "トラフィックがまだ観測できていません。ネットワーク パケットを待っています...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 インターネットに接続していて、正しいアダプターを選択していますか?"
        ),
        Language::UZ => format!(
            "Hali hech qanday trafik aniqlanmadi. Tarmoq paketlari kutilmoqda...\n\n\
            {network_adapter_translation}: {adapter}\n\n\
            Internetga ulanganingizga va to'g'ri adapterni tanlaganingizga ishonchingiz komilmi?"
        ),
        Language::VI => format!(
            "Chưa có lưu lượng để quan sát. Đang đợi các gói tin...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Bạn có chắc là đã kết nối với internet và đã chọn đúng network adapter?"
        ),
        Language::ID => format!(
            "Tidak ada sinyal yang bisa dipantau. Menunggu paket jaringan...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Apa kamu yakin kamu terhubung ke internet, dan memilih adapter yang benar?"
        ),
        Language::NL => format!(
            "Er is nog geen verkeer waargenomen. Wachten op netwerkpakketten...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Weet je zeker dat je verbonden bent met het internet en de juiste adapter hebt geselecteerd?"
        ),
    })
}

#[allow(clippy::too_many_lines)]
pub fn some_observed_translation<'a>(language: Language, observed: u128) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => format!(
            "Total intercepted packets: {observed}\n\n\
                                 Filtered packets: 0\n\n\
                                 Some packets have been intercepted, but still none has been selected according to the filters you specified..."
        ),
        Language::IT => format!(
            "Totale pacchetti intercettati: {observed}\n\n\
                                 Pacchetti filtrati: 0\n\n\
                                 Alcuni pacchetti sono stati intercettati, ma ancora nessuno è stato selezionato secondo i filtri specificati..."
        ),
        Language::FR => format!(
            "Total des paquets interceptés: {observed}\n\n\
                                 Paquets filtrés: 0\n\n\
                                 Certains paquets ont été interceptés, mais aucun ne satisfait les critères des filtres sélectionnés..."
        ),
        Language::ES => format!(
            "Total de paquetes interceptados: {observed}\n\n\
                                 Paquetes filtrados: 0\n\n\
                                 Se interceptaron algunos paquetes, pero ninguno de ellos cumplía los criterios de los filtros seleccionados..."
        ),
        Language::PL => format!(
            "Suma przechwyconych pakietów: {observed}\n\n\
                                 Przefiltrowane pakiety: 0\n\n\
                                 Niektóre pakiety zostały przechwycone, ale żaden nie został wybrany zgodnie z wskazanymi filtrami..."
        ),
        Language::DE => format!(
            "Anzahl der empfangenen Pakete: {observed}\n\n\
                                 Gefilterte Pakete: 0\n\n\
                                 Ein Paar Pakete wurden empfangen, aber es entsprechen noch keine den gewählten Filtern..."
        ),
        Language::UK => format!(
            "Сума перехоплених пакетів: {observed}\n\n\
                                 Відфільтровані пакети: 0\n\n\
                                 Деякі пакети були перехоплені, але жоден з них не був вибраний відповідно до вказаних фільтрів..."
        ),
        Language::ZH => format!(
            "监测到的数据包总数: {observed}\n\n\
                                 目标数据包总数: 0\n\n\
                                 当前已监测到一些数据包, 但其中并未包含您的目标数据包......"
        ),
        Language::ZH_TW => format!(
            "攔截到的封包總數： {observed}\n\n\
                                 已篩選封包： 0\n\n\
                                 已攔截到部分封包，但尚未有任何封包符合您指定的篩選條件..."
        ),
        Language::RO => format!(
            "Total pachete interceptate: {observed}\n\n\
                                Pachete filtrate: 0\n\n\
                                Unele pachete au fost interceptate, dar încă niciunul nu a fost selectat conform filtrelor pe care le-ați specificat..."
        ),
        Language::KO => format!(
            "감지한 총 패킷: {observed}\n\n\
                                필터링된 패킷: 0\n\n\
                                일부 패킷이 감지되었지만, 지정한 필터에 따라 선택되지 않았습니다..."
        ),
        Language::TR => format!(
            "Toplam yakalanan paketler: {observed}\n\n\
                                 Filterelenen paketler: 0\n\n\
                                 Bazı paketler yakalandı, fakat belirttiğiniz filtrelere göre hiç biri seçilmedi..."
        ),
        Language::RU => format!(
            "Всего пакетов перехвачено: {observed}\n\n\
                                 Фильтровано пакетов: 0\n\n\
                                 Сетевые пакеты были перехвачены, но ни один из них не соответствует заданным фильтрам..."
        ),
        Language::PT => format!(
            "Total de pacotes interceptados: {observed}\n\n\
                                Pacotes filtrados: 0\n\n\
                                Alguns pacotes foram interceptados, mas nenhum deles foi selecionado de acordo com os filtros especificados..."
        ),
        Language::EL => format!(
            "Συνολικά αναχαιτισμένα πακέτα: {observed}\n\n\
                                 Φιλτραρισμένα πακέτα: 0\n\n\
                                 Κάποια από τα πακέτα έχουν αναχαιτιστεί, αλλά κανένα ακόμη δεν έχει επιλεγεί σύμφωνα με τα φίλτρα που επέλεξες..."
        ),
        // Language::FA => format!("مجموع بسته های رهگیری شده: {observed}\n\n\
        //                         بسته های صاف شده: 0\n\n\
        //                         شماری از بسته ها رهگیری شده اند، ولی هنوز هیچ کدام بر اساس صافی تعیین شده شما انتخاب نشده اند..."),
        Language::SV => format!(
            "Antal fångade paket: {observed}\n\n\
                                 Filtrerade paket: 0\n\n\
                                 Några paket har fångats, men än har inget valts enligt de angivna filtren ..."
        ),
        Language::FI => format!(
            "Siepattuja paketteja yhteensä: {observed}\n\n\
                                 Suodatettuja paketteja: 0\n\n\
                                 Joitakin paketteja on siepattu, mutta yhtäkään ei ole valittu määrittämiesi suodattimien mukaan..."
        ),
        Language::JA => format!(
            "取得したパケット数: {observed}\n\n\
                                 フィルター後のパケット数: 0\n\n\
                                 パケットは取得できていますが、設定されたフィルタリングにより表示されません..."
        ),
        Language::UZ => format!(
            "Jami ushlangan paketlar: {observed}\n\n\
            Filtrlangan paketlar: 0\n\n\
            Tarmoq paketlari ushlandi, lekin ularning hech biri belgilangan filtrlarga mos kelmadi..."
        ),
        Language::VI => format!(
            "Tổng số gói tin bị chặn: {observed}\n\n\
                                 Các gói tin đã lọc: 0\n\n\
                                 Một số gói đã bị chặn, nhưng vẫn chưa có gói tin nào được bắt theo bộ lọc bạn đã chọn..."
        ),
        Language::ID => format!(
            "Total paket yang dilacak: {observed}\n\n\
                                 Paket yg difilter: 0\n\n\
                                 Beberapa paket dilacak, tetapi tidak ada yg terlihat berdasarkan filter yang kamu pilih..."
        ),
        Language::NL => format!(
            "Totaal aantal onderschepte pakketten: {observed}\n\n\
                                 Gefilterde pakketten: 0\n\n\
                                 Er zijn enkele pakketten onderschept, maar nog geen enkele is geselecteerd volgens de filters die je hebt opgegeven..."
        ),
    })
}

// pub fn filtered_packets_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Filtered packets",
//         Language::IT => "Pacchetti filtrati",
//         Language::FR => "Paquets filtrés",
//         Language::ES => "Paquetes filtrados",
//         Language::PL => "Przefiltrowane pakiety",
//         Language::DE => "Gefilterte Pakete",
//         Language::UK => "Відфільтровані пакети",
//         Language::ZH => "目标数据包计数",
//         Language::ZH_TW => "已篩選封包",
//         Language::RO => "Pachete filtrate",
//         Language::KO => "필터링된 패킷",
//         Language::TR => "Filtrelenen paketler",
//         Language::RU => "Отфильтровано пакетов",
//         Language::PT => "Pacotes filtrados",
//         Language::EL => "Φιλτραρισμένα πακέτα",
//         // Language::FA => "بسته های صاف شده",
//         Language::SV => "Filtrerade paket",
//         Language::FI => "Suodatettuja paketteja",
//         Language::JA => "フィルタリングされたパケット",
//         Language::UZ => "Filtrlangan paketlar",
//         Language::VI => "Các gói tin đã được lọc",
//         Language::ID => "Paket data tersaring",
//         Language::NL => "Gefilterde pakketten",
//     }
// }

// pub fn filtered_bytes_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Filtered bytes",
//         Language::IT => "Byte filtrati",
//         Language::FR => "Octets filtrés",
//         Language::ES | Language::PT => "Bytes filtrados",
//         Language::PL => "Przechwycone bajty",
//         Language::DE => "Gefilterte Bytes",
//         Language::UK => "Відфільтровані байти",
//         Language::ZH => "目标网络流量计数",
//         Language::ZH_TW => "已篩選位元組",
//         Language::RO => "Octeți filtrați",
//         Language::KO => "필터링된 바이트",
//         Language::TR => "Filtrelenen bayt",
//         Language::RU => "Отфильтровано байт",
//         Language::EL => "Φιλτραρισμένα bytes",
//         // Language::FA => "بایت های صاف شده",
//         Language::SV => "Filtrerade bytes",
//         Language::FI => "Suodatettuja tavuja",
//         Language::JA => "フィルタリングされたバイト",
//         Language::UZ => "Filtrlangan baytlar",
//         Language::VI => "Các bytes đã được lọc",
//         Language::ID => "Bytes tersaring",
//         Language::NL => "Gefilterde bytes",
//     }
// }

// pub fn of_total_translation(language: Language, percentage: &str) -> String {
//     match language {
//         Language::EN => format!("({percentage} of the total)"),
//         Language::IT => format!("({percentage} del totale)"),
//         Language::FR => format!("({percentage} du total)"),
//         Language::ES => format!("({percentage} del total)"),
//         Language::PL => format!("({percentage} z całości)"),
//         Language::DE => format!("({percentage} der Gesamtzahl)"),
//         Language::UK => {
//             format!("({percentage} від загальної суми)")
//         }
//         Language::ZH => {
//             format!("(占所有数据包的 {percentage})")
//         }
//         Language::ZH_TW => {
//             format!("(佔總數的 {percentage})")
//         }
//         Language::RO => {
//             format!("({percentage} din total)")
//         }
//         Language::KO => {
//             format!("({percentage} 의 일부)")
//         }
//         Language::TR => format!("toplamın ({percentage})"),
//         Language::RU => {
//             format!("({percentage} от общего числа)")
//         }
//         Language::PT => {
//             format!("({percentage} do total)")
//         }
//         Language::EL => {
//             format!("({percentage} από τα συνολικά)")
//         }
//         // Language::FA => format!("({percentage} از مجموع)"),
//         Language::SV => format!("({percentage} av totalen)"),
//         Language::FI => format!("({percentage} kokonaismäärästä)"),
//         Language::JA => format!("(トータル: {percentage} )"),
//         Language::UZ => format!("(Jami: {percentage} )"),
//         Language::VI => format!("({percentage} trên tổng cộng)"),
//         Language::ID => format!("({percentage} dari total)"),
//         Language::NL => format!("({percentage} van het totaal)"),
//     }
// }

// pub fn filtered_application_translation(language: Language) -> Text<StyleType> {
//     Text::new(match language {
//         Language::EN => "Filtered packets per application protocol:",
//         Language::IT => "Pacchetti filtrati per protocollo applicativo:",
//         Language::FR => "Paquets filtrés par protocole applicatif:",
//         Language::ES => "Paquetes filtrados por protocolo de aplicación:",
//         Language::PL => "Przefiltrowane pakiety według protokołu aplikacji:",
//         Language::DE => "Gefilterte Pakete je Anwendungs-Protokoll:",
//         Language::UK => "Кількість відфільтрованих пакетів на протокол програми:",
//         Language::ZH => "按应用层协议分类的目标数据包计数:",
//         Language::ZH_TW => "依應用程式通訊協定篩選的封包：",
//         Language::RO => "Pachete filtrate pe protocol de aplicație:",
//         Language::KO => "애플리케이션 프로토콜당 필터링된 패킷 수:",
//         Language::TR => "Uygulama protokolü bazında filtrelenen paketler:",
//         Language::RU => "Отфильтровано пакетов прикладного протокола:",
//         Language::PT => "Pacotes filtrados por protocolo de aplicação:",
//         Language::EL => "Φιλτραρισμένα πακέτα ανά πρωτόκολλο εφαρμογής:",
//         Language::FA => "بسته های صاف شده برای هر پیوندنامهٔ درخواست:",
//         Language::SE => "Filtrerade paket per applikationsprotokoll:",
//         Language::UZ => "Har bir dastur protokoli uchun filtrlangan paketlar:",
//         Language::ID => "Paket data tersaring dari setiap protokol aplikasi:",
//         Language::NL => "Gefilterde pakketten per applicatieprotocol:",
//     })
// }

// pub fn no_favorites_translation(language: Language) -> Text<StyleType> {
//     Text::new(match language {
//         Language::EN => "Nothing to show at the moment.\n\
//                          To add a connection to your favorites, click on the star symbol near the connection.",
//         Language::IT => "Nulla da vedere per il momento.\n\
//                          Per aggiungere una connessione ai tuoi preferiti, clicca sul simbolo della stella vicino alla connessione.",
//         Language::FR => "Rien a voir pour le moment.\n\
//                          Pour ajouter une connexion à vos favoris, cliquez sur l'étoile à côté de la connexion.",
//         Language::ES => "Nada que mostrar por el momento.\n\
//                          Para añadir una conexión a sus favoritos, haga clic en el símbolo de la estrella situado junto a la conexión.",
//         Language::PL => "Nie ma nic do pokazania w tej chwili.\n\
//                          Aby dodać połączenie do ulubionych, kliknij na ikonę 'gwiazdki' obok połączenia.",
//         Language::DE => "Im Moment nichts zu zeigen.\n\
//                          Um eine Verbindung zu deinen Favoriten hinzuzufügen, klick das auf das Stern-Symbol neben der Verbindung.",
//         Language::UK => "Немає, що показати в цей момент.\n\
//                          Щоб додати підключення до улюблених, натисніть на іконку 'зірочки' біля підключення.",
//         Language::ZH => "收藏夹还是空的.\n\
//                          小贴士: 点击连接右侧的小星星即可收藏到这里哦.",
//         Language::ZH_TW => "目前沒有任何內容。\n若要將連線新增至我的最愛，請按一下連線旁的星號圖示。",
//         Language::RO => "Nimic de arătat în acest moment.\n\
//                         Pentru a adăuga o conexiune la favorite, faceți clic pe simbolul stea din apropierea conexiunii.",
//         Language::KO => "현재는 보여줄게 없습니다.\n\
//                          즐겨찾기에 연결을 추가하려면 별을 눌러주세요.",
//         Language::TR => "Şu an gösterecek bir şey yok.\n\
//                          Favorilere bağlantı eklemek için, bağlantı yanındaki yıldız sembolüne tıklayınız.",
//         Language::RU => "Нечего показать в настоящий момент.\n\
//                          Для добавления соединения в избранные, нажмите на символ звезды возле соединения.",
//         Language::PT => "Nada para mostrar de momento.\n\
//                          Para adicionar uma conexão aos seus favoritos, clique na estrela perto da conexão.",
//         Language::EL => "Δεν υπάρχει κάτι για απεικόνιση.\n\
//                          Για να προσθέσεις μια σύνδεση στα αγαπημένα σου, κλίκαρε στο σύμβολο με το αστέρι δίπλα στη σύνδεση.",
//         Language::FA => "در حال حاضر هیچ چیزی برای نمایش نیست.\n\
//                         برای افزودن یک پیوند به پسندیده های خود، روی نشان ستاره کنار پیوند کلیک کنید.",
//         Language::SE => "Inget att visa för tillfälet.\n\
//                          För att lägga till anslutningar till dina favoriter, klicka på stjärnsymbolen nära anslutningen.",
//         Language::UZ => "Ayni paytda ko'rsatiladigan hech narsa yo'q.\n\
//         Ulanishni sevimlilar ro'yhatiga qo'shish uchun ulanish yaqinidagi yulduzcha belgisini bosing."
//         Language::ID => "Tidak ada yang ditampilkan untuk saatini.\n\
//                          untuk menambah koneksi ke favorit, klik dilogo bintang dekat dengan koneksi.",
//        Language::NL => "Niets om te laten zien op dit moment.\n\
//                          Om een verbinding toe te voegen aan je favorieten, klik op het ster symbool naast de verbinding.",
//     })
// }

pub fn error_translation(language: Language, error: &str) -> Text<StyleType> {
    Text::new(match language {
        Language::EN => format!(
            "An error occurred! \n\n\
                                {error}"
        ),
        Language::IT => format!(
            "Si è verificato un errore! \n\n\
                                {error}"
        ),
        Language::FR => format!(
            "Une erreur est survenue! \n\n\
                                {error}"
        ),
        Language::ES => format!(
            "¡Se ha producido un error! \n\n\
                                {error}"
        ),
        Language::PL => format!(
            "Wystąpił błąd! \n\n\
                                {error}"
        ),
        Language::DE => format!(
            "Es ist ein Fehler aufgetreten! \n\n\
                                {error}"
        ),
        Language::UK => format!(
            "Зʼявилась помилка! \n\n\
                                {error}"
        ),
        Language::ZH => format!(
            "发生了一些错误! \n\n\
                                {error}"
        ),
        Language::ZH_TW => format!(
            "發生錯誤！\n\n\
                                {error}"
        ),
        Language::RO => format!(
            "A apărut o eroare! \n\n\
                                {error}"
        ),
        Language::KO => format!(
            "오류가 발생하였습니다! \n\n\
                                {error}"
        ),
        Language::TR => format!(
            "Bir hata oluştu! \n\n\
                                 {error}"
        ),
        Language::RU => format!(
            "Произошла ошибка! \n\n\
                                 {error}"
        ),
        Language::PT => format!(
            "Ocorreu um erro! \n\n\
                                {error}"
        ),
        Language::EL => format!(
            "Κάποιο σφάλμα συνέβη! \n\n\
                                {error}"
        ),
        // Language::FA => format!(
        //     "خطایی رخ داد! \n\n\
        //                         {error}"
        // ),
        Language::SV => format!(
            "Ett fel inträffade! \n\n\
                                {error}"
        ),
        Language::FI => format!(
            "Tapahtui virhe! \n\n\
                                {error}"
        ),
        Language::JA => format!(
            "エラーが発生しました! \n\n\
                                {error}"
        ),
        Language::UZ => format!(
            "Xatolik yuz berdi!\n\n\
                                {error}"
        ),
        Language::VI => format!(
            "Đã có lỗi xảy ra! \n\n\
                                {error}"
        ),
        Language::ID => format!(
            "Terjadi kesalahan! \n\n\
                                {error}"
        ),
        Language::NL => format!(
            "Er is een fout opgetreden! \n\n\
                                {error}"
        ),
    })
}

// pub fn both_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "both",
//         Language::IT => "entrambi",
//         Language::FR => "les deux",
//         Language::ES | Language::PT => "ambos",
//         Language::PL => "oba",
//         Language::DE => "beide",
//         Language::UK => "обидва",
//         Language::ZH => "皆需",
//         Language::ZH_TW => "兩者皆是",
//         Language::RO => "ambele",
//         Language::KO => "둘다",
//         Language::TR => "ikiside",
//         Language::RU => "оба",
//         Language::EL => "αμφότερα",
//         // Language::FA => "هر دو",
//         Language::SV => "båda",
//         Language::FI => "molemmat",
//         Language::JA => "両方",
//         Language::UZ => "ikkalasi ham",
//         Language::ID => "keduanya",
//         Language::NL => "beide",
//     }
// }

// pub fn all_protocols_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "All protocols",
//         Language::IT => "Tutti i protocolli",
//         Language::FR => "Tous les protocoles",
//         Language::ES => "Todos los protocolos",
//         Language::PL => "Wszystkie protokoły",
//         Language::DE => "Alle Protokolle",
//         Language::RU => "Все протоколы",
//         Language::ZH_TW => "所有通訊協定",
//         Language::FA => "همهٔ پیوندنامه ها",
//         Language::SE => "Alla protokoll",
//         Language::UZ => "Barcha protokollar",
//         Language::UK => "Усі протоколи",
//         Language::ID => "Semua protokol",
//         Language::NL => "Alle protocollen",
//     }
// }

// pub fn all_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "All",
//         Language::IT => "Tutti",
//         Language::FR => "Tous",
//         Language::ES | Language::PT => "Todos",
//         Language::PL => "Wszystkie",
//         Language::DE => "Alle",
//         Language::UK => "Усі",
//         Language::ZH => "所有",
//         Language::ZH_TW => "全部",
//         Language::RO => "Toate",
//         Language::KO => "모두",
//         Language::TR => "Hepsi",
//         Language::RU => "Всё",
//         Language::EL => "Όλα",
//         // Language::FA => "همه",
//         Language::SV => "Alla",
//         Language::FI => "Kaikki",
//         Language::JA => "すべて",
//         Language::UZ => "Barchasi",
//         Language::ID => "Semua",
//         Language::NL => "Alle",
//     }
// }

pub fn packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets",
        Language::IT => "pacchetti",
        Language::FR => "paquets",
        Language::ES => "paquetes",
        Language::PL => "pakiety",
        Language::DE => "Pakete",
        Language::UK => "пакети",
        Language::ZH => "数据包",
        Language::ZH_TW => "封包",
        Language::RO => "pachete",
        Language::KO => "패킷",
        Language::TR | Language::SV | Language::ID => "paket",
        Language::RU => "пакетов",
        Language::PT => "pacotes",
        Language::EL => "πακέτα",
        // Language::FA => "بسته ها",
        Language::FI => "paketit",
        Language::JA => "パケット",
        Language::UZ => "paketlar",
        Language::VI => "các gói tin",
        Language::NL => "pakketten",
    }
}

// pub fn packets_chart_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "packets per second",
//         Language::IT => "pacchetti al secondo",
//         Language::FR => "paquets par seconde",
//         Language::ES => "paquetes por segundo",
//         Language::PL => "pakiety na sekundę",
//         Language::DE => "Pakete pro Sekunde",
//         Language::UK => "пакети на секунду",
//         Language::ZH => "数据包",
//         Language::ZH_TW => "每秒封包數",
//         Language::RO => "pachete pe secundă",
//         Language::KO => "초당 패킷",
//         Language::TR => "saniye başı paket",
//         Language::RU => "пакетов в секунду",
//         Language::PT => "pacotes por segundo",
//         Language::EL => "πακέτα ανά δευτερόλεπτο",
//         // Language::FA => "بسته در ثانیه",
//         Language::SV => "paket per sekund",
//         Language::FI => "pakettia sekunnissa",
//         Language::JA => "1 秒あたりのパケット数",
//         Language::UZ => "paket soniyasiga",
//         Language::VI => "gói tin trên giây",
//         Language::ID => "paket per detik",
//         Language::NL => "pakketten per seconde",
//     }
// }

pub fn bytes_translation(language: Language) -> &'static str {
    match language {
        Language::EN
        | Language::IT
        | Language::ES
        | Language::PT
        | Language::EL
        | Language::SV
        | Language::VI
        | Language::ID
        | Language::NL
        | Language::DE => "bytes",
        Language::FR => "octets",
        Language::PL => "bajty",
        Language::UK => "байти",
        Language::ZH => "网络流量",
        Language::ZH_TW => "位元組",
        Language::RO => "octeți",
        Language::KO => "바이트",
        Language::TR | Language::UZ => "bayt",
        Language::RU => "байтов",
        // Language::FA => "بایت ها",
        Language::FI => "tavua",
        Language::JA => "バイト",
    }
}

// pub fn bytes_chart_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "bytes per second",
//         Language::IT => "byte al secondo",
//         Language::FR => "octets par seconde",
//         Language::ES | Language::PT => "bytes por segundo",
//         Language::PL => "bajty na sekundę",
//         Language::DE => "Bytes pro Sekunde",
//         Language::ZH_TW => "每秒位元組",
//         Language::UK => "байти на секунду",
//         Language::ZH => "网络流量",
//         Language::RO => "octeți pe secundă",
//         Language::KO => "초당 바이트",
//         Language::TR => "saniye başı bayt",
//         Language::RU => "байтов в секунду",
//         Language::EL => "bytes ανά δευτερόλεπτο",
//         // Language::FA => "بایت در ثانیه",
//         Language::SV => "bytes per sekund",
//         Language::FI => "tavua sekunnissa",
//         Language::JA => "1 秒あたりのバイト量",
//         Language::UZ => "bayt soniyasiga",
//         Language::VI => "byte trên giây",
//         Language::ID => "bytes per detik",
//         Language::NL => "bytes per seconde",
//     }
// }

// pub fn recent_report_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "most recent",
//         Language::IT => "più recenti",
//         Language::FR => "la plus récente",
//         Language::ES => "más reciente",
//         Language::PL => "najnowsze",
//         Language::DE => "zuletzt",
//         Language::UK => "найновіші",
//         Language::ZH => "按时间",
//         Language::ZH_TW => "最新",
//         Language::RO => "cea mai recentă",
//         Language::KO => "가장 최근",
//         Language::TR => "en son",
//         Language::RU => "новейшие",
//         Language::PT => "mais recente",
//         Language::EL => "πιο πρόσφατα",
//         // Language::FA => "آخرین",
//         Language::SV => "senaste",
//         Language::FI => "viimeisin",
//         Language::JA => "新しい順",
//         Language::UZ => "eng so'nggi",
//         Language::ID => "paling terakhir",
//         Language::NL => "meest recent",
//     }
// }

// pub fn packets_report_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "most packets",
//         Language::IT => "più pacchetti",
//         Language::FR => "le plus de paquets",
//         Language::ES => "mayoría de los paquetes",
//         Language::PL => "najwięcej pakietów",
//         Language::DE => "meiste Pakete",
//         Language::UK => "найбільше пакетів",
//         Language::ZH => "按数据包",
//         Language::ZH_TW => "最多封包",
//         Language::RO => "cele mai multe pachete",
//         Language::KO => "대부분의 패킷",
//         Language::TR => "en çok paket",
//         Language::RU => "больше всего пакетов",
//         Language::PT => "mais pacotes",
//         Language::EL => "περισσότερα πακέτα",
//         // Language::FA => "بیشترین بسته ها",
//         Language::SV => "flest paket",
//         Language::FI => "eniten paketteja",
//         Language::JA => "パケット数の多い順",
//         Language::UZ => "eng ko'p paketlar",
//         Language::ID => "paket terbanyak",
//         Language::NL => "meeste pakketten",
//     }
// }

// pub fn bytes_report_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "most bytes",
//         Language::IT => "più byte",
//         Language::FR => "le plus de données",
//         Language::ES => "mayoría de los bytes",
//         Language::PL => "najwięcej bajtów",
//         Language::DE => "meiste Bytes",
//         Language::UK => "найбільше байтів",
//         Language::ZH => "按流量",
//         Language::ZH_TW => "最多位元組",
//         Language::RO => "cei mai mulți octeți",
//         Language::KO => "대부분의 바이트",
//         Language::TR => "en çok bayt",
//         Language::RU => "больше всего байт",
//         Language::PT => "mais bytes",
//         Language::EL => "περισσότερα bytes",
//         // Language::FA => "بیشترین بایت ها",
//         Language::SV => "flest bytes",
//         Language::FI => "eniten tavuja",
//         Language::JA => "バイト量の多い順",
//         Language::UZ => "eng ko'p bayt",
//         Language::NL => "meeste bytes",
//     }
// }

// pub fn favorite_report_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "favorites",
//         Language::IT => "preferiti",
//         Language::FR => "favoris",
//         Language::ES | Language::PT => "favoritos",
//         Language::PL => "ulubione",
//         Language::DE => "Favoriten",
//         Language::UK => "улюблені",
//         Language::ZH => "收藏夹",
//         Language::ZH_TW => "我的最愛",
//         Language::RO => "favorite",
//         Language::KO => "즐겨찾기",
//         Language::TR => "favoriler",
//         Language::RU => "избранное",
//         Language::EL => "αγαπημένα",
//         Language::FA => "پسندیده ها",
//         Language::SE => "favoriter",
//         Language::UZ => "sevimlilar",
//         Language::ID => "favorit",
//         Language::NL => "favorieten",
//     }
// }

pub fn notifications_title_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Customize your notifications",
        Language::IT => "Personalizza le tue notifiche",
        Language::FR => "Personnalisez vos notifications",
        Language::ES => "Personaliza tus notificaciones",
        Language::PL => "Dostosuj powiadomienia",
        Language::DE => "Stell deine Benachrichtigungen ein",
        Language::UK => "Налаштування повідомлень",
        Language::ZH => "自定义通知",
        Language::ZH_TW => "自訂通知",
        Language::RO => "Personalizați-vă notificările",
        Language::KO => "사용자 지정 알림",
        Language::TR => "Bildirimlerinizi özelleştirin",
        Language::RU => "Настройка уведомлений",
        Language::PT => "Personalize as suas notificações",
        Language::EL => "Εξατομίκευση ειδοποιήσεων",
        // Language::FA => "اعلان های خود را سفارشی کنید",
        Language::SV => "Anpassa dina notifikationer",
        Language::FI => "Muokkaa ilmoituksiasi",
        Language::JA => "通知のカスタマイズ",
        Language::UZ => "Bildirishnomalaringizni sozlang",
        Language::VI => "Tùy chỉnh thông báo của bạn",
        Language::ID => "Sesuaikan notifikasi Anda",
        Language::NL => "Pas je meldingen aan",
    })
}

pub fn appearance_title_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => "Choose your favorite theme",
        Language::IT => "Scegli il tuo tema preferito",
        Language::FR => "Sélectionnez votre thème préféré",
        Language::ES => "Elige tu tema favorito",
        Language::PL => "Wybierz swój ulubiony motyw",
        Language::DE => "Wähl dein Lieblingsdesign",
        Language::UK => "Виберіть улюблену тему",
        Language::ZH_TW => "選擇您偏好的主題",
        Language::ZH => "选择您喜欢的主题",
        Language::RO => "Selectați tema preferată",
        Language::KO => "태마를 선택하세요",
        Language::TR => "Favori temanızı seçin",
        Language::RU => "Выберите предпочительную тему",
        Language::PT => "Escolha o seu tema favorito",
        Language::EL => "Επιλέξτε το αγαπημένο σας θέμα",
        // Language::FA => "زمینه دلخواه خود را انتخاب کنید",
        Language::SV => "Välj ditt favorittema",
        Language::FI => "Valitse suosikkiteemasi",
        Language::JA => "テーマを選択してください",
        Language::UZ => "Sevimli mavzuingizni tanlang",
        Language::VI => "Chọn chủ đề bạn muốn",
        Language::ID => "Pilih tema favorit Kamu",
        Language::NL => "Kies je favoriete thema",
    })
}

pub fn active_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Active filters",
        Language::IT => "Filtri attivi",
        Language::FR => "Filtres actifs",
        Language::ES => "Filtros activos",
        Language::PL => "Aktywne filtry",
        Language::DE => "Aktive Filter",
        Language::UK => "Активні фільтри",
        Language::ZH => "活动的过滤器",
        Language::ZH_TW => "啟用的篩選器",
        Language::RO => "Filtre active",
        Language::KO => "활성화된 필터",
        Language::TR => "Aktif filtreler",
        Language::RU => "Выбранные фильтры",
        Language::PT => "Filtros ativos",
        Language::EL => "Ενεργά φίλτρα",
        // Language::FA => "صافی های فعال",
        Language::SV => "Aktiva filter",
        Language::FI => "Aktiiviset suodattimet",
        Language::JA => "適用されているフィルター",
        Language::UZ => "Faol filtrlar",
        Language::VI => "Bộ lọc đang hoạt động",
        Language::ID => "Filter aktif",
        Language::NL => "Actieve filters",
    }
}

pub fn none_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "none",
        Language::IT => "nessuno",
        Language::FR => "aucun",
        Language::ES => "ninguno",
        Language::PL => "brak",
        Language::DE => "Keine",
        Language::UK => "бракує",
        Language::ZH_TW => "無",
        Language::ZH => "无",
        Language::RO => "niciunul",
        Language::KO => "없음",
        Language::TR => "hiç biri",
        Language::RU => "ничего",
        Language::PT => "nenhum",
        Language::EL => "κανένα",
        // Language::FA => "هیچ کدام",
        Language::SV => "inga",
        Language::FI => "ei mitään",
        Language::JA => "なし",
        Language::UZ => "hech biri",
        Language::VI => "không có",
        Language::ID => "Tidak ada",
        Language::NL => "geen",
    }
}

// pub fn yeti_night_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Sniffnet's original dark theme",
//         Language::IT => "Il tema scuro originale di Sniffnet",
//         Language::FR => "Thème original sombre de Sniffnet",
//         Language::ES => "Tema oscuro original de Sniffnet",
//         Language::PL => "Oryginalny, ciemny motyw Sniffnet",
//         Language::DE => "Sniffnets urspüngliches, dunkles Design",
//         Language::UK => "Оригінальний, темний стиль Sniffnet-у",
//         Language::ZH_TW => "Sniffnet 原始深色主題",
//         Language::ZH => "Sniffnet暗黑",
//         Language::RO => "Tema întunecată originală Sniffnet",
//         Language::KO => "Sniffnet의 기본 다크테마",
//         Language::TR => "Sniffnet'in orjinal koyu teması",
//         Language::RU => "Оригинальная тёмная тема Sniffnet'а",
//         Language::PT => "Tema escuro original de Sniffnet",
//         Language::EL => "Το αυθεντικό σκούρο θέμα του Sniffnet",
//         // Language::FA => "زمینه تاریک اصلی Sniffnet",
//         Language::SV => "Sniffnets ursprungliga mörka tema",
//         Language::FI => "Sniffnetin alkuperäinen tumma teema",
//         Language::JA => "Sniffnet のオリジナル テーマ",
//         Language::UZ => "Sniffnet-ning asl qora mavzusi",
//         Language::VI => "Chủ đề tối của Sniffnet",
//         Language::ID => "Tema gelap bawaan Sniffnet",
//         Language::NL => "Sniffnet's originele donkere thema",
//     }
// }

// pub fn yeti_day_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Sniffnet's original light theme",
//         Language::IT => "Il tema chiaro originale di Sniffnet",
//         Language::FR => "Thème original clair de Sniffnet",
//         Language::ES | Language::PT => "Tema claro original de Sniffnet",
//         Language::PL => "Oryginalny, jasny motyw Sniffnet",
//         Language::DE => "Sniffnets urspüngliches, helles Design",
//         Language::UK => "Оригінальний, світлий стиль Sniffnet-у",
//         Language::ZH_TW => "Sniffnet 原始淺色主題",
//         Language::ZH => "Sniffnet浅色",
//         Language::RO => "Tema deschisă originală Sniffnet",
//         Language::KO => "Sniffnet의 기본 라이트테마",
//         Language::TR => "Sniffnet'in orjinal açık teması",
//         Language::RU => "Оригинальная светая тема Sniffnet'а",
//         Language::EL => "Το αυθεντικό ανοιχτόχρωμο θέμα του Sniffnet",
//         // Language::FA => "زمینه روشن اصلی Sniffnet",
//         Language::SV => "Sniffnets ursprungliga ljusa tema",
//         Language::FI => "Sniffnetin alkuperäinen vaalea teema",
//         Language::JA => "Sniffnet のオリジナル ライト テーマ",
//         Language::UZ => "Sniffnet-ning asl oq mavzusi",
//         Language::VI => "Chủ đề sáng của Sniffnet",
//         Language::ID => "Tema terang bawaan Sniffnet",
//         Language::NL => "Sniffnet's originele lichte thema",
//     }
// }

// pub fn deep_sea_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "To dive into network traffic",
//         Language::IT => "Per immergersi nel traffico di rete",
//         Language::FR => "Pour plonger dans votre trafic réseau",
//         Language::ES => "Para sumergirse en el tráfico de la red",
//         Language::PL => "Aby zanurzyć się w ruchu sieciowym",
//         Language::DE => "Um in den Netzwerkverkehr einzutauchen",
//         Language::UK => "Проаналізувати мережевий рух",
//         Language::ZH_TW => "深入探索網路流量",
//         Language::ZH => "潜入网络活动的海洋",
//         Language::RO => "Pentru a vă scufunda în traficul de rețea",
//         Language::KO => "네트워크 트레픽으로 바로가기",
//         Language::TR => "Ağ trafiğine dalmak",
//         Language::RU => "Для погружения в сетевой трафик",
//         Language::PT => "Para mergulhar no tráfego de rede",
//         Language::EL => "Βουτιά μέσα στην κίνηση του δικτύου",
//         // Language::FA => "شیرجه رفتن در آمد و شد شبکه",
//         Language::SV => "För att dyka ned i nätverkstrafiken",
//         Language::FI => "Sukeltaaksesi verkkoliikenteeseen",
//         Language::JA => "ネットワーク トラフィックにダイブ",
//         Language::UZ => "Tarmoq trafigiga qo'shilish uchun",
//         Language::VI => "Đắm chìm vào lưu lượng mạng",
//         Language::ID => "Untuk mendalami lalu lintas jaringan",
//         Language::NL => "Om in het netwerkverkeer te duiken",
//     }
// }

// pub fn mon_amour_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Lovely theme made for dreamers",
//         Language::IT => "Tema incantevole fatto per i sognatori",
//         Language::FR => "Thème romantique fait pour les rêveurs",
//         Language::ES => "Tema encantador hecho para soñadores",
//         Language::PL => "Uroczy motyw stworzony dla marzycieli",
//         Language::DE => "Schönes Design für Träumer",
//         Language::UK => "Прекрасна тема для мрійників",
//         Language::ZH_TW => "為夢想家打造的可愛主題",
//         Language::ZH => "梦想家的主题",
//         Language::RO => "O temă minunată creată pentru visători",
//         Language::KO => "사랑스러운 몽환가들을 위한 테마",
//         Language::TR => "Hayal perestler için yapılmış güzel tema",
//         Language::RU => "Милая тема для мечтателей",
//         Language::PT => "Tema encantador feito para sonhadores",
//         Language::EL => "Φτιαγμένο για ονειροπόλους",
//         // Language::FA => "زمینه دلپذیر ساخته شده برای رویا پردازان",
//         Language::SV => "Ljuvligt tema gjort för drömmare",
//         Language::FI => "Ihana teema unelmoijille",
//         Language::JA => "ドリーマーのためのテーマ",
//         Language::UZ => "Xayolparastlar uchun chiroyli mavzu",
//         Language::VI => "Chủ đề mộng mơ cho những kẻ mơ mộng",
//         Language::ID => "Tema yang indah dibuat untuk para pemimpi",
//         Language::NL => "Liefelijk thema gemaakt voor dromers",
//     }
// }

pub fn incoming_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming",
        Language::IT => "In entrata",
        Language::FR => "Entrant",
        Language::ES => "Entrante",
        Language::PL => "Przychodzące",
        Language::DE => "Eingehend",
        Language::UK => "Вхідні",
        Language::ZH | Language::ZH_TW => "入站",
        Language::RO => "De intrare",
        Language::KO => "수신중",
        Language::TR => "Gelen",
        Language::RU => "Входящий",
        Language::PT => "Entrando",
        Language::EL => "Εισερχόμενα",
        // Language::FA => "ورودی",
        Language::SV => "Inkommande",
        Language::FI => "Saapuva",
        Language::JA => "受信",
        Language::UZ => "Kiruvchi",
        Language::VI => "Đang tới",
        Language::ID => "Masuk",
        Language::NL => "Inkomend",
    }
}

pub fn outgoing_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Outgoing",
        Language::IT => "In uscita",
        Language::FR => "Sortant",
        Language::ES => "Saliente",
        Language::PL => "Wychodzące",
        Language::DE => "Ausgehend",
        Language::UK => "Вихідні",
        Language::ZH | Language::ZH_TW => "出站",
        Language::RO => "De ieșire",
        Language::KO => "발신중",
        Language::TR => "Giden",
        Language::RU => "Исходящий",
        Language::PT => "Saindo",
        Language::EL => "Εξερχόμενα",
        // Language::FA => "خروجی",
        Language::SV => "Utgående",
        Language::FI => "Lähtevä",
        Language::JA => "送信",
        Language::UZ => "Chiquvchi",
        Language::VI => "Đang hướng ra ngoài",
        Language::ID => "Keluar",
        Language::NL => "Uitgaand",
    }
}

pub fn notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Notifications",
        Language::IT => "Notifiche",
        Language::ES => "Notificaciones",
        Language::PL => "Powiadomienia",
        Language::DE => "Benachrichtigungen",
        Language::UK => "Повідомлення",
        Language::ZH | Language::JA | Language::ZH_TW => "通知",
        Language::RO => "Notificări",
        Language::KO => "알림",
        Language::TR => "Bildirimler",
        Language::RU => "Уведомления",
        Language::PT => "Notificações",
        Language::EL => "Ειδοποιήσεις",
        // Language::FA => "اعلان ها",
        Language::SV => "Notifikationer",
        Language::FI => "Ilmoitukset",
        Language::UZ => "Bildirishnomalar",
        Language::VI => "Thông báo",
        Language::ID => "Pemberitahuan",
        Language::NL => "Notificaties",
    }
}

pub fn style_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Style",
        Language::IT => "Stile",
        Language::ES | Language::PT => "Estilo",
        Language::PL => "Styl",
        Language::RO | Language::TR | Language::SV => "Stil",
        Language::DE => "Design",
        Language::UK | Language::RU => "Стиль",
        Language::ZH => "主题",
        Language::ZH_TW => "樣式",
        Language::KO => "스타일",
        Language::EL => "Στυλ",
        // Language::FA => "شیوه",
        Language::FI => "Tyyli",
        Language::JA => "スタイル",
        Language::UZ => "Uslub",
        Language::VI => "Chủ đề",
        Language::ID => "Gaya",
        Language::NL => "Stijl",
    }
}

pub fn language_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Language",
        Language::IT => "Lingua",
        Language::FR => "Langue",
        Language::ES => "Idioma",
        Language::PL => "Język",
        Language::DE => "Sprache",
        Language::UK => "Мова",
        Language::ZH => "语言",
        Language::ZH_TW => "語言",
        Language::RO => "Limbă",
        Language::KO => "언어",
        Language::TR => "Dil",
        Language::RU => "Язык",
        Language::PT => "Língua",
        Language::EL => "Γλώσσα",
        // Language::FA => "زبان",
        Language::SV => "Språk",
        Language::FI => "Kieli",
        Language::JA => "表示言語",
        Language::UZ => "Til",
        Language::VI => "Ngôn ngữ",
        Language::ID => "Bahasa",
        Language::NL => "Taal",
    }
}

pub fn overview_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Overview",
        Language::IT => "Panoramica",
        Language::FR => "Résumé",
        Language::ES => "Resumen",
        Language::PL => "Przegląd",
        Language::DE => "Übersicht",
        Language::UK => "Огляд",
        Language::ZH => "概览",
        Language::ZH_TW => "概覽",
        Language::RO => "Prezentare generală",
        Language::KO => "개요",
        Language::TR => "Ön izleme",
        Language::RU => "Обзор",
        Language::PT => "Visão geral",
        Language::EL => "Επισκόπηση",
        // Language::FA => "نمای کلی",
        Language::SV => "Översikt",
        Language::FI => "Yleiskatsaus",
        Language::JA => "概要",
        Language::UZ => "Ko'rib chiqish",
        Language::VI => "Tổng quan",
        Language::ID => "Ringkasan",
        Language::NL => "Overzicht",
    }
}

// pub fn packets_threshold_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Notify me when a packets threshold is exceeded",
//         Language::IT => "Notificami quando una soglia di pacchetti è superata",
//         Language::FR => "Me notifier lorsqu'un seuil de paquet est atteint",
//         Language::ES => "Notificarme cuando se supere un límite de paquetes",
//         Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg pakietów",
//         Language::DE => "Benachrichtige mich, wenn die Pakete eine Schwelle überschreiten",
//         Language::UK => "Повідом мене про перевищення ліміту пакетів",
//         Language::ZH => "超过设定的数据包数量阈值时通知我",
//         Language::ZH_TW => "當封包數量超過閾值時通知我",
//         Language::RO => "Anunță-mă când este depășit un prag de pachete",
//         Language::KO => "패킷 임계값을 초과하면 알림",
//         Language::TR => "Paket eşiği aşıldığında beni bilgilendir",
//         Language::RU => "Уведомить, когда порог по частоте пакетов превышен",
//         Language::PT => "Notifique-me quando um limite de pacotes for excedido",
//         Language::EL => "Ειδοποίησέ με όταν το όριο τον πακέτων ξεπεραστεί",
//         // Language::FA => "به من اطلاع بده وقتی آستانه یک بسته فراتر رفت",
//         Language::SV => "Notifiera mig när en paketgräns har överstigits",
//         Language::FI => "Ilmoita minulle, kun pakettiraja on ylittynyt",
//         Language::JA => "パケット数の閾値を超過した場合に通知する",
//         Language::UZ => "Paket chegarasi oshib ketganda xabar bering",
//         Language::VI => "Báo cho tôi biết khi vượt quá ngưỡng gói tin",
//         Language::ID => "Beritahu saya ketika ambang batas paket terlampaui",
//         Language::NL => "Geef me een melding wanneer een pakketdrempel is overschreden",
//     }
// }

// pub fn bytes_threshold_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Notify me when a bytes threshold is exceeded",
//         Language::IT => "Notificami quando una soglia di byte è superata",
//         Language::FR => "Me notifier lorsqu'un seuil de donnée est atteint",
//         Language::ES => "Notificarme cuando se exceda un límite de bytes",
//         Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg bajtów",
//         Language::DE => "Benachrichtige mich, wenn die Bytes eine Schwelle überschreiten",
//         Language::UK => "Повідом мене про перевищення ліміту байтів",
//         Language::ZH => "超过设定的网络流量阈值时通知我",
//         Language::ZH_TW => "當位元組數超過閾值時通知我",
//         Language::RO => "Anunță-mă când este depășit un prag de octeți",
//         Language::KO => "바이트 임계값을 초과하면 알림",
//         Language::TR => "Bayt eşiği aşıldığında beni bilgilendir",
//         Language::RU => "Уведомить, когда порог в байтах превышен",
//         Language::PT => "Notifique-me quando um limite de bytes for excedido",
//         Language::EL => "Ειδοποίησέ με όταν το όριο των bytes ξεπεραστεί",
//         // Language::FA => "به من اطلاع بده وقتی آستانه یک بایت فراتر رفت",
//         Language::SV => "Notifiera mig när en gräns för bytes har överstigits",
//         Language::FI => "Ilmoita minulle, kun tavuraja on ylittynyt",
//         Language::JA => "バイト量の閾値を超過した場合に通知する",
//         Language::UZ => "Bayt chegarasi oshib ketganda menga xabar bering",
//         Language::VI => "Báo cho tôi biết khi vượt quá ngưỡng bytes",
//         Language::ID => "Beritahu saya ketika ambang batas bytes terlampaui",
//         Language::NL => "Geef me een melding wanneer een byte-drempel is overschreden",
//     }
// }

pub fn per_second_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "(per second)",
        Language::IT => "(al secondo)",
        Language::FR => "(par seconde)",
        Language::ES | Language::PT => "(por segundo)",
        Language::PL => "(na sekundę)",
        Language::DE => "(pro Sekunde)",
        Language::UK => "(на секунду)",
        Language::ZH | Language::JA | Language::ZH_TW => "(每秒)",
        Language::RO => "(pe secundă)",
        Language::KO => "(초당)",
        Language::TR => "(her saniye)",
        Language::RU => "(в секунду)",
        Language::EL => "(ανά δευτερόλεπτο)",
        // Language::FA => "(در ثانیه)",
        Language::SV => "(per sekund)",
        Language::FI => "(sekunnissa)",
        Language::UZ => "(soniyasiga)",
        Language::VI => "(trên giây)",
        Language::ID => "(per detik)",
        Language::NL => "(per seconde)",
    }
}

// pub fn specify_multiples_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "you can also specify 'K', 'M' and 'G'",
//         Language::IT => "puoi anche specificare 'K', 'M' e 'G'",
//         Language::FR => "vous pouvez également spécifier 'K', 'M' et 'G'",
//         Language::ES => "también puede especificar 'K', 'M' y 'G'",
//         Language::PL => "możesz również określić 'K', 'M' i 'G'",
//         Language::DE => "du kannst auch 'K', 'M' und 'G' verwenden",
//         Language::UK => "можете також вибрати 'K', 'M' i 'G'",
//         Language::ZH => "您可指定 'K', 'M', 'G'",
//         Language::ZH_TW => "您也可指定 'K'、'M' 和 'G' 等單位",
//         Language::RO => "puteți specifica 'K', 'M', 'G'",
//         Language::KO => "지정 가능합니다 'K', 'M', 'G'",
//         Language::TR => "şunları da kullanabilirsin 'K', 'M' ve 'G'",
//         Language::RU => "Так же можно указать 'K', 'M' или 'G'",
//         Language::PT => "também pode especificar 'K', 'M' e 'G'",
//         Language::EL => "μπορείς επίσης να καθορίσεις τα 'K', 'M' και 'G'",
//         // Language::FA => "؛ شما همچنین می توانید 'M'، 'K' و 'G' را تعیین کنید",
//         Language::SV => "du kan också ange 'K', 'M' och 'G'",
//         Language::FI => "voit myös määrittää 'K', 'M' tai 'G'",
//         Language::JA => "'K', 'M', 'G' が選択可能です",
//         Language::UZ => "'K', 'M' va 'G' ni ham belgilashingiz mumkin",
//         Language::VI => "bạn cũng có thể chọn 'K', 'M' and 'G'",
//         Language::ID => "Anda juga dapat menentukan 'K', 'M' dan 'G'",
//         Language::NL => "je kunt ook 'K', 'M' en 'G' specificeren",
//     }
// }

// pub fn favorite_notification_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Notify me when new data are exchanged from my favorites",
//         Language::IT => "Notificami quando nuovi dati sono scambiati dai miei preferiti",
//         Language::FR => "Notifiez-moi lorsque des données sont échangées depuis mes favoris",
//         Language::ES => "Notificarme cuando se intercambien nuevos datos de mis favoritos",
//         Language::PL => "Powiadom mnie, gdy nowe dane z moich ulubionych zostaną wymienione",
//         Language::DE => {
//             "Benachrichtige mich, wenn neue Daten mit meinen Favoriten ausgetauscht werden"
//         }
//         Language::UK => "Повідом мене, коли буде обмін даними з моїх улюблених",
//         Language::ZH => "收藏夹内的连接有新活动时通知我",
//         Language::ZH_TW => "當我的最愛中的連線有新的資料交換時通知我",
//         Language::RO => "Anunță-mă când sunt transferate date noi de la favoritele mele",
//         Language::KO => "즐겨찾기에서 새 데이터가 교환될 때 알림",
//         Language::TR => "Favorilerimde veri akışı olduğunda beni uyar",
//         Language::RU => "Уведомить, если произошёл обмен данными в соединениях из избранного",
//         Language::PT => "Notificar-me quando novos dados forem trocados dos meus favoritos",
//         Language::EL => "Ειδοποίησέ με όταν νέα δεδομένα έχουν ανταλλαγεί από τα αγαπημένα μου",
//         // Language::FA => "به من اطلاع بده وقتی داده جدید از پسندیده های من مبادله شد",
//         Language::SV => "Notifiera mig när ny data utbyts av mina favoriter",
//         Language::FI => "Ilmoita minulle, kun suosikkini vaihtavat uusia tietoja",
//         Language::JA => "お気に入りに指定したホストに関してデータ送受信があった場合に通知する",
//         Language::UZ => "Sevimlilar ro'yhatidan yangi ma'lumotlar almashganda xabar bering",
//         Language::VI => "Báo cho tôi biết khi dữ liệu mới được trao đổi từ mục yêu thích của tôi",
//         Language::ID => "Beritahu saya ketika data baru dipertukarkan dari favorit saya",
//         Language::NL => "Geef me een melding wanneer nieuwe gegevens worden uitgewisseld van mijn favorieten",
//     }
// }

pub fn threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Threshold",
        Language::IT => "Soglia",
        Language::FR => "Seuil",
        Language::ES => "Límite",
        Language::PL => "Próg",
        Language::DE => "Schwellenwert",
        Language::UK => "Ліміт",
        Language::ZH => "阈值",
        Language::ZH_TW => "閾值",
        Language::RO => "Prag",
        Language::KO => "임계값",
        Language::TR => "Eşik",
        Language::RU => "Порог",
        Language::PT => "Limite",
        Language::EL => "όριο",
        // Language::FA => "آستانه",
        Language::SV => "Gräns",
        Language::FI => "Raja",
        Language::JA => "閾値",
        Language::UZ => "Eshik",
        Language::VI => "Ngưỡng",
        Language::ID => "Ambang batas",
        Language::NL => "Grens",
    }
}

pub fn volume_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT | Language::FR | Language::PT | Language::NL => "Volume",
        Language::ES => "Volumen",
        Language::PL => "Głośność",
        Language::DE => "Lautstärke",
        Language::UK => "Гучність",
        Language::ZH | Language::JA => "通知音量",
        Language::ZH_TW => "音量",
        Language::RO => "Volum",
        Language::KO => "볼륨",
        Language::TR => "Ses",
        Language::RU => "Объём",
        Language::EL => "Ένταση",
        // Language::FA => "حجم",
        Language::SV => "Volym",
        Language::FI => "Äänenvoimakkuus",
        Language::UZ => "Ovoz balandligi",
        Language::VI => "Âm lượng",
        Language::ID => "Bunyi",
    }
}

pub fn sound_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sound",
        Language::IT => "Suono",
        Language::FR => "Son",
        Language::ES => "Sonido",
        Language::PL => "Dźwięk",
        Language::DE => "Ton",
        Language::UK | Language::RU => "Звук",
        Language::ZH | Language::JA => "通知音",
        Language::ZH_TW => "音效",
        Language::RO => "Sunet",
        Language::KO => "사운드",
        Language::TR => "Ses",
        Language::PT => "Som",
        Language::EL => "Ήχος",
        // Language::FA => "صدا",
        Language::SV => "Ljud",
        Language::FI => "Ääni",
        Language::UZ => "Ovoz",
        Language::VI => "Âm thanh",
        Language::ID => "Suara",
        Language::NL => "Geluid",
    }
}

pub fn bytes_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bytes threshold exceeded",
        Language::IT => "Soglia di Byte superata",
        Language::FR => "Seuil de donnée atteint",
        Language::ES => "¡Límite de bytes superado",
        Language::PL => "Próg bajtów przekroczony",
        Language::DE => "Byte-Schwellenwert überschritten",
        Language::UK => "Ліміт байтів перевищено",
        Language::ZH => "达到设定的网络流量阈值",
        Language::ZH_TW => "已超過位元組閾值",
        Language::RO => "Prag de octeți depășit",
        Language::KO => "바이트 임계값 초과",
        Language::TR => "Bayt eşik değeri aşıldı",
        Language::RU => "Порог в байтах превышен",
        Language::PT => "Limite de bytes excedido",
        Language::EL => "Το όριο των bytes ξεπεράστηκε",
        // Language::FA => "آستانه بایت فراتر رفت",
        Language::SV => "Gräns för bytes överskriden",
        Language::FI => "Tavuraja ylitetty",
        Language::JA => "バイト量の閾値を調査しました",
        Language::UZ => "Bayt chegarasidan oshib ketdi",
        Language::VI => "Bytes đã vượt ngưỡng",
        Language::ID => "Ambang batas byte terlampaui",
        Language::NL => "Byte-drempel overschreden",
    }
}

// pub fn bytes_exceeded_value_translation(language: Language, value: &str) -> String {
//     match language {
//         Language::EN => format!("{value} have been exchanged"),
//         Language::IT => format!("{value} sono stati scambiati"),
//         Language::FR => format!("{value} ont été échangé"),
//         Language::ES => format!("{value} han sido intercambiado/s"),
//         Language::PL => format!("Wymieniono {value}"),
//         Language::DE => format!("{value} wurden ausgetauscht"),
//         Language::UK => format!("{value} було обміняно"),
//         Language::ZH => format!("已交换字节 {value}"),
//         Language::ZH_TW => format!("已交換 {value} 位元組"),
//         Language::RO => format!("au fost transferați {value}"),
//         Language::KO => format!("바이트 {value} 가 교환되었습니다"),
//         Language::TR => format!("{value} aktarıldı"),
//         Language::RU => format!("{value} обмена информацией"),
//         Language::PT => format!("Foram trocados {value}"),
//         Language::EL => format!("{value} έχουν ανταλλαγεί"),
//         // Language::FA => format!("{value} بایت مبادله شده است"),
//         Language::SV => format!("{value} har utbytts"),
//         Language::FI => format!("{value} on vaihdettu"),
//         Language::JA => format!("{value} の送受信が発生しました"),
//         Language::UZ => format!("{value} ma'lumot almashinuvi"),
//         Language::VI => format!("{value} đã được trao đổi"),
//         Language::ID => format!("{value} telah dipertukarkan"),
//         Language::NL => format!("{value} zijn uitgewisseld"),
//     }
// }

pub fn packets_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Packets threshold exceeded",
        Language::IT => "Soglia di pacchetti superata",
        Language::FR => "Le seuil de paquet a été atteint",
        Language::ES => "¡Se ha superado el límite de paquetes",
        Language::PL => "Przekroczono próg pakietów",
        Language::DE => "Paket-Schwellenwert überschritten",
        Language::UK => "Ліміт пакетів перевищено",
        Language::ZH => "达到设定的数据包数量阈值",
        Language::ZH_TW => "已超過封包閾值",
        Language::RO => "Prag de pachete depășit",
        Language::KO => "패킷 임계값 초과",
        Language::TR => "Paket eşik değeri aşıldı",
        Language::RU => "Порог по числу пакетов превышен",
        Language::PT => "Limite de pacotes excedido",
        Language::EL => "Το όριο των πακέτων ξεπεράστηκε",
        // Language::FA => "آستانه بسته فراتر رفت",
        Language::SV => "Paketgräns överskriden",
        Language::FI => "Pakettiraja ylitetty",
        Language::JA => "パケット数のしきい値を超過しました",
        Language::UZ => "Paket chegarasidan oshib ketdi",
        Language::VI => "Gói tin đã vượt ngưỡng",
        Language::ID => "Ambang batas paket terlampaui",
        Language::NL => "Pakketdrempel overschreden",
    }
}

// pub fn packets_exceeded_value_translation(language: Language, value: u32) -> String {
//     match language {
//         Language::EN => match value {
//             1 => "1 packet has been exchanged".to_owned(),
//             npackets => format!("{npackets} packets have been exchanged"),
//         },
//         Language::IT => match value {
//             1 => "1 pacchetto è stato scambiato".to_owned(),
//             npackets => format!("{npackets} pacchetti sono stati scambiati"),
//         },
//         Language::FR => match value {
//             1 => "1 paquet a été échangé".to_owned(),
//             npackets => format!("{npackets} paquets ont été échangés"),
//         },
//         Language::ES => format!("{value} paquete/s han sido intercambiado/s"),
//         Language::PL => format!("Wymieniono {value} pakietów"),
//         Language::DE => match value {
//             1 => "1 Paket wurde ausgetauscht".to_owned(),
//             npackets => format!("{npackets} Pakete wurden ausgetauscht"),
//         },
//         Language::UK => format!("Обміняно {value} пакетів"),
//         Language::ZH => format!("已交换数据包 {value}"),
//         Language::ZH_TW => format!("已交換 {value} 個封包"),
//         Language::RO => format!("au fost transferate {value} pachete"),
//         Language::KO => format!("패킷 {value} 가 교환되었습니다"),
//         Language::TR => format!("{value} paket aktarıldı"),
//         Language::RU => format!("{value} пакет(ов) обмена информацией"),
//         Language::PT => match value {
//             1 => "Foi trocado 1 pacote".to_owned(),
//             npackets => format!("Foram trocados {npackets} pacotes"),
//         },
//         Language::EL => match value {
//             1 => "1 πακέτο έχει ανταλλαγεί".to_owned(),
//             npackets => format!("{npackets} πακέτα έχουν ανταλλαγεί"),
//         },
//         // Language::FA => format!("{value} بسته مبادله شده است"),
//         Language::SV => match value {
//             1 => "1 paket har utbytts".to_owned(),
//             npackets => format!("{npackets} paket har utbytts"),
//         },
//         Language::FI => match value {
//             1 => "1 paketti vaihdettu".to_owned(),
//             npackets => format!("{npackets} pakettia vaihdettu"),
//         },
//         Language::JA => format!("{value} パケットの送受信が発生しました"),
//         Language::UZ => format!("{value} paket uzatildi"),
//         Language::VI => format!("{value} gói tin đã được trao đổi"),
//         Language::ID => format!("{value} paket telah dipertukarkan"),
//         Language::NL => format!("{value} pakketten zijn uitgewisseld"),
//     }
// }

pub fn favorite_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from favorites",
        Language::IT => "Nuovi dati scambiati dai preferiti",
        Language::FR => "Nouvel échange de donnée depuis un favori",
        Language::ES => "¡Nuevos datos intercambiados de favoritos",
        Language::PL => "Nowe dane wymienione z ulubionych",
        Language::DE => "Neue Daten mit deinen Favoriten ausgetauscht",
        Language::UK => "Нові дані обміняно з улюблених",
        Language::ZH => "收藏夹内的连接有新活动",
        Language::ZH_TW => "我的最愛中有新的資料交換",
        Language::RO => "Date noi transferate de la favorite",
        Language::KO => "즐겨찾기에서 새 데이터 교환",
        Language::TR => "Favorilerden yeni veri aktarıldı",
        Language::RU => "Новый обмен данными в избранных соедиениях",
        Language::PT => "Novos dados trocados dos favoritos",
        Language::EL => "Νέα δεδομένα έχουν ανταλλαγεί στα αγαπημένα",
        // Language::FA => "مبادله داده جدید از پسندیده ها",
        Language::SV => "Ny data utbytt av favoriter",
        Language::FI => "Uusia tietoja vaihdettu suosikeista",
        Language::JA => "お気に入りのホストで新しいデータ送受信が発生しました",
        Language::UZ => "Sevimli ulanishlar ro'yhatida yangi ma'lumotlar almashinuvi",
        Language::VI => "Mục ưa thích vừa có trao đổi",
        Language::ID => "Data baru dipertukarkan dari favorit",
        Language::NL => "Nieuwe gegevens uitgewisseld van favorieten",
    }
}

#[allow(clippy::too_many_lines)]
pub fn no_notifications_set_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => {
            "You haven't enabled notifications yet!\n\n\
                                 After enabling them, this page will display a log of your notifications\n\n\
                                 You can enable notifications from settings:"
        }
        Language::IT => {
            "Non hai ancora abilitato le notifiche!\n\n\
                                Dopo che le avrai abilitate, questa pagina mostrerà una collezione delle tue notifiche\n\n\
                                Puoi abilitare le notifiche dalle impostazioni:"
        }
        Language::FR => {
            "Vous n'avez pas activé les notifications!\n\n\
                                    Une fois activées, cette page affichera le journal des notifications\n\n\
                                    Vous pouvez les activer dans les paramètres:"
        }
        Language::ES => {
            "¡Aún no has activado las notificaciones!\n\n\
                                 Después de activarlas, esta página mostrará un registro de sus notificaciones\n\n\
                                 Puedes activar las notificaciones desde los ajustes:"
        }
        Language::PL => {
            "Nie włączyłeś jeszcze powiadomień!\n\n\
                                 Po ich włączeniu, ta strona wyświetli dziennik twoich powiadomień\n\n\
                                 Możesz włączyć powiadomienia w ustawieniach:"
        }
        Language::DE => {
            "Benachrichtigungen wurden noch nicht aktiviert!\n\n\
                         Nachdem du sie aktiviert hast, wird diese Seite eine Liste deiner Benachrichtigungen anzeigen\n\n\
                         Du kannst die Benachrichtigungen in den Einstellungen aktivieren:"
        }
        Language::UK => {
            "Повідомлення не активовані!\n\n\
                                 Після їх активації на цій сторінці побачите список своїх повідомлень\n\n\
                                 Можете вимкнути повідомлення в налаштуваннях:"
        }
        Language::ZH => {
            "您还没有设定任何通知!\n\n\
                                 启用它们后，此页面将显示您的通知日志\n\n\
                                 您可以从设置中设定:"
        }
        Language::ZH_TW => {
            "您尚未啟用任何通知！\n\n\
                                啟用通知後，此頁面將會顯示您的通知紀錄。\n\n\
                                您可以從「設定」中啟用通知："
        }
        Language::RO => {
            "Încă nu ați activat notificările!\n\n\
                                 După ce le veți activa, această pagină va afișa un jurnal al notificărilor dvs\n\n\
                                 Puteți activa notificările din setări:"
        }
        Language::KO => {
            "아직 알림을 활성화하지 않았습니다!\n\n\
                                 활성화로 설정하면 이 페이지에 알림 로그가 표시됩니다\n\n\
                                 설정에서 알림을 활성화할 수 있습니다:"
        }
        Language::TR => {
            "Henüz bildirimleri etkinleştirmedin!\n\n\
                         Etkinleştirdikten sonra bu sayfada bildirimlerine ait kütüğü görebilirsin\n\n\
                         Bildirimleri, ayarlardan etkinleştirebilirsin:"
        }
        Language::RU => {
            "Уведомления пока не настроены!\n\n\
                         После настройки, эта страница будет показывать журнал уведомлений\n\n\
                         Вы можете включить уведомления в настройках:"
        }
        Language::PT => {
            "Ainda não ativou as notificações!\n\n\
                                Depois de ativá-las, esta página irá mostrar um registo das suas notificações\n\n\
                                Pode ativar as notificações nas definições:"
        }
        Language::EL => {
            "Δεν έχετε ενεργοποιήσει τις ειδοποιήσεις ακόμη!\n\n\
                                 Αφότου τις ενεργοποιήσετε, αυτή η σελίδα θα εμφανίσει ένα αρχείο καταγραφής των ειδοποιήσεών σας\n\n\
                                 Μπορείτε να ενεργοποιήσετε τις ειδοποιήσεις από τις ρυθμίσεις:"
        }
        // Language::FA => "شما هنوز اعلان ها را فعال نکرده اید!\n\n\
        //                          پس از آنکه آن ها را فعال کنید، این صفحه یک کارنامه از اعلان های شما را نمایش خواهد داد\n\n
        //                          شما می توانید اعلان ها را از پیکربندی فعال کنید:",
        Language::SV => {
            "Du har inte aktiverat notifikationer än!\n\n\
                                 Efter att du aktiverat dem så kommer denna sida att visa en logg av dina notifikationer\n\n\
                                 Du kan aktivera notifikationer i inställingarna"
        }
        Language::FI => {
            "Et ole vielä ottanut ilmoituksia käyttöön!\n\n\
                                 Kun olet ottanut ne käyttöön, tällä sivulla näkyy loki ilmoituksistasi\n\n\
                                 Voit ottaa ilmoitukset käyttöön asetuksista:"
        }
        Language::JA => {
            "まだ通知を有効にしていません!\n\n\
                                 有効化後、このページ上で通知のログが確認できます。\n\n\
                                 通知設定は設定画面から変更可能です:"
        }
        Language::UZ => {
            "Siz hali bildirishnomalarni yoqmagansiz!\n\n\
                        Ularni faollashtirgandan so'ng, bu sahifada bildirishnomalaringiz jurnali ko'rsatiladi\n\n\
                        Sozlamalardan bildirishnomalarni yoqishingiz mumkin:"
        }
        Language::VI => {
            "Bạn chưa bật tính năng thông báo!\n\n\
                         au khi bật, trang này sẽ hiển thị thông báo\n\n\
                         Hãy bật thông báo trong phần cài đặt:"
        }
        Language::ID => {
            "Anda belum mengaktifkan notifikasi!\n\n\
                                 Setelah mengaktifkannya, halaman ini akan menampilkan log notifikasi Anda\n\n\
                                 Anda dapat mengaktifkan notifikasi dari pengaturan:"
        }
        Language::NL => {
            "Je hebt nog geen meldingen ingeschakeld!\n\n\
                                 Nadat je ze hebt ingeschakeld, zal deze pagina een logboek van je meldingen weergeven\n\n\
                                 Je kunt meldingen inschakelen vanuit de instellingen:"
        }
    })
}

pub fn no_notifications_received_translation<'a>(language: Language) -> Text<'a, StyleType> {
    Text::new(match language {
        Language::EN => {
            "Nothing to see at the moment...\n\n\
                                 When you receive a notification, it will be displayed here"
        }
        Language::IT => {
            "Nulla da vedere al momento...\n\n\
                                Quando riceverai una notifica, essa verrà mostrata qui"
        }
        Language::FR => {
            "Rien à voir pour le moment...\n\n\
                                 Lorsque vous recevrez une notification, elle s'affichera ici"
        }
        Language::ES => {
            "Nada que ver por el momento...\n\n\
                                 Cuando reciba una notificación, aparecerá aquí"
        }
        Language::PL => {
            "Nic do wyświetlenia w tej chwili...\n\n\
                                 Gdy otrzymasz powiadomienie, pojawi się ono tutaj"
        }
        Language::DE => {
            "Im Moment nichts zu sehen...\n\n\
                                 Wenn du eine Benachrichtigung erhälst, wird sie hier angezeigt"
        }
        Language::UK => {
            "Немає що показати у цей момент...\n\n\
                                 Коли отримаєте повідомлення, побачите його тут"
        }
        Language::ZH => {
            "还没有任何通知...\n\n\
                                 当您收到通知时，它会显示在这里"
        }
        Language::ZH_TW => {
            "目前沒有任何通知...\n\n\
                                當您收到通知時，將會顯示在這裡。"
        }
        Language::RO => {
            "Nimic de văzut momentan...\n\n\
                                 Când veți primi o notificare, aceasta va fi afișată aici"
        }
        Language::KO => {
            "현재는 볼 것이 없습니다...\n\n\
                                 알림을 받으면 여기에 표시됩니다"
        }
        Language::TR => {
            "Şu an görecek bir şey yok...\n\n\
                         Bildirim aldığınız zaman burada gözükecektir"
        }
        Language::RU => {
            "Нечего показывать в текущий момент...\n\n\
                                Когда придут уведомления, они будут показаны тут"
        }
        Language::PT => {
            "Nada para ver neste momento...\n\n\
                                Quando receber uma notificação, ela será mostrada aqui"
        }
        Language::EL => {
            "Δεν υπάρχουν ειδοποιήσεις αυτή τη στιγμή...\n\n\
                                 Όταν λάβετε μια ειδοποίηση, θα εμφανιστεί εδώ"
        }
        // Language::FA => {
        //     "در حال حاضر هیچ چیزی برای دیدن نیست...\n\n\
        //                          وقتی شما اعلانی دریافت می کنید، در اینجا نمایش داده خواهد شد"
        // }
        Language::SV => {
            "Inget att se för tillfället ...\n\n\
                                 När du tar emot en notifikation så kommer den att visas här"
        }
        Language::FI => {
            "Ei mitään nähtävää tällä hetkellä...\n\n\
                                 Kun saat ilmoituksen, se näkyy tässä"
        }
        Language::JA => {
            "通知はまだ何もありません...\n\n\
                                 通知があると、ここに表示されます"
        }
        Language::UZ => {
            "Ayni paytda ko'rsatiladigan hech narsa yo'q...\n\n\
                                Bildirishnomalar kelganda, ular shu yerda ko'rsatiladi"
        }
        Language::VI => {
            "Hiện tại không có gì để quan sát...\n\n\
                                 Khi có thông báo, chúng sẽ hiển thị ở đây"
        }
        Language::ID => {
            "Tidak ada yang bisa dilihat saat ini...\n\n\
                                 Saat Anda menerima pemberitahuan, akan ditampilkan di sini"
        }
        Language::NL => {
            "Er is momenteel niets te zien...\n\n\
                                 Wanneer je een melding ontvangt, wordt deze hier weergegeven"
        }
    })
}

pub fn only_last_30_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the last 30 notifications are displayed",
        Language::IT => "Solo le ultime 30 notifiche sono mostrate",
        Language::FR => "Seulement les 30 dernières notifications sont affichées",
        Language::ES => "Sólo se muestran las últimas 30 notificaciones",
        Language::PL => "Wyświetlane jest tylko 30 ostatnich powiadomień",
        Language::DE => "Nur die letzten 30 Benachrichtigungen werden angezeigt",
        Language::UK => "Можете побачити лише 30 останніх повідомлень",
        Language::ZH => "仅显示最近 30 条通知",
        Language::ZH_TW => "僅顯示最近 30 則通知",
        Language::RO => "Sunt afișate doar ultimele 30 de notificări",
        Language::KO => "최근 30개의 알림만 표시됩니다",
        Language::TR => "Sadece son 30 bildirim gösterilmektedir",
        Language::RU => "Показываются только последние 30 уведомлений",
        Language::PT => "São mostradas apenas as últimas 30 notificações",
        Language::EL => "Μόνο οι τελευταίες 30 ειδοποιήσεις απεικονίζονται",
        // Language::FA => "تنها ۳۰ اعلان آخر نمایش داده شده اند",
        Language::SV => "Endast de senaste 30 notifikationerna visas",
        Language::FI => "Vain viimeiset 30 ilmoitusta näytetään",
        Language::JA => "最新の通知 30 件のみ表示されます",
        Language::UZ => "Faqat oxirgi 30 ta bildirishnoma ko'rsatiladi",
        Language::VI => "Chỉ có 30 thông báo gần nhất được hiển thị",
        Language::ID => "Hanya 30 notifikasi terakhir yang ditampilkan",
        Language::NL => "Alleen de laatste 30 meldingen worden weergegeven",
    }
}
