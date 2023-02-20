use crate::utility::translations::{bytes_chart_translation, packets_chart_translation};
use crate::Language;

/// Enum representing the possible kind of chart displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    Packets,
    Bytes,
}

impl ChartType {
    pub(crate) const ALL: [ChartType; 2] = [ChartType::Packets, ChartType::Bytes];

    pub fn get_radio_label(&self, language: Language) -> &str {
        match self {
            ChartType::Packets => packets_chart_translation(language),
            ChartType::Bytes => bytes_chart_translation(language),
        }
    }
}
