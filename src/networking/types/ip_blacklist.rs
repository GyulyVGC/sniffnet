use std::collections::HashSet;
use std::fs;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Clone, Default, Debug)]
pub struct IpBlacklist {
    ips: Arc<HashSet<IpAddr>>,
    is_loading: bool,
}

impl IpBlacklist {
    pub async fn from_file(path: String) -> Self {
        let Ok(buf) = fs::read_to_string(&path) else {
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
