/// Enum representing the possible traffic type (incoming, outgoing or multicast).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrafficType {
    /// Incoming traffic (from remote address to local interface)
    Incoming,
    /// Outgoing traffic (from local interface to remote address)
    Outgoing,
    /// Multicast traffic (from remote address to multicast address)
    Multicast,
    /// Not identified
    Other,
}
