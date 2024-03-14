use crate::translations::translations_3::{
    database_from_file_translation, select_directory_translation, style_from_file_translation,
};
use crate::translations::types::language::Language;

#[derive(Debug, Clone, PartialEq)]
pub enum FileInfo {
    Style,
    Database,
    Directory,
}

impl FileInfo {
    pub fn get_extension(&self) -> &'static str {
        match self {
            FileInfo::Style => "toml",
            FileInfo::Database => "mmdb",
            FileInfo::Directory => "",
        }
    }

    pub fn action_info(&self, language: Language) -> &'static str {
        match self {
            FileInfo::Style => style_from_file_translation(language),
            FileInfo::Database => database_from_file_translation(language),
            FileInfo::Directory => select_directory_translation(language),
        }
    }
}
