#![allow(clippy::enum_variant_names)]
/// Enum representing the possible kinds of displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn get_radio_label(&self) -> &str {
        match self {
            ReportType::MostRecent => "most recent",
            ReportType::MostPackets => "most packets",
            ReportType::MostBytes => "most bytes",
            ReportType::Favorites => "favorites",
        }
    }
}
