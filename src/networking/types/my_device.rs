use std::sync::{Arc, Mutex};

use pcap::{Address, Device, DeviceFlags};

use crate::networking::types::my_link_type::MyLinkType;

/// Represents the current inspected device.
/// Used to keep in sync the device addresses in case of changes
/// (e.g., device not connected to the internet acquires new IP address)
#[derive(Clone)]
pub struct MyDevice {
    pub name: String,
    #[cfg(target_os = "windows")]
    pub desc: Option<String>,
    pub addresses: Arc<Mutex<Vec<Address>>>,
    pub link_type: MyLinkType,
}

impl MyDevice {
    pub fn to_pcap_device(&self) -> Device {
        for device in Device::list().unwrap_or_default() {
            if device.name.eq(&self.name) {
                return device;
            }
        }
        Device::lookup().unwrap_or(None).unwrap_or_else(|| Device {
            name: String::new(),
            desc: None,
            addresses: vec![],
            flags: DeviceFlags::empty(),
        })
    }
}
