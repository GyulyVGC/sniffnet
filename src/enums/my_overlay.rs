use crate::utility::translations::{
    language_translation, notifications_translation, style_translation,
};
use crate::Language;

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MyOverlay {
    /// Settings Notifications page.
    SettingsNotifications,
    /// Settings Appearance page.
    SettingsAppearance,
    /// Settings Language page.
    SettingsLanguage,
    /// Quit modal.
    Quit,
    /// Clear all modal.
    ClearAll,
}

impl MyOverlay {
    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            MyOverlay::SettingsNotifications => notifications_translation(language),
            MyOverlay::SettingsAppearance => style_translation(language),
            MyOverlay::SettingsLanguage => language_translation(language),
            MyOverlay::Quit | MyOverlay::ClearAll => "",
        }
    }
}
