use std::collections::HashSet;
use std::fmt;
use std::net::IpAddr;
use std::sync::Arc;

use ipnet::IpNet;
use prefix_trie::joint::set::JointPrefixSet;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BlacklistSource {
    None,
    File(String),
    Remote {
        url: String,
        min_score: u32,
        cache_path: String,
        last_update: u64,
    },
}

impl Default for BlacklistSource {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for BlacklistSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlacklistSource::None => write!(f, "None"),
            BlacklistSource::File(_) => write!(f, "Local file"),
            BlacklistSource::Remote { .. } => write!(f, "Remote (ipsum)"),
        }
    }
}

impl BlacklistSource {
    pub fn is_remote(&self) -> bool {
        matches!(self, BlacklistSource::Remote { .. })
    }

    pub fn is_file(&self) -> bool {
        matches!(self, BlacklistSource::File(_))
    }
}

#[derive(Clone, Default, Debug)]
pub struct IpBlacklist {
    ips: Arc<HashSet<IpAddr>>,
    cidrs: Arc<JointPrefixSet<IpNet>>,
    is_loading: bool,
    source: BlacklistSource,
}

impl IpBlacklist {
    pub async fn from_file(path: String) -> Self {
        let Ok(buf) = tokio::fs::read_to_string(&path).await else {
            return IpBlacklist::default();
        };
        let (ips, cidrs) = Self::parse_content(&buf);
        IpBlacklist {
            ips: Arc::new(ips),
            cidrs: Arc::new(cidrs),
            is_loading: false,
            source: BlacklistSource::File(path),
        }
    }

    pub async fn from_remote(url: String, min_score: u32, cache_path: String) -> Self {
        // First try to load from cache
        let mut blacklist = Self::load_from_cache(&cache_path).await;
        
        // Also fetch from remote
        let remote_result = Self::fetch_and_parse_remote(&url, min_score).await;
        
        if let Some((remote_ips, remote_cidrs)) = remote_result {
            // Merge remote data with cached data
            let mut merged_ips = (*blacklist.ips).clone();
            let mut merged_cidrs = (*blacklist.cidrs).clone();
            
            merged_ips.extend(remote_ips);
            for cidr in remote_cidrs {
                merged_cidrs.insert(cidr);
            }
            
            blacklist.ips = Arc::new(merged_ips);
            blacklist.cidrs = Arc::new(merged_cidrs);
        }
        
        blacklist.source = BlacklistSource::Remote {
            url: url.clone(),
            min_score,
            cache_path: cache_path.clone(),
            last_update: 0,
        };
        blacklist.start_loading();
        blacklist
    }

    async fn load_from_cache(cache_path: &str) -> Self {
        let Ok(buf) = tokio::fs::read_to_string(cache_path).await else {
            return IpBlacklist::default();
        };
        let (ips, cidrs) = Self::parse_content(&buf);
        IpBlacklist {
            ips: Arc::new(ips),
            cidrs: Arc::new(cidrs),
            is_loading: false,
            source: BlacklistSource::None,
        }
    }

    async fn fetch_and_parse_remote(url: &str, min_score: u32) -> Option<(HashSet<IpAddr>, JointPrefixSet<IpNet>)> {
        let response = reqwest::get(url).await.ok()?;
        let text = response.text().await.ok()?;
        let (ips, cidrs) = Self::parse_ipsum_content(&text, min_score);
        if ips.is_empty() && cidrs.is_empty() {
            None
        } else {
            Some((ips, cidrs))
        }
    }

    fn parse_content(content: &str) -> (HashSet<IpAddr>, JointPrefixSet<IpNet>) {
        let mut ips = HashSet::new();
        let mut cidrs = JointPrefixSet::new();

        for line in content.lines() {
            let Some(first) = line.split_whitespace().next() else {
                continue;
            };

            if let Ok(ip) = first.parse::<IpAddr>() {
                ips.insert(ip);
            } else if let Ok(cidr) = first.parse::<IpNet>() {
                cidrs.insert(cidr);
            }
        }

        (ips, cidrs)
    }

