#![allow(clippy::upper_case_acronyms)]
use crate::utility::translations::both_translation;
use crate::Language;
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
        write!(f, "{self:?}")
    }
}

impl TransProtocol {
    pub(crate) const ALL: [TransProtocol; 3] =
        [TransProtocol::TCP, TransProtocol::UDP, TransProtocol::Other];

    pub fn get_radio_label(&self, language: Language) -> &str {
        match self {
            TransProtocol::TCP => "TCP",
            TransProtocol::UDP => "UDP",
            TransProtocol::Other => both_translation(language),
        }
    }
}
