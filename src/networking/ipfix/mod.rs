//! IPFIX collector — RFC 7011 flow records received over UDP.
//!
//! Provides a third capture source for Sniffnet, parallel to live adapter capture and
//! offline PCAP import. The collector listens on a UDP socket, decodes incoming
//! IPFIX messages, tracks per-exporter templates, and feeds 5-tuple flow records
//! into the same `InfoTraffic` aggregate the pcap pipeline produces.

pub mod collect;
pub mod templates;
pub mod wire;

use crate::location;
use crate::utils::error_logger::ErrorLogger;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;

/// IANA-registered default IPFIX collector port.
pub const DEFAULT_IPFIX_PORT: u16 = 4739;

/// Persisted IPFIX collector configuration.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct MyIpfixSocket {
    addr: String,
    port: String,
}

impl MyIpfixSocket {
    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn set_addr(&mut self, addr: String) {
        self.addr = addr;
    }

    pub fn set_port(&mut self, port: String) {
        self.port = port;
    }

    pub fn display_name(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }

    pub fn socket_addr(&self) -> Result<SocketAddr, String> {
        let port = self
            .port
            .parse::<u16>()
            .map_err(|_| format!("Invalid port number: {}", self.port))?;
        let ip_addr = self
            .addr
            .parse::<IpAddr>()
            .map_err(|_| format!("Invalid IP address: {}", self.addr))?;
        Ok(SocketAddr::new(ip_addr, port))
    }
}

impl Default for MyIpfixSocket {
    fn default() -> Self {
        Self {
            addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED).to_string(),
            port: DEFAULT_IPFIX_PORT.to_string(),
        }
    }
}
