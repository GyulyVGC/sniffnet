use serde::de::{self, Deserializer, VariantAccess};
use serde::{Deserialize, Serialize};

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
