use serde::de::{self, Deserializer, VariantAccess};
use serde::{Deserialize, Serialize};

use crate::networking::types::service_category::ServiceCategory;

include!(concat!(env!("OUT_DIR"), "/service_categories.rs"));

/// Upper layer services.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
pub enum Service {
    /// One of the known services.
    Name(&'static str),
    /// Not identified
    #[default]
    Unknown,
    /// Not applicable
    NotApplicable,
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceVisitor;

        impl<'de> de::Visitor<'de> for ServiceVisitor {
            type Value = Service;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Service enum")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: de::EnumAccess<'de>,
            {
                let (variant, access) = data.variant::<String>()?;
                match variant.as_str() {
                    "Name" => {
                        let s: String = access.newtype_variant()?;
                        let leaked: &'static str = Box::leak(s.into_boxed_str());
                        Ok(Service::Name(leaked))
                    }
                    "Unknown" => {
                        access.unit_variant()?;
                        Ok(Service::Unknown)
                    }
                    "NotApplicable" => {
                        access.unit_variant()?;
                        Ok(Service::NotApplicable)
                    }
                    other => Err(de::Error::unknown_variant(
                        other,
                        &["Name", "Unknown", "NotApplicable"],
                    )),
                }
            }
        }

        deserializer.deserialize_enum(
            "Service",
            &["Name", "Unknown", "NotApplicable"],
            ServiceVisitor,
        )
    }
}

impl Service {
    pub fn category(self) -> ServiceCategory {
        match self {
            Service::Name(name) => SERVICE_CATEGORIES.get(name).copied().unwrap_or_default(),
            Service::Unknown | Service::NotApplicable => ServiceCategory::Other,
        }
    }

