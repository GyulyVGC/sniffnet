#![allow(clippy::match_same_arms)]

use crate::Language;

pub fn new_version_available_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "A newer version is available on GitHub",
        Language::IT => "Una versione più recente è disponibile su GitHub",
        Language::RU => "Новая версия доступна на GitHub",
        Language::EL => "Μια νεότερη έκδοση είναι διαθέσιμη στο GitHub",
        Language::FA => "یک نسخه جدیدتر روی GitHub موجود است",
        Language::SV => "En nyare version finns tillgänglig på GitHub",
        Language::DE => "Eine neue Version ist auf GitHub verfügbar",
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
        Language::DE => "Inspizieren",
        Language::RU => "Инспектировать",
        Language::SV => "Inspektera",
        _ => "Inspect",
    }
}

pub fn connection_details_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Connection details",
        Language::IT => "Dettagli della connessione",
        Language::RU => "Подробнее о соединении",
        Language::SV => "Anslutningsdetaljer",
        Language::DE => "Verbindungsdetails",
        _ => "Connection details",
    }
}

pub fn dropped_packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Dropped packets",
        Language::IT => "Pacchetti mancati",
        Language::RU => "Потеряно пакетов",
        Language::SV => "Tappade paket",
        Language::DE => "Verlorene Pakete",
        _ => "Dropped packets",
    }
}

pub fn data_representation_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data representation",
        Language::IT => "Rappresentazione dei dati",
        Language::RU => "Показывать в виде", // there is selector below: "байтов" or "пакетов"
        Language::SV => "Datarepresentation",
        Language::DE => "Daten Darstellung",
        _ => "Data representation",
    }
}

pub fn host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Network host",
        Language::IT => "Host di rete",
        Language::RU => "Сетевой хост",
        Language::SV => "Nätverksvärd",
        Language::DE => "Netzwerk-Host",
        _ => "Network host",
    }
}

pub fn only_top_30_hosts_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the top 30 hosts are displayed here",
        Language::IT => "Solo i maggiori 30 host sono mostrati qui",
        Language::RU => "Тут показываются только первые 30 хостов",
        Language::SV => "Endast de 30 främsta värdarna visas här",
        Language::DE => "Nur die obersten 30 Hosts werden hier angezeigt",
        _ => "Only the top 30 hosts are displayed here",
    }
}

pub fn sort_by_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sort by",
        Language::IT => "Ordina per",
        Language::RU => "Сортировка",
        Language::SV => "Sortera efter",
        Language::DE => "Sortieren nach",
        _ => "Sort by",
    }
}

pub fn local_translation(language: Language) -> String {
    match language {
        Language::EN => "Local network",
        Language::IT => "Rete locale",
        Language::RU => "Локальная сеть",
        Language::SV => "Lokalt nätverk",
        Language::DE => "Lokales Netzwerk",
        _ => "Local network",
    }
    .to_string()
}

pub fn unknown_translation(language: Language) -> String {
    match language {
        Language::EN => "Unknown location",
        Language::IT => "Localizzazione sconosciuta",
        Language::RU => "Неизвестный регион",
        Language::SV => "Okänd plats",
        Language::DE => "Ort unbekannt",
        _ => "Unknown location",
    }
    .to_string()
}

pub fn your_network_adapter_translation(language: Language) -> String {
    match language {
        Language::EN => "Your network adapter",
        Language::IT => "La tua scheda di rete",
        Language::RU => "Ваш сетевой адаптер",
        Language::SV => "Din nätverksadapter",
        Language::DE => "Dein Netzwerk-Adapter",
        _ => "Your network adapter",
    }
    .to_string()
}

pub fn socket_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Socket address",
        Language::IT => "Indirizzo del socket",
        Language::RU => "Адрес сокекта",
        Language::SV => "Socketadress",
        Language::DE => "Socket Adresse",
        _ => "Socket address",
    }
}

pub fn mac_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "MAC address",
        Language::IT => "Indirizzo MAC",
        Language::RU => "MAC адрес",
        Language::SV => "MAC-adress",
        Language::DE => "MAC Adresse",
        _ => "MAC address",
    }
}

pub fn source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Source",
        Language::IT => "Sorgente",
        Language::RU => "Источник",
        Language::SV => "Källa",
        Language::DE => "Quelle",
        _ => "Source",
    }
}

pub fn destination_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::SV => "Destination",
        Language::IT => "Destinazione",
        Language::RU => "Получатель",
        Language::DE => "Ziel",
        _ => "Destination",
    }
}

pub fn fqdn_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Fully qualified domain name",
        Language::IT => "Nome di dominio completo",
        Language::RU => "Полное доменное имя",
        Language::SV => "Fullständigt domännamn",
        Language::DE => "Vollständig qualifizierter Domain Name",
        _ => "Fully qualified domain name",
    }
}

pub fn administrative_entity_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Administrative entity",
        Language::IT => "Entità amministrativa",
        Language::RU => "Имя автономной системы",
        Language::SV => "Administrativ enhet",
        Language::DE => "Name des autonomen Systems",
        _ => "Administrative entity",
    }
}

pub fn transmitted_data_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transmitted data",
        Language::IT => "Dati trasmessi",
        Language::RU => "Передано данных",
        Language::SV => "Överförd data",
        Language::DE => "Übermittelte Daten",
        _ => "Transmitted data",
    }
}

pub fn country_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Country",
        Language::IT => "Paese",
        Language::RU => "Страна",
        Language::SV => "Land",
        Language::DE => "Land",
        _ => "Country",
    }
}

pub fn domain_name_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Domain name",
        Language::IT => "Nome di dominio",
        Language::RU => "Доменное имя",
        Language::SV => "Domännamn",
        Language::DE => "Domain Name",
        _ => "Domain name",
    }
}

pub fn only_show_favorites_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show favorites",
        Language::IT => "Mostra solo i preferiti",
        Language::RU => "Показывать только избранные",
        Language::SV => "Visa endast favoriter",
        Language::DE => "Zeige nur die Favoriten",
        _ => "Only show favorites",
    }
}

pub fn search_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Search filters",
        Language::IT => "Filtri di ricerca",
        Language::RU => "Фильтры для поиска",
        Language::SV => "Sökfilter",
        Language::DE => "Filter suchen",
        _ => "Search filters",
    }
}

pub fn no_search_results_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No result available according to the specified search filters",
        Language::IT => "Nessun risultato disponibile secondo i filtri di ricerca specificati",
        Language::RU => "После применения выбранных фильтров результат поиска пустой",
        Language::SV => "Inga resultat tillgängliga utifrån de angivna sökfilterna",
        Language::DE => "Keine Resultate für die spezifizierten Such-Filter verfügbar",
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
        Language::RU => format!("Показываются {start}-{end} из {total} общего числа результатов"),
        Language::SV => format!("Visar {start}-{end} av {total} totala resultat"),
        Language::DE => format!("{start}-{end} von insgesamt {total} Resultaten werden angezeigt"),
        _ => format!("Showing {start}-{end} of {total} total results"),
    }
}

#[allow(dead_code)]
pub fn color_gradients_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Apply color gradients",
        Language::IT => "Applica sfumature di colore",
        Language::RU => "Применить цветовой градиент", // recheck
        Language::SV => "Applicera färggradient",
        Language::DE => "Farb-Gradienten anwenden",
        _ => "Apply color gradients",
    }
}
