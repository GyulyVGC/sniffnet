use crate::translations::translations_3::select_dest_directory_translation;
use crate::translations::translations_4::select_file_translation;
use crate::translations::types::language::Language;

pub const ALLOWED_PCAP_EXTENSIONS: [&str; 4] = ["pcap", "pcapng", "cap", "dmp"];

#[derive(Debug, Clone, PartialEq)]
pub enum FileInfo {
    Style,
    Database,
    Directory,
    PcapImport,
    Blacklist,
}

impl FileInfo {
    pub fn get_extensions(&self) -> Vec<&'static str> {
        match self {
            FileInfo::Style => vec!["toml"],
            FileInfo::Database => vec!["mmdb"],
            FileInfo::PcapImport => ALLOWED_PCAP_EXTENSIONS.to_vec(),
            FileInfo::Directory | FileInfo::Blacklist => vec![],
        }
    }

    pub fn action_info(&self, language: Language) -> &'static str {
        match self {
            FileInfo::Directory => select_dest_directory_translation(language),
            _ => select_file_translation(language),
        }
    }
}
