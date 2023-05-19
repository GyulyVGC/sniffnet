#![allow(clippy::match_same_arms)]

use crate::Language;

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
        Language::FA => "بازرسی",
        _ => "Inspect",
    }
}

pub fn connection_details_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Connection details",
        Language::IT => "Dettagli della connessione",
        Language::FA => "مشخصات اتصال",
        _ => "Connection details",
    }
}

pub fn dropped_packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Dropped packets",
        Language::IT => "Pacchetti mancati",
        Language::FA => "بسته های رها شده",
        _ => "Dropped packets",
    }
}

pub fn data_representation_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data representation",
        Language::IT => "Rappresentazione dei dati",
        Language::FA => "بازنمایی داده ها", // TODO: or نمایندگی داده ها depending on context
        _ => "Data representation",
    }
}

pub fn host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Network host",
        Language::IT => "Host di rete",
        Language::FA => "میزبان شبکه",
        _ => "Network host",
    }
}

pub fn only_top_30_hosts_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the top 30 hosts are displayed here",
        Language::IT => "Solo i maggiori 30 host sono mostrati qui",
        Language::FA => "تنها ۳۰ میزبان برتر در اینجا نمایش داده شده اند",
        _ => "Only the top 30 hosts are displayed here",
    }
}

pub fn sort_by_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sort by",
        Language::IT => "Ordina per",
        Language::FA => "مرتب سازی بر اساس",
        _ => "Sort by",
    }
}

pub fn local_translation(language: Language) -> String {
    match language {
        Language::EN => "Local network",
        Language::IT => "Rete locale",
        Language::FA => "شبکه محلی",
        _ => "Local network",
    }
    .to_string()
}

pub fn unknown_translation(language: Language) -> String {
    match language {
        Language::EN => "Unknown location",
        Language::IT => "Localizzazione sconosciuta",
        Language::FA => "محل نامعلوم",
        _ => "Unknown location",
    }
    .to_string()
}

pub fn your_network_adapter_translation(language: Language) -> String {
    match language {
        Language::EN => "Your network adapter",
        Language::IT => "La tua scheda di rete",
        Language::FA => "مبدل شبکه شما",
        _ => "Your network adapter",
    }
    .to_string()
}

pub fn socket_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Socket address",
        Language::IT => "Indirizzo del socket",
        Language::FA => "پریز شبکه",
        _ => "Socket address",
    }
}

pub fn mac_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "MAC address",
        Language::IT => "Indirizzo MAC",
        Language::FA => "آدرس MAC",
        _ => "MAC address",
    }
}

pub fn source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Source",
        Language::IT => "Sorgente",
        Language::FA => "منبع",
        _ => "Source",
    }
}

pub fn destination_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Destination",
        Language::IT => "Destinazione",
        Language::FA => "مقصد",
        _ => "Destination",
    }
}

pub fn fqdn_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Fully qualified domain name",
        Language::IT => "Nome di dominio completo",
        Language::FA => "نام دامنه جامع الشرایط",
        _ => "Fully qualified domain name",
    }
}

pub fn administrative_entity_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Administrative entity",
        Language::IT => "Entità amministrativa",
        Language::FA => "واحد اجرایی", // TODO: or واحد اداری depending on context
        _ => "Administrative entity",
    }
}

pub fn transmitted_data_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transmitted data",
        Language::IT => "Dati trasmessi",
        Language::FA => "دادهٔ منتقل شده",
        _ => "Transmitted data",
    }
}

pub fn country_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Country",
        Language::IT => "Paese",
        Language::FA => "کشور",
        _ => "Country",
    }
}

pub fn domain_name_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Domain name",
        Language::IT => "Nome di dominio",
        Language::FA => "نام دامنه",
        _ => "Domain name",
    }
}

pub fn only_show_favorites_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show favorites",
        Language::IT => "Mostra solo i preferiti",
        Language::FA => "فقط پسندیده ها را نمایش بده",
        _ => "Only show favorites",
    }
}

pub fn search_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Search filters",
        Language::IT => "Filtri di ricerca",
        Language::FA => "صافی های جستجو",
        _ => "Search filters",
    }
}

pub fn no_search_results_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No result available according to the specified search filters",
        Language::IT => "Nessun risultato disponibile secondo i filtri di ricerca specificati",
        Language::FA => "هیچ نتیجه ای بر اساس صافی های جستجوی تعیین شده وجود ندارد",
        _ => "No result available according to the specified search filters",
    }
}

pub fn showing_results_translation(
    language: Language,
    start: usize,
    end: usize,
    total: usize,
) -> String {
    match language {
        Language::EN => format!("Showing {start}-{end} of {total} total results"),
        Language::IT => format!("Sono mostrati {start}-{end} di {total} risultati totali"),
        Language::FA => format!("نمایش {start}-{end} از تمامی {total} نتیجه"),
        _ => format!("Showing {start}-{end} of {total} total results"),
    }
}

#[allow(dead_code)]
pub fn color_gradients_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Apply color gradients",
        Language::IT => "Applica sfumature di colore",
        Language::FA => "اعمال گرادیان های رنگ",
        _ => "Apply color gradients",
    }
}
