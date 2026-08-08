//! Per-exporter IPFIX template cache.
//!
//! Templates are carried in their own sets and referenced by data records that
//! arrive in later packets. The cache is keyed by `(peer, observation_domain,
//! template_id)` so a misbehaving exporter cannot corrupt another's templates.
//!
//! Entries expire once they stop being used, so templates belonging to an
//! exporter that has gone away don't accumulate for the life of the capture.
//! Decoding a record counts as use, which matters for exporters that announce
//! their templates far less often than they send data.

use std::net::SocketAddr;
use std::time::Instant;

use crate::networking::ipfix::ttl_map::TtlMap;
use crate::networking::ipfix::wire::FieldSpec;

pub struct TemplateCache {
    map: TtlMap<(SocketAddr, u32, u16), Vec<FieldSpec>>,
}

impl TemplateCache {
    pub fn new(now: Instant) -> Self {
        Self {
            map: TtlMap::new(now),
        }
    }

    pub fn insert(
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

    pub fn get(
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
    fn insertion_and_lookup_are_per_peer() {
        let now = Instant::now();
        let mut cache = TemplateCache::new(now);
        let f1 = fields(8);
        let f2 = fields(12);
        cache.insert(peer(1000), 0, 256, f1.clone(), now);
        cache.insert(peer(1001), 0, 256, f2.clone(), now);

        assert_eq!(cache.get(peer(1000), 0, 256, now), Some(f1.as_slice()));
        assert_eq!(cache.get(peer(1001), 0, 256, now), Some(f2.as_slice()));
        assert_eq!(cache.get(peer(1002), 0, 256, now), None);
    }

    #[test]
    fn redefinition_replaces_existing_template() {
        let now = Instant::now();
        let mut cache = TemplateCache::new(now);
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
        cache.insert(peer(1000), 0, 256, fields(8), now);
        cache.insert(peer(1000), 0, 256, replacement.clone(), now);
        assert_eq!(
            cache.get(peer(1000), 0, 256, now),
            Some(replacement.as_slice())
        );
    }

    #[test]
    fn a_template_still_decoding_records_is_never_evicted() {
        // Exporters may announce templates far less often than they send data,
        // so use has to be what keeps one alive.
        let start = Instant::now();
        let mut cache = TemplateCache::new(start);
        cache.insert(peer(1000), 0, 256, fields(8), start);

        let mut now = start;
        for _ in 0..3 {
            now += ENTRY_TTL - Duration::from_secs(1);
            assert!(cache.get(peer(1000), 0, 256, now).is_some());
        }
        assert!(now.duration_since(start) > ENTRY_TTL * 2);
    }

    #[test]
    fn a_template_from_a_departed_exporter_is_evicted() {
        let start = Instant::now();
        let mut cache = TemplateCache::new(start);
        cache.insert(peer(1000), 0, 256, fields(8), start);

        let long_after = start + ENTRY_TTL + Duration::from_secs(1);
        assert_eq!(cache.get(peer(1000), 0, 256, long_after), None);
    }
}
