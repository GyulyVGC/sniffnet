#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn filter_traffic_translation(language: Language) -> String {
    match language {
        Language::EN => "Filter traffic",
        Language::CS => "Filtr provozu",
        Language::IT => "Filtra il traffico",
        Language::ZH => "筛选流量",
        Language::ZH_TW => "篩選流量",
        Language::TR => "Trafiği filtrele",
        Language::JA => "トラフィックをフィルタリング",
        Language::ES => "Filtrar tráfico",
        Language::RO => "Filtrează traficul",
        _ => "Filter traffic",
    }
    .to_string()
}

// the source from which Sniffnet reads network traffic (i.e., a capture file or a network adapter)
pub fn traffic_source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Traffic source",
        Language::CS => "Zdroj provozu",
        Language::IT => "Fonte del traffico",
        Language::ZH => "流量来源",
        Language::ZH_TW => "流量來源",
        Language::TR => "Trafik kaynağı",
        Language::JA => "トラフィック元",
        Language::ES => "Fuente de tráfico",
        Language::RO => "Sursa traficului",
        _ => "Traffic source",
    }
}

pub fn remote_notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Remote notifications",
        Language::IT => "Notifiche remote",
        Language::ZH => "远程通知",
        Language::ZH_TW => "遠端通知",
        Language::TR => "Uzak bildirimler",
        Language::JA => "リモート通知",
        Language::ES => "Notificaciones remotas",
        Language::RO => "Notificări la distanță",
        _ => "Remote notifications",
    }
}

pub fn ip_blacklist_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IP blacklist",
        Language::IT => "Blacklist IP",
        Language::ZH => "IP 黑名单",
        Language::ZH_TW => "IP 黑名單",
        Language::TR => "IP kara listesi",
        Language::JA => "IP ブラックリスト",
        Language::ES => "Blacklist de IPs",
        Language::RO => "Blacklist IP-uri",
        _ => "IP blacklist",
    }
}

pub fn blacklisted_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from a blacklisted IP",
        Language::IT => "Nuovi dati scambiati da un IP in blacklist",
        Language::ZH => "与黑名单 IP 交换的新数据",
        Language::ZH_TW => "與黑名單 IP 交換的新資料",
        Language::TR => "Kara listedeki bir IP ile veri alışverişi yapıldı",
        Language::JA => "ブラックリストに登録されたIPから新しいデータが交換されました",
        Language::ES => "Nuevos datos intercambiados con una IP en la blacklist",
        Language::RO => "Noi date schimbate de la un IP în blacklist",
        _ => "New data exchanged from a blacklisted IP",
    }
}

pub fn only_show_blacklisted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show blacklisted",
        Language::IT => "Mostra solo in blacklist",
        Language::ZH => "仅显示黑名单",
        Language::ZH_TW => "僅顯示黑名單",
        Language::TR => "Sadece kara listedekileri göster",
        Language::JA => "ブラックリストのみ表示",
        Language::ES => "Mostrar solo blacklist",
        Language::RO => "Afișează doar blacklist",
        _ => "Only show blacklisted",
    }
}

pub fn program_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::TR | Language::RO => "Program",
        Language::IT => "Programma",
        Language::ZH => "程序",
        Language::ZH_TW => "程式",
        Language::JA => "プログラム",
        Language::ES => "Programa",
        _ => "Program",
    }
}

pub fn no_favorites_saved_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No favorites saved yet",
        Language::IT => "Nessun preferito salvato",
        Language::ZH => "尚未保存任何收藏",
        Language::ZH_TW => "尚未儲存任何我的最愛",
        Language::TR => "Henüz kaydedilmiş favori yok",
        Language::JA => "お気に入りはまだ保存されていません",
        Language::ES => "Aún no hay favoritos guardados",
        Language::RO => "Niciun favorit salvat încă",
        _ => "No favorites saved yet",
    }
}
