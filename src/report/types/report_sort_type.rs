use crate::translations::translations::{
    bytes_report_translation, packets_report_translation, recent_report_translation,
};
use crate::Language;

/// Enum representing the possible kinds of displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::enum_variant_names)]
pub enum ReportSortType {
    MostRecent,
    MostBytes,
    MostPackets,
}

impl ReportSortType {
    pub fn all_strings(language: Language) -> Vec<String> {
        vec![
            recent_report_translation(language).to_string(),
            bytes_report_translation(language).to_string(),
            packets_report_translation(language).to_string(),
        ]
    }

    pub fn get_picklist_label(self, language: Language) -> String {
        match self {
            ReportSortType::MostRecent => recent_report_translation(language),
            ReportSortType::MostBytes => bytes_report_translation(language),
            ReportSortType::MostPackets => packets_report_translation(language),
        }
        .to_string()
    }
}
