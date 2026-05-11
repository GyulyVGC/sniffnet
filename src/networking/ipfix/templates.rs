//! Per-exporter IPFIX template cache.
//!
//! Templates are carried in their own sets and referenced by data records that
//! arrive in later packets. The cache is keyed by `(peer, observation_domain,
//! template_id)` so a misbehaving exporter cannot corrupt another's templates.

use std::collections::HashMap;
use std::net::SocketAddr;

use crate::networking::ipfix::wire::FieldSpec;

#[derive(Default, Debug)]
pub struct TemplateCache {
    map: HashMap<(SocketAddr, u32, u16), Vec<FieldSpec>>,
}

impl TemplateCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        peer: SocketAddr,
        observation_domain_id: u32,
        template_id: u16,
        fields: Vec<FieldSpec>,
    ) {
        self.map
            .insert((peer, observation_domain_id, template_id), fields);
    }

    pub fn get(
        &self,
        peer: SocketAddr,
        observation_domain_id: u32,
        template_id: u16,
    ) -> Option<&[FieldSpec]> {
        self.map
            .get(&(peer, observation_domain_id, template_id))
            .map(Vec::as_slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peer(port: u16) -> SocketAddr {
        format!("127.0.0.1:{port}").parse().unwrap()
    }

    #[test]
    fn insertion_and_lookup_are_per_peer() {
        let mut cache = TemplateCache::new();
        let f1 = vec![FieldSpec {
            ie_id: 8,
            length: 4,
            enterprise: None,
        }];
        let f2 = vec![FieldSpec {
            ie_id: 12,
            length: 4,
            enterprise: None,
        }];
        cache.insert(peer(1000), 0, 256, f1.clone());
        cache.insert(peer(1001), 0, 256, f2.clone());

        assert_eq!(cache.get(peer(1000), 0, 256), Some(f1.as_slice()));
        assert_eq!(cache.get(peer(1001), 0, 256), Some(f2.as_slice()));
        assert_eq!(cache.get(peer(1002), 0, 256), None);
    }

    #[test]
    fn redefinition_replaces_existing_template() {
        let mut cache = TemplateCache::new();
        let original = vec![FieldSpec {
            ie_id: 8,
            length: 4,
            enterprise: None,
        }];
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
        cache.insert(peer(1000), 0, 256, original);
        cache.insert(peer(1000), 0, 256, replacement.clone());
        assert_eq!(cache.get(peer(1000), 0, 256), Some(replacement.as_slice()));
    }
}
