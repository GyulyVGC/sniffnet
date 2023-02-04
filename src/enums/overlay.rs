use crate::utility::translations::{
    language_translation, notifications_translation, style_translation,
};
use crate::Language;

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Overlay {
    /// Settings Notifications page.
    SettingsNotifications,
    /// Settings Appearance page.
    SettingsAppearance,
    /// Settings Language page.
    SettingsLanguage,
    /// Alert modal.
    Alert,
}

impl Overlay {
    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            Overlay::SettingsNotifications => notifications_translation(language),
            Overlay::SettingsAppearance => style_translation(language),
            Overlay::SettingsLanguage => language_translation(language),
            Overlay::Alert => "",
        }
    }
}
