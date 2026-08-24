use std::time::Instant;

use crate::networking::ipfix::ttl_map::TtlMap;
use crate::networking::types::address_port_pair::AddressPortPair;

/// Per-exporter flow cumulative counter tracking.
/// Useful because some exporters report `octetTotalCount` / `packetTotalCount` instead of the delta counters,
/// and we need to convert them to deltas for our own processing.
/// The cache is keyed by `AddressPortPair`, which already includes the exporter identity.
pub(super) struct BaselineCache {
    map: TtlMap<AddressPortPair, Baseline>,
}

#[derive(Debug, Default)]
struct Baseline {
    bytes: u128,
    packets: u128,
}

impl BaselineCache {
    pub(super) fn new(now: Instant) -> Self {
        Self {
            map: TtlMap::new(now),
        }
    }

    /// Turn a record's cumulative counters into the increment since the same flow's previous report
    pub(super) fn delta(
        &mut self,
        key: &AddressPortPair,
        bytes_total: Option<u128>,
        packets_total: Option<u128>,
        now: Instant,
    ) -> (u128, u128) {
        // no need to store a baseline for a flow that doesn't report any totals
        if bytes_total.is_none() && packets_total.is_none() {
            return (0, 0);
        }

        let baseline = self.map.get_or_insert_with(*key, now, Baseline::default);

        // advance stored baseline and return the increment it implies
        let step = |baseline: &mut u128, total: Option<u128>| {
            let Some(total) = total else {
                return 0;
            };
            // if total went backwards, assume the flow was restarted and take the new total in full
            let delta = total.checked_sub(*baseline).unwrap_or(total);
            *baseline = total;
            delta
        };

        (
            step(&mut baseline.bytes, bytes_total),
            step(&mut baseline.packets, packets_total),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::ipfix::ttl_map::{ENTRY_TTL, PRUNE_INTERVAL};
    use crate::networking::types::ipfix_exporter::IpfixExporter;
    use crate::networking::types::protocol::Protocol;
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Duration;

    fn key(sport: u16, exporter_addr: u8, odid: u32) -> AddressPortPair {
        AddressPortPair {
            source: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            sport: Some(sport),
            dest: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            dport: Some(443),
            protocol: Protocol::TCP,
            exporter: Some(IpfixExporter {
                addr: IpAddr::V4(Ipv4Addr::new(203, 0, 113, exporter_addr)),
                observation_domain_id: odid,
            }),
        }
    }

    #[test]
    fn test_baseline_cache_delta() {
        let now = Instant::now();
        let mut cache = BaselineCache::new(now);

        // initial report: the delta is the total itself
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(1500), Some(10), now),
            (1500, 10)
        );
        // subsequent report: the delta is the difference from the previous total
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(4000), Some(25), now),
            (2500, 15)
        );
        // no change in totals: the delta is zero
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(4000), Some(25), now),
            (0, 0)
        );
        // the flow was restarted and the total went backwards: the delta is the new total in full
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(200), Some(2), now),
            (200, 2)
        );
        // new increment: the delta is the difference from the previous total
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(500), Some(5), now),
            (300, 3)
        );
        assert_eq!(cache.map.len(), 1);

        // same flow and domain, different exporter: not the same counter
        assert_eq!(
            cache.delta(&key(1000, 2, 0), Some(1500), Some(10), now),
            (1500, 10)
        );
        // same exporter and flow, different observation domain: not the same counter
        assert_eq!(
            cache.delta(&key(1000, 1, 7), Some(2000), Some(10), now),
            (2000, 10)
        );
        // same exporter and domain, different flow: not the same counter
        assert_eq!(
            cache.delta(&key(1001, 1, 0), Some(1000), Some(54), now),
            (1000, 54)
        );
        assert_eq!(cache.map.len(), 4);
    }

    #[test]
    fn test_baseline_cache_records_without_totals_are_ignored() {
        let now = Instant::now();
        let mut cache = BaselineCache::new(now);
        assert_eq!(cache.delta(&key(1000, 1, 0), None, None, now), (0, 0));
        assert_eq!(cache.map.len(), 0);
    }

    #[test]
    fn test_baseline_cache_missing_one_counter_leaves_it_untouched() {
        let now = Instant::now();
        let mut cache = BaselineCache::new(now);
        cache.delta(&key(1000, 1, 0), Some(1500), Some(10), now);
        // byte totals only: the packet baseline must not be reset to zero
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(2000), None, now),
            (500, 0)
        );
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(2000), Some(12), now),
            (0, 2)
        );
        // packet totals only: the byte baseline must not be reset to zero
        assert_eq!(cache.delta(&key(1000, 1, 0), None, Some(15), now), (0, 3));
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(2500), Some(15), now),
            (500, 0)
        );
    }

    #[test]
    fn test_baseline_cache_stale_entries_are_swept() {
        let start = Instant::now();
        let mut cache = BaselineCache::new(start);
        cache.delta(&key(1000, 1, 0), Some(1500), Some(10), start);

        // before the TTL: still differencing against the stored baseline
        let fresh = start + PRUNE_INTERVAL + Duration::from_secs(1);
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(2000), Some(12), fresh),
            (500, 2)
        );

        // past the TTL with no reports in between: the entry is swept
        let stale = fresh + ENTRY_TTL + Duration::from_secs(1);
        assert_eq!(
            cache.delta(&key(1000, 1, 0), Some(3000), Some(20), stale),
            (3000, 20)
        );
        assert_eq!(cache.map.len(), 1);
    }
}
