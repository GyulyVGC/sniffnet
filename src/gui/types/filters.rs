use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Filters {
    expanded: bool,
    bpf: String,
}

impl Filters {
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn set_bpf(&mut self, bpf: String) {
        self.bpf = bpf;
    }

    pub fn expanded(&self) -> bool {
        self.expanded
    }

    pub fn bpf(&self) -> &str {
        &self.bpf
    }

    pub fn is_some_filter_active(&self) -> bool {
        self.expanded && !self.bpf.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let filters = Filters::default();
        assert_eq!(filters.expanded(), false);
        assert_eq!(filters.bpf(), "");
    }

    #[test]
    fn test_toggle() {
        let mut filters = Filters::default();
        assert_eq!(filters.expanded(), false);

        filters.toggle();
        assert_eq!(filters.expanded(), true);

        filters.toggle();
        assert_eq!(filters.expanded(), false);
    }

    #[test]
    fn test_set_bpf() {
        let mut filters = Filters::default();
        assert_eq!(filters.bpf(), "");

        filters.set_bpf("tcp port 80".to_string());
        assert_eq!(filters.bpf(), "tcp port 80");

        filters.set_bpf("  udp port 53  ".to_string());
        assert_eq!(filters.bpf(), "  udp port 53  ");
    }

    #[test]
    fn test_is_some_filter_active() {
        let mut filters = Filters::default();
        assert_eq!(filters.is_some_filter_active(), false);

        filters.toggle();
        assert_eq!(filters.is_some_filter_active(), false);

        filters.set_bpf("tcp port 80".to_string());
        assert_eq!(filters.is_some_filter_active(), true);

        filters.toggle();
        assert_eq!(filters.is_some_filter_active(), false);

        filters.toggle();
        assert_eq!(filters.is_some_filter_active(), true);

        filters.set_bpf(" \t \n ".to_string());
        assert_eq!(filters.is_some_filter_active(), false);
    }
}
