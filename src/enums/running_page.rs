use crate::utility::translations::{notifications_translation, overview_translation};
use crate::Language;

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RunningPage {
    /// Overview page.
    Overview,
    // /// Inspect page.
    // Inspect,
    /// Notifications page.
    Notifications,
}

impl RunningPage {
    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            RunningPage::Overview => overview_translation(language),
            // RunningPage::Inspect => inspect_translation(language),
            RunningPage::Notifications => notifications_translation(language),
        }
    }
}
