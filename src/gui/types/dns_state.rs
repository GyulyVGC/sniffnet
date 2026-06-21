//! GUI-side state for the DNS analyzer page.
//!
//! The capture backend emits [`DnsEvent`]s; the GUI drains them each tick and
//! folds them into this state, which backs the live DNS log shown in the DNS
//! page.

use std::collections::VecDeque;
use std::net::IpAddr;

use crate::networking::dns::types::{DnsEvent, DnsRCode, DnsRecordType};
use crate::networking::types::protocol::Protocol;
use crate::utils::types::timestamp::Timestamp;

/// Maximum number of entries kept in the live log (older ones are discarded).
const MAX_LOG_ENTRIES: usize = 2000;

/// State backing the DNS page.
#[derive(Debug, Default, Clone)]
pub struct DnsState {
    /// Live log of observed DNS messages, oldest first.
    pub log: VecDeque<DnsEntry>,
}

impl DnsState {
    /// Folds a batch of freshly parsed DNS events into the state.
    pub fn ingest(&mut self, events: Vec<DnsEvent>) {
        for event in events {
            if self.log.len() >= MAX_LOG_ENTRIES {
                self.log.pop_front();
            }
            self.log.push_back(DnsEntry::from(event));
        }
    }

    /// Total number of logged DNS messages.
    pub fn len(&self) -> usize {
        self.log.len()
    }

    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }
}

/// A single, display-ready row of the DNS log.
#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl From<DnsEvent> for DnsEntry {
    fn from(event: DnsEvent) -> Self {
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
        }
    }
}
