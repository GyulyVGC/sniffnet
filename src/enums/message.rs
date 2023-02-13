use crate::enums::my_overlay::MyOverlay;
use crate::enums::running_page::RunningPage;
use crate::structs::notifications::{BytesNotification, FavoriteNotification, PacketsNotification};
use crate::{AppProtocol, ChartType, IpVersion, Language, ReportType, StyleType, TransProtocol};

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
    ReportSelection(ReportType),
    /// Saves the given connection into the favorites
    SaveConnection(usize),
    /// Un-saves the given connection into the favorites
    UnSaveConnection(usize),
    /// Open Sniffnet's complete textual report
    OpenReport,
    /// Open Sniffnet's GitHub page
    OpenGithub,
    /// Start sniffing packets
    Start,
    /// Stop sniffing process and return to initial page
    Reset,
    /// Change application style
    Style(StyleType),
    /// Manage waiting time
    Waiting,
    /// Displays an overlay
    ShowModal(MyOverlay),
    /// Hides the current overlay modal; if true is passed, config file is updated
    HideModal(bool),
    /// Permits to change the current running page
    ChangeRunningPage(RunningPage),
    /// Select language
    LanguageSelection(Language),
    /// Set packets notification
    UpdatePacketsNotification(PacketsNotification, bool),
    /// Set bytes notification
    UpdateBytesNotification(BytesNotification, bool),
    /// Set favorite notification
    UpdateFavoriteNotification(FavoriteNotification, bool),
    /// Clear all received notifications
    ClearAllNotifications,
    /// Set notifications volume
    ChangeVolume(u8),
    /// Quits the app. Used when Ctrl+Q keys are pressed.
    Exit,
}
