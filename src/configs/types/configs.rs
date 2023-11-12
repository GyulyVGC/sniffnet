use crate::{ConfigAdvancedSettings, ConfigDevice, ConfigSettings, ConfigWindow};

#[derive(Default)]
pub struct Configs {
    pub settings: ConfigSettings,
    pub device: ConfigDevice,
    pub advanced_settings: ConfigAdvancedSettings,
    pub window: ConfigWindow,
}

impl Configs {
    pub fn load() -> Self {
        Configs {
            settings: ConfigSettings::load(),
            device: ConfigDevice::load(),
            advanced_settings: ConfigAdvancedSettings::load(),
            window: ConfigWindow::load(),
        }
    }

    pub fn store(self) {
        self.settings.store();
        self.device.store();
        self.advanced_settings.store();
        self.window.store();
    }
}
