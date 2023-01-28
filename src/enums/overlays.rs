use crate::utility::translations::{
    appearance_translation, language_translation, notifications_translation,
};
use crate::Language;

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Overlays {
    /// Settings Notifications page.
    SettingsNotifications,
    /// Settings Appearance page.
    SettingsAppearance,
    /// Settings Language page.
    SettingsLanguage,
    /// Alert modal.
    Alert,
}

impl Overlays {
    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            Overlays::SettingsNotifications => notifications_translation(language),
            Overlays::SettingsAppearance => appearance_translation(language),
            Overlays::SettingsLanguage => language_translation(language),
            _ => "",
        }
    }
}
