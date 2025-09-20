#![allow(clippy::match_same_arms)]

use crate::gui::styles::types::style_type::StyleType;
use crate::translations::translations_3::file_name_translation;
use crate::translations::types::language::Language;
use iced::widget::Text;

pub fn reserved_address_translation(language: Language, info: &str) -> String {
    match language {
        Language::EN => format!("Reserved address ({info})"),
        Language::IT => format!("Indirizzo riservato ({info})"),
        Language::JA => format!("予約済みアドレス ({info})"),
        Language::PT => format!("Endereço reservado ({info})"),
        Language::UK => format!("Зарезервована адреса ({info})"),
        Language::ZH => format!("预留地址 ({info})"),
        Language::ZH_TW => format!("保留的網路位址 ({info})"),
        Language::FR => format!("Adresse réservée ({info})"),
        Language::NL => format!("Gereserveerd adres ({info})"),
        Language::RO => format!("Adresă rezervată ({info})"),
        Language::DE => format!("Reservierte Adresse ({info})"),
        Language::UZ => format!("Rezervlangan manzil ({info})"),
        Language::ID => format!("Alamat disimpan ({info})"),
        Language::EL => format!("Δεσμευμένη διεύθυνση ({info})"),
        _ => format!("Reserved address ({info})"),
    }
}

pub fn share_feedback_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Share your feedback",
        Language::IT => "Condividi il tuo feedback",
        Language::JA => "フィードバックを共有",
        Language::ZH => "分享您的反馈",
        Language::ZH_TW => "分享您的意見回饋",
        Language::FR => "Partagez vos commentaires",
        Language::NL => "Deel uw feedback",
        Language::RO => "Împărtășiți feedback-ul dvs",
        Language::DE => "Feedback geben",
        Language::UZ => "Fikr-mulohazalaringizni ulashing",
        Language::ID => "Berikan masukanmu",
        Language::EL => "Μοιραστείτε τα σχόλιά σας",
        _ => "Share your feedback",
    }
}

// refers to bytes or packets excluded because of the filters
// pub fn excluded_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Excluded",
//         Language::IT => "Esclusi",
//         Language::JA => "除外",
//         Language::ZH => "已被过滤",
//         Language::UZ => "Chiqarib tashlangan",
//         Language::ZH_TW => "已排除",
//         Language::FR => "Exclus",
//         Language::NL => "Uitgesloten",
//         Language::DE => "Herausgefiltert",
//         Language::EL => "Εξαιρούμενα",
//         Language::RO => "Excluși",
//         Language::ID => "Kecuali",
//         _ => "Excluded",
//     }
// }

pub fn capture_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Capture file",
        Language::IT => "File di cattura",
        Language::FR => "Fichier de capture",
        Language::JA => "キャプチャファイル",
        Language::ZH => "捕获文件",
        Language::NL => "Capture bestand",
        Language::DE => "Aufzeichnungsdatei",
        Language::UZ => "Tahlil faylini",
        Language::EL => "Αρχείου καταγραφής",
        Language::RO => "Fișierul de captură",
        Language::ZH_TW => "擷取文件",
        Language::ID => "File tangkapan",
        _ => "Capture file",
    }
}

pub fn select_capture_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select capture file",
        Language::IT => "Seleziona file di cattura",
        Language::FR => "Sélectionner un fichier de capture",
        Language::JA => "キャプチャファイルを選択",
        Language::ZH => "选择捕获文件",
        Language::NL => "Selecteer capture bestand",
        Language::RO => "Selectează fișierul de captură",
        Language::DE => "Aufzeichnungsdatei auswählen",
        Language::UZ => "Tahlil faylini tanlang",
        Language::ID => "Pilih file tangkapan",
        Language::ZH_TW => "選擇擷取文件",
        Language::EL => "Επιλογή αρχείου καταγραφής",
        _ => "Select capture file",
    }
}

