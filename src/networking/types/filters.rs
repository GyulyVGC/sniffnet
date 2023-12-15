//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::networking::types::ip_collection::IpCollection;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::networking::types::port_collection::PortCollection;
use crate::{IpVersion, TransProtocol};
use std::collections::HashSet;

/// Possible filters applicable to network traffic
#[derive(Clone)]
pub struct Filters {
    /// Internet Protocol versions
    pub ip: HashSet<IpVersion>,
    /// Transport layer protocols
    pub transport: HashSet<TransProtocol>,
    /// IP addresses string in Initial page text input
    pub address_str: String,
    /// IP address collection to match against traffic
    pub address_collection: IpCollection,
    /// Ports string in Initial page text input
    pub port_str: String,
    /// Port collection to match against traffic
    pub port_collection: PortCollection,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            ip: HashSet::from(IpVersion::ALL),
            transport: HashSet::from(TransProtocol::ALL),
            address_str: String::new(),
            address_collection: IpCollection::default(),
            port_str: String::new(),
            port_collection: PortCollection::default(),
        }
    }
}

impl Filters {
    /// Checks whether the filters match the current packet's protocols
    pub fn matches(&self, packet_filters_fields: &PacketFiltersFields) -> bool {
        self.ip.contains(&packet_filters_fields.ip)
            && self.transport.contains(&packet_filters_fields.transport)
            && (self
                .address_collection
                .contains(&packet_filters_fields.source)
                || self
                    .address_collection
                    .contains(&packet_filters_fields.dest))
            && (self.port_collection.contains(packet_filters_fields.sport)
                || self.port_collection.contains(packet_filters_fields.dport))
    }

    pub fn are_valid(&self) -> bool {
        !self.ip.is_empty()
            && !self.transport.is_empty()
            && IpCollection::new(&self.address_str).is_some()
            && PortCollection::new(&self.port_str).is_some()
    }
}
