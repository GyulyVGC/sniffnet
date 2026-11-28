use crate::gui::types::filters::Filters;
use crate::location;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource, CaptureType};
use crate::networking::types::my_device::MyDevice;
use crate::utils::error_logger::{ErrorLogger, Location};
use async_channel::Sender;
use pcap::Device;
use sniffnet_packet_parser::LinkType;
use sniffnet_packet_parser::ParsedPacket;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Default, Clone, Debug)]
pub struct TrafficPreview {
    pub data: Vec<(MyDevice, u128)>,
}

enum PreviewMsg {
    Packet(PacketOwned),
    Heartbeat,
    Closed { dev_name: String, stream_id: u64 },
}

#[derive(Debug, PartialEq, Eq)]
enum StreamStatus {
    Active(u64),
    Unsupported,
}

#[derive(Debug, PartialEq, Eq)]
struct PreviewDeviceState {
    status: StreamStatus,
    packets: u128,
}

pub fn traffic_preview(tx: &Sender<TrafficPreview>) {
    let mut ticks = Instant::now();
    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);

    let mut data = HashMap::new();
    let mut next_stream_id = 0_u64;
    handle_devices_and_previews(&mut data, &mut next_stream_id, tx, &pcap_tx);

    loop {
        let msg = pcap_rx
            .recv_timeout(Duration::from_millis(150))
            .unwrap_or(PreviewMsg::Heartbeat);

        if tx.is_closed() {
            return;
        }

        if ticks.elapsed() >= Duration::from_secs(1) {
            ticks = ticks
                .checked_add(Duration::from_secs(1))
                .unwrap_or(Instant::now());
            handle_devices_and_previews(&mut data, &mut next_stream_id, tx, &pcap_tx);
        }

        match msg {
            PreviewMsg::Packet(packet) => {
                let dev_info = packet.dev_info;
                if ParsedPacket::from_bytes(&packet.data, dev_info.link_type).is_some()
                    && let Some(entry) = data.get_mut(&dev_info.name)
                {
                    entry.packets += 1;
                }
            }
            PreviewMsg::Closed {
                dev_name,
                stream_id,
            } => {
                handle_stream_closed(&mut data, &dev_name, stream_id);
            }
            PreviewMsg::Heartbeat => {}
        }
    }
}

fn handle_stream_closed(
    data: &mut HashMap<String, PreviewDeviceState>,
    dev_name: &str,
    stream_id: u64,
) {
    if data
        .get(dev_name)
        .is_some_and(|state| state.status == StreamStatus::Active(stream_id))
    {
        data.remove(dev_name);
    }
}

fn retain_present_devices(data: &mut HashMap<String, PreviewDeviceState>, devices: &[Device]) {
    data.retain(|name, _| devices.iter().any(|dev| dev.name == *name));
}

fn handle_devices_and_previews(
    data: &mut HashMap<String, PreviewDeviceState>,
    next_stream_id: &mut u64,
    tx: &Sender<TrafficPreview>,
    pcap_tx: &std::sync::mpsc::SyncSender<PreviewMsg>,
) {
    let devices: Vec<Device> = Device::list()
        .unwrap_or_default()
        .into_iter()
        .filter(is_device_allowed)
        .collect();
    retain_present_devices(data, &devices);

    let mut traffic_preview = TrafficPreview::default();
    for dev in devices {
        let dev_name = dev.name.clone();
        let my_dev = MyDevice::from_pcap_device(dev);
        if let Some(state) = data.get(&dev_name) {
            traffic_preview.data.push((my_dev, state.packets));
            continue;
        }
        traffic_preview.data.push((my_dev.clone(), 0));
        let capture_source = CaptureSource::Device(my_dev);
        let capture_context = CaptureContext::new(&capture_source, None, &Filters::default());
        let Some(link_type) = capture_context.link_type() else {
            continue;
        };
        if !link_type.is_supported() {
            data.insert(
                dev_name,
                PreviewDeviceState {
                    status: StreamStatus::Unsupported,
                    packets: 0,
                },
            );
            continue;
        }
        let (Some(cap), _) = capture_context.consume() else {
            continue;
        };
        let stream_id = *next_stream_id;
        *next_stream_id += 1;
        let pcap_tx = pcap_tx.clone();
        let thread_name = format!("thread_traffic_preview_{dev_name}");
        let dev_info = DevInfo {
            name: dev_name.clone(),
            link_type,
        };
        if thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                packet_stream(cap, &pcap_tx, &dev_info, stream_id);
            })
            .log_err(location!())
            .is_ok()
        {
            data.insert(
                dev_name,
                PreviewDeviceState {
                    status: StreamStatus::Active(stream_id),
                    packets: 0,
                },
            );
        }
    }
    let _ = tx.send_blocking(traffic_preview);
    for state in data.values_mut() {
        state.packets = 0;
    }
}

