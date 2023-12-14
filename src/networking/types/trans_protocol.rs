use std::fmt;

/// Enum representing the possible observed values of transport layer protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum TransProtocol {
    /// Transmission Control Protocol
    TCP,
    /// User Datagram Protocol
    UDP,
}

impl fmt::Display for TransProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TransProtocol {
    pub(crate) const ALL: [TransProtocol; 2] = [TransProtocol::TCP, TransProtocol::UDP];
}
