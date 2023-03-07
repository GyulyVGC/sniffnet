use crate::utility::translations::{
    language_translation, notifications_translation, style_translation,
};
use crate::Language;

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SettingsPage {
    /// Settings Notifications page.
    Notifications,
    /// Settings Appearance page.
    Appearance,
    /// Settings Language page.
    Language,
}

impl SettingsPage {
    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            SettingsPage::Notifications => notifications_translation(language),
            SettingsPage::Appearance => style_translation(language),
            SettingsPage::Language => language_translation(language),
        }
    }

    pub fn next(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::Appearance,
            SettingsPage::Appearance => SettingsPage::Language,
            SettingsPage::Language => SettingsPage::Notifications,
        }
    }
}
