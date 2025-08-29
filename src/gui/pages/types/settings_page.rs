use crate::gui::types::message::Message;
use crate::translations::translations::{notifications_translation, style_translation};
use crate::translations::translations_3::general_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};
use serde::{Deserialize, Serialize};

/// This enum defines the current settings page.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub enum SettingsPage {
    /// Settings Notifications page.
    #[default]
    Notifications,
    /// Settings Appearance page.
    Appearance,
    /// General settings.
    General,
}

impl SettingsPage {
    pub const ALL: [SettingsPage; 3] = [
        SettingsPage::Notifications,
        SettingsPage::Appearance,
        SettingsPage::General,
    ];

    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            SettingsPage::Notifications => notifications_translation(language),
            SettingsPage::Appearance => style_translation(language),
            SettingsPage::General => general_translation(language),
        }
    }

    pub fn next(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::Appearance,
            SettingsPage::Appearance => SettingsPage::General,
            SettingsPage::General => SettingsPage::Notifications,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::General,
            SettingsPage::Appearance => SettingsPage::Notifications,
            SettingsPage::General => SettingsPage::Appearance,
        }
    }

    pub fn icon<'a>(self) -> iced::widget::Text<'a, StyleType> {
        match self {
            SettingsPage::Notifications => Icon::Notification,
            SettingsPage::Appearance => Icon::HalfSun,
            SettingsPage::General => Icon::Generals,
        }
        .to_text()
    }

    pub fn action(self) -> Message {
        Message::OpenSettings(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::gui::pages::types::settings_page::SettingsPage;

    #[test]
    fn test_previous_settings_page() {
        assert_eq!(
            SettingsPage::Notifications.previous(),
            SettingsPage::General
        );
        assert_eq!(
            SettingsPage::Appearance.previous(),
            SettingsPage::Notifications
        );
        assert_eq!(SettingsPage::General.previous(), SettingsPage::Appearance);
    }

    #[test]
    fn test_next_settings_page() {
        assert_eq!(SettingsPage::Notifications.next(), SettingsPage::Appearance);
        assert_eq!(SettingsPage::Appearance.next(), SettingsPage::General);
        assert_eq!(SettingsPage::General.next(), SettingsPage::Notifications);
    }
}
