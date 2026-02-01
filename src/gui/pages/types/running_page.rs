use crate::gui::types::message::Message;
use crate::translations::translations::{notifications_translation, overview_translation};
use crate::translations::translations_2::inspect_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};
use serde::{Deserialize, Serialize};

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub enum RunningPage {
    /// Overview page.
    #[default]
    Overview,
    /// Inspect page.
    Inspect,
    /// Notifications page.
    Notifications,
}

impl RunningPage {
    pub const ALL: [RunningPage; 3] = [
        RunningPage::Overview,
        RunningPage::Inspect,
        RunningPage::Notifications,
    ];

    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            RunningPage::Overview => overview_translation(language),
            RunningPage::Inspect => inspect_translation(language),
            RunningPage::Notifications => notifications_translation(language),
        }
    }

    pub fn next(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Inspect,
            RunningPage::Inspect => RunningPage::Notifications,
            RunningPage::Notifications => RunningPage::Overview,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Notifications,
            RunningPage::Inspect => RunningPage::Overview,
            RunningPage::Notifications => RunningPage::Inspect,
        }
    }

    pub fn icon<'a>(self) -> iced::widget::Text<'a, StyleType> {
        match self {
            RunningPage::Overview => Icon::Overview,
            RunningPage::Inspect => Icon::Inspect,
            RunningPage::Notifications => Icon::Notification,
        }
        .to_text()
    }

    pub fn action(self) -> Message {
        Message::ChangeRunningPage(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::RunningPage;

    #[test]
    fn test_previous_running_page() {
        assert_eq!(RunningPage::Overview.previous(), RunningPage::Notifications);
        assert_eq!(RunningPage::Notifications.previous(), RunningPage::Inspect);
        assert_eq!(RunningPage::Inspect.previous(), RunningPage::Overview);
    }

    #[test]
    fn test_next_running_page() {
        assert_eq!(RunningPage::Overview.next(), RunningPage::Inspect);
        assert_eq!(RunningPage::Inspect.next(), RunningPage::Notifications);
        assert_eq!(RunningPage::Notifications.next(), RunningPage::Overview);
    }
}
