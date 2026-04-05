#![allow(clippy::match_same_arms)]

use crate::translations::types::language::Language;

pub fn reserved_address_translation(language: Language, info: &str) -> String {
    match language {
        Language::EN => format!("Reserved address ({info})"),
        Language::CS => format!("Rezervovaná adresa ({info})"),
        Language::IT => format!("Indirizzo riservato ({info})"),
        Language::JA => format!("予約済みアドレス ({info})"),
        Language::PT => format!("Endereço reservado ({info})"),
        Language::UK => format!("Зарезервована адреса ({info})"),
        Language::ZH => format!("预留地址 ({info})"),
        Language::ZH_TW => format!("保留的網路位址 ({info})"),
        Language::FR => format!("Adresse réservée ({info})"),
        Language::NL => format!("Gereserveerd adres ({info})"),
        Language::ES => format!("Dirección reservada ({info})"),
        Language::RO => format!("Adresă rezervată ({info})"),
        Language::DE => format!("Reservierte Adresse ({info})"),
        Language::UZ => format!("Rezervlangan manzil ({info})"),
        Language::ID => format!("Alamat disimpan ({info})"),
        Language::EL => format!("Δεσμευμένη διεύθυνση ({info})"),
        Language::VI => format!("Địa chỉ dự trữ ({info})"),
        Language::SV => format!("Reserverad adress ({info})"),
        _ => format!("Reserved address ({info})"),
    }
}

pub fn share_feedback_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Share your feedback",
        Language::CS => "Sdílejte své hodnocení",
        Language::IT => "Condividi il tuo feedback",
        Language::JA => "フィードバックを共有",
        Language::ZH => "分享您的反馈",
        Language::ZH_TW => "分享您的意見回饋",
        Language::FR => "Partagez vos commentaires",
        Language::NL => "Deel uw feedback",
        Language::ES => "Comparte tus comentarios",
        Language::RO => "Împărtășiți feedback-ul dvs",
        Language::DE => "Feedback geben",
        Language::UZ => "Fikr-mulohazalaringizni ulashing",
        Language::ID => "Berikan masukanmu",
        Language::EL => "Μοιραστείτε τα σχόλιά σας",
        Language::VI => "Chia sẻ phản hồi của bạn",
        Language::SV => "Dela din feedback",
        _ => "Share your feedback",
    }
}

// refers to bytes or packets excluded because of the filters
// pub fn excluded_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Excluded",
//         Language::CS => "Vyloučeno",
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
//         Language::ES => "Excluidos",
//         Language::VI => "Loại trừ",
//         _ => "Excluded",
//     }
// }

pub fn capture_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Capture file",
        Language::CS => "Soubor zachycení",
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
        Language::ES => "Archivo de captura",
        Language::VI => "Bắt tệp tin",
        Language::SV => "Inspelningsfil",
        _ => "Capture file",
    }
}

pub fn select_file_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Select file",
        Language::CS => "Výběr souboru",
        Language::IT => "Seleziona file",
        Language::FR => "Sélectionner fichier",
        Language::JA => "ファイルを選択",
        Language::ZH => "选择文件",
        Language::NL => "Selecteer bestand",
        Language::ES => "Seleccionar archivo",
        Language::RO => "Selectează fișierul",
        Language::DE => "Datei auswählen",
        Language::UZ => "Faylini tanlang",
        Language::ID => "Pilih file",
        Language::ZH_TW => "選擇文件",
        Language::EL => "Επιλογή αρχείου",
        Language::VI => "Chọn tệp",
        Language::SV => "Välj fil",
        _ => "Select file",
    }
}

pub fn reading_from_pcap_translation(language: Language) -> &'static str {
    match language {
        Language::EN => {
            "Reading packets from file...\n\n\
                                 Are you sure the file you selected isn't empty?"
        }
        Language::CS => {
            "Čtení paketů ze souuboru...\n\n\
                                 Jste si jistý že vybraný soubor není prázdný?"
        }
        Language::IT => {
            "Lettura pacchetti da file...\n\n\
                                Sei sicuro che il file che hai selezionato non sia vuoto?"
        }
        Language::FR => {
            "Lecture des paquets depuis le fichier...\n\n\
                                 Êtes-vous sûr que le fichier sélectionné n'est pas vide?"
        }
        Language::JA => {
            "ファイルからパケットを読み込み中...\n\n\
                                 選択したファイルが空でないことを確認しましたか？"
        }
        Language::ZH => {
            "从文件中读取数据包...\n\n\
                                您确定选中的文件不是空的吗?"
        }
        Language::NL => {
            "Pakketten lezen uit bestand...\n\n\
                                 Weet je zeker dat het geselecteerde bestand niet leeg is?"
        }
        Language::ES => {
            "Leyendo paquetes desde el archivo...\n\n\
                                ¿Seguro que el archivo seleccionado no está vacío?"
        }
        Language::RO => {
            "Citirea pachetelor din fișier...\n\n\
                                 Ești sigur că fișierul selectat nu este gol?"
        }
        Language::DE => {
            "Pakete aus Datei laden... \n\n\
                                Bist du sicher, dass die gewählte Datei nicht leer ist?"
        }
        Language::UZ => {
            "Faylni o'qish...\n\n\
                                Fayl bo'sh emasligiga aminmisiz?"
        }
        Language::ID => {
            "Membaca paket dari berkas...\n\n\
                                Apa kamu yakin berkasnya tidak kosong?"
        }
        Language::ZH_TW => {
            "從檔案讀取資料包...\n\n\
                                您確定您選擇的檔案不是空的嗎？"
        }
        Language::EL => {
            "Ανάγνωση πακέτων από αρχείο...\n\n\
                                 Είστε βέβαιοι ότι το επιλεγμένο αρχείο δεν είναι κενό;"
        }
        Language::VI => {
            "Đang đọc gói tin từ tệp...\n\n\
                                 Bạn có chắc tệp tin đã chọn không bị trống?"
        }
        Language::SV => {
            "Läser paket från fil...\n\n\
                                Är du säker på att filen du valde inte är tom?"
        }
        _ => {
            "Reading packets from file...\n\n\
                                Are you sure the file you selected isn't empty?"
        }
    }
}

