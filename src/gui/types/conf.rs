use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::types::config_window::ConfigWindow;
use crate::gui::types::export_pcap::ExportPcap;
use crate::gui::types::filters::Filters;
use crate::gui::types::settings::Settings;
use crate::networking::types::capture_context::CaptureSourcePicklist;
use crate::networking::types::config_device::ConfigDevice;
use crate::report::types::sort_type::SortType;
#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};
#[cfg(not(test))]
use crate::{SNIFFNET_LOWERCASE, location};
#[cfg(not(test))]
use confy::ConfyError;
use serde::{Deserialize, Serialize};

pub static CONF: std::sync::LazyLock<Conf> = std::sync::LazyLock::new(Conf::load);

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct Conf {
    /// Parameters from settings pages
    pub settings: Settings,
    /// Last selected network device name
    pub device: ConfigDevice,
    /// Window configuration, such as size and position
    pub window: ConfigWindow,
    /// Capture source picklist, to select the source of the capture
    pub capture_source_picklist: CaptureSourcePicklist,
    /// BPF filter program to be applied to the capture
    pub filters: Filters,
    /// Report sort type (inspect page)
    pub report_sort_type: SortType,
    /// Host sort type (overview page)
    pub host_sort_type: SortType,
    /// Service sort type (overview page)
    pub service_sort_type: SortType,
    /// Remembers the last opened setting page
    pub last_opened_setting: SettingsPage,
    /// Remembers the last opened running page
    pub last_opened_page: RunningPage,
    /// Information about PCAP file export
    pub export_pcap: ExportPcap,
    /// Import path for PCAP file
    pub import_pcap_path: String,
}

impl Conf {
    const FILE_NAME: &'static str = "conf";

    /// This should only be used directly to load fresh configurations;
    /// use `CONF` instead to access the initial instance
    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(conf) = confy::load::<Conf>(SNIFFNET_LOWERCASE, Self::FILE_NAME) {
            conf
        } else {
            let _ = Conf::default().store();
            Conf::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) -> Result<(), ConfyError> {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).log_err(location!())
    }
}

#[cfg(test)]
mod tests {
    use crate::gui::types::conf::Conf;

    impl Conf {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<Conf>(Conf::test_path()).unwrap_or_else(|_| Conf::default())
        }

        pub fn store(self) -> Result<(), confy::ConfyError> {
            confy::store_path(Conf::test_path(), self)
        }
    }
}
