//! Module defining the `ConfigDevice` struct, which allows to save and reload
//! the application default configuration.

use crate::networking::types::my_device::MyDevice;
#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};
#[cfg(not(test))]
use crate::{SNIFFNET_LOWERCASE, location};
use pcap::{Device, DeviceFlags};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ConfigDevice {
    pub device_name: String,
}

impl Default for ConfigDevice {
    fn default() -> Self {
        Self {
            device_name: Device::lookup()
                .unwrap_or(None)
                .unwrap_or_else(|| Device {
                    name: String::new(),
                    desc: None,
                    addresses: vec![],
                    flags: DeviceFlags::empty(),
                })
                .name,
        }
    }
}

impl ConfigDevice {
    const FILE_NAME: &'static str = "device";

    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(device) = confy::load::<ConfigDevice>(SNIFFNET_LOWERCASE, Self::FILE_NAME) {
            device
        } else {
            let _ = confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, ConfigDevice::default())
                .log_err(location!());
            ConfigDevice::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) -> Result<(), confy::ConfyError> {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).log_err(location!())
    }

    pub fn to_my_device(&self) -> MyDevice {
        for device in Device::list().unwrap_or_default() {
            if device.name.eq(&self.device_name) {
                return MyDevice::from_pcap_device(device);
            }
        }
        let standard_device = Device::lookup().unwrap_or(None).unwrap_or_else(|| Device {
            name: String::new(),
            desc: None,
            addresses: vec![],
            flags: DeviceFlags::empty(),
        });
        MyDevice::from_pcap_device(standard_device)
    }
}

#[cfg(test)]
mod tests {
    use crate::ConfigDevice;

    impl ConfigDevice {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<ConfigDevice>(ConfigDevice::test_path())
                .unwrap_or_else(|_| ConfigDevice::default())
        }

        pub fn store(self) -> Result<(), confy::ConfyError> {
            confy::store_path(ConfigDevice::test_path(), self)
        }
    }
}
