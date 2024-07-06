use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::networking::types::host::Host;
use crate::notifications::types::notifications::Notification;
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::utils::types::file_info::FileInfo;
use crate::utils::types::web_page::WebPage;
use crate::{ChartType, IpVersion, Language, Protocol, ReportSortType, StyleType};

#[derive(Debug, Clone)]
/// Messages types that permit to react to application interactions/subscriptions
pub enum Message {
    /// Every 5 seconds
    TickInit,
    /// Every 1 second
    TickRun,
    /// Select adapter
    AdapterSelection(String),
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
    /// Manage waiting time
    Waiting,
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
    /// Set packets notification
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
    WindowMoved(i32, i32),
    /// The app window size has been changed
    WindowResized(u32, u32),
    /// The country MMDB custom path has been updated
    CustomCountryDb(String),
    /// The ASN MMDB custom path has been updated
    CustomAsnDb(String),
    /// Save the configurations of the app and quit
    CloseRequested,
    /// Copies the given string to clipboard
    CopyIp(String),
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
}
