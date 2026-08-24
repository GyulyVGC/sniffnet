use std::net::SocketAddr;
use std::time::Instant;

use crate::networking::ipfix::ttl_map::TtlMap;
use crate::networking::ipfix::wire::FieldSpec;

/// Per-exporter IPFIX template cache.
/// The cache is keyed by `(peer, observation_domain, template_id)`:
/// the spec requires scoping templates by transport session + ODID,
/// so we can't use `IpfixExporter` because also the port number is needed.
pub(super) struct TemplateCache {
    map: TtlMap<(SocketAddr, u32, u16), Vec<FieldSpec>>,
}

impl TemplateCache {
    pub(super) fn new(now: Instant) -> Self {
        Self {
            map: TtlMap::new(now),
        }
    }

    pub(super) fn insert(
        &mut self,
        peer: SocketAddr,
        observation_domain_id: u32,
        template_id: u16,
        fields: Vec<FieldSpec>,
        now: Instant,
    ) {
        self.map
            .insert((peer, observation_domain_id, template_id), fields, now);
    }

    pub(super) fn get(
        &mut self,
        peer: SocketAddr,
        observation_domain_id: u32,
        template_id: u16,
        now: Instant,
    ) -> Option<&[FieldSpec]> {
        self.map
            .get(&(peer, observation_domain_id, template_id), now)
            .map(Vec::as_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::ipfix::ttl_map::ENTRY_TTL;
    use std::time::Duration;

    fn peer(port: u16) -> SocketAddr {
        format!("127.0.0.1:{port}").parse().unwrap()
    }

    fn fields(ie_id: u16) -> Vec<FieldSpec> {
        vec![FieldSpec {
            ie_id,
            length: 4,
            enterprise: None,
        }]
    }

    #[test]
    fn test_template_cache_insertion_and_lookup() {
        let now = Instant::now();
        let mut cache = TemplateCache::new(now);
        let f1 = fields(8);
        let f2 = fields(12);
        let f3 = fields(16);
        let f4 = fields(20);
        cache.insert(peer(1000), 0, 256, f1.clone(), now);
        cache.insert(peer(1001), 0, 256, f2.clone(), now);
        cache.insert(peer(1000), 1, 256, f3.clone(), now);
        cache.insert(peer(1001), 0, 257, f4.clone(), now);

        assert_eq!(cache.get(peer(1000), 0, 256, now), Some(f1.as_slice()));
        assert_eq!(cache.get(peer(1001), 0, 256, now), Some(f2.as_slice()));
        assert_eq!(cache.get(peer(1000), 1, 256, now), Some(f3.as_slice()));
        assert_eq!(cache.get(peer(1001), 0, 257, now), Some(f4.as_slice()));

        assert_eq!(cache.get(peer(1000), 0, 257, now), None);
        assert_eq!(cache.get(peer(1001), 1, 256, now), None);
        assert_eq!(cache.get(peer(1002), 0, 256, now), None);

        let replacement = vec![
            FieldSpec {
                ie_id: 8,
                length: 4,
                enterprise: None,
            },
            FieldSpec {
                ie_id: 12,
                length: 4,
                enterprise: None,
            },
        ];
        cache.insert(peer(1000), 0, 256, replacement.clone(), now);
        assert_eq!(
            cache.get(peer(1000), 0, 256, now),
            Some(replacement.as_slice())
        );
    }

    #[test]
    fn test_template_cache_still_decoding_records_is_never_evicted() {
        let start = Instant::now();
        let mut cache = TemplateCache::new(start);
        cache.insert(peer(1000), 0, 256, fields(8), start);

        let mut now = start;
        for _ in 0..3 {
            now += ENTRY_TTL - Duration::from_secs(1);
            assert!(cache.get(peer(1000), 0, 256, now).is_some());
        }
        assert!(now.duration_since(start) > ENTRY_TTL * 2);
        assert!(cache.get(peer(1000), 0, 256, now).is_some());
    }

    #[test]
    fn test_template_cache_departed_exporter_is_evicted() {
        let start = Instant::now();
        let mut cache = TemplateCache::new(start);
        cache.insert(peer(1000), 0, 256, fields(8), start);

        let long_after = start + ENTRY_TTL + Duration::from_secs(1);
        assert_eq!(cache.get(peer(1000), 0, 256, long_after), None);
    }
}