trait PreviewPacketSource {
    fn next_packet_data(&mut self) -> Result<Box<[u8]>, pcap::Error>;
}

impl PreviewPacketSource for CaptureType {
    fn next_packet_data(&mut self) -> Result<Box<[u8]>, pcap::Error> {
        self.next_packet().map(|p| p.data.into())
    }
}

fn packet_stream(
    mut cap: impl PreviewPacketSource,
    tx: &std::sync::mpsc::SyncSender<PreviewMsg>,
    dev_info: &DevInfo,
    stream_id: u64,
) {
    loop {
        match cap.next_packet_data() {
            Ok(data) => {
                let packet_owned = PacketOwned {
                    data,
                    dev_info: dev_info.clone(),
                };
                if tx.send(PreviewMsg::Packet(packet_owned)).is_err() {
                    return;
                }
            }
            Err(pcap::Error::TimeoutExpired) => {
                if tx.send(PreviewMsg::Heartbeat).is_err() {
                    return;
                }
            }
            Err(_) => {
                // Non-timeout error (e.g., interface disappeared / went down):
                // notify preview loop so a fresh capture can be instantiated if it comes back up
                let _ = tx.send(PreviewMsg::Closed {
                    dev_name: dev_info.name.clone(),
                    stream_id,
                });
                return;
            }
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

// avoid opening captures on Linux D-Bus devices, causing system operations to hang (issue #1190)
#[allow(clippy::used_underscore_binding)]
fn is_device_allowed(_device: &Device) -> bool {
    #[cfg(not(target_os = "linux"))]
    return true;

    #[cfg(target_os = "linux")]
    return !matches!(_device.name.as_str(), "dbus-system" | "dbus-session")
        && !_device.name.starts_with("dbus://");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcap::DeviceFlags;
    use std::collections::VecDeque;

    struct MockPreviewCapture {
        responses: VecDeque<Result<Box<[u8]>, pcap::Error>>,
        calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }

    impl PreviewPacketSource for MockPreviewCapture {
        fn next_packet_data(&mut self) -> Result<Box<[u8]>, pcap::Error> {
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.responses
                .pop_front()
                .unwrap_or(Err(pcap::Error::PcapError(
                    "The interface disappeared".to_string(),
                )))
        }
    }

    #[test]
    fn test_preview_packet_stream_exits_on_interface_disappeared() {
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        let dev_info = DevInfo {
            name: "en0".to_string(),
            link_type: LinkType::Ethernet(pcap::Linktype::ETHERNET),
        };
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let cap = MockPreviewCapture {
            responses: VecDeque::from([
                Ok(vec![1, 2, 3].into_boxed_slice()),
                Err(pcap::Error::TimeoutExpired),
                Err(pcap::Error::PcapError(
                    "The interface disappeared".to_string(),
                )),
                // Must never be reached: stream must exit on the error above instead of spinning
                Ok(vec![4, 5, 6].into_boxed_slice()),
            ]),
            calls: calls.clone(),
        };

        packet_stream(cap, &tx, &dev_info, 7);

        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 3);
        assert!(matches!(rx.try_recv(), Ok(PreviewMsg::Packet(_))));
        assert!(matches!(rx.try_recv(), Ok(PreviewMsg::Heartbeat)));
        assert!(matches!(
            rx.try_recv(),
            Ok(PreviewMsg::Closed {
                dev_name,
                stream_id: 7
            }) if dev_name == "en0"
        ));
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_handle_stream_closed_respects_stream_id() {
        let mut data = HashMap::from([
            (
                "en0".to_string(),
                PreviewDeviceState {
                    status: StreamStatus::Active(2),
                    packets: 5,
                },
            ),
            (
                "lo0".to_string(),
                PreviewDeviceState {
                    status: StreamStatus::Unsupported,
                    packets: 0,
                },
            ),
        ]);

        // Stale stream_id must not remove the newer active stream
        handle_stream_closed(&mut data, "en0", 1);
        assert!(data.contains_key("en0"));

        // Matching stream_id removes the device so the next tick can reopen it
        handle_stream_closed(&mut data, "en0", 2);
        assert!(!data.contains_key("en0"));
        assert!(data.contains_key("lo0"));
    }

    #[test]
    fn test_retain_present_devices_prunes_disappeared_devices() {
        let mut data = HashMap::from([
            (
                "en0".to_string(),
                PreviewDeviceState {
                    status: StreamStatus::Active(1),
                    packets: 3,
                },
            ),
            (
                "utun4".to_string(),
                PreviewDeviceState {
                    status: StreamStatus::Active(2),
                    packets: 1,
                },
            ),
        ]);

        let devices = vec![Device {
            name: "en0".to_string(),
            desc: None,
            addresses: vec![],
            flags: DeviceFlags::empty(),
        }];

        retain_present_devices(&mut data, &devices);
        assert!(data.contains_key("en0"));
        assert!(!data.contains_key("utun4"));
    }
}