    fn parse_ipsum_content(content: &str, min_score: u32) -> (HashSet<IpAddr>, JointPrefixSet<IpNet>) {
        let mut ips = HashSet::new();
        let mut cidrs = JointPrefixSet::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 2 {
                continue;
            }

            let ip_str = parts[0];
            let score: u32 = parts[1].parse().unwrap_or(0);

            if score < min_score {
                continue;
            }

            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                ips.insert(ip);
            } else if let Ok(cidr) = ip_str.parse::<IpNet>() {
                cidrs.insert(cidr);
            }
        }

        (ips, cidrs)
    }

    pub fn contains(&self, ip: &IpAddr) -> bool {
        self.ips.contains(ip) || self.cidrs.get_lpm(&IpNet::from(*ip)).is_some()
    }

    pub fn is_invalid(&self) -> bool {
        self.ips.is_empty() && self.cidrs.is_empty() && !self.is_loading
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn imported_items_info(&self) -> Option<String> {
        match (self.ips.len(), self.cidrs.len()) {
            (0, 0) => None,
            (ips, 0) => Some(format!("(IPs: {ips})")),
            (0, cidrs) => Some(format!("(CIDRs: {cidrs})")),
            (ips, cidrs) => Some(format!("(IPs: {ips}, CIDRs: {cidrs})")),
        }
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
        assert_eq!(blacklist.cidrs.len(), 0);
        assert_eq!(
            blacklist.imported_items_info(),
            Some("(IPs: 4)".to_string())
        );

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
        assert_eq!(blacklist.cidrs.len(), 0);
        assert_eq!(blacklist.imported_items_info(), None);

        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))));
        assert!(!blacklist.contains(&"::123".parse::<IpAddr>().unwrap()));
        assert!(!blacklist.contains(&"::".parse::<IpAddr>().unwrap()));
    }

    #[tokio::test]
    async fn test_ip_blacklist_valid_with_cidr() {
        let blacklist =
            IpBlacklist::from_file("resources/test/ip_blacklist_valid_with_cidr.txt".to_string())
                .await;

        assert!(!blacklist.is_invalid());
        assert!(!blacklist.is_loading());
        assert_eq!(blacklist.ips.len(), 2);
        assert_eq!(blacklist.cidrs.len(), 4);
        assert_eq!(
            blacklist.imported_items_info(),
            Some("(IPs: 2, CIDRs: 4)".to_string())
        );

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(blacklist.contains(&"2001:db8::1".parse::<IpAddr>().unwrap()));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 9))));
        assert!(!blacklist.contains(&"2001:db8::2".parse::<IpAddr>().unwrap()));

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 3, 1))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 3, 255))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 5, 1))));

        assert!(blacklist.contains(&"2001:db9::1".parse::<IpAddr>().unwrap()));
        assert!(blacklist.contains(&"2001:db9:ffff::1".parse::<IpAddr>().unwrap()));
        assert!(!blacklist.contains(&"2001:dba::1".parse::<IpAddr>().unwrap()));

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(5, 6, 7, 10))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(5, 6, 8, 10))));

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(9, 9, 10, 9))));
    }

    #[tokio::test]
    async fn test_ip_blacklist_valid_with_cidr_only() {
        let blacklist =
            IpBlacklist::from_file("resources/test/ip_blacklist_valid_cidr_only.txt".to_string())
                .await;

        assert!(!blacklist.is_invalid());
        assert!(!blacklist.is_loading());
        assert_eq!(blacklist.ips.len(), 0);
        assert_eq!(blacklist.cidrs.len(), 1);
        assert_eq!(
            blacklist.imported_items_info(),
            Some("(CIDRs: 1)".to_string())
        );

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 3, 1))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(1, 2, 4, 1))));
    }

    #[tokio::test]
    async fn test_ip_blacklist_real_cidr_ranges() {
        let blacklist =
            IpBlacklist::from_file("resources/test/ip_blacklist_real_cidr_ranges.txt".to_string())
                .await;

        assert!(!blacklist.is_invalid());
        assert_eq!(blacklist.ips.len(), 0);
        assert_eq!(blacklist.cidrs.len(), 6);
        assert_eq!(
            blacklist.imported_items_info(),
            Some("(CIDRs: 6)".to_string())
        );

        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 20, 0))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 23, 255))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 24, 0))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 31, 255))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 224, 0))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 231, 255))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 232, 0))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 235, 255))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 236, 0))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 236, 255))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 233, 156, 0))));
        assert!(blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 233, 159, 255))));

        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 19, 255))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 32, 0))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 186, 237, 0))));
        assert!(!blacklist.contains(&IpAddr::V4(Ipv4Addr::new(209, 233, 160, 0))));
    }

    #[test]
    fn test_parse_ipsum_content() {
        let content = "193.46.255.86\t10\n1.2.3.4\t5\n# comment\n203.0.113.0/24\t15\n";
        let (ips, cidrs) = IpBlacklist::parse_ipsum_content(content, 5);

        assert_eq!(ips.len(), 2);
        assert!(ips.contains(&"193.46.255.86".parse::<IpAddr>().unwrap()));
        assert!(ips.contains(&"1.2.3.4".parse::<IpAddr>().unwrap()));
        assert_eq!(cidrs.len(), 1);
        assert!(cidrs.get_lpm(&"203.0.113.0/24".parse::<IpNet>().unwrap()).is_some());
    }

    #[test]
    fn test_parse_ipsum_content_min_score() {
        let content = "193.46.255.86\t10\n1.2.3.4\t5\n";
        let (ips, _) = IpBlacklist::parse_ipsum_content(content, 8);

        assert_eq!(ips.len(), 1);
        assert!(ips.contains(&"193.46.255.86".parse::<IpAddr>().unwrap()));
    }
}