//! Module defining the `IpfixExporter` struct, which identifies the origin of IPFIX flow records.

use std::fmt;
use std::net::IpAddr;

/// Struct representing the identity of an IPFIX exporter, as observed by the collector.
///
/// An exporter is identified by the address it sends from and by the observation domain it
/// declares in the message header: a single device can export several observation domains,
/// which are distinct sources as far as the collector is concerned.
///
/// The transport source port is deliberately not part of the identity: it's ephemeral for most
/// exporters, so including it would make the same device appear as a new one after every restart.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpfixExporter {
    /// Address the flow records were received from.
    pub addr: IpAddr,
    /// Observation domain declared in the IPFIX message header.
    pub observation_domain_id: u32,
}

impl fmt::Display for IpfixExporter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (ODID {})", self.addr, self.observation_domain_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    fn exporter(addr: IpAddr, observation_domain_id: u32) -> IpfixExporter {
        IpfixExporter {
            addr,
            observation_domain_id,
        }
    }

    fn v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(a, b, c, d))
    }

    #[test]
    fn test_ipfix_exporter_display() {
        assert_eq!(
            exporter(v4(203, 0, 113, 9), 0).to_string(),
            "203.0.113.9 (ODID 0)"
        );
        assert_eq!(
            exporter(
                IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
                3
            )
            .to_string(),
            "2001:db8::1 (ODID 3)"
        );
    }

    #[test]
    fn test_ipfix_exporter_sorts_by_address_then_domain() {
        let mut exporters = vec![
            exporter(v4(10, 0, 0, 2), 1),
            exporter(v4(9, 0, 0, 1), 0),
            exporter(v4(10, 0, 0, 2), 0),
        ];
        exporters.sort_unstable();

        // addresses are ordered numerically (not lexicographically: 9.x would come last)
        // and the domains of a same address are kept together
        assert_eq!(
            exporters,
            [
                exporter(v4(9, 0, 0, 1), 0),
                exporter(v4(10, 0, 0, 2), 0),
                exporter(v4(10, 0, 0, 2), 1),
            ]
        );
    }
}
