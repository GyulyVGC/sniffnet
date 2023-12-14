//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::{AppProtocol, IpVersion, TransProtocol};
use std::collections::HashSet;

/// Possible filters applicable to network traffic
#[derive(Clone)]
pub struct Filters {
    /// Internet Protocol version
    pub ip: HashSet<IpVersion>,
    /// Transport layer protocol
    pub transport: HashSet<TransProtocol>,
    /// Application layer protocol
    pub application: AppProtocol,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            ip: HashSet::from(IpVersion::ALL),
            transport: HashSet::from(TransProtocol::ALL),
            application: AppProtocol::Other,
        }
    }
}

impl Filters {
    /// Checks whether the filters match the current packet's protocols
    pub fn matches(&self, packet_filters_fields: &PacketFiltersFields) -> bool {
        self.ip.contains(&packet_filters_fields.ip)
            && self.transport.contains(&packet_filters_fields.transport)
    }
}
