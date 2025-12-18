use crate::gui::types::filters::Filters;
use crate::location;
use crate::networking::parse_packets::{PacketOwned, get_sniffable_headers, packet_stream};
use crate::networking::types::capture_context::{CaptureContext, CaptureSource};
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::my_link_type::MyLinkType;
use crate::utils::error_logger::{ErrorLogger, Location};
use async_channel::Sender;
use pcap::{Device, Stat};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::Receiver;

#[derive(Default, Clone, Debug)]
pub struct TrafficPreview {
    pub data: Vec<(MyDevice, u128)>,
}

pub fn traffic_preview(tx: &Sender<TrafficPreview>, freeze_rxs: (Receiver<()>, Receiver<()>)) {
    let mut ticks = Instant::now();
    let (freeze_tx, mut freeze_rx) = tokio::sync::broadcast::channel(1_048_575);
    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);

    let mut data = HashMap::new();
    handle_devices_and_previews(&mut data, tx, freeze_tx.clone(), pcap_tx.clone());

    loop {
        // check if we need to freeze the parsing
        if freeze_rx.try_recv().is_ok() {
            // wait until unfreeze
            let _ = freeze_rx.blocking_recv();
            // reset the first packet ticks
            // first_packet_ticks = Instant::now();
        }

        let (packet_res, _) = pcap_rx
            .recv_timeout(Duration::from_millis(150))
            .unwrap_or((Err(pcap::Error::TimeoutExpired), None));

        if ticks.elapsed() >= Duration::from_millis(1000) {
            ticks = ticks
                .checked_add(Duration::from_millis(1000))
                .unwrap_or(Instant::now());
            handle_devices_and_previews(&mut data, tx, freeze_tx.clone(), pcap_tx.clone());
        }

        if let Ok(packet) = packet_res {
            let my_link_type = packet.dev_info.as_ref().unwrap().my_link_type;
            if get_sniffable_headers(&packet.data, my_link_type).is_some() {
                let Some(dev_info) = packet.dev_info else {
                    continue;
                };

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
    freeze_tx: tokio::sync::broadcast::Sender<()>,
    pcap_tx: std::sync::mpsc::SyncSender<(Result<PacketOwned, pcap::Error>, Option<Stat>)>,
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
        let mut freeze_rx = freeze_tx.subscribe();
        let pcap_tx = pcap_tx.clone();
        let capture_source = CaptureSource::Device(my_dev);
        let capture_context = CaptureContext::new(&capture_source, None, &Filters::default());
        let my_link_type = capture_context.my_link_type();
        let dev_info = Some(DevInfo {
            name: dev_name,
            my_link_type,
        });
        let (cap, _) = capture_context.consume();
        let _ = thread::Builder::new()
            .name("thread_device_traffic_preview".to_string())
            .spawn(move || {
                packet_stream(
                    cap,
                    &pcap_tx,
                    &mut freeze_rx,
                    &Filters::default(),
                    dev_info.as_ref(),
                );
            })
            .log_err(location!());
    }
    let _ = tx.send_blocking(traffic_preview);
    data.iter_mut().for_each(|(_, v)| *v = 0);
}

#[derive(Clone)]
pub(super) struct DevInfo {
    name: String,
    my_link_type: MyLinkType,
}
