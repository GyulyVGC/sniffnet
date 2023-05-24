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
        Language::SV => "Inspektera",
        _ => "Inspect",
    }
}

pub fn connection_details_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Connection details",
        Language::IT => "Dettagli della connessione",
        Language::SV => "Anslutningsdetaljer",
        _ => "Connection details",
    }
}

pub fn dropped_packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Dropped packets",
        Language::IT => "Pacchetti mancati",
        Language::SV => "Tappade paket",
        _ => "Dropped packets",
    }
}

pub fn data_representation_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data representation",
        Language::IT => "Rappresentazione dei dati",
        Language::SV => "Datarepresentation",
        _ => "Data representation",
    }
}

pub fn host_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Network host",
        Language::IT => "Host di rete",
        Language::SV => "Nätverksvärd",
        _ => "Network host",
    }
}

pub fn only_top_30_hosts_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the top 30 hosts are displayed here",
        Language::IT => "Solo i maggiori 30 host sono mostrati qui",
        Language::SV => "Endast de 30 främsta värdarna visas här",
        _ => "Only the top 30 hosts are displayed here",
    }
}

pub fn sort_by_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sort by",
        Language::IT => "Ordina per",
        Language::SV => "Sortera efter",
        _ => "Sort by",
    }
}

pub fn local_translation(language: Language) -> String {
    match language {
        Language::EN => "Local network",
        Language::IT => "Rete locale",
        Language::SV => "Lokalt nätverk",
        _ => "Local network",
    }
    .to_string()
}

pub fn unknown_translation(language: Language) -> String {
    match language {
        Language::EN => "Unknown location",
        Language::IT => "Localizzazione sconosciuta",
        Language::SV => "Okänd plats",
        _ => "Unknown location",
    }
    .to_string()
}

pub fn your_network_adapter_translation(language: Language) -> String {
    match language {
        Language::EN => "Your network adapter",
        Language::IT => "La tua scheda di rete",
        Language::SV => "Din nätverksadapter",
        _ => "Your network adapter",
    }
    .to_string()
}

pub fn socket_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Socket address",
        Language::IT => "Indirizzo del socket",
        Language::SV => "Socketadress",
        _ => "Socket address",
    }
}

pub fn mac_address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "MAC address",
        Language::IT => "Indirizzo MAC",
        Language::SV => "MAC-adress",
        _ => "MAC address",
    }
}

pub fn source_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Source",
        Language::IT => "Sorgente",
        Language::SV => "Källa",
        _ => "Source",
    }
}

pub fn destination_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::SV => "Destination",
        Language::IT => "Destinazione",
        _ => "Destination",
    }
}

pub fn fqdn_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Fully qualified domain name",
        Language::IT => "Nome di dominio completo",
        Language::SV => "Fullständigt domännamn",
        _ => "Fully qualified domain name",
    }
}

pub fn administrative_entity_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Administrative entity",
        Language::IT => "Entità amministrativa",
        Language::SV => "Administrativ enhet",
        _ => "Administrative entity",
    }
}

pub fn transmitted_data_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transmitted data",
        Language::IT => "Dati trasmessi",
        Language::SV => "Överförd data",
        _ => "Transmitted data",
    }
}

pub fn country_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Country",
        Language::IT => "Paese",
        Language::SV => "Land",
        _ => "Country",
    }
}

pub fn domain_name_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Domain name",
        Language::IT => "Nome di dominio",
        Language::SV => "Domännamn",
        _ => "Domain name",
    }
}

pub fn only_show_favorites_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only show favorites",
        Language::IT => "Mostra solo i preferiti",
        Language::SV => "Visa endast favoriter",
        _ => "Only show favorites",
    }
}

pub fn search_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Search filters",
        Language::IT => "Filtri di ricerca",
        Language::SV => "Sökfilter",
        _ => "Search filters",
    }
}

pub fn no_search_results_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No result available according to the specified search filters",
        Language::IT => "Nessun risultato disponibile secondo i filtri di ricerca specificati",
        Language::SV => "Inga resultat tillgängliga utifrån de angivna sökfilterna",
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
        Language::SV => format!("Visar {start}-{end} av {total} totala resultat"),
        _ => format!("Showing {start}-{end} of {total} total results"),
    }
}

#[allow(dead_code)]
pub fn color_gradients_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Apply color gradients",
        Language::IT => "Applica sfumature di colore",
        Language::SV => "Applicera färggradient",
        _ => "Apply color gradients",
    }
}
