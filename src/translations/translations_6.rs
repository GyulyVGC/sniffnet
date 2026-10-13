#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn latency_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Latency",
        Language::IT => "Latenza",
        Language::ZH_TW => "延遲",
        _ => "Latency",
    }
}

pub fn ipfix_exporter_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX exporter",
        Language::IT => "Esportatore IPFIX",
        Language::ZH_TW => "IPFIX 匯出器",
        _ => "IPFIX exporter",
    }
}

pub fn ipfix_collector_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX collector",
        Language::IT => "Collettore IPFIX",
        Language::ZH_TW => "IPFIX 收集器",
        _ => "IPFIX collector",
    }
}

pub fn waiting_ipfix_connections_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No traffic has been observed yet. Waiting for IPFIX flows...",
        Language::IT => "Nessun tipo di traffico è stato osservato finora. Attendo flussi IPFIX...",
        Language::ZH_TW => "尚未觀察到任何流量。正在等待 IPFIX 流量...",
        _ => "No traffic has been observed yet. Waiting for IPFIX flows...",
    }
}

pub fn invalid_ipfix_received_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming traffic isn't decodable as IPFIX...",
        Language::IT => "Il traffico in entrata non è decodificabile come IPFIX...",
        Language::ZH_TW => "傳入的流量無法解碼為 IPFIX...",
        _ => "Incoming traffic isn't decodable as IPFIX...",
    }
}

pub fn make_sure_valid_ipfix_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Make sure the exporter is sending valid IPFIX flows to this collector.",
        Language::IT => {
            "Assicurati che l'esportatore stia inviando flussi IPFIX validi a questo collettore."
        }
        Language::ZH_TW => "請確保匯出器正在向此收集器傳送有效的 IPFIX 流量。",
        _ => "Make sure the exporter is sending valid IPFIX flows to this collector.",
    }
}

pub fn disable_update_checks_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Disable update checks",
        Language::IT => "Disattiva il controllo degli aggiornamenti",
        Language::ZH_TW => "停用更新檢查",
        _ => "Disable update checks",
    }
}

pub fn notify_on_updates_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify when updates are available",
        Language::IT => "Avvisami quando sono disponibili aggiornamenti",
        Language::ZH_TW => "有可用更新時通知我",
        _ => "Notify when updates are available",
    }
}

pub fn update_status_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Update status",
        Language::IT => "Stato degli aggiornamenti",
        Language::ZH_TW => "更新狀態",
        _ => "Update status",
    }
}

pub fn up_to_date_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "You are up to date!",
        Language::IT => "Sei aggiornato!",
        Language::ZH_TW => "您已是最新版本！",
        _ => "You are up to date!",
    }
}

pub fn expanded_view_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Expanded view",
        Language::IT => "Vista espansa",
        Language::ZH_TW => "展開檢視",
        _ => "Expanded view",
    }
}
