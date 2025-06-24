use iced::window;
use std::net::IpAddr;

use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::networking::types::host::{Host, HostMessage};
use crate::networking::types::info_traffic::InfoTraffic;
use crate::notifications::types::notifications::Notification;
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::web_page::WebPage;
use crate::{ChartType, IpVersion, Language, Protocol, ReportSortType, StyleType};

#[derive(Debug, Clone)]
/// Messages types that permit reacting to application interactions/subscriptions
pub enum Message {
    /// Run tasks to initialize the app
    StartApp(Option<window::Id>),
    /// Sent by the backend parsing packets; includes the capture id, new data, new hosts batched data, and whether an offline capture has finished
    TickRun(usize, InfoTraffic, Vec<HostMessage>, bool),
    /// Select network device
    DeviceSelection(String),
    /// Select IP filter
    IpVersionSelection(IpVersion, bool),
    /// Select protocol filter
    ProtocolSelection(Protocol, bool),
    /// Changed address filter
    AddressFilter(String),
    /// Changed port filter
    PortFilter(String),
    /// Select chart type to be displayed
    ChartSelection(ChartType),
    /// Select report sort type to be displayed (inspect page)
    ReportSortSelection(ReportSortType),
    /// Select host sort type to be displayed (overview page)
    HostSortSelection(SortType),
    /// Select service sort type to be displayed (overview page)
    ServiceSortSelection(SortType),
    /// Adds or removes the given host into/from the favorites
    AddOrRemoveFavorite(Host, bool),
    /// Open the supplied web page
    OpenWebPage(WebPage),
    /// Start sniffing packets
    Start,
    /// Stop sniffing process and return to initial page
    Reset,
    /// Change application style
    Style(StyleType),
    /// Deserialize a style from a path
    LoadStyle(String),
    /// Displays a modal
    ShowModal(MyModal),
    /// Opens the specified settings page
    OpenSettings(SettingsPage),
    /// Opens the last opened settings page
    OpenLastSettings,
    /// Hides the current modal
    HideModal,
    /// Hides the current setting page
    CloseSettings,
    /// Permits to change the current running page
    ChangeRunningPage(RunningPage),
    /// Select language
    LanguageSelection(Language),
    /// Set notification settings
    UpdateNotificationSettings(Notification, bool),
    /// Clear all received notifications
    ClearAllNotifications,
    /// Set notifications volume
    ChangeVolume(u8),
    /// Switch from a page to the next (previous) one if true (false), when the tab (shift+tab) key is pressed.
    SwitchPage(bool),
    /// The enter (return) key has been pressed
    ReturnKeyPressed,
    /// The esc key has been pressed
    EscKeyPressed,
    /// The reset button has been pressed or the backspace key has been pressed while running
    ResetButtonPressed,
    /// Ctrl+D keys have been pressed
    CtrlDPressed,
    /// Update search parameters of inspect page
    Search(SearchParameters),
    /// Update page result number in inspect
    UpdatePageNumber(bool),
    /// Left (false) or Right (true) arrow key has been pressed
    ArrowPressed(bool),
    /// Emit when the main window be focused
    WindowFocused,
    /// Enable or disable gradients
    GradientsSelection(GradientType),
    /// Set UI scale factor
    ChangeScaleFactor(f64),
    /// The app window position has been changed
    WindowMoved(f32, f32),
    /// The app window size has been changed
    WindowResized(f32, f32),
    /// The country MMDB custom path has been updated
    CustomCountryDb(String),
    /// The ASN MMDB custom path has been updated
    CustomAsnDb(String),
    /// Wrapper around the Quit message
    QuitWrapper,
    /// Save the configurations of the app and quit
    Quit,
    /// Copies the given string to clipboard
    CopyIp(IpAddr),
    /// Launch a new file dialog
    OpenFile(String, FileInfo, fn(String) -> Message),
    /// Toggle export pcap file
    ToggleExportPcap,
    /// The output PCAP directory has been updated
    OutputPcapDir(String),
    /// The output PCAP file name has been updated
    OutputPcapFile(String),
    /// Toggle thumbnail mode
    ToggleThumbnail(bool),
    /// Drag the window
    Drag,
    /// Ctrl+T keys have been pressed
    CtrlTPressed,
    /// Edit scale factor via keyboard shortcut
    ScaleFactorShortcut(bool),
    /// Set new release status
    SetNewerReleaseStatus(Option<bool>),
    /// Set the pcap import path
    SetPcapImport(String),
    /// Sent by the backend parsing packets at the end of an offline capture; includes all the pending hosts
    PendingHosts(usize, Vec<HostMessage>),
    /// Sent by offline captures: ticks without packets
    OfflineGap(usize, u32),
    /// Emitted every second to repeat certain tasks (such as fetching the network devices)
    Periodic,
    /// Expand or collapse the given logged notification
    ExpandNotification(usize, bool),
}
