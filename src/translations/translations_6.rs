#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn latency_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Latency",
        Language::IT => "Latenza",
        Language::RU => "Задержка",
        _ => "Latency",
    }
}

pub fn ipfix_exporter_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX exporter",
        Language::IT => "Esportatore IPFIX",
        Language::RU => "Экспортёр IPFIX",
        _ => "IPFIX exporter",
    }
}

pub fn ipfix_collector_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX collector",
        Language::IT => "Collettore IPFIX",
        Language::RU => "Коллектор IPFIX",
        _ => "IPFIX collector",
    }
}

pub fn waiting_ipfix_connections_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No traffic has been observed yet. Waiting for IPFIX flows...",
        Language::IT => "Nessun tipo di traffico è stato osservato finora. Attendo flussi IPFIX...",
        Language::RU => "Пока трафик не наблюдался. Ожидание потоков IPFIX...",
        _ => "No traffic has been observed yet. Waiting for IPFIX flows...",
    }
}

pub fn invalid_ipfix_received_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming traffic isn't decodable as IPFIX...",
        Language::IT => "Il traffico in entrata non è decodificabile come IPFIX...",
        Language::RU => "Входящий трафик невозможно декодировать как IPFIX...",
        _ => "Incoming traffic isn't decodable as IPFIX...",
    }
}

pub fn make_sure_valid_ipfix_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Make sure the exporter is sending valid IPFIX flows to this collector.",
        Language::IT => {
            "Assicurati che l'esportatore stia inviando flussi IPFIX validi a questo collettore."
        }
        Language::RU => {
            "Убедитесь, что экспортёр отправляет корректные потоки IPFIX на этот коллектор."
        }
        _ => "Make sure the exporter is sending valid IPFIX flows to this collector.",
    }
}

pub fn disable_update_checks_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Disable update checks",
        Language::IT => "Disattiva il controllo degli aggiornamenti",
        Language::RU => "Отключить проверку обновлений",
        _ => "Disable update checks",
    }
}

pub fn notify_on_updates_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify when updates are available",
        Language::IT => "Avvisami quando sono disponibili aggiornamenti",
        Language::RU => "Уведомлять о доступных обновлениях",
        _ => "Notify when updates are available",
    }
}

pub fn update_status_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Update status",
        Language::IT => "Stato degli aggiornamenti",
        Language::RU => "Статус обновлений",
        _ => "Update status",
    }
}

pub fn up_to_date_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "You are up to date!",
        Language::IT => "Sei aggiornato!",
        Language::RU => "У вас актуальная версия!",
        _ => "You are up to date!",
    }
}

pub fn expanded_view_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Expanded view",
        Language::IT => "Vista espansa",
        Language::RU => "Расширенный вид",
        _ => "Expanded view",
    }
}

pub fn totals_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Totals",
        Language::IT => "Totali",
        Language::ZH => "总计",
        Language::ZH_TW => "總計",
        Language::RU => "Итого",
        _ => "Totals",
    }
}

pub fn current_rates_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Current rates",
        Language::IT => "Intensità attuale",
        Language::ZH => "当前速率",
        Language::ZH_TW => "目前速率",
        Language::RU => "Текущие скорости",
        _ => "Current rates",
    }
}
