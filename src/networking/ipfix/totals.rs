//! Per-flow cumulative counter tracking.
//!
//! Many exporters report `octetTotalCount` / `packetTotalCount` instead of the
//! delta counters: values that are cumulative for the lifetime of the flow
//! rather than an increment since the previous report. The collector adds every
//! record's counts onto a running tally, so a total has to be turned into an
//! increment first, by differencing it against the same flow's previous report.
//!
//! Keyed by `(peer, observation_domain, flow)` for the same reason the template
//! cache is: one exporter's numbers must never be differenced against another's.

use std::net::SocketAddr;
use std::time::Instant;

use crate::networking::ipfix::ttl_map::TtlMap;
use crate::networking::types::address_port_pair::AddressPortPair;

/// The counters a flow last reported, against which the next report is
/// differenced. A flow we've never seen starts from zero, which is what makes
/// its first report count in full.
#[derive(Debug, Default)]
struct Baseline {
    bytes: u128,
    packets: u128,
}

pub struct TotalsCache {
    map: TtlMap<(SocketAddr, u32, AddressPortPair), Baseline>,
}

impl TotalsCache {
    pub fn new(now: Instant) -> Self {
        Self {
            map: TtlMap::new(now),
        }
    }

    /// Turn a record's cumulative counters into the increment since the same
    /// flow's previous report, and remember them as the new baseline.
    ///
    /// The first report of a flow contributes its totals in full: exporters
    /// that emit a single record per flow at expiry are the common case, and
    /// there the total *is* the whole flow. The cost is that a collector
    /// started mid-flow counts the part it never saw, once.
    ///
    /// A total below the stored baseline means the 5-tuple has been reused by a
    /// new flow, so the counter restarted; the new total is taken in full.
    pub fn delta(
        &mut self,
        peer: SocketAddr,
        observation_domain_id: u32,
        key: &AddressPortPair,
        bytes_total: Option<u128>,
        packets_total: Option<u128>,
        now: Instant,
    ) -> (u128, u128) {
        // Nothing to difference, and no reason to remember this flow.
        if bytes_total.is_none() && packets_total.is_none() {
            return (0, 0);
        }

        let baseline = self.map.get_or_insert_with(
            (peer, observation_domain_id, *key),
            now,
            Baseline::default,
        );

        (
            step(&mut baseline.bytes, bytes_total),
            step(&mut baseline.packets, packets_total),
        )
    }
}

/// Advance one stored baseline and return the increment it implies. A record
/// that doesn't carry this counter leaves the baseline untouched.
fn step(baseline: &mut u128, total: Option<u128>) -> u128 {
    let Some(total) = total else {
        return 0;
    };
    let delta = total.checked_sub(*baseline).unwrap_or(total);
    *baseline = total;
    delta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::ipfix::ttl_map::{ENTRY_TTL, PRUNE_INTERVAL};
    use crate::networking::types::protocol::Protocol;
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Duration;

    fn peer(port: u16) -> SocketAddr {
        format!("203.0.113.9:{port}").parse().unwrap()
    }

    fn key(sport: u16) -> AddressPortPair {
        AddressPortPair {
            source: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            sport: Some(sport),
            dest: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            dport: Some(443),
            protocol: Protocol::TCP,
        }
    }

    #[test]
    fn first_report_of_a_flow_counts_the_whole_total() {
        let now = Instant::now();
        let mut cache = TotalsCache::new(now);
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(1500), Some(10), now),
            (1500, 10)
        );
    }

    #[test]
    fn later_reports_count_only_the_increment() {
        let now = Instant::now();
        let mut cache = TotalsCache::new(now);
        cache.delta(peer(1), 0, &key(1000), Some(1500), Some(10), now);
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(4000), Some(25), now),
            (2500, 15)
        );
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(4000), Some(25), now),
            (0, 0),
            "a repeated total is no new traffic"
        );
    }

    #[test]
    fn a_counter_restart_is_taken_in_full() {
        // The 5-tuple got reused by a new flow, so the total went backwards.
        let now = Instant::now();
        let mut cache = TotalsCache::new(now);
        cache.delta(peer(1), 0, &key(1000), Some(9000), Some(60), now);
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(200), Some(2), now),
            (200, 2)
        );
        // ...and the new flow becomes the baseline from there on.
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(500), Some(5), now),
            (300, 3)
        );
    }

    #[test]
    fn baselines_are_per_exporter_and_per_flow() {
        let now = Instant::now();
        let mut cache = TotalsCache::new(now);
        cache.delta(peer(1), 0, &key(1000), Some(1500), Some(10), now);

        // Same flow, different exporter: not the same counter.
        assert_eq!(
            cache.delta(peer(2), 0, &key(1000), Some(1500), Some(10), now),
            (1500, 10)
        );
        // Same exporter, different observation domain: likewise.
        assert_eq!(
            cache.delta(peer(1), 7, &key(1000), Some(1500), Some(10), now),
            (1500, 10)
        );
        // Same exporter and domain, different flow.
        assert_eq!(
            cache.delta(peer(1), 0, &key(1001), Some(1500), Some(10), now),
            (1500, 10)
        );
    }

    #[test]
    fn records_without_totals_are_inert() {
        let now = Instant::now();
        let mut cache = TotalsCache::new(now);
        assert_eq!(cache.delta(peer(1), 0, &key(1000), None, None, now), (0, 0));
        assert!(cache.map.is_empty(), "nothing to remember");
    }

    #[test]
    fn a_missing_counter_leaves_its_baseline_untouched() {
        let now = Instant::now();
        let mut cache = TotalsCache::new(now);
        cache.delta(peer(1), 0, &key(1000), Some(1500), Some(10), now);
        // Byte totals only: the packet baseline must not be reset to zero.
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(2000), None, now),
            (500, 0)
        );
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(2000), Some(12), now),
            (0, 2)
        );
    }

    #[test]
    fn stale_entries_are_swept_and_start_over() {
        let start = Instant::now();
        let mut cache = TotalsCache::new(start);
        cache.delta(peer(1), 0, &key(1000), Some(1500), Some(10), start);

        // Before the TTL: still differencing against the stored baseline.
        let fresh = start + PRUNE_INTERVAL + Duration::from_secs(1);
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(2000), Some(12), fresh),
            (500, 2)
        );

        // Past the TTL with no reports in between: the entry is swept, so the
        // flow reads as new again.
        let stale = fresh + ENTRY_TTL + Duration::from_secs(1);
        assert_eq!(
            cache.delta(peer(1), 0, &key(1000), Some(3000), Some(20), stale),
            (3000, 20)
        );
        assert_eq!(cache.map.len(), 1, "the swept entry was replaced, not kept");
    }
}
