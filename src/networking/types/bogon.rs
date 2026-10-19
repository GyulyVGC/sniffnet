#![allow(clippy::unwrap_used)]

use ipnet::IpNet;
use std::net::IpAddr;

pub struct Bogon {
    pub ranges: Vec<IpNet>,
    pub description: &'static str,
}

// IPv4 bogons

static THIS_NETWORK: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["0.0.0.0/8".parse().unwrap()],
    description: "\"this\" network",
});

static PRIVATE_USE: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec![
        "10.0.0.0/8".parse().unwrap(),
        "172.16.0.0/12".parse().unwrap(),
        "192.168.0.0/16".parse().unwrap(),
    ],
    description: "private-use",
});

static CARRIER_GRADE: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["100.64.0.0/10".parse().unwrap()],
    description: "carrier-grade NAT",
});

static LOOPBACK: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["127.0.0.0/8".parse().unwrap()],
    description: "loopback",
});

static LINK_LOCAL: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["169.254.0.0/16".parse().unwrap()],
    description: "link local",
});

static IETF_PROTOCOL: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["192.0.0.0/24".parse().unwrap()],
    description: "IETF protocol assignments",
});

static TEST_NET_1: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["192.0.2.0/24".parse().unwrap()],
    description: "TEST-NET-1",
});

static NETWORK_INTERCONNECT: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["198.18.0.0/15".parse().unwrap()],
    description: "network interconnect device benchmark testing",
});

static TEST_NET_2: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["198.51.100.0/24".parse().unwrap()],
    description: "TEST-NET-2",
});

static TEST_NET_3: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["203.0.113.0/24".parse().unwrap()],
    description: "TEST-NET-3",
});

static MULTICAST: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["224.0.0.0/4".parse().unwrap()],
    description: "multicast",
});

static FUTURE_USE: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["240.0.0.0/4".parse().unwrap()],
    description: "future use",
});

// IPv6 bogons

static NODE_SCOPE_UNSPECIFIED: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["::/128".parse().unwrap()],
    description: "node-scope unicast unspecified",
});

static NODE_SCOPE_LOOPBACK: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["::1/128".parse().unwrap()],
    description: "node-scope unicast loopback",
});

static IPV4_MAPPED: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["::ffff:0.0.0.0/96".parse().unwrap()],
    description: "IPv4-mapped",
});

static IPV4_COMPATIBLE: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["::/96".parse().unwrap()],
    description: "IPv4-compatible",
});

static REMOTELY_TRIGGERED: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["100::/64".parse().unwrap()],
    description: "remotely triggered black hole",
});

static ORCHID: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["2001:10::/28".parse().unwrap()],
    description: "ORCHID",
});

static DOCUMENTATION_PREFIX: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec![
        "2001:db8::/32".parse().unwrap(),
        "3fff::/20".parse().unwrap(),
    ],
    description: "documentation prefix",
});

static ULA: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["fc00::/7".parse().unwrap()],
    description: "ULA",
});

static LINK_LOCAL_UNICAST: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["fe80::/10".parse().unwrap()],
    description: "link-local unicast",
});

static SITE_LOCAL_UNICAST: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["fec0::/10".parse().unwrap()],
    description: "site-local unicast",
});

static MULTICAST_V6: std::sync::LazyLock<Bogon> = std::sync::LazyLock::new(|| Bogon {
    ranges: vec!["ff00::/8".parse().unwrap()],
    description: "multicast v6",
});

// all bogons

static BOGONS: std::sync::LazyLock<Vec<&'static Bogon>> = std::sync::LazyLock::new(|| {
    vec![
        &THIS_NETWORK,
        &PRIVATE_USE,
        &CARRIER_GRADE,
        &LOOPBACK,
        &LINK_LOCAL,
        &IETF_PROTOCOL,
        &TEST_NET_1,
        &NETWORK_INTERCONNECT,
        &TEST_NET_2,
        &TEST_NET_3,
        &MULTICAST,
        &FUTURE_USE,
        &NODE_SCOPE_UNSPECIFIED,
        &NODE_SCOPE_LOOPBACK,
        &IPV4_MAPPED,
        &IPV4_COMPATIBLE,
        &REMOTELY_TRIGGERED,
        &ORCHID,
        &DOCUMENTATION_PREFIX,
        &ULA,
        &LINK_LOCAL_UNICAST,
        &SITE_LOCAL_UNICAST,
        &MULTICAST_V6,
    ]
});

