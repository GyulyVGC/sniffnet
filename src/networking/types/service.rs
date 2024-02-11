/// Upper layer services.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Service {
    /// One of the known services.
    Name(&'static str),
    /// Not identified
    #[default]
    Unknown,
    /// Not applicable
    NotApplicable,
}

impl Service {
    pub fn to_string_with_equal_prefix(self) -> String {
        match self {
            Service::Name(_) | Service::NotApplicable => ["=", &self.to_string()].concat(),
            Service::Unknown => self.to_string(),
        }
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
        // unknown should not have the prefix
        assert_eq!(Service::Unknown.to_string_with_equal_prefix(), "?");
    }
}
