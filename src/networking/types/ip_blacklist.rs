use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Clone, Default, Debug)]
pub struct IpBlacklist {
    ips: Arc<HashSet<IpAddr>>,
    is_loading: bool,
}

impl IpBlacklist {
    pub async fn from_file(path: String) -> Self {
        let Ok(buf) = tokio::fs::read_to_string(&path).await else {
            return IpBlacklist::default();
        };
        let mut set = HashSet::new();
        for line in buf.lines() {
            if let Ok(ip) = line.trim().parse::<IpAddr>() {
                set.insert(ip);
            }
        }
        IpBlacklist {
            ips: Arc::new(set),
            is_loading: false,
        }
    }

    pub fn contains(&self, ip: &IpAddr) -> bool {
        self.ips.contains(ip)
    }

    pub fn is_invalid(&self) -> bool {
        self.ips.is_empty() && !self.is_loading
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn start_loading(&mut self) {
        self.is_loading = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_ip_blacklist_valid() {
        let blacklist =
            IpBlacklist::from_file("resources/test/ip_blacklist_valid.txt".to_string()).await;

        assert!(!blacklist.is_invalid());
        assert!(!blacklist.is_loading());
        assert_eq!(blacklist.ips.len(), 4);

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 3, 255))));
        assert!(blacklist.contains(&"::123".parse::<IpAddr>().unwrap()));
        assert!(blacklist.contains(&"fe80::99".parse::<IpAddr>().unwrap()));

        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 9))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(10, 2, 3, 255))));
        assert!(!blacklist.contains(&"::124".parse::<IpAddr>().unwrap()));
        assert!(!blacklist.contains(&"fe80::98".parse::<IpAddr>().unwrap()));
    }

    #[tokio::test]
    async fn test_ip_blacklist_invalid() {
        let blacklist =
            IpBlacklist::from_file("resources/test/ip_blacklist_invalid.txt".to_string()).await;

        assert!(blacklist.is_invalid());
        assert!(!blacklist.is_loading());
        assert_eq!(blacklist.ips.len(), 0);

        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))));
        assert!(!blacklist.contains(&"::123".parse::<IpAddr>().unwrap()));
        assert!(!blacklist.contains(&"::".parse::<IpAddr>().unwrap()));
    }
}
