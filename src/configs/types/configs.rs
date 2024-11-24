use crate::{ConfigDevice, ConfigSettings, ConfigWindow};
use once_cell::sync::Lazy;

pub static CONFIGS: Lazy<Configs> = Lazy::new(|| Configs::load());

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Configs {
    pub settings: ConfigSettings,
    pub device: ConfigDevice,
    pub window: ConfigWindow,
}

impl Configs {
    /// This shouldn't be used directly outside tests, use `CONFIGS` instead
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
