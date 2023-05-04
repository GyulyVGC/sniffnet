use pcap::Address;
use std::sync::{Arc, Mutex};

/// Represents the current inspected device.
/// Used to keep in sync the device addresses in case of changes
/// (e.g., device not connected to the internet acquires new IP address)
#[derive(Clone)]
pub struct MyDevice {
    pub name: String,
    pub desc: Option<String>,
    pub addresses: Arc<Mutex<Vec<Address>>>,
}
