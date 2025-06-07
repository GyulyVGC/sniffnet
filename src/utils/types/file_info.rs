use crate::translations::translations_3::{
    database_from_file_translation, select_directory_translation, style_from_file_translation,
};
use crate::translations::translations_4::select_capture_translation;
use crate::translations::types::language::Language;

#[derive(Debug, Clone, PartialEq)]
pub enum FileInfo {
    Style,
    Database,
    Directory,
    PcapImport,
}

impl FileInfo {
    pub fn get_extensions(&self) -> Vec<&'static str> {
        match self {
            FileInfo::Style => vec!["toml"],
            FileInfo::Database => vec!["mmdb"],
            FileInfo::Directory => vec![],
            FileInfo::PcapImport => vec!["pcap", "pcapng", "cap"],
        }
    }

    pub fn action_info(&self, language: Language) -> &'static str {
        match self {
            FileInfo::Style => style_from_file_translation(language),
            FileInfo::Database => database_from_file_translation(language),
            FileInfo::Directory => select_directory_translation(language),
            FileInfo::PcapImport => select_capture_translation(language),
        }
    }
}
