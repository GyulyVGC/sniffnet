use crate::countries::types::country::Country;
use crate::networking::types::asn::Asn;
use crate::networking::types::data_info_host::DataInfoHost;
use std::net::IpAddr;

/// Struct to represent a network host
#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Host {
    /// Hostname (domain). Obtained from the reverse DNS.
    pub domain: String,
    /// Autonomous System which operates the host
    pub asn: Asn,
    /// Country
    pub country: Country,
}

impl Host {
    /// Used in the host bars
    pub fn to_host_entry_string(&self) -> String {
        let mut ret_val = self.domain.clone();
        if !self.asn.name.is_empty() {
            ret_val.push_str(" - ");
            ret_val.push_str(&self.asn.name);
        }
        ret_val
    }

    /// Used in the thumbnail
    pub fn to_host_thumbnail_string(&self) -> String {
        let domain = &self.domain;
        let asn = &self.asn.name;
        if asn.is_empty() || (!domain.trim().is_empty() && domain.parse::<IpAddr>().is_err()) {
            domain
        } else {
            asn
        }
        .to_string()
    }

    /// Used in the blacklist notifications
    pub fn to_host_blacklist_string(&self) -> String {
        let domain = &self.domain;
        let asn = &self.asn.name;
        let mut ret_val = String::new();

        if !domain.trim().is_empty() && domain.parse::<IpAddr>().is_err() {
            ret_val.push_str(domain);
        }

        if !asn.is_empty() {
            if !ret_val.is_empty() {
                ret_val.push_str(" - ");
            }
            ret_val.push_str(asn);
        }

        if ret_val.is_empty() {
            String::new()
        } else {
            format!("({ret_val})")
        }
    }
}

/// Struct to represent a network host for representation in the thumbnail
///
/// This is necessary to remove possible duplicates in the thumbnail host list
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq)]
pub struct ThumbnailHost {
    /// Country
    pub country: Country,
    /// Text describing the host in the thumbnail
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct HostMessage {
    pub host: Host,
    pub data_info_host: DataInfoHost,
    pub address_to_lookup: IpAddr,
    pub rdns: String,
}
