use crate::translations::translations::{bytes_translation, packets_translation};
use crate::Language;

/// Enum representing the possible kind of chart displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChartType {
    Packets,
    Bytes,
}

impl ChartType {
    pub(crate) const ALL: [ChartType; 2] = [ChartType::Bytes, ChartType::Packets];

    pub fn get_label(&self, language: Language) -> &str {
        match self {
            ChartType::Packets => packets_translation(language),
            ChartType::Bytes => bytes_translation(language),
        }
    }
}
