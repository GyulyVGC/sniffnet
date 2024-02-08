/// Upper layer services.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Service {
    /// One of the known services.
    Name(&'static str),
    /// Not identified
    #[default]
    Unknown,
    /// Not applicable
    NotApplicable,
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
        let test_str = Service::Unknown.to_string();
        assert_eq!(test_str, "?");
    }

    #[test]
    fn test_service_display_not_applicable() {
        let test_str = Service::NotApplicable.to_string();
        assert_eq!(test_str, "-");
    }
}
