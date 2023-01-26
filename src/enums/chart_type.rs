/// Enum representing the possible kind of chart displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChartType {
    Packets,
    Bytes,
}

impl ChartType {
    pub(crate) const ALL: [ChartType; 2] = [ChartType::Packets, ChartType::Bytes];

    pub fn get_radio_label(&self) -> &str {
        match self {
            ChartType::Packets => "packets per second",
            ChartType::Bytes => "bytes per second",
        }
    }
}
