//! GUI-side state for the DNS analyzer page.
//!
//! The capture backend emits [`DnsEvent`]s; the GUI drains them each tick and
//! folds them into this state, which backs the live DNS log, the per-domain
//! ranking, and the query/response latency correlation shown in the DNS page.

use std::collections::{HashMap, VecDeque};
use std::net::IpAddr;

use crate::networking::dns::types::{DnsEvent, DnsRCode, DnsRecordType};
use crate::networking::types::protocol::Protocol;
use crate::utils::types::timestamp::Timestamp;

/// Maximum number of entries kept in the live log (older ones are discarded).
const MAX_LOG_ENTRIES: usize = 2000;
/// Safety cap on the number of in-flight (unanswered) queries tracked for
/// latency correlation, to bound memory if many queries go unanswered.
const MAX_PENDING_QUERIES: usize = 10_000;

/// Correlation key matching a query with its response: transaction id plus the
/// (client, server) endpoint pair. For a query the client is the source and the
/// server the destination; for a response the roles are reversed.
type CorrelationKey = (u16, IpAddr, IpAddr);

/// State backing the DNS page.
#[derive(Debug, Default, Clone)]
pub struct DnsState {
    /// Live log of observed DNS messages, oldest first.
    pub log: VecDeque<DnsEntry>,
    /// Number of queries seen per domain, for the "most queried" ranking.
    pub ranking: HashMap<String, u64>,
    /// Timestamp of in-flight queries awaiting a response, keyed for
    /// correlation, used to compute resolution latency.
    pending: HashMap<CorrelationKey, Timestamp>,
}

impl DnsState {
    /// Folds a batch of freshly parsed DNS events into the state.
    pub fn ingest(&mut self, events: Vec<DnsEvent>) {
        for event in events {
            let mut entry = DnsEntry::from(&event);

            if event.message.is_response {
                // Match against the originating query: client = dst, server = src.
                let key = (entry.id, event.dst, event.src);
                if let Some(query_ts) = self.pending.remove(&key) {
                    entry.latency_ms = latency_ms(query_ts, event.timestamp);
                }
            } else {
                // Record the query for later correlation and count it in the ranking.
                if self.pending.len() < MAX_PENDING_QUERIES {
                    let key = (entry.id, event.src, event.dst);
                    self.pending.insert(key, event.timestamp);
                }
                if !entry.domain.is_empty() && entry.domain != "-" {
                    *self.ranking.entry(entry.domain.clone()).or_insert(0) += 1;
                }
            }

            if self.log.len() >= MAX_LOG_ENTRIES {
                self.log.pop_front();
            }
            self.log.push_back(entry);
        }
    }

    /// Total number of logged DNS messages.
    pub fn len(&self) -> usize {
        self.log.len()
    }

    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    /// The `n` most queried domains, most frequent first.
    pub fn top_domains(&self, n: usize) -> Vec<(String, u64)> {
        let mut ranked: Vec<(String, u64)> = self
            .ranking
            .iter()
            .map(|(d, c)| (d.clone(), *c))
            .collect();
        // Sort by descending count, then by domain name for a stable order.
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        ranked.truncate(n);
        ranked
    }
}

/// Computes resolution latency in milliseconds between a query and its
/// response. Returns `None` if the timestamps are unusable or out of order.
fn latency_ms(query: Timestamp, response: Timestamp) -> Option<f64> {
    let q = query.to_usecs()?;
    let r = response.to_usecs()?;
    if r < q {
        return None;
    }
    Some((r - q) as f64 / 1000.0)
}

/// A single, display-ready row of the DNS log.
#[derive(Debug, Clone, PartialEq)]
pub struct DnsEntry {
    pub timestamp: Timestamp,
    pub src: IpAddr,
    pub dst: IpAddr,
    pub transport: Protocol,
    pub id: u16,
    pub is_response: bool,
    /// Queried domain name, or "-" if the message carried no question.
    pub domain: String,
    /// Queried record type, if a question was present.
    pub qtype: Option<DnsRecordType>,
    pub rcode: DnsRCode,
    /// Comma-separated summary of the answers (empty for queries).
    pub answers: String,
    /// Resolution latency in milliseconds (set on responses that were matched
    /// to a previously seen query).
    pub latency_ms: Option<f64>,
}

impl From<&DnsEvent> for DnsEntry {
    fn from(event: &DnsEvent) -> Self {
        let message = &event.message;
        DnsEntry {
            timestamp: event.timestamp,
            src: event.src,
            dst: event.dst,
            transport: event.transport,
            id: message.id,
            is_response: message.is_response,
            domain: message.query_name().unwrap_or("-").to_string(),
            qtype: message.query_type(),
            rcode: message.rcode,
            answers: message.answers_summary(),
            latency_ms: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::dns::types::{DnsFlags, DnsMessage, DnsQuestion};
    use std::net::Ipv4Addr;

    const CLIENT: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2));
    const SERVER: IpAddr = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));

    fn message(id: u16, is_response: bool, domain: &str) -> DnsMessage {
        DnsMessage {
            id,
            is_response,
            opcode: 0,
            flags: DnsFlags::default(),
            rcode: DnsRCode::NoError,
            questions: vec![DnsQuestion {
                name: domain.to_string(),
                qtype: DnsRecordType::A,
                qclass: 1,
            }],
            answers: Vec::new(),
        }
    }

    fn query(id: u16, domain: &str, secs: i64) -> DnsEvent {
        DnsEvent {
            timestamp: Timestamp::new(secs, 0),
            src: CLIENT,
            dst: SERVER,
            transport: Protocol::UDP,
            message: message(id, false, domain),
        }
    }

    fn response(id: u16, domain: &str, secs: i64, usecs: i64) -> DnsEvent {
        DnsEvent {
            timestamp: Timestamp::new(secs, usecs),
            src: SERVER,
            dst: CLIENT,
            transport: Protocol::UDP,
            message: message(id, true, domain),
        }
    }

    #[test]
    fn correlates_response_to_query_and_computes_latency() {
        let mut state = DnsState::default();
        state.ingest(vec![
            query(0x1234, "example.com", 10),
            response(0x1234, "example.com", 10, 25_000), // 25 ms later
        ]);
        assert_eq!(state.len(), 2);
        let resp = state.log.back().unwrap();
        assert!(resp.is_response);
        assert_eq!(resp.latency_ms, Some(25.0));
    }

    #[test]
    fn response_without_matching_query_has_no_latency() {
        let mut state = DnsState::default();
        state.ingest(vec![response(0x9999, "orphan.com", 5, 0)]);
        assert_eq!(state.log.back().unwrap().latency_ms, None);
    }

    #[test]
    fn ranking_counts_queries_per_domain() {
        let mut state = DnsState::default();
        state.ingest(vec![
            query(1, "a.com", 1),
            query(2, "a.com", 2),
            query(3, "b.com", 3),
            response(1, "a.com", 1, 1000), // responses don't add to ranking
        ]);
        let top = state.top_domains(5);
        assert_eq!(top, vec![("a.com".to_string(), 2), ("b.com".to_string(), 1)]);
    }
}
