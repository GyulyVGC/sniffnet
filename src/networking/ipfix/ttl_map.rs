use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// How long an entry survives without being touched.
pub(super) const ENTRY_TTL: Duration = Duration::from_mins(30);

/// How often expired entries are checked for sweeping.
pub(super) const PRUNE_INTERVAL: Duration = Duration::from_mins(1);

/// A hash map whose entries expire after a period without use
pub(super) struct TtlMap<K, V> {
    map: HashMap<K, Aging<V>>,
    last_prune: Instant,
}

struct Aging<V> {
    value: V,
    last_used: Instant,
}

impl<K: Eq + Hash, V> TtlMap<K, V> {
    pub(super) fn new(now: Instant) -> Self {
        Self {
            map: HashMap::new(),
            last_prune: now,
        }
    }

    pub(super) fn insert(&mut self, key: K, value: V, now: Instant) {
        self.maybe_prune(now);

        self.map.insert(
            key,
            Aging {
                value,
                last_used: now,
            },
        );
    }

    pub(super) fn get(&mut self, key: &K, now: Instant) -> Option<&V> {
        self.maybe_prune(now);

        let aging = self.map.get_mut(key)?;
        aging.last_used = now;
        Some(&aging.value)
    }

    pub(super) fn get_or_insert_with(
        &mut self,
        key: K,
        now: Instant,
        default: impl FnOnce() -> V,
    ) -> &mut V {
        self.maybe_prune(now);

        let aging = self.map.entry(key).or_insert_with(|| Aging {
            value: default(),
            last_used: now,
        });
        aging.last_used = now;
        &mut aging.value
    }

    fn maybe_prune(&mut self, now: Instant) {
        if now.duration_since(self.last_prune) < PRUNE_INTERVAL {
            return;
        }
        self.map
            .retain(|_, aging| now.duration_since(aging.last_used) < ENTRY_TTL);
        self.last_prune = now;
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn after_entry_ttl(from: Instant) -> Instant {
        from + ENTRY_TTL + Duration::from_secs(1)
    }

    fn after_prune_interval(from: Instant) -> Instant {
        from + PRUNE_INTERVAL + Duration::from_secs(1)
    }

    #[test]
    fn test_stores_and_reads_back() {
        let now = Instant::now();
        let mut map = TtlMap::new(now);
        assert_eq!(map.len(), 0);
        map.insert("k", 1, now);
        assert_eq!(map.get(&"k", now), Some(&1));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"absent", now), None);
        map.insert("k", 2, now);
        assert_eq!(map.get(&"k", now), Some(&2));
        assert_eq!(map.len(), 1);
        map.insert("x", 2, now);
        assert_eq!(map.get(&"x", now), Some(&2));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_idle_entries_are_swept() {
        let start = Instant::now();
        let mut map = TtlMap::new(start);
        map.insert("k", 1, start);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"k", after_entry_ttl(start)), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_reading_an_entry_keeps_it_alive() {
        let start = Instant::now();
        let mut map = TtlMap::new(start);
        map.insert("k", 1, start);

        let mut now = start;
        for _ in 0..3 {
            now += ENTRY_TTL - Duration::from_secs(1);
            assert_eq!(map.get(&"k", now), Some(&1));
        }

        assert!(now.duration_since(start) > ENTRY_TTL * 2);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_sweeping_spares_entries_still_in_use() {
        let start = Instant::now();
        let mut map = TtlMap::new(start);
        map.insert("used", 1, start);
        map.insert("idle", 2, start);
        assert_eq!(map.len(), 2);

        let mid = start + ENTRY_TTL - Duration::from_secs(1);
        assert_eq!(map.get_or_insert_with("used", mid, || 3), &1);
        let sweep = after_prune_interval(mid);
        assert_eq!(map.get(&"idle", sweep), None);
        assert_eq!(map.get(&"used", sweep), Some(&1));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_get_or_insert_with_creates_then_reuses() {
        let now = Instant::now();
        let mut map = TtlMap::new(now);
        *map.get_or_insert_with("k", now, || 10) += 1;
        assert_eq!(map.get(&"k", now), Some(&11));
        *map.get_or_insert_with("k", now, || 10) += 1;
        assert_eq!(map.get(&"k", now), Some(&12));
    }
}