pub fn data_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Data threshold exceeded",
        Language::CS => "Překročen limit dat",
        Language::IT => "Soglia di dati superata",
        Language::FR => "Seuil de données dépassé",
        Language::JA => "データの閾値を超えました",
        Language::ZH | Language::ZH_TW => "已超出数据阈值",
        Language::NL => "Gegevenslimiet overschreden",
        Language::ES => "Umbral de datos superado",
        Language::RO => "Limita de date depășită",
        Language::DE => "Datenschwelle überschritten",
        Language::UZ => "Ma'lumotlar chegarasidan oshib ketdi",
        Language::ID => "Ambang batas data terlampaui",
        Language::EL => "Υπέρβαση ορίου δεδομένων",
        Language::VI => "Đã vượt ngưỡng dữ liệu",
        Language::SV => "Datagräns överskriden",
        _ => "Data threshold exceeded",
    }
}

pub fn bits_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bits threshold exceeded",
        Language::CS => "Překročen limit bitů",
        Language::IT => "Soglia di bit superata",
        Language::FR => "Seuil de bits dépassé",
        Language::JA => "ビットの閾値を超えました",
        Language::ZH => "已超出比特阈值",
        Language::NL => "Bits limiet overschreden",
        Language::ES => "Umbral de bits superado",
        Language::RO => "Limita de biți depășită",
        Language::DE => "Bitschwelle überschritten",
        Language::UZ => "Bitlar chegarasidan oshib ketdi",
        Language::ID => "Ambang batas bit terlampaui",
        Language::ZH_TW => "超出數據界限",
        Language::EL => "Υπέρβαση ορίου δυφίων",
        Language::VI => "Đã vượt ngưỡng bit",
        Language::SV => "Bitgräns överskriden",
        _ => "Bits threshold exceeded",
    }
}

pub fn bits_translation(language: Language) -> &'static str {
    match language {
        Language::EN
        | Language::IT
        | Language::NL
        | Language::DE
        | Language::FR
        | Language::ID
        | Language::ES
        | Language::SV => "bits",
        Language::CS => "bity",
        Language::JA => "ビット",
        Language::ZH => "比特",
        Language::UZ => "bitlar",
        Language::EL => "Δυφία",
        Language::RO => "biți",
        Language::ZH_TW => "位元",
        Language::VI => "bit",
        _ => "bits",
    }
}

#[allow(dead_code)]
pub fn pause_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE | Language::FR => "Pause",
        Language::CS => "Pauza",
        Language::IT | Language::ES | Language::SV => "Pausa",
        Language::JA => "一時停止",
        Language::ZH => "暂停",
        Language::NL => "Pauzeren",
        Language::RO => "Pauză",
        Language::UZ => "To'xtatish",
        Language::ID => "Dijeda",
        Language::ZH_TW => "暫停",
        Language::EL => "Παύση",
        Language::VI => "Tạm dừng",
        _ => "Pause",
    }
}

#[allow(dead_code)]
pub fn resume_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Resume",
        Language::CS => "Obnovit",
        Language::IT => "Riprendi",
        Language::FR => "Reprendre",
        Language::JA => "再開",
        Language::ZH => "恢复",
        Language::NL => "Hervatten",
        Language::ES => "Reanudar",
        Language::RO => "Continuă",
        Language::DE => "Fortsetzen",
        Language::UZ => "Davom ettirish",
        Language::ID => "Dilanjut",
        Language::ZH_TW => "繼續",
        Language::EL => "Συνέχεια",
        Language::VI => "Tiếp tục",
        Language::SV => "Återuppta",
        _ => "Resume",
    }
}
