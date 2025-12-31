use std::collections::HashSet;
use std::fs;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Clone, Default, Debug)]
pub struct IpBlacklist {
    ips: Arc<HashSet<IpAddr>>,
}

impl IpBlacklist {
    // TODO: optimize loading time for huge files!?
    pub async fn from_file(path: String) -> Self {
        let Ok(buf) = fs::read_to_string(&path) else {
            return IpBlacklist::default();
        };
        // capacity to avoid reallocations and make insertions more efficient (average IP length assumption)
        // TODO: check for overflows when file is big!?
        // let mut set = HashSet::with_capacity(buf.lines().count());
        let mut set = HashSet::new();
        for line in buf.lines() {
            if let Ok(ip) = line.trim().parse::<IpAddr>() {
                set.insert(ip);
            }
        }
        println!("Loaded {} blacklisted IPs from {}", set.len(), path);
        IpBlacklist { ips: Arc::new(set) }
    }

    pub fn contains(&self, ip: &IpAddr) -> bool {
        self.ips.contains(ip)
    }
}
