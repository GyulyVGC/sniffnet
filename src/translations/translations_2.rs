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
        Language::TR => "Daha yeni bir versiyon GitHub'ta mevcut",
        Language::ES => "Hay una nueva versión disponible en GitHub",
        Language::KO => "GitHub에 새로운 버전이 출시되었습니다.",
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
        Language::TR => "İncele",
        Language::FA => "بازرسی",
        Language::KO => "검사",
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
        Language::TR => "Bağlantı detayları",
        Language::FA => "مشخصات اتصال",
        Language::ES => "Detalles de la Conexión",
        Language::KO => "연결 상세",
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
        Language::TR => "Düşen paketler",
        Language::FA => "بسته های رها شده",
        Language::ES => "Paquetes perdidos",
        Language::KO => "손실 패킷",
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
        Language::TR => "Veri gösterimi",
        Language::FA => "بازنمایی داده ها", // TODO: or نمایندگی داده ها depending on context
        Language::ES => "Representación de los datos",
        Language::KO => "데이터 단위",
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
        Language::TR => "Ağ sunucusu",
        Language::FA => "میزبان شبکه",
        Language::ES => "Host de red",
        Language::KO => "네트워크 호스트",
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
        Language::TR => "Sadece ilk 30 sunucu burda gösterilmektedir",
        Language::FA => "تنها ۳۰ میزبان برتر در اینجا نمایش داده شده اند",
        Language::ES => "Aquí sólo se muestran los 30 mejores anfitriones",
        Language::KO => "상위 30개의 호스트만 노출됩니다",
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
        Language::TR => "Şuna göre sırala",
        Language::FA => "مرتب سازی بر اساس",
        Language::ES => "Ordenar por",
        Language::KO => "정렬",
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
        Language::TR => "Yerel ağ",
        Language::FA => "شبکه محلی",
        Language::ES => "Red local",
        Language::KO => "로컬 네트워크",
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
        Language::TR => "Bilinmeyen yer",
        Language::FA => "محل نامعلوم",
        Language::ES => "Localización desconocida",
        Language::KO => "알 수 없는 위치",
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
        Language::TR => "Ağ adaptörün",
        Language::FA => "مبدل شبکه شما",
        Language::ES => "Su adaptador de red",
        Language::KO => "네트워크 어댑터",
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
        Language::TR => "Soket adresi",
        Language::FA => "پریز شبکه",
        Language::ES => "Dirección del socket",
        Language::KO => "소켓 어드레스",
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
        Language::TR => "MAC adresi",
        Language::FA => "آدرس MAC",
        Language::ES => "Dirección MAC",
        Language::KO => "맥 어드레스",
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
        Language::TR => "Kaynak",
        Language::FA => "منبع",
        Language::ES => "Origen",
        Language::KO => "소스",
        _ => "Source",
    }
}

pub fn destination_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::SV => "Destination",
        Language::IT => "Destinazione",
        Language::RU => "Получатель",
        Language::DE => "Ziel",
        Language::TR => "Hedef",
        Language::FA => "مقصد",
        Language::ES => "Destino",
        Language::KO => "목적지",
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
        Language::TR => "Tam nitelikli alan adı",
        Language::FA => "نام دامنه جامع الشرایط",
        Language::ES => "Nombre de dominio completo",
        Language::KO => "절대 도메인 네임",
        _ => "Fully qualified domain name",
    }
}

pub fn administrative_entity_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Autonomous System name",
        Language::IT => "Nome del sistema autonomo",
        Language::RU => "Имя автономной системы",
        Language::SV => "Administrativ enhet",
        Language::DE => "Name des autonomen Systems",
        Language::TR => "Yönetim varlığı",
        Language::FA => "واحد اجرایی", // TODO: or واحد اداری depending on context
        Language::ES => "Entidad Administrativa",
        Language::KO => "관리 엔티티",
        _ => "Autonomous System name",
    }
}

pub fn transmitted_data_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transmitted data",
        Language::IT => "Dati trasmessi",
        Language::RU => "Передано данных",
        Language::SV => "Överförd data",
        Language::DE => "Übermittelte Daten",
        Language::TR => "Aktarılan veri",
        Language::FA => "دادهٔ منتقل شده",
        Language::ES => "Datos transmitidos",
        Language::KO => "수신된 데이터",
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
        Language::TR => "Ülke",
        Language::FA => "کشور",
        Language::ES => "País",
        Language::KO => "국가",
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
        Language::TR => "Alan adı",
        Language::FA => "نام دامنه",
        Language::ES => "Nombre de dominio",
        Language::KO => "도메인 네임",
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
        Language::TR => "Sadece favorileri göster",
        Language::FA => "فقط پسندیده ها را نمایش بده",
        Language::ES => "Mostrar solo los favoritos",
        Language::KO => "즐겨찾기만 보기",
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
        Language::TR => "Arama filtresi",
        Language::FA => "صافی های جستجو",
        Language::ES => "Filtros de búsqueda",
        Language::KO => "검색 필터",
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
        Language::TR => "Belirtilen arama filtrelerine göre herhangi bir sonuç bulunmamaktadır",
        Language::FA => "هیچ نتیجه ای بر اساس صافی های جستجوی تعیین شده وجود ندارد",
        Language::ES => "No hay resultados disponibles según los filtros de búsqueda especificados",
        Language::KO => "해당 검색 필터로 검색된 결과가 없습니다.",
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
        Language::TR => format!("{total} sonuç içinde {start}-{end}"),
        Language::FA => format!("نمایش {start}-{end} از تمامی {total} نتیجه"),
        Language::ES => format!("Mostrando {start}-{end} de {total} resultados totales"),
        Language::KO => format!("총 {total}개의 결과 중 {start}-{end}을(를) 보여줍니다"),
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
        Language::TR => "Renk grandyanı uygula",
        Language::FA => "اعمال گرادیان های رنگ",
        Language::ES => "Aplicar gradientes de color",
        Language::KO => "그라디언트 색상 적용",
        _ => "Apply color gradients",
    }
}
