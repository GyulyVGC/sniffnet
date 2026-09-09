#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpdatesStatus {
    #[default]
    Unknown,
    InProgress,
    UpToDate,
    UpdateAvailable,
}
