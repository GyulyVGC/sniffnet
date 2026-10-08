use crate::gui::types::filters::Filters;
use crate::location;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource, CaptureType};
use crate::networking::types::my_device::MyDevice;
use crate::utils::error_logger::{ErrorLogger, Location};
use async_channel::Sender;
use pcap::{Device, Stat};
use sniffnet_packet_parser::LinkType;
use sniffnet_packet_parser::ParsedPacket;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Default, Clone, Debug)]
pub struct TrafficPreview {
    pub data: Vec<(MyDevice, u128)>,
}

pub fn traffic_preview(tx: &Sender<TrafficPreview>) {
    let mut ticks = Instant::now();
    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);

    let mut data = HashMap::new();
    handle_devices_and_previews(&mut data, tx, &pcap_tx);

    loop {
        let (packet_res, _) = pcap_rx
            .recv_timeout(Duration::from_millis(150))
            .unwrap_or((Err(pcap::Error::TimeoutExpired), None));

        if tx.is_closed() {
            return;
        }

        if ticks.elapsed() >= Duration::from_secs(1) {
            ticks = ticks
                .checked_add(Duration::from_secs(1))
                .unwrap_or(Instant::now());
            handle_devices_and_previews(&mut data, tx, &pcap_tx);
        }

        if let Ok(packet) = packet_res {
            let dev_info = packet.dev_info;
            if ParsedPacket::from_bytes(&packet.data, dev_info.link_type).is_some() {
                data.entry(dev_info.name)
                    .and_modify(|p| *p += 1)
                    .or_insert(1);
            }
        }
    }
}

fn handle_devices_and_previews(
    data: &mut HashMap<String, u128>,
    tx: &Sender<TrafficPreview>,
    pcap_tx: &std::sync::mpsc::SyncSender<(Result<PacketOwned, pcap::Error>, Option<Stat>)>,
) {
    let mut traffic_preview = TrafficPreview::default();
    for dev in Device::list()
        .unwrap_or_default()
        .into_iter()
        .filter(is_preview_device)
    {
        let dev_name = dev.name.clone();
        let my_dev = MyDevice::from_pcap_device(dev);
        if let Some(n) = data.get(&dev_name) {
            traffic_preview.data.push((my_dev, *n));
            continue;
        }
        data.insert(dev_name.clone(), 0);
        traffic_preview.data.push((my_dev.clone(), 0));
        let capture_source = CaptureSource::Device(my_dev);
        let capture_context = CaptureContext::new(&capture_source, None, &Filters::default());
        let Some(link_type) = capture_context.link_type() else {
            continue;
        };
        if !link_type.is_supported() {
            continue;
        }
        let pcap_tx = pcap_tx.clone();
        let thread_name = format!("thread_traffic_preview_{dev_name}");
        let dev_info = DevInfo {
            name: dev_name,
            link_type,
        };
        let (Some(cap), _) = capture_context.consume() else {
            continue;
        };
        let _ = thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                packet_stream(cap, &pcap_tx, &dev_info);
            })
            .log_err(location!());
    }
    let _ = tx.send_blocking(traffic_preview);
    for v in data.values_mut() {
        *v = 0;
    }
}

fn packet_stream(
    mut cap: CaptureType,
    tx: &std::sync::mpsc::SyncSender<(Result<PacketOwned, pcap::Error>, Option<pcap::Stat>)>,
    dev_info: &DevInfo,
) {
    loop {
        let packet_res = cap.next_packet();
        let packet_owned = packet_res.map(|p| PacketOwned {
            data: p.data.into(),
            dev_info: dev_info.clone(),
        });
        if tx.send((packet_owned, cap.stats().ok())).is_err() {
            return;
        }
    }
}

#[derive(Clone)]
struct DevInfo {
    name: String,
    link_type: LinkType,
}

struct PacketOwned {
    data: Box<[u8]>,
    dev_info: DevInfo,
}

// Opening a D-Bus capture installs bus-wide monitoring rules that can outlive
// the capture handle and retain file descriptors used by desktop applications.
// Sniffnet cannot parse D-Bus messages, so exclude these devices before opening them.
fn is_preview_device(device: &Device) -> bool {
    !matches!(device.name.as_str(), "dbus-system" | "dbus-session")
        && !device.name.starts_with("dbus://")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcap::DeviceFlags;

    #[test]
    fn test_preview_devices_exclude_dbus_without_excluding_network_interfaces() {
        let devices = [
            "eth0",
            "lo",
            "any",
            "dbus-system",
            "dbus-session",
            "dbus://unix:path=/tmp/test-bus",
            "dbus-session0",
        ]
        .map(|name| Device {
            name: name.to_string(),
            desc: None,
            addresses: vec![],
            flags: DeviceFlags::empty(),
        });

        let names: Vec<_> = devices
            .into_iter()
            .filter(is_preview_device)
            .map(|device| device.name)
            .collect();

        assert_eq!(names, ["eth0", "lo", "any", "dbus-session0"]);
    }
}
