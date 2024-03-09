use crate::gui::types::message::Message;
use crate::translations::translations::{notifications_translation, overview_translation};
use crate::translations::translations_2::inspect_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

/// This enum defines the current GUI page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RunningPage {
    /// Initial page.
    Init,
    /// Overview page.
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
            RunningPage::Init => "",
        }
    }

    pub fn next(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Inspect,
            RunningPage::Inspect => RunningPage::Notifications,
            RunningPage::Notifications => RunningPage::Overview,
            RunningPage::Init => RunningPage::Init,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Notifications,
            RunningPage::Inspect => RunningPage::Overview,
            RunningPage::Notifications => RunningPage::Inspect,
            RunningPage::Init => RunningPage::Init,
        }
    }

    pub fn icon(self) -> iced::widget::Text<'static, StyleType> {
        match self {
            RunningPage::Overview => Icon::Overview,
            RunningPage::Inspect => Icon::Inspect,
            RunningPage::Notifications => Icon::Notification,
            RunningPage::Init => Icon::Sniffnet,
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
