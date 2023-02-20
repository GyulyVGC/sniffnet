use crate::utility::translations::both_translation;
use crate::Language;
use std::fmt;

/// Enum representing the possible observed values of IP protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpVersion {
    /// Internet Protocol version 4
    IPv4,
    /// Internet Protocol version 6
    IPv6,
    /// Not identified
    Other,
}

impl fmt::Display for IpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IpVersion {
    pub(crate) const ALL: [IpVersion; 3] = [IpVersion::IPv4, IpVersion::IPv6, IpVersion::Other];

    pub fn get_radio_label(&self, language: Language) -> &str {
        match self {
            IpVersion::IPv4 => "IPv4",
            IpVersion::IPv6 => "IPv6",
            IpVersion::Other => both_translation(language),
        }
    }
}
