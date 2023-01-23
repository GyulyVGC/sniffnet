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
