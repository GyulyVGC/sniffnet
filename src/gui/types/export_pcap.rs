use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::gui::types::conf::deserialize_or_default;
use crate::utils::types::file_info::ALLOWED_PCAP_EXTENSIONS;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct ExportPcap {
    #[serde(deserialize_with = "deserialize_or_default")]
    enabled: bool,
    #[serde(deserialize_with = "deserialize_or_default")]
    file_name: String,
    #[serde(deserialize_with = "deserialize_or_default")]
    directory: String,
}

impl ExportPcap {
    pub const DEFAULT_FILE_NAME: &'static str = "sniffnet.pcap";

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }

    pub fn set_directory(&mut self, directory: String) {
        self.directory = directory;
    }

    pub fn sanitize_file_name(&mut self) {
        let file_name = self.file_name.trim();

        // remove forward and backward slashes to avoid directory traversal
        let file_name = file_name.replace(['/', '\\'], "");

        // append a PCAP extension when the name doesn't already carry one
        let has_pcap_extension = Path::new(&file_name)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .is_some_and(|ext| {
                ALLOWED_PCAP_EXTENSIONS
                    .iter()
                    .any(|allowed| ext.eq_ignore_ascii_case(allowed))
            });

        self.file_name = if file_name.is_empty() || has_pcap_extension {
            file_name
        } else {
            format!("{file_name}.{}", ALLOWED_PCAP_EXTENSIONS[0])
        };
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

    fn sanitized(file_name: &str) -> String {
        let mut export_pcap = ExportPcap::default();
        export_pcap.set_file_name(file_name.to_string());
        export_pcap.sanitize_file_name();
        export_pcap.file_name().to_string()
    }

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

        export_pcap.set_file_name("test.pcap".to_string());
        assert_eq!(export_pcap.file_name(), "test.pcap");

        // the setter stores the name verbatim: sanitization is deferred
        export_pcap.set_file_name("./ciao/test\\hello.pcap".to_string());
        assert_eq!(export_pcap.file_name(), "./ciao/test\\hello.pcap");

        export_pcap.set_file_name("".to_string());
        assert_eq!(export_pcap.file_name(), "");
    }

    #[test]
    fn test_sanitize_file_name() {
        // names already carrying an allowed extension are left untouched
        for ext in ALLOWED_PCAP_EXTENSIONS {
            assert_eq!(sanitized(&format!("test.{ext}")), format!("test.{ext}"));
        }
        // the extension check is case insensitive
        assert_eq!(sanitized("test.PCAP"), "test.PCAP");
        assert_eq!(sanitized("test.PcapNg"), "test.PcapNg");

        // the default extension is appended when the name doesn't carry an allowed one
        assert_eq!(sanitized("test"), "test.pcap");
        assert_eq!(sanitized("test.txt"), "test.txt.pcap");
        assert_eq!(sanitized("test.pcap.bak"), "test.pcap.bak.pcap");
        assert_eq!(sanitized("my capture"), "my capture.pcap");
        // a leading dot doesn't make an extension
        assert_eq!(sanitized(".pcap"), ".pcap.pcap");

        // surrounding whitespace is trimmed, inner whitespace is preserved
        assert_eq!(sanitized("  test.pcap  "), "test.pcap");
        assert_eq!(sanitized("\t test \n"), "test.pcap");
        assert_eq!(sanitized(" my capture.cap "), "my capture.cap");

        // forward and backward slashes are removed to avoid directory traversal
        assert_eq!(sanitized("./ciao/test\\hello.pcap"), ".ciaotesthello.pcap");
        assert_eq!(sanitized("/tmp/test.cap"), "tmptest.cap");
        assert_eq!(sanitized("../../etc/passwd"), "....etcpasswd.pcap");
        assert_eq!(sanitized("..\\..\\etc\\passwd.pcap"), "....etcpasswd.pcap");

        // blank names are left empty, so that full_path falls back to the default one
        assert_eq!(sanitized(""), "");
        assert_eq!(sanitized("   "), "");
        assert_eq!(sanitized("/\\/"), "");

        // sanitization is idempotent
        let mut export_pcap = ExportPcap::default();
        export_pcap.set_file_name("  ./my capture  ".to_string());
        export_pcap.sanitize_file_name();
        assert_eq!(export_pcap.file_name(), ".my capture.pcap");
        export_pcap.sanitize_file_name();
        assert_eq!(export_pcap.file_name(), ".my capture.pcap");
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

        export_pcap.set_file_name("test.pcap".to_string());
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

        export_pcap.set_file_name("".to_string());
        assert_eq!(
            export_pcap.full_path(),
            Some(full_path.to_string_lossy().to_string())
        );

        export_pcap.set_directory("".to_string());
        assert_eq!(export_pcap.full_path(), Some("sniffnet.pcap".to_string()));
    }
}
