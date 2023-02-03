use crate::enums::overlay::Overlay;
use crate::enums::running_page::RunningPage;
use crate::structs::notifications::{FavoriteNotification, ThresholdNotification};
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
    //OpenReport,
    /// Open Sniffnet's GitHub page
    OpenGithub,
    /// Start sniffing packets
    Start,
    /// Stop sniffing process and return to initial page
    Reset,
    /// Change application style (day or night)
    Style(StyleType),
    /// Manage waiting time
    Waiting,
    /// Displays an overlay
    ShowModal(Overlay),
    /// Hides the current overlay modal; if true is passed, config file is updated
    HideModal(bool),
    /// Permits to change the current running page
    ChangeRunningPage(RunningPage),
    /// Select language
    LanguageSelection(Language),
    /// Set packets notification
    UpdatePacketsNotification(ThresholdNotification, bool),
    /// Set packets notification
    UpdateBytesNotification(ThresholdNotification, bool),
    /// Set packets notification
    UpdateFavoriteNotification(FavoriteNotification, bool),
    /// Set notifications volume
    ChangeVolume(u8),
}