    pub fn to_string_with_equal_prefix(self) -> String {
        format!("={self}")
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Service::Name(name) => write!(f, "{name}"),
            Service::Unknown => write!(f, "?"),
            Service::NotApplicable => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_category() {
        // FTP
        assert_eq!(Service::Name("ftp-data").category(), ServiceCategory::Files);
        assert_eq!(Service::Name("ftp").category(), ServiceCategory::Files);

        // SSH
        assert_eq!(Service::Name("ssh").category(), ServiceCategory::Remote);

        // Telnet
        assert_eq!(Service::Name("telnet").category(), ServiceCategory::Remote);

        // SMTP
        assert_eq!(Service::Name("smtp").category(), ServiceCategory::Email);

        // TACACS
        assert_eq!(
            Service::Name("tacacs").category(),
            ServiceCategory::Identity
        );

        // DNS
        assert_eq!(
            Service::Name("domain").category(),
            ServiceCategory::Discovery
        );

        // DHCP
        assert_eq!(Service::Name("dhcps").category(), ServiceCategory::Network);
        assert_eq!(Service::Name("dhcpc").category(), ServiceCategory::Network);

        // TFTP
        assert_eq!(Service::Name("tftp").category(), ServiceCategory::Files);

        // HTTP
        assert_eq!(Service::Name("http").category(), ServiceCategory::Web);

        // POP
        assert_eq!(Service::Name("pop2").category(), ServiceCategory::Email);
        assert_eq!(Service::Name("pop3").category(), ServiceCategory::Email);

        // NTP
        assert_eq!(Service::Name("ntp").category(), ServiceCategory::Network);

        // NetBIOS
        assert_eq!(
            Service::Name("netbios-ns").category(),
            ServiceCategory::Discovery
        );
        assert_eq!(
            Service::Name("netbios-dgm").category(),
            ServiceCategory::Middleware
        );
        assert_eq!(
            Service::Name("netbios-ssn").category(),
            ServiceCategory::Middleware
        );

        // IMAP
        assert_eq!(Service::Name("imap").category(), ServiceCategory::Email);
        assert_eq!(Service::Name("imap3").category(), ServiceCategory::Email);

        // SNMP
        assert_eq!(
            Service::Name("snmp").category(),
            ServiceCategory::Management
        );
        assert_eq!(
            Service::Name("snmptrap").category(),
            ServiceCategory::Management
        );
        assert_eq!(
            Service::Name("smux").category(),
            ServiceCategory::Management
        );

        // BGP
        assert_eq!(Service::Name("bgp").category(), ServiceCategory::Network);

        // LDAP
        assert_eq!(Service::Name("ldap").category(), ServiceCategory::Identity);

        // HTTPS
        assert_eq!(Service::Name("https").category(), ServiceCategory::Web);

        // FTPS
        assert_eq!(
            Service::Name("ftps-data").category(),
            ServiceCategory::Files
        );
        assert_eq!(Service::Name("ftps").category(), ServiceCategory::Files);

        // IMAPS
        assert_eq!(Service::Name("imaps").category(), ServiceCategory::Email);

        // POP3S
        assert_eq!(Service::Name("pop3s").category(), ServiceCategory::Email);

        // SSDP
        assert_eq!(Service::Name("upnp").category(), ServiceCategory::Discovery);

        // XMPP
        assert_eq!(
            Service::Name("xmpp-client").category(),
            ServiceCategory::Chat
        );

        // HTTP
        assert_eq!(Service::Name("http-proxy").category(), ServiceCategory::Vpn);
        assert_eq!(Service::Name("http-alt").category(), ServiceCategory::Web);

        // LDAPS
        assert_eq!(
            Service::Name("ldapssl").category(),
            ServiceCategory::Identity
        );
        assert_eq!(Service::Name("ldaps").category(), ServiceCategory::Identity);

        // mDNS
        assert_eq!(Service::Name("mdns").category(), ServiceCategory::Discovery);
        assert_eq!(
            Service::Name("zeroconf").category(),
            ServiceCategory::Discovery
        );

        assert_eq!(Service::Name("dicom").category(), ServiceCategory::Other);
        assert_eq!(Service::Unknown.category(), ServiceCategory::Other);
        assert_eq!(Service::NotApplicable.category(), ServiceCategory::Other);
        assert_eq!(
            Service::Name("unregistered-service").category(),
            ServiceCategory::Other
        );
    }

    #[test]
    fn test_deserialized_service_category() {
        let json = serde_json::to_string(&Service::Name("https")).unwrap();
        let service: Service = serde_json::from_str(&json).unwrap();
        assert_eq!(service.category(), ServiceCategory::Web);
    }

    #[test]
    fn test_service_display_unknown() {
        assert_eq!(Service::Unknown.to_string(), "?");
    }

    #[test]
    fn test_service_display_not_applicable() {
        assert_eq!(Service::NotApplicable.to_string(), "-");
    }

    #[test]
    fn test_service_display_known() {
        assert_eq!(Service::Name("https").to_string(), "https");
        assert_eq!(Service::Name("mpp").to_string(), "mpp");
    }

    #[test]
    fn test_service_to_string_with_equal_prefix() {
        assert_eq!(Service::Name("mdns").to_string_with_equal_prefix(), "=mdns");
        assert_eq!(Service::Name("upnp").to_string_with_equal_prefix(), "=upnp");
        assert_eq!(Service::NotApplicable.to_string_with_equal_prefix(), "=-");
        assert_eq!(Service::Unknown.to_string_with_equal_prefix(), "=?");
    }

    #[test]
    fn test_deserialize_name() {
        let json = serde_json::to_string(&Service::Name("https")).unwrap();
        let deserialized: Service = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Service::Name("https"));
    }

    #[test]
    fn test_deserialize_unknown() {
        let json = serde_json::to_string(&Service::Unknown).unwrap();
        let deserialized: Service = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Service::Unknown);
    }

    #[test]
    fn test_deserialize_not_applicable() {
        let json = serde_json::to_string(&Service::NotApplicable).unwrap();
        let deserialized: Service = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Service::NotApplicable);
    }

    #[test]
    fn test_deserialize_invalid_variant() {
        let json = r#""InvalidVariant""#;
        assert!(serde_json::from_str::<Service>(json).is_err());
    }
}
