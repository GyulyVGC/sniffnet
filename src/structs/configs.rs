//! Module defining the `Config` struct, which allows to save and reload
//! the application default configuration.

use crate::enums::language::Language;
use crate::structs::notifications::Notifications;
use crate::StyleType;
use pcap::Device;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigSettings {
    pub style: StyleType,
    pub language: Language,
    pub notifications: Notifications,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigDevice {
    pub device_name: String,
}

impl Default for ConfigDevice {
    fn default() -> Self {
        Self {
            device_name: Device::lookup().unwrap().unwrap().name,
        }
    }
}

impl ConfigDevice {
    pub fn to_pcap_device(&self) -> Device {
        for device in Device::list().unwrap() {
            if device.name.eq(&self.device_name) {
                return device;
            }
        }
        Device::lookup().unwrap().unwrap()
    }
}
