use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::types::config_window::ConfigWindow;
use crate::gui::types::export_pcap::ExportPcap;
use crate::gui::types::filters::Filters;
use crate::gui::types::settings::Settings;
use crate::networking::types::capture_context::CaptureSourcePicklist;
use crate::networking::types::config_device::ConfigDevice;
use crate::report::types::report_sort_type::ReportSortType;
use crate::report::types::sort_type::SortType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
    pub report_sort_type: ReportSortType,
    /// Host sort type (overview page)
    pub host_sort_type: SortType,
    /// Service sort type (overview page)
    pub service_sort_type: SortType,
    /// Remembers the last opened setting page
    pub last_opened_setting: SettingsPage,
    /// Information about PCAP file export
    pub export_pcap: ExportPcap,
    /// Import path for PCAP file
    pub import_pcap_path: String,
}
