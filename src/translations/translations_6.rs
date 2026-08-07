#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn latency_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Latency",
        Language::IT => "Latenza",
        _ => "Latency",
    }
}

pub fn ipfix_collector_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX collector",
        Language::IT => "Collettore IPFIX",
        _ => "IPFIX collector",
    }
}

pub fn waiting_ipfix_connections_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No traffic has been observed yet. Waiting for IPFIX flows...",
        Language::IT => "Nessun tipo di traffico è stato osservato finora. Attendo flussi IPFIX...",
        _ => "No traffic has been observed yet. Waiting for IPFIX flows...",
    }
}

pub fn invalid_ipfix_received_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming traffic isn't decodable as IPFIX...",
        Language::IT => "Il traffico in entrata non è decodificabile come IPFIX...",
        _ => "Incoming traffic isn't decodable as IPFIX...",
    }
}

pub fn make_sure_valid_ipfix_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Make sure the exporter is sending valid IPFIX flows to this collector.",
        Language::IT => {
            "Assicurati che l'esportatore stia inviando flussi IPFIX validi a questo collettore."
        }
        _ => "Make sure the exporter is sending valid IPFIX flows to this collector.",
    }
}
