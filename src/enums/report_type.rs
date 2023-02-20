#![allow(clippy::enum_variant_names)]

use crate::utility::translations::{
    bytes_report_translation, favorite_report_translation, packets_report_translation,
    recent_report_translation,
};
use crate::Language;

/// Enum representing the possible kinds of displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    MostRecent,
    MostPackets,
    MostBytes,
    Favorites,
}

impl ReportType {
    pub(crate) const ALL: [ReportType; 4] = [
        ReportType::MostRecent,
        ReportType::MostPackets,
        ReportType::MostBytes,
        ReportType::Favorites,
    ];

    pub fn get_radio_label(&self, language: Language) -> &str {
        match self {
            ReportType::MostRecent => recent_report_translation(language),
            ReportType::MostPackets => packets_report_translation(language),
            ReportType::MostBytes => bytes_report_translation(language),
            ReportType::Favorites => favorite_report_translation(language),
        }
    }
}
