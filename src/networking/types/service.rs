use std::fmt;
use std::str::FromStr;

/// Upper layer services.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Service {
    /// One of the known services.
    Name(String),
    /// Not identified
    #[default]
    Unknown,
    /// Not applicable
    NotApplicable,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Service::Name(name) => write!(f, "{name}"),
            Service::Unknown => write!(f, "?"),
            Service::NotApplicable => write!(f, "-"),
        }
    }
}

impl FromStr for Service {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "" | "?" => Self::Unknown,
            "-" => Self::NotApplicable,
            name => Self::Name(name.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_protocol_display_unknown() {
        let test_str = Service::Unknown.to_string();
        assert_eq!(test_str, "?");
    }

    #[test]
    fn app_protocol_display_not_applicable() {
        let test_str = Service::NotApplicable.to_string();
        assert_eq!(test_str, "-");
    }
}
