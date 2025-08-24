use crate::networking::types::my_device::MyDevice;
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
