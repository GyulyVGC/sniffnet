//! Module defining the `ConfigDevice` struct, which allows to save and reload
//! the application default configuration.

use std::sync::{Arc, Mutex};

use pcap::{Device, DeviceFlags};
use serde::{Deserialize, Serialize};

use crate::networking::types::my_device::MyDevice;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigDevice {
    pub device_name: String,
}

impl Default for ConfigDevice {
    fn default() -> Self {
        Self {
            device_name: Device::lookup()
                .unwrap_or(None)
                .unwrap_or(Device {
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
    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(device) = confy::load::<ConfigDevice>("sniffnet", "device") {
            device
        } else {
            confy::store("sniffnet", "device", ConfigDevice::default()).unwrap_or(());
            ConfigDevice::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) {
        confy::store("sniffnet", "device", self).unwrap_or(());
    }

    pub fn to_my_device(&self) -> MyDevice {
        for device in Device::list().unwrap_or_default() {
            if device.name.eq(&self.device_name) {
                return MyDevice {
                    name: device.name,
                    desc: device.desc,
                    addresses: Arc::new(Mutex::new(device.addresses)),
                };
            }
        }
        let standard_device = Device::lookup().unwrap_or(None).unwrap_or(Device {
            name: String::new(),
            desc: None,
            addresses: vec![],
            flags: DeviceFlags::empty(),
        });
        MyDevice {
            name: standard_device.name,
            desc: standard_device.desc,
            addresses: Arc::new(Mutex::new(standard_device.addresses)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ConfigDevice;

    impl ConfigDevice {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), "device")
        }

        pub fn load() -> Self {
            confy::load_path::<ConfigDevice>(ConfigDevice::test_path())
                .unwrap_or(ConfigDevice::default())
        }

        pub fn store(self) {
            confy::store_path(ConfigDevice::test_path(), self).unwrap_or(());
        }
    }
}
