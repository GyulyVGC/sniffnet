use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::types::config_window::ConfigWindow;
use crate::gui::types::export_pcap::ExportPcap;
use crate::gui::types::filters::Filters;
use crate::gui::types::settings::Settings;
use crate::networking::types::capture_context::CaptureSourcePicklist;
use crate::networking::types::config_device::ConfigDevice;
use crate::networking::types::data_representation::DataRepr;
use crate::report::types::sort_type::SortType;
#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};
#[cfg(not(test))]
use crate::{SNIFFNET_LOWERCASE, location};
#[cfg(not(test))]
use confy::ConfyError;
use serde::{Deserialize, Deserializer, Serialize};

pub static CONF: std::sync::LazyLock<Conf> = std::sync::LazyLock::new(Conf::load);

/// Application configurations structure
///
/// This structure holds all the configuration parameters for the application,
/// and is serialized/deserialized using `confy` crate to store/load from disk.
///
/// ### IMPORTANT NOTE
///
/// In order to load it in a robust, fault-tolerant, backward-compatible way,
/// there are various constraints to be satisfied when deserializing:
/// - missing fields must be filled with default values
///   - the main struct and all nested structs must implement `Default` and be annotated with `#[serde(default)]`
///   - this populates missing fields from the struct's `Default` implementation
/// - invalid fields must be replaced with default values
///   - all fields must be annotated with `#[serde(deserialize_with = "deserialize_or_default")]`
///   - this populates invalid fields from the field's type `Default` implementation
/// - extra fields must be ignored
///   - this is the default behavior of `serde`
/// - right after deserialization, certain fields must be sanitized
///   - this is to ensure that fields deserialized correctly but with "weird" values are fixed
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
#[serde(default)]
pub struct Conf {
    /// Capture source picklist, to select the source of the capture
    #[serde(deserialize_with = "deserialize_or_default")]
    pub capture_source_picklist: CaptureSourcePicklist,
    /// Import path for PCAP file
    #[serde(deserialize_with = "deserialize_or_default")]
    pub import_pcap_path: String,
    /// Remembers the last opened setting page
    #[serde(deserialize_with = "deserialize_or_default")]
    pub last_opened_setting: SettingsPage,
    /// Remembers the last opened running page
    #[serde(deserialize_with = "deserialize_or_default")]
    pub last_opened_page: RunningPage,
    /// Data representation
    #[serde(deserialize_with = "deserialize_or_default")]
    pub data_repr: DataRepr,
    /// Host sort type (overview page)
    #[serde(deserialize_with = "deserialize_or_default")]
    pub host_sort_type: SortType,
    /// Service sort type (overview page)
    #[serde(deserialize_with = "deserialize_or_default")]
    pub service_sort_type: SortType,
    /// Program sort type (overview page)
    #[serde(deserialize_with = "deserialize_or_default")]
    pub program_sort_type: SortType,
    /// Report sort type (inspect page)
    #[serde(deserialize_with = "deserialize_or_default")]
    pub report_sort_type: SortType,
    // ---------------------------------------------------------------------------------------------
    /// Window configuration, such as size and position
    #[serde(deserialize_with = "deserialize_or_default")]
    pub window: ConfigWindow,
    /// Last selected network device name
    #[serde(deserialize_with = "deserialize_or_default")]
    pub device: ConfigDevice,
    /// BPF filter program to be applied to the capture
    #[serde(deserialize_with = "deserialize_or_default")]
    pub filters: Filters,
    /// Information about PCAP file export
    #[serde(deserialize_with = "deserialize_or_default")]
    pub export_pcap: ExportPcap,
    /// Parameters from settings pages
    #[serde(deserialize_with = "deserialize_or_default")]
    pub settings: Settings,
}

impl Conf {
    pub(crate) const FILE_NAME: &'static str = "conf";

    /// This should only be used directly to load fresh configurations;
    /// use `CONF` instead to access the initial instance
    #[cfg(not(test))]
    fn load() -> Self {
        let mut conf = if let Ok(conf) = confy::load::<Conf>(SNIFFNET_LOWERCASE, Self::FILE_NAME) {
            conf
        } else {
            let _ = Conf::default().store();
            Conf::default()
        };

        // sanitize Conf...

        // check scale factor validity
        if !(0.3..=3.0).contains(&conf.settings.scale_factor) {
            conf.settings.scale_factor = 1.0;
        }

        // sanitize window parameters
        conf.window.sanitize(conf.settings.scale_factor);

        // check sound volume validity
        if !(0..=100).contains(&conf.settings.notifications.volume) {
            conf.settings.notifications.volume = 50;
        }

        conf
    }

    #[cfg(not(test))]
    pub fn store(self) -> Result<(), ConfyError> {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).log_err(location!())
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn deserialize_or_default<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    Ok(T::deserialize(deserializer).unwrap_or_default())
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
