use crate::ConfigWindow;
use crate::gui::types::settings::Settings;
use crate::networking::types::config_device::ConfigDevice;
use confy::ConfyError;

pub static CONFIGS: std::sync::LazyLock<Configs> = std::sync::LazyLock::new(Configs::load);

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Configs {
    pub settings: Settings,
    pub device: ConfigDevice,
    pub window: ConfigWindow,
}

impl Configs {
    /// This should only be used directly to load fresh configs;
    /// use `CONFIGS` instead to access the initial instance
    pub fn load() -> Self {
        Configs {
            settings: Settings::load(),
            device: ConfigDevice::load(),
            window: ConfigWindow::load(),
        }
    }

    pub fn store(self) -> Result<(), ConfyError> {
        self.settings.store()?;
        self.device.store()?;
        self.window.store()?;
        Ok(())
    }
}
