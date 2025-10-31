/// Enum representing the possible traffic direction (incoming or outgoing).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TrafficDirection {
    /// Incoming traffic (from remote address to local interface)
    #[default]
    Incoming,
    /// Outgoing traffic (from local interface to remote address)
    Outgoing,
}
