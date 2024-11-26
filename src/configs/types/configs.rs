use crate::{ConfigDevice, ConfigSettings, ConfigWindow};
use once_cell::sync::Lazy;

pub static CONFIGS: Lazy<Configs> = Lazy::new(Configs::load);

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Configs {
    pub settings: ConfigSettings,
    pub device: ConfigDevice,
    pub window: ConfigWindow,
}

impl Configs {
    /// This should only be used directly to load fresh configs;
    /// use `CONFIGS` instead to access the initial instance
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
