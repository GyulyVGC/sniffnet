//! A hash map whose entries expire after a spell without use.
//!
//! Both per-exporter caches in this module need the same thing: state that has
//! to survive quiet periods, but must not accumulate forever as exporters come
//! and go. Entries are kept alive by *use*, not just by being written, so a
//! template that keeps decoding records never expires however long ago the
//! exporter announced it.

use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// How long an entry survives without being touched. Comfortably longer than
/// the template refresh and active timeout intervals exporters use in practice
/// (tens of seconds to a few minutes), so only state belonging to an exporter
/// that has genuinely gone away is dropped.
pub(super) const ENTRY_TTL: Duration = Duration::from_mins(30);

/// How often expired entries are swept out. Sweeping walks the whole map, so it
/// deliberately doesn't run on every lookup.
pub(super) const PRUNE_INTERVAL: Duration = Duration::from_mins(1);

struct Aging<V> {
    value: V,
    last_used: Instant,
}

pub(super) struct TtlMap<K, V> {
    map: HashMap<K, Aging<V>>,
    last_prune: Instant,
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

    /// Look up an entry, marking it as still in use.
    pub(super) fn get(&mut self, key: &K, now: Instant) -> Option<&V> {
        self.maybe_prune(now);
        let aging = self.map.get_mut(key)?;
        aging.last_used = now;
        Some(&aging.value)
    }

    /// Look up an entry for modification, creating it from `default` if it isn't
    /// there. Either way it counts as use.
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

    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A point in time far enough past the TTL that a sweep run then drops
    /// anything last used at `from`.
    fn long_after(from: Instant) -> Instant {
        from + ENTRY_TTL + Duration::from_secs(1)
    }

    /// A point in time far enough past `from` that the next call sweeps.
    fn after_prune_interval(from: Instant) -> Instant {
        from + PRUNE_INTERVAL + Duration::from_secs(1)
    }

    #[test]
    fn test_stores_and_reads_back() {
        let now = Instant::now();
        let mut map = TtlMap::new(now);
        map.insert("k", 1, now);
        assert_eq!(map.get(&"k", now), Some(&1));
        assert_eq!(map.get(&"absent", now), None);
    }

    #[test]
    fn test_insertion_replaces_the_previous_value() {
        let now = Instant::now();
        let mut map = TtlMap::new(now);
        map.insert("k", 1, now);
        map.insert("k", 2, now);
        assert_eq!(map.get(&"k", now), Some(&2));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_idle_entries_are_swept() {
        let start = Instant::now();
        let mut map = TtlMap::new(start);
        map.insert("k", 1, start);
        assert_eq!(map.get(&"k", long_after(start)), None);
        assert!(map.is_empty());
    }

    #[test]
    fn test_reading_an_entry_keeps_it_alive() {
        let start = Instant::now();
        let mut map = TtlMap::new(start);
        map.insert("k", 1, start);

        // Read it repeatedly, each read just inside the TTL of the previous one.
        let mut now = start;
        for _ in 0..3 {
            now += ENTRY_TTL - Duration::from_secs(1);
            assert_eq!(map.get(&"k", now), Some(&1), "still in use");
        }

        // Long past the TTL counted from insertion — only last use matters.
        assert!(now.duration_since(start) > ENTRY_TTL * 2);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_sweeping_spares_entries_still_in_use() {
        let start = Instant::now();
        let mut map = TtlMap::new(start);
        map.insert("used", 1, start);
        map.insert("idle", 2, start);

        // Keep only the first alive, then run a sweep past the TTL.
        let mid = start + ENTRY_TTL - Duration::from_secs(1);
        assert_eq!(map.get(&"used", mid), Some(&1));
        let sweep = after_prune_interval(mid);
        assert_eq!(map.get(&"used", sweep), Some(&1));

        assert_eq!(map.len(), 1, "only the idle entry was swept");
    }

    #[test]
    fn test_get_or_insert_with_creates_then_reuses() {
        let now = Instant::now();
        let mut map = TtlMap::new(now);
        *map.get_or_insert_with("k", now, || 10) += 1;
        assert_eq!(map.get(&"k", now), Some(&11));
        *map.get_or_insert_with("k", now, || 10) += 1;
        assert_eq!(map.get(&"k", now), Some(&12), "default not applied twice");
    }
}
