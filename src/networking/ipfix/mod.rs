//! IPFIX collector — RFC 7011 flow records received over UDP.
//!
//! Provides a third capture source for Sniffnet, parallel to live adapter capture and
//! offline PCAP import. The collector listens on a UDP socket, decodes incoming
//! IPFIX messages, tracks per-exporter templates, and feeds 5-tuple flow records
//! into the same `InfoTraffic` aggregate the pcap pipeline produces.

pub mod collect;
pub mod templates;
pub mod totals;
pub mod ttl_map;
pub mod wire;
