use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// How long an entry survives without being touched.
pub(super) const ENTRY_TTL: Duration = Duration::from_mins(30);

/// How often expired entries are checked for sweeping.
pub(super) const PRUNE_INTERVAL: Duration = Duration::from_mins(1);

/// Fraction of the entries evicted at once when a full map has nothing expired to drop
const EVICTION_FRACTION: usize = 4; // 25%

/// A hash map whose entries expire after a period without use, holding at most `max_entries`
pub(super) struct TtlMap<K, V> {
    map: HashMap<K, Aging<V>>,
    last_prune: Instant,
    max_entries: usize,
}

struct Aging<V> {
    value: V,
    last_used: Instant,
}

impl<K: Eq + Hash + Clone + Copy, V> TtlMap<K, V> {
    pub(super) fn new(now: Instant, max_entries: usize) -> Self {
        Self {
            map: HashMap::new(),
            last_prune: now,
            max_entries,
        }
    }

    pub(super) fn insert(&mut self, key: K, value: V, now: Instant) {
        self.maybe_prune(now);
        self.make_room(&key, now);

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
        self.make_room(&key, now);

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
        self.prune(now);
    }

    fn prune(&mut self, now: Instant) {
        self.map
            .retain(|_, aging| now.duration_since(aging.last_used) < ENTRY_TTL);
        self.last_prune = now;
    }

    fn make_room(&mut self, key: &K, now: Instant) {
        if self.map.len() < self.max_entries || self.map.contains_key(key) {
            return;
        }

        // expired entries are the natural candidates, whenever the last sweep happened
        self.prune(now);
        if self.map.len() < self.max_entries {
            return;
        }

        // nothing expired: drop the batch of least recently used entries
        let to_evict =
            self.map.len() - self.max_entries + (self.max_entries / EVICTION_FRACTION).max(1);
        let mut by_age: Vec<(Instant, K)> = self
            .map
            .iter()
            .map(|(key, aging)| (aging.last_used, *key))
            .collect();
        by_age.sort_unstable_by_key(|(last_used, _)| *last_used);
        for (_, key) in by_age.iter().take(to_evict) {
            self.map.remove(key);
        }
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
    fn test_ttl_map_stores_and_reads_back() {
        let now = Instant::now();
        let mut map = TtlMap::new(now, 1_000);
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
    fn test_ttl_map_idle_entries_are_swept() {
        let start = Instant::now();
        let mut map = TtlMap::new(start, 1_000);
        map.insert("k", 1, start);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"k", after_entry_ttl(start)), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_ttl_map_reading_an_entry_keeps_it_alive() {
        let start = Instant::now();
        let mut map = TtlMap::new(start, 1_000);
        map.insert("k", 1, start);

        let mut now = start;
        for _ in 0..3 {
            now += ENTRY_TTL - Duration::from_secs(1);
            assert_eq!(map.get(&"k", now), Some(&1));
        }

        assert!(now.duration_since(start) > ENTRY_TTL * 2);
        assert_eq!(map.get(&"k", now), Some(&1));
    }

    #[test]
    fn test_ttl_map_sweeping_spares_entries_still_in_use() {
        let start = Instant::now();
        let mut map = TtlMap::new(start, 1_000);
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
    fn test_ttl_map_full_map_evicts_least_recently_used() {
        let start = Instant::now();
        let max = 100;
        let mut map = TtlMap::new(start, max);

        for i in 0..max {
            map.insert(
                i,
                i,
                start + Duration::from_millis(u64::try_from(i).unwrap()),
            );
        }
        assert_eq!(map.len(), max);

        // touching the oldest entry makes it the most recently used one
        let now = start + Duration::from_secs(1);
        assert_eq!(map.get(&0, now), Some(&0));

        // inserting past the max evicts a batch of the least recently used entries
        map.insert(max, max, now);
        assert_eq!(map.len(), max - max / EVICTION_FRACTION + 1);
        assert_eq!(map.get(&max, now), Some(&max));
        assert_eq!(map.get(&0, now), Some(&0));
        assert_eq!(map.get(&1, now), None);
        assert_eq!(map.get(&2, now), None);
        assert_eq!(map.get(&3, now), None);
        assert_eq!(map.get(&4, now), None);
        assert_eq!(map.get(&23, now), None);
        assert_eq!(map.get(&24, now), None);
        assert_eq!(map.get(&25, now), None);
        assert_eq!(map.get(&26, now), Some(&26));
    }

    #[test]
    fn test_ttl_map_never_grows_past_its_max() {
        let start = Instant::now();
        let max = 64;
        let mut map = TtlMap::new(start, max);
        for i in 0..10_000 {
            map.insert(i, i, start);
            assert!(map.len() <= max);
        }
        assert_eq!(map.get(&9_999, start), Some(&9_999));
    }

    #[test]
    fn test_ttl_map_get_or_insert_with_creates_then_reuses() {
        let now = Instant::now();
        let mut map = TtlMap::new(now, 1_000);
        *map.get_or_insert_with("k", now, || 10) += 1;
        assert_eq!(map.get(&"k", now), Some(&11));
        *map.get_or_insert_with("k", now, || 10) += 1;
        assert_eq!(map.get(&"k", now), Some(&12));
    }
}
