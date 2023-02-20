/// Enum representing the possible traffic type (incoming, outgoing or multicast).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrafficType {
    /// Incoming traffic (from remote address to local interface)
    Incoming,
    /// Outgoing traffic (from local interface to remote address)
    Outgoing,
    /// Multicast traffic (from remote address to multicast address)
    Multicast,
    /// Multicast traffic (from remote address to broadcast address)
    Broadcast,
    /// Not identified
    Other,
}
