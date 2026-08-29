use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Source/destination IPs of a packet, paired by family so mixed-family
/// values are unrepresentable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetAddrs {
    V4 { src: Ipv4Addr, dst: Ipv4Addr },
    V6 { src: Ipv6Addr, dst: Ipv6Addr },
}

impl NetAddrs {
    #[must_use]
    pub fn src(self) -> IpAddr {
        match self {
            NetAddrs::V4 { src, .. } => IpAddr::V4(src),
            NetAddrs::V6 { src, .. } => IpAddr::V6(src),
        }
    }

    #[must_use]
    pub fn dst(self) -> IpAddr {
        match self {
            NetAddrs::V4 { dst, .. } => IpAddr::V4(dst),
            NetAddrs::V6 { dst, .. } => IpAddr::V6(dst),
        }
    }
}
