use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::types::running_page::RunningPage;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::networking::types::host::Host;
use crate::networking::types::search_parameters::SearchParameters;
use crate::notifications::types::notifications::Notification;
use crate::{
    AppProtocol, ChartType, IpVersion, Language, ReportSortType, StyleType, TransProtocol,
};

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
    IpVersionSelection(IpVersion),
    /// Select transport filter
    TransportProtocolSelection(TransProtocol),
    /// Select application filter
    AppProtocolSelection(AppProtocol),
    /// Select chart type to be displayed
    ChartSelection(ChartType),
    /// Select report type to be displayed
    ReportSortSelection(ReportSortType),
    /// Adds or removes the given host into/from the favorites
    AddOrRemoveFavorite(Host, bool),
    /// Open Sniffnet's complete textual report
    OpenReport,
    /// Open Sniffnet's GitHub main page if true is passed, latest release page otherwise
    OpenGithub(bool),
    /// Start sniffing packets
    Start,
    /// Stop sniffing process and return to initial page
    Reset,
    /// Change application style
    Style(StyleType),
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
    /// Quits the app. Used when Ctrl+Q keys are pressed.
    Quit,
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
}
