use crate::gui::types::filters::Filters;
use crate::location;
use crate::networking::parse_packets::{get_sniffable_headers, packet_stream};
use crate::networking::types::capture_context::{CaptureContext, CaptureSource};
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::my_link_type::MyLinkType;
use crate::utils::error_logger::{ErrorLogger, Location};
use pcap::Device;
use std::thread;
use std::time::{Duration, Instant};

pub fn traffic_preview() {
    let (freeze_tx, mut freeze_rx) = tokio::sync::broadcast::channel(1_048_575);
    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);
    for dev in Device::list().unwrap_or_default() {
        let mut freeze_rx = freeze_tx.subscribe();
        let pcap_tx = pcap_tx.clone();
        let dev_name = dev.name.clone();
        let my_dev = MyDevice::from_pcap_device(dev);
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
                packet_stream(cap, &pcap_tx, &mut freeze_rx, &Filters::default(), dev_info);
            })
            .log_err(location!());
    }

    loop {
        // check if we need to freeze the parsing
        if freeze_rx.try_recv().is_ok() {
            // wait until unfreeze
            let _ = freeze_rx.blocking_recv();
            // reset the first packet ticks
            // first_packet_ticks = Some(Instant::now());
        }

        let (packet_res, cap_stats) = pcap_rx
            .recv_timeout(Duration::from_millis(150))
            .unwrap_or((Err(pcap::Error::TimeoutExpired), None));

        // if tx.is_closed() {
        //     return;
        // }

        if let Ok(packet) = packet_res {
            let my_link_type = packet.dev_info.as_ref().unwrap().my_link_type;
            if get_sniffable_headers(&packet.data, my_link_type).is_some() {
                println!(
                    "Received packet on {}",
                    packet.dev_info.as_ref().map_or("?", |d| d.name.as_str()),
                );
            }
        }
    }
}

#[derive(Clone)]
pub(super) struct DevInfo {
    name: String,
    my_link_type: MyLinkType,
}
