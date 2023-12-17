//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use std::collections::HashSet;

use crate::networking::types::ip_collection::AddressCollection;
use crate::networking::types::packet_filters_fields::PacketFiltersFields;
use crate::networking::types::port_collection::PortCollection;
use crate::{IpVersion, Protocol};

/// Possible filters applicable to network traffic
#[derive(Clone)]
pub struct Filters {
    /// Internet Protocol versions
    pub ip_versions: HashSet<IpVersion>,
    /// Protocols
    pub protocols: HashSet<Protocol>,
    /// IP addresses string in Initial page text input
    pub address_str: String,
    /// IP address collection to match against traffic
    pub address_collection: AddressCollection,
    /// Ports string in Initial page text input
    pub port_str: String,
    /// Port collection to match against traffic
    pub port_collection: PortCollection,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            ip_versions: HashSet::from(IpVersion::ALL),
            protocols: HashSet::from(Protocol::ALL),
            address_str: String::new(),
            address_collection: AddressCollection::default(),
            port_str: String::new(),
            port_collection: PortCollection::default(),
        }
    }
}

impl Filters {
    /// Checks whether the filters match the current packet's protocols
    pub fn matches(&self, packet_filters_fields: &PacketFiltersFields) -> bool {
        self.ip_versions.contains(&packet_filters_fields.ip_version)
            && self.protocols.contains(&packet_filters_fields.protocol)
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
        self.ip_version_valid()
            && self.protocol_valid()
            && self.address_valid()
            && self.port_valid()
    }

    pub fn ip_version_valid(&self) -> bool {
        !self.ip_versions.is_empty()
    }

    pub fn protocol_valid(&self) -> bool {
        !self.protocols.is_empty()
    }

    pub fn address_valid(&self) -> bool {
        AddressCollection::new(&self.address_str).is_some()
    }

    pub fn port_valid(&self) -> bool {
        PortCollection::new(&self.port_str).is_some()
    }

    pub fn none_active(&self) -> bool {
        !self.ip_version_active()
            && !self.protocol_active()
            && !self.address_active()
            && !self.port_active()
    }

    pub fn ip_version_active(&self) -> bool {
        self.ip_versions.len() != IpVersion::ALL.len()
    }

    pub fn protocol_active(&self) -> bool {
        self.protocols.len() != Protocol::ALL.len()
    }

    pub fn address_active(&self) -> bool {
        self.address_collection != AddressCollection::default()
    }

    pub fn port_active(&self) -> bool {
        self.port_collection != PortCollection::default()
    }

    pub fn pretty_print_ip(&self) -> String {
        format!("{:?}", self.ip_versions)
            .replace('{', "")
            .replace('}', "")
    }

    pub fn pretty_print_protocol(&self) -> String {
        format!("{:?}", self.protocols)
            .replace('{', "")
            .replace('}', "")
    }
}
