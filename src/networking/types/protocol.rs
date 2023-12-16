use std::fmt;

/// Enum representing the possible observed values of protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Protocol {
    /// Transmission Control Protocol
    TCP,
    /// User Datagram Protocol
    UDP,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Protocol {
    pub(crate) const ALL: [Protocol; 2] = [Protocol::TCP, Protocol::UDP];
}
