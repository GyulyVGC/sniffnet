/// Enum representing the possible traffic type (unicast, multicast or broadcast).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TrafficType {
    /// Unicast traffic
    Unicast,
    /// Multicast traffic (destination is a multicast address)
    Multicast,
    /// Broadcast traffic (destination is a broadcast address)
    Broadcast,
}

impl Default for TrafficType {
    fn default() -> Self {
        Self::Unicast
    }
}
