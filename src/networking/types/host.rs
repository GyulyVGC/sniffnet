use crate::countries::types::country::Country;
use crate::networking::types::asn::Asn;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::utils::formatted_strings::clip_text;
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
    pub fn to_entry_string(&self) -> String {
        let mut ret_val = self.domain.clone();
        if !self.asn.name.is_empty() {
            ret_val.push_str(" - ");
            ret_val.push_str(&self.asn.name);
        }
        ret_val
    }

    /// Used in the blacklist notifications
    pub fn to_blacklist_string(&self) -> String {
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

impl ThumbnailHost {
    /// Constructor from a Host
    pub fn from_host(host: &Host, max_text_chars: usize) -> Self {
        let domain = &host.domain;
        let asn = &host.asn.name;
        let unclipped =
            if asn.is_empty() || (!domain.trim().is_empty() && domain.parse::<IpAddr>().is_err()) {
                domain
            } else {
                asn
            };
        Self {
            country: host.country,
            text: clip_text(unclipped, max_text_chars),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HostMessage {
    pub host: Host,
    pub data_info_host: DataInfoHost,
    pub address_to_lookup: IpAddr,
    pub rdns: String,
}

#[cfg(test)]
mod tests {
    use crate::networking::types::asn::Asn;
    use crate::networking::types::host::{Host, ThumbnailHost};

    fn host_for_tests(domain: &str, asn: &str) -> Host {
        Host {
            domain: domain.to_string(),
            asn: Asn {
                name: asn.to_string(),
                code: "512".to_string(),
            },
            country: Default::default(),
        }
    }

    #[test]
    fn test_host_to_entry_string() {
        let host = host_for_tests("iphone-di-doofenshmirtz.local", "AS1234");
        assert_eq!(
            host.to_entry_string(),
            "iphone-di-doofenshmirtz.local - AS1234"
        );

        let host = host_for_tests("", "");
        assert_eq!(host.to_entry_string(), "");

        let host = host_for_tests("192.168.1.113", "AS1234");
        assert_eq!(host.to_entry_string(), "192.168.1.113 - AS1234");

        let host = host_for_tests("192.168.1.113", "");
        assert_eq!(host.to_entry_string(), "192.168.1.113");

        let host = host_for_tests("", "FASTLY");
        assert_eq!(host.to_entry_string(), " - FASTLY");

        let host = host_for_tests("::", "GOOGLE");
        assert_eq!(host.to_entry_string(), ":: - GOOGLE");

        let host = host_for_tests("::f", "AKAMAI-TECHNOLOGIES-INCORPORATED");
        assert_eq!(
            host.to_entry_string(),
            "::f - AKAMAI-TECHNOLOGIES-INCORPORATED"
        );

        let host = host_for_tests("::g", "GOOGLE");
        assert_eq!(host.to_entry_string(), "::g - GOOGLE");

        let host = host_for_tests(" ", "GOOGLE");
        assert_eq!(host.to_entry_string(), "  - GOOGLE");
    }

    #[test]
    fn test_host_to_blacklist_string() {
        let host = host_for_tests("iphone-di-doofenshmirtz.local", "AS1234");
        assert_eq!(
            host.to_blacklist_string(),
            "(iphone-di-doofenshmirtz.local - AS1234)"
        );

        let host = host_for_tests("", "");
        assert_eq!(host.to_blacklist_string(), "");

        let host = host_for_tests("192.168.1.113", "AS1234");
        assert_eq!(host.to_blacklist_string(), "(AS1234)");

        let host = host_for_tests("192.168.1.113", "");
        assert_eq!(host.to_blacklist_string(), "");

        let host = host_for_tests("", "FASTLY");
        assert_eq!(host.to_blacklist_string(), "(FASTLY)");

        let host = host_for_tests("::", "GOOGLE");
        assert_eq!(host.to_blacklist_string(), "(GOOGLE)");

        let host = host_for_tests("::f", "AKAMAI-TECHNOLOGIES-INCORPORATED");
        assert_eq!(
            host.to_blacklist_string(),
            "(AKAMAI-TECHNOLOGIES-INCORPORATED)"
        );

        let host = host_for_tests("::g", "GOOGLE");
        assert_eq!(host.to_blacklist_string(), "(::g - GOOGLE)");

        let host = host_for_tests(" ", "GOOGLE");
        assert_eq!(host.to_blacklist_string(), "(GOOGLE)");
    }

    #[test]
    fn test_thumbnail_host_text() {
        let host = host_for_tests("iphone-di-doofenshmirtz.local", "AS1234");
        assert_eq!(
            ThumbnailHost::from_host(&host, 26).text,
            "iphone-di-doofenshmirtz.…"
        );

        let host = host_for_tests("", "");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "");

        let host = host_for_tests("192.168.1.113", "AS1234");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "AS1234");

        let host = host_for_tests("192.168.1.113", "");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "192.168.1.113");

        let host = host_for_tests("", "FASTLY");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "FASTLY");

        let host = host_for_tests("::", "GOOGLE");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "GOOGLE");

        let host = host_for_tests("::f", "AKAMAI-TECHNOLOGIES-INCORPORATED");
        assert_eq!(
            ThumbnailHost::from_host(&host, 26).text,
            "AKAMAI-TECHNOLOGIES-INCO…"
        );

        let host = host_for_tests("::g", "GOOGLE");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "::g");

        let host = host_for_tests(" ", "GOOGLE");
        assert_eq!(ThumbnailHost::from_host(&host, 26).text, "GOOGLE");
    }
}
