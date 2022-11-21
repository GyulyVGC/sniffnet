use std::fmt;

/// Enum representing the possible observed values of transport layer protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransProtocol {
    /// Transmission Control Protocol
    TCP,
    /// User Datagram Protocol
    UDP,
    /// Not identified
    Other,
}


impl fmt::Display for TransProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}