pub fn reading_from_pcap_translation<'a>(language: Language, file: &str) -> Text<'a, StyleType> {
    let file_name_translation = file_name_translation(language);
    Text::new(match language {
        Language::EN => format!(
            "Reading packets from file...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Are you sure the file you selected isn't empty?"
        ),
        Language::IT => format!(
            "Lettura pacchetti da file...\n\n\
                                {file_name_translation}: {file}\n\n\
                                Sei sicuro che il file che hai selezionato non sia vuoto?"
        ),
        Language::FR => format!(
            "Lecture des paquets depuis le fichier...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Êtes-vous sûr que le fichier sélectionné n'est pas vide?"
        ),
        Language::JA => format!(
            "ファイルからパケットを読み込み中...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 選択したファイルが空でないことを確認しましたか？"
        ),
        Language::ZH => format!(
            "从文件中读取数据包...\n\n\
                                {file_name_translation}: {file}\n\n\
                                您确定选中的文件不是空的吗?"
        ),
        Language::NL => format!(
            "Pakketten lezen uit bestand...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Weet je zeker dat het geselecteerde bestand niet leeg is?"
        ),
        Language::RO => format!(
            "Citirea pachetelor din fișier...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Ești sigur că fișierul selectat nu este gol?"
        ),
        Language::DE => format!(
            "Pakete aus Datei laden... \n\n\
                                {file_name_translation}: {file}\n\n\
                                Bist du sicher, dass die gewählte Datei nicht leer ist?"
        ),
        Language::UZ => format!(
            "Faylni o'qish...\n\n\
                                {file_name_translation}: {file}\n\n\
                                Fayl bo'sh emasligiga aminmisiz?"
        ),
        Language::ID => format!(
            "Membaca paket dari berkas...\n\n\
                                {file_name_translation}: {file}\n\n\
                                Apa kamu yakin berkasnya tidak kosong?"
        ),
        Language::ZH_TW => format!(
            "從檔案讀取資料包...\n\n\
                                {file_name_translation}: {file}\n\n\
                                您確定您選擇的檔案不是空的嗎？"
        ),
        Language::EL => format!(
            "Ανάγνωση πακέτων από αρχείο...\n\n\
                                 {file_name_translation}: {file}\n\n\
                                 Είστε βέβαιοι ότι το επιλεγμένο αρχείο δεν είναι κενό;"
        ),
        _ => format!(
            "Reading packets from file...\n\n\
                                {file_name_translation}: {file}\n\n\
                                Are you sure the file you selected isn't empty?"
        ),
    })
}

pub fn data_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data threshold exceeded",
        Language::IT => "Soglia di dati superata",
        Language::FR => "Seuil de données dépassé",
        Language::JA => "データの閾値を超えました",
        Language::ZH | Language::ZH_TW => "已超出数据阈值",
        Language::NL => "Gegevenslimiet overschreden",
        Language::RO => "Limita de date depășită",
        Language::DE => "Datenschwelle überschritten",
        Language::UZ => "Ma'lumotlar chegarasidan oshib ketdi",
        Language::ID => "Ambang batas data terlampaui",
        Language::EL => "Υπέρβαση ορίου δεδομένων",
        _ => "Data threshold exceeded",
    }
}

pub fn bits_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bits threshold exceeded",
        Language::IT => "Soglia di bit superata",
        Language::FR => "Seuil de bits dépassé",
        Language::JA => "ビットの閾値を超えました",
        Language::ZH => "已超出比特阈值",
        Language::NL => "Bits limiet overschreden",
        Language::RO => "Limita de biți depășită",
        Language::DE => "Bitschwelle überschritten",
        Language::UZ => "Bitlar chegarasidan oshib ketdi",
        Language::ID => "Ambang batas bit terlampaui",
        Language::ZH_TW => "超出數據界限",
        Language::EL => "Υπέρβαση ορίου δυφίων",
        _ => "Bits threshold exceeded",
    }
}

pub fn bits_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT | Language::NL | Language::DE | Language::FR | Language::ID => {
            "bits"
        }
        Language::JA => "ビット",
        Language::ZH => "比特",
        Language::UZ => "bitlar",
        Language::EL => "Δυφία",
        Language::RO => "biți",
        Language::ZH_TW => "位元",
        _ => "bits",
    }
}

#[allow(dead_code)]
pub fn pause_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE | Language::FR => "Pause",
        Language::IT => "Pausa",
        Language::JA => "一時停止",
        Language::ZH => "暂停",
        Language::NL => "Pauzeren",
        Language::RO => "Pauză",
        Language::UZ => "To'xtatish",
        Language::ID => "Dijeda",
        Language::ZH_TW => "暫停",
        Language::EL => "Παύση",
        _ => "Pause",
    }
}

#[allow(dead_code)]
pub fn resume_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Resume",
        Language::IT => "Riprendi",
        Language::FR => "Reprendre",
        Language::JA => "再開",
        Language::ZH => "恢复",
        Language::NL => "Hervatten",
        Language::RO => "Continuă",
        Language::DE => "Fortsetzen",
        Language::UZ => "Davom ettirish",
        Language::ID => "Dilanjut",
        Language::ZH_TW => "繼續",
        Language::EL => "Συνέχεια",
        _ => "Resume",
    }
}
