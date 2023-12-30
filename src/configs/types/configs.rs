use crate::{ConfigDevice, ConfigSettings, ConfigWindow};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Configs {
    pub settings: ConfigSettings,
    pub device: ConfigDevice,
    pub window: ConfigWindow,
}

impl Configs {
    pub fn load() -> Self {
        Configs {
            settings: ConfigSettings::load(),
            device: ConfigDevice::load(),
            window: ConfigWindow::load(),
        }
    }

    pub fn store(self) {
        self.settings.store();
        self.device.store();
        self.window.store();
    }
}
