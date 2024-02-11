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

impl IpVersion {
    pub(crate) const ALL: [IpVersion; 2] = [IpVersion::IPv4, IpVersion::IPv6];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_version_display() {
        for version in IpVersion::ALL {
            match version {
                IpVersion::IPv4 => assert_eq!(version.to_string(), "IPv4"),
                IpVersion::IPv6 => assert_eq!(version.to_string(), "IPv6"),
            }
        }
    }

    #[test]
    fn test_all_ip_versions_collection() {
        assert_eq!(IpVersion::ALL.len(), 2);
        assert_eq!(IpVersion::ALL.get(0).unwrap(), &IpVersion::IPv4);
        assert_eq!(IpVersion::ALL.get(1).unwrap(), &IpVersion::IPv6);
    }
}
