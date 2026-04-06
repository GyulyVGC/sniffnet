use crate::gui::types::filters::Filters;
use crate::location;
use crate::networking::manage_packets::analyze_headers;
use crate::networking::parse_packets::get_sniffable_headers;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource, CaptureType};
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::my_link_type::MyLinkType;
use crate::utils::error_logger::{ErrorLogger, Location};
use async_channel::Sender;
use pcap::{Device, Stat};
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

        if ticks.elapsed() >= Duration::from_millis(1000) {
            ticks = ticks
                .checked_add(Duration::from_millis(1000))
                .unwrap_or(Instant::now());
            handle_devices_and_previews(&mut data, tx, &pcap_tx);
        }

        if let Ok(packet) = packet_res {
            let dev_info = packet.dev_info;
            let my_link_type = dev_info.my_link_type;
            if let Some(headers) = get_sniffable_headers(&packet.data, my_link_type)
                && analyze_headers(
                    headers,
                    &mut (None, None),
                    &mut 0,
                    &mut IcmpType::default(),
                    &mut ArpType::default(),
                )
                .is_some()
            {
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
    for dev in Device::list().unwrap_or_default() {
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
        let my_link_type = capture_context.my_link_type();
        if !my_link_type.is_supported() {
            continue;
        }
        let pcap_tx = pcap_tx.clone();
        let thread_name = format!("thread_traffic_preview_{dev_name}");
        let dev_info = DevInfo {
            name: dev_name,
            my_link_type,
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
    my_link_type: MyLinkType,
}

struct PacketOwned {
    data: Box<[u8]>,
    dev_info: DevInfo,
}
