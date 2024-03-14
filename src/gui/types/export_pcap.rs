use std::path::PathBuf;

pub struct ExportPcap {
    enabled: bool,
    file_name: String,
    directory: PathBuf,
}

impl ExportPcap {
    pub const DEFAULT_FILE_NAME: &'static str = "sniffnet.pcap";

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }

    pub fn set_directory(&mut self, directory: PathBuf) {
        self.directory = directory;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn full_path(&self) -> Option<String> {
        if self.enabled {
            let mut full_path = self.directory.clone();
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
            directory: PathBuf::from(std::env::var("HOME").unwrap_or_default()),
        }
    }
}
