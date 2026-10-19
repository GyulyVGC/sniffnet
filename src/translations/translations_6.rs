#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn latency_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Latency",
        Language::IT => "Latenza",
        Language::ZH => "延迟",
        Language::ZH_TW => "延遲",
        Language::RU => "Задержка",
        Language::FR => "Latence",
        Language::AR => "زمن الاستجابة",
        _ => "Latency",
    }
}

pub fn ipfix_exporter_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX exporter",
        Language::IT => "Esportatore IPFIX",
        Language::ZH => "IPFIX 导出器",
        Language::ZH_TW => "IPFIX 匯出器",
        Language::RU => "Экспортёр IPFIX",
        Language::FR => "Exportateur IPFIX",
        Language::AR => "مُصدِّر IPFIX",
        _ => "IPFIX exporter",
    }
}

pub fn ipfix_collector_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IPFIX collector",
        Language::IT => "Collettore IPFIX",
        Language::ZH => "IPFIX 收集器",
        Language::ZH_TW => "IPFIX 收集器",
        Language::RU => "Коллектор IPFIX",
        Language::FR => "Collecteur IPFIX",
        Language::AR => "مُجمِّع IPFIX",
        _ => "IPFIX collector",
    }
}

pub fn waiting_ipfix_connections_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "No traffic has been observed yet. Waiting for IPFIX flows...",
        Language::IT => "Nessun tipo di traffico è stato osservato finora. Attendo flussi IPFIX...",
        Language::ZH => "尚未观察到任何流量。正在等待 IPFIX 流...",
        Language::ZH_TW => "尚未觀察到任何流量。正在等待 IPFIX 流量...",
        Language::RU => "Пока трафик не наблюдался. Ожидание потоков IPFIX...",
        Language::FR => "Aucun trafic n'a encore été observé. En attente de flux IPFIX...",
        Language::AR => "لم تُرصد أي حركة مرور بعد. في انتظار تدفقات IPFIX...",
        _ => "No traffic has been observed yet. Waiting for IPFIX flows...",
    }
}

pub fn invalid_ipfix_received_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming traffic isn't decodable as IPFIX...",
        Language::IT => "Il traffico in entrata non è decodificabile come IPFIX...",
        Language::ZH => "传入的流量无法解码为 IPFIX...",
        Language::ZH_TW => "傳入的流量無法解碼為 IPFIX...",
        Language::RU => "Входящий трафик невозможно декодировать как IPFIX...",
        Language::FR => "Le trafic entrant ne peut pas être décodé au format IPFIX...",
        Language::AR => "يتعذر فك ترميز حركة المرور الواردة بصيغة IPFIX...",
        _ => "Incoming traffic isn't decodable as IPFIX...",
    }
}

pub fn make_sure_valid_ipfix_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Make sure the exporter is sending valid IPFIX flows to this collector.",
        Language::IT => {
            "Assicurati che l'esportatore stia inviando flussi IPFIX validi a questo collettore."
        }
        Language::ZH => "请确保导出器正在向此收集器发送有效的 IPFIX 流。",
        Language::ZH_TW => "請確保匯出器正在向此收集器傳送有效的 IPFIX 流量。",
        Language::RU => {
            "Убедитесь, что экспортёр отправляет корректные потоки IPFIX на этот коллектор."
        }
        Language::FR => "Vérifiez que l'exportateur envoie des flux IPFIX valides à ce collecteur.",
        Language::AR => "تأكد من أن المُصدِّر يرسل تدفقات IPFIX صالحة إلى هذا المُجمِّع.",
        _ => "Make sure the exporter is sending valid IPFIX flows to this collector.",
    }
}

pub fn disable_update_checks_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Disable update checks",
        Language::IT => "Disattiva il controllo degli aggiornamenti",
        Language::ZH => "禁用更新检查",
        Language::ZH_TW => "停用更新檢查",
        Language::RU => "Отключить проверку обновлений",
        Language::FR => "Désactiver la recherche de mises à jour",
        Language::AR => "تعطيل التحقق من وجود تحديثات",
        _ => "Disable update checks",
    }
}

pub fn notify_on_updates_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify when updates are available",
        Language::IT => "Avvisami quando sono disponibili aggiornamenti",
        Language::ZH => "有可用更新时通知我",
        Language::ZH_TW => "有可用更新時通知我",
        Language::RU => "Уведомлять о доступных обновлениях",
        Language::FR => "M'avertir lorsque des mises à jour sont disponibles",
        Language::AR => "إشعاري عند توفر تحديثات",
        _ => "Notify when updates are available",
    }
}

pub fn update_status_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Update status",
        Language::IT => "Stato degli aggiornamenti",
        Language::ZH => "更新状态",
        Language::ZH_TW => "更新狀態",
        Language::RU => "Статус обновлений",
        Language::FR => "État des mises à jour",
        Language::AR => "حالة التحديث",
        _ => "Update status",
    }
}

pub fn up_to_date_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "You are up to date!",
        Language::IT => "Sei aggiornato!",
        Language::ZH => "您已是最新版本！",
        Language::ZH_TW => "您已是最新版本！",
        Language::RU => "У вас актуальная версия!",
        Language::FR => "Vous êtes à jour !",
        Language::AR => "أنت تستخدم أحدث إصدار!",
        _ => "You are up to date!",
    }
}

pub fn expanded_view_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Expanded view",
        Language::IT => "Vista espansa",
        Language::ZH => "展开视图",
        Language::ZH_TW => "展開檢視",
        Language::RU => "Расширенный вид",
        Language::FR => "Vue développée",
        Language::AR => "العرض الموسع",
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
        Language::FR => "Totaux",
        Language::AR => "الإجماليات",
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
        Language::FR => "Débits actuels",
        Language::AR => "معدلات النقل الحالية",
        _ => "Current rates",
    }
}
