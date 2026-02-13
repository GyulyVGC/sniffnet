/// Process listening on a local port.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Process {
    /// A known process.
    Known(listeners::Process),
    /// Not identified
    #[default]
    Unknown,
    /// Not applicable
    NotApplicable,
}

impl Process {
    pub fn to_string_with_equal_prefix(&self) -> String {
        ["=", &self.to_string()].concat()
    }
}

impl std::fmt::Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Process::Known(p) => write!(f, "{} [{}]", p.name, p.pid),
            Process::Unknown => write!(f, "?"),
            Process::NotApplicable => write!(f, "-"),
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
