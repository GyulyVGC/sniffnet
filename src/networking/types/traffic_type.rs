/// Enum representing the possible traffic type (unicast, multicast or broadcast).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum TrafficType {
    /// Unicast traffic
    #[default]
    Unicast,
    /// Multicast traffic (destination is a multicast address)
    Multicast,
    /// Broadcast traffic (destination is a broadcast address)
    Broadcast,
}
