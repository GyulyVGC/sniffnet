use listeners::Process;

/// Program / App.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Program {
    /// A known program.
    Name(String),
    /// Not identified
    Unknown,
    /// Not applicable (ARP and ICMP)
    #[default]
    NotApplicable,
}

impl Program {
    pub fn to_string_with_equal_prefix(&self) -> String {
        match self {
            Program::Name(_) | Program::NotApplicable => ["=", &self.to_string()].concat(),
            Program::Unknown => self.to_string(),
        }
    }

    pub fn is_known(&self) -> bool {
        matches!(self, Program::Name(_))
    }

    pub fn from_proc(proc: Option<Process>) -> Self {
        proc.map_or(Program::Unknown, |proc| Program::Name(proc.name))
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Program::Name(name) => write!(f, "{name}"),
            Program::Unknown => write!(f, "?"),
            Program::NotApplicable => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_display_unknown() {
        assert_eq!(Program::Unknown.to_string(), "?");
    }

    #[test]
    fn test_program_display_not_applicable() {
        assert_eq!(Program::NotApplicable.to_string(), "-");
    }

    #[test]
    fn test_program_display_known() {
        assert_eq!(
            Program::Name("Telegram".to_string()).to_string(),
            "Telegram"
        );
        assert_eq!(
            Program::Name("Google Chrome Helper".to_string()).to_string(),
            "Google Chrome Helper"
        );
    }

    #[test]
    fn test_program_to_string_with_equal_prefix() {
        assert_eq!(
            Program::Name("Telegram".to_string()).to_string_with_equal_prefix(),
            "=Telegram"
        );
        assert_eq!(
            Program::Name("Google Chrome Helper".to_string()).to_string_with_equal_prefix(),
            "=Google Chrome Helper"
        );
        assert_eq!(Program::NotApplicable.to_string_with_equal_prefix(), "=-");
        // unknown should not have the prefix
        assert_eq!(Program::Unknown.to_string_with_equal_prefix(), "?");
    }
}
