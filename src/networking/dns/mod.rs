//! Application-layer DNS protocol analysis (RFC 1035).
//!
//! [`parser`] turns the raw bytes of a UDP/TCP payload into the strongly-typed
//! [`types::DnsMessage`]; [`types`] defines those data types together with the
//! [`types::DnsEvent`] capture wrapper.

pub mod parser;
pub mod types;
