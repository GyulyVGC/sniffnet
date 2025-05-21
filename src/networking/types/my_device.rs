use pcap::{Address, Device, DeviceFlags};

use crate::networking::types::my_link_type::MyLinkType;

/// Represents the current inspected device.
/// Used to keep in sync the device addresses in case of changes
/// (e.g., device not connected to the internet acquires new IP address)
#[derive(Clone)]
pub struct MyDevice {
    name: String,
    desc: Option<String>,
    addresses: Vec<Address>,
    link_type: MyLinkType,
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

    pub fn from_pcap_device(device: Device) -> Self {
        MyDevice {
            name: device.name,
            desc: device.desc,
            addresses: device.addresses,
            link_type: MyLinkType::default(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_desc(&self) -> Option<&String> {
        self.desc.as_ref()
    }

    pub fn get_addresses(&self) -> &Vec<Address> {
        &self.addresses
    }

    pub fn set_addresses(&mut self, addresses: Vec<Address>) {
        self.addresses = addresses;
    }

    pub fn get_link_type(&self) -> MyLinkType {
        self.link_type
    }

    pub fn set_link_type(&mut self, link_type: MyLinkType) {
        self.link_type = link_type;
    }
}
