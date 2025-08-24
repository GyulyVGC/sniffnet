use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ExportPcap {
    enabled: bool,
    file_name: String,
    directory: String,
}

impl ExportPcap {
    pub const DEFAULT_FILE_NAME: &'static str = "sniffnet.pcap";

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_file_name(&mut self, file_name: &str) {
        // remove forward and backward slashes to avoid directory traversal
        self.file_name = file_name.replace(['/', '\\'], "");
    }

    pub fn set_directory(&mut self, directory: String) {
        self.directory = directory;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn full_path(&self) -> Option<String> {
        if self.enabled {
            let mut full_path = PathBuf::from(&self.directory);
            let file_name = if self.file_name.is_empty() {
                Self::DEFAULT_FILE_NAME
            } else {
                &self.file_name
            };
            full_path.push(file_name);
            Some(full_path.to_string_lossy().to_string())
        } else {
            None
        }
    }
}

impl Default for ExportPcap {
    fn default() -> Self {
        ExportPcap {
            enabled: false,
            file_name: String::from(Self::DEFAULT_FILE_NAME),
            directory: std::env::var("HOME").unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let export_pcap = ExportPcap::default();
        assert_eq!(export_pcap.enabled(), false);
        assert_eq!(export_pcap.file_name(), "sniffnet.pcap");
        assert_eq!(
            export_pcap.directory(),
            std::env::var("HOME").unwrap_or_default()
        );
    }

    #[test]
    fn test_toggle() {
        let mut export_pcap = ExportPcap::default();
        assert_eq!(export_pcap.enabled(), false);

        export_pcap.toggle();
        assert_eq!(export_pcap.enabled(), true);

        export_pcap.toggle();
        assert_eq!(export_pcap.enabled(), false);
    }

    #[test]
    fn test_set_file_name() {
        let mut export_pcap = ExportPcap::default();
        assert_eq!(export_pcap.file_name(), "sniffnet.pcap");

        export_pcap.set_file_name("test.pcap");
        assert_eq!(export_pcap.file_name(), "test.pcap");

        export_pcap.set_file_name("./ciao/test\\hello.pcap");
        assert_eq!(export_pcap.file_name(), ".ciaotesthello.pcap");

        export_pcap.set_file_name("");
        assert_eq!(export_pcap.file_name(), "");
    }

    #[test]
    fn test_set_directory() {
        let mut export_pcap = ExportPcap::default();
        assert_eq!(
            export_pcap.directory(),
            std::env::var("HOME").unwrap_or_default()
        );

        export_pcap.set_directory("/tmp".to_string());
        assert_eq!(export_pcap.directory(), "/tmp");
    }

    #[test]
    fn test_full_path() {
        let mut dir = std::env::var("HOME").unwrap_or_default();
        if !dir.is_empty() {
            dir.push('/');
        }

        let mut export_pcap = ExportPcap::default();
        assert_eq!(export_pcap.full_path(), None);

        export_pcap.toggle();
        assert_eq!(
            export_pcap.full_path(),
            Some(format!("{dir}sniffnet.pcap",))
        );

        export_pcap.set_file_name("test.pcap");
        assert_eq!(export_pcap.full_path(), Some(format!("{dir}test.pcap",)));

        let mut full_path = PathBuf::from("/tmp");
        full_path.push("test.pcap");

        export_pcap.set_directory("/tmp".to_string());
        assert_eq!(
            export_pcap.full_path(),
            Some(full_path.to_string_lossy().to_string())
        );

        export_pcap.toggle();
        assert_eq!(export_pcap.full_path(), None);

        export_pcap.toggle();
        assert_eq!(
            export_pcap.full_path(),
            Some(full_path.to_string_lossy().to_string())
        );

        let mut full_path = PathBuf::from("/tmp");
        full_path.push("sniffnet.pcap");

        export_pcap.set_file_name("");
        assert_eq!(
            export_pcap.full_path(),
            Some(full_path.to_string_lossy().to_string())
        );

        export_pcap.set_directory("".to_string());
        assert_eq!(export_pcap.full_path(), Some("sniffnet.pcap".to_string()));
    }
}
