#![allow(clippy::enum_variant_names)]
/// Enum representing the possible kinds of displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportType {
    MostRecent,
    MostPackets,
    MostBytes,
    Favorites,
}