pub fn is_bogon(address: &IpAddr) -> Option<&'static str> {
    for bogon in BOGONS.iter() {
        if bogon.ranges.iter().any(|net| net.contains(address)) {
            return Some(bogon.description);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_is_bogon_no() {
        assert_eq!(is_bogon(&IpAddr::from_str("8.8.8.8").unwrap()), None);
        assert_eq!(
            is_bogon(&IpAddr::from_str("2001:4860:4860::8888").unwrap()),
            None
        );
    }

    #[test]
    fn test_is_bogon_this_network() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("0.1.2.3").unwrap()),
            Some("\"this\" network")
        );
    }

    #[test]
    fn test_is_bogon_private_use_networks() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("10.1.2.3").unwrap()),
            Some("private-use")
        );
        assert_eq!(
            is_bogon(&IpAddr::from_str("172.22.2.3").unwrap()),
            Some("private-use")
        );
        assert_eq!(
            is_bogon(&IpAddr::from_str("192.168.255.3").unwrap()),
            Some("private-use")
        );
    }

    #[test]
    fn test_is_bogon_carrier_grade() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("100.99.2.1").unwrap()),
            Some("carrier-grade NAT")
        );
    }

    #[test]
    fn test_is_bogon_loopback() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("127.99.2.1").unwrap()),
            Some("loopback")
        );
    }

    #[test]
    fn test_is_bogon_link_local() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("169.254.0.0").unwrap()),
            Some("link local")
        );
    }

    #[test]
    fn test_is_bogon_ietf() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("192.0.0.255").unwrap()),
            Some("IETF protocol assignments")
        );
    }

    #[test]
    fn test_is_bogon_test_net_1() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("192.0.2.128").unwrap()),
            Some("TEST-NET-1")
        );
    }

    #[test]
    fn test_is_bogon_network_interconnect() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("198.18.2.128").unwrap()),
            Some("network interconnect device benchmark testing")
        );
    }

    #[test]
    fn test_is_bogon_test_net_2() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("198.51.100.128").unwrap()),
            Some("TEST-NET-2")
        );
    }

    #[test]
    fn test_is_bogon_test_net_3() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("203.0.113.128").unwrap()),
            Some("TEST-NET-3")
        );
    }

    #[test]
    fn test_is_bogon_multicast() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("224.12.13.255").unwrap()),
            Some("multicast")
        );
    }

    #[test]
    fn test_is_bogon_future_use() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("240.0.0.0").unwrap()),
            Some("future use")
        );
    }

    #[test]
    fn test_node_scope_unspecified() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("::").unwrap()),
            Some("node-scope unicast unspecified")
        );
    }

    #[test]
    fn test_node_scope_loopback() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("::1").unwrap()),
            Some("node-scope unicast loopback")
        );
    }

    #[test]
    fn test_ipv4_mapped() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("::ffff:8.8.8.8").unwrap()),
            Some("IPv4-mapped")
        );
    }

    #[test]
    fn test_ipv4_compatible() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("::8.8.8.8").unwrap()),
            Some("IPv4-compatible")
        );
    }

    #[test]
    fn test_remotely_triggered() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("100::beef").unwrap()),
            Some("remotely triggered black hole")
        );
    }

    #[test]
    fn test_orchid() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("2001:10::feed").unwrap()),
            Some("ORCHID")
        );
    }

    #[test]
    fn test_documentation_prefix() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("2001:db8::fe90").unwrap()),
            Some("documentation prefix")
        );
        assert_eq!(
            is_bogon(&IpAddr::from_str("3fff::").unwrap()),
            Some("documentation prefix")
        );
    }

    #[test]
    fn test_ula() {
        assert_eq!(is_bogon(&IpAddr::from_str("fdff::").unwrap()), Some("ULA"));
    }

    #[test]
    fn test_link_local_unicast() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("feaf::").unwrap()),
            Some("link-local unicast")
        );
    }

    #[test]
    fn test_site_local_unicast() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("feea::1").unwrap()),
            Some("site-local unicast")
        );
    }

    #[test]
    fn test_is_bogon_multicast_v6() {
        assert_eq!(
            is_bogon(&IpAddr::from_str("ff02::1").unwrap()),
            Some("multicast v6")
        );
    }
}
