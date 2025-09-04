use std::fmt;

/// Enum representing the possible observed values of IP protocol version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpVersion {
    /// Internet Protocol version 4
    IPv4,
    /// Internet Protocol version 6
    IPv6,
}

impl fmt::Display for IpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
