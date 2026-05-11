//! IPFIX collector — RFC 7011 flow records received over UDP.
//!
//! Provides a third capture source for Sniffnet, parallel to live adapter capture and
//! offline PCAP import. The collector listens on a UDP socket, decodes incoming
//! IPFIX messages, tracks per-exporter templates, and feeds 5-tuple flow records
//! into the same `InfoTraffic` aggregate the pcap pipeline produces.

pub mod collect;
pub mod templates;
pub mod wire;

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

/// IANA-registered default IPFIX collector port.
pub const DEFAULT_IPFIX_PORT: u16 = 4739;

/// Persisted IPFIX collector configuration.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct IpfixCollectorConf {
    pub bind_addr: String,
    pub bind_port: u16,
}

impl Default for IpfixCollectorConf {
    fn default() -> Self {
        Self {
            bind_addr: String::from("0.0.0.0"),
            bind_port: DEFAULT_IPFIX_PORT,
        }
    }
}

/// Runtime handle for an IPFIX collector source, embedded in `CaptureSource::Ipfix`.
#[derive(Clone, Debug)]
pub struct MyIpfixCollector {
    bind_addr: String,
    bind_port: u16,
}

impl MyIpfixCollector {
    pub fn new(bind_addr: String, bind_port: u16) -> Self {
        Self {
            bind_addr,
            bind_port,
        }
    }

    pub fn from_conf(conf: &IpfixCollectorConf) -> Self {
        Self::new(conf.bind_addr.clone(), conf.bind_port)
    }

    /// Parse the configured bind address + port into a `SocketAddr`. Returns
    /// `None` if `bind_addr` is not a valid IP literal.
    pub fn socket_addr(&self) -> Option<SocketAddr> {
        let ip: IpAddr = self.bind_addr.parse().ok()?;
        Some(SocketAddr::new(ip, self.bind_port))
    }

    /// Whether the current config can be used to start a capture.
    pub fn is_valid(&self) -> bool {
        self.socket_addr().is_some()
    }

    pub fn display_name(&self) -> String {
        format!("{}:{}", self.bind_addr, self.bind_port)
    }
}
