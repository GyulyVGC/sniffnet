/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RunningPage {
    /// Overview page.
    Overview,
    /// Inspect page.
    Inspect,
    /// Notifications page.
    Notifications,
}
