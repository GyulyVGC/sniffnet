use crate::gui::styles::types::custom_palette::{CustomPalette, ExtraStyles};
use crate::gui::styles::types::style_type::StyleType;
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
        let mut settings = ConfigSettings::load();
        let advanced_settings = ConfigAdvancedSettings::load();

        if let Some(style_path) = &advanced_settings.style_path {
            // Don't clobber the previously set style if the path is broken
            if let Ok(style) = CustomPalette::from_file(style_path)
                .map(|palette| StyleType::Custom(ExtraStyles::CustomToml(palette)))
            {
                settings.style = style;
            }
        }

        Configs {
            settings,
            device: ConfigDevice::load(),
            advanced_settings,
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
