/// Enum representing the possible traffic direction (incoming or outgoing).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrafficDirection {
    /// Incoming traffic (from remote address to local interface)
    Incoming,
    /// Outgoing traffic (from local interface to remote address)
    Outgoing,
}

impl Default for TrafficDirection {
    fn default() -> Self {
        Self::Incoming
    }
}
