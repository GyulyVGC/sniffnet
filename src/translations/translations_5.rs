#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::CS => "Filtr provozu",
        // Language::FA => "فیلتر ترافیک",
        Language::IT => "Filtra il traffico",
        Language::DE => "Datenverkehr filtern",
        Language::ZH => "筛选流量",
        Language::ZH_TW => "篩選流量",
        Language::TR => "Trafiği filtrele",
        Language::JA => "トラフィックをフィルタリング",
        Language::ES => "Filtrar tráfico",
        Language::RO => "Filtrează traficul",
        Language::ID => "Filter lalulintas data",
        Language::FR => "Filtrer le traffic",
        Language::UK => "Фільтр трафіку",
        Language::SV => "Filtrera trafik",
        Language::EL => "Φιλτράρισμα ροής",
        Language::HU => "Forgalom szűrése",
        _ => "Filter traffic",
    }
    .to_string()
}

// the source from which Sniffnet reads network traffic (i.e., a capture file or a network adapter)
pub fn traffic_source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Traffic source",
        Language::CS => "Zdroj provozu",
        // Language::FA => "منبع ترافیک",
        Language::IT => "Fonte del traffico",
        Language::DE => "Datenquelle",
        Language::ZH => "流量来源",
        Language::ZH_TW => "流量來源",
        Language::TR => "Trafik kaynağı",
        Language::JA => "トラフィック元",
        Language::ES => "Fuente de tráfico",
        Language::RO => "Sursa traficului",
        Language::ID => "Asal lalulintas data",
        Language::FR => "Source du traffic",
        Language::UK => "Джерело трафіку",
        Language::SV => "Trafikkälla",
        Language::EL => "Πηγή ροής",
        Language::HU => "Forgalom forrása",
        _ => "Traffic source",
    }
}

pub fn remote_notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Remote notifications",
        Language::IT => "Notifiche remote",
        // Language::FA => "اعلان‌های راه دور",
        Language::DE => "Remote-Benachrichtigungen",
        Language::ZH => "远程通知",
        Language::ZH_TW => "遠端通知",
        Language::TR => "Uzak bildirimler",
        Language::JA => "リモート通知",
        Language::ES => "Notificaciones remotas",
        Language::RO => "Notificări la distanță",
        Language::ID => "Pemberitahuan jarak jauh",
        Language::FR => "Notifications distantes",
        Language::UK => "Віддалені сповіщення",
        Language::SV => "Fjärrnotiser",
        Language::EL => "Απομακρυσμένες ειδοποιήσεις",
        Language::HU => "Távoli értesítések",
        _ => "Remote notifications",
    }
}

pub fn ip_blacklist_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IP blacklist",
        Language::IT => "Blacklist IP",
        // Language::FA => "لیست سیاه IP",
        Language::DE => "IP-Blacklist",
        Language::ZH => "IP 黑名单",
        Language::ZH_TW => "IP 黑名單",
        Language::TR => "IP kara listesi",
        Language::JA => "IP ブラックリスト",
        Language::ES => "Blacklist de IPs",
        Language::RO => "Blacklist IP-uri",
        Language::ID => "Daftar hitam IP",
        Language::FR => "Blacklist d'IP",
        Language::UK => "Чорний список IP-адрес",
        Language::SV => "IP-svartlista",
        Language::EL => "Λίστα μπλοκαρισμένων διευθύνσεων",
        Language::HU => "IP feketelista",
        _ => "IP blacklist",
    }
}

pub fn blacklisted_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from a blacklisted IP",
        Language::IT => "Nuovi dati scambiati da un IP in blacklist",
        // Language::FA => "داده‌های جدیدی از یک IP موجود در لیست سیاه مبادله شد",
        Language::DE => "Neue Daten von einer IP auf der Blacklist ausgetauscht",
        Language::ZH => "与黑名单 IP 交换的新数据",
        Language::ZH_TW => "與黑名單 IP 交換的新資料",
        Language::TR => "Kara listedeki bir IP ile veri alışverişi yapıldı",
        Language::JA => "ブラックリストに登録されたIPから新しいデータが交換されました",
        Language::ES => "Nuevos datos intercambiados con una IP en la blacklist",
        Language::RO => "Noi date schimbate de la un IP în blacklist",
        Language::ID => "Data baru didapat dari daftar hitam IP",
        Language::FR => "Nouvelles données échangées depuis une IP blacklistée",
        Language::UK => "Отримано нові дані з IP-адреси з чорного списку",
        Language::SV => "Ny data utbytt från en svartlistad IP-adress",
        Language::EL => "Νέα δεδομένα ανταλλαχθηκαν από μια μπλοκαρισμένη διεύθυνση",
        Language::HU => "Új adatforgalom egy feketelistán lévő IP-vel",
        _ => "New data exchanged from a blacklisted IP",
    }
}

pub fn only_show_blacklisted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show blacklisted",
        Language::IT => "Mostra solo in blacklist",
        // Language::FA => "فقط موارد لیست سیاه را نمایش بده",
        Language::DE => "Zeige nur auf der Blacklist Stehende",
        Language::ZH => "仅显示黑名单",
        Language::ZH_TW => "僅顯示黑名單",
        Language::TR => "Sadece kara listedekileri göster",
        Language::JA => "ブラックリストのみ表示",
        Language::ES => "Mostrar solo blacklist",
        Language::RO => "Afișează doar blacklist",
        Language::ID => "Hanya tampilkan daftar hitam",
        Language::FR => "Montrer seulement les blacklistées",
        Language::UK => "Показувати лише заблоковані",
        Language::SV => "Visa endast svartlistade",
        Language::EL => "Εμφάνιση μόνο μπλοκαρισμένων διευθύνσεων",
        Language::HU => "Csak feketelistán lévők mutatása",
        _ => "Only show blacklisted",
    }
}

pub fn program_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::TR | Language::RO | Language::ID | Language::SV | Language::HU => {
            "Program"
        }
        Language::IT => "Programma",
        // Language::FA => "برنامه",
        Language::DE => "Programm",
        Language::ZH => "程序",
        Language::ZH_TW => "程式",
        Language::JA => "プログラム",
        Language::ES => "Programa",
        Language::FR => "Programme",
        Language::UK => "Програма",
        Language::EL => "Πρόγραμμα",
        _ => "Program",
    }
}

pub fn no_favorites_saved_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No favorites saved yet",
        Language::IT => "Nessun preferito salvato",
        // Language::FA => "هنوز هیچ علاقه‌مندی ذخیره نشده است",
        Language::DE => "Noch keine Favoriten gespeichert",
        Language::ZH => "尚未保存任何收藏",
        Language::ZH_TW => "尚未儲存任何我的最愛",
        Language::TR => "Henüz kaydedilmiş favori yok",
        Language::JA => "お気に入りはまだ保存されていません",
        Language::ES => "Aún no hay favoritos guardados",
        Language::RO => "Niciun favorit salvat încă",
        Language::ID => "Belum ada favorit yang tersimpan",
        Language::FR => "Aucun favoris enregistrés pour le moment",
        Language::UK => "Ще немає збережених улюблених",
        Language::SV => "Inga favoriter sparade ännu",
        Language::EL => "Δεν έχει αποθηκευτεί κανένα αγαπημένο στοιχείο ακόμη",
        Language::HU => "Még nincsenek elmentve kedvencek",
        _ => "No favorites saved yet",
    }
}
