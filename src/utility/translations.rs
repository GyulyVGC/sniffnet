use crate::enums::language::Language;
use iced::widget::Text;

pub fn choose_adapters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
    })
}

pub fn application_protocol_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Application protocol",
        Language::IT => "Protocollo applicativo",
    })
}

pub fn select_filters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
    })
}

pub fn start_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Run!",
        Language::IT => "Avvia!",
    })
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddress:",
        Language::IT => "\nIndirizzo:",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddresses:",
        Language::IT => "\nIndirizzi:",
    }
}

pub fn ip_version_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "IP version",
        Language::IT => "Versione IP",
    })
}

pub fn transport_protocol_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Transport protocol",
        Language::IT => "Protocollo di trasporto",
    })
}

pub fn traffic_rate_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Traffic rate:",
        Language::IT => "Intensità del traffico:",
    })
}

pub fn relevant_connections_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Relevant connections:",
        Language::IT => "Connessioni rilevanti:",
    })
}
