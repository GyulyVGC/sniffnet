//! `pcap`-based capture backend

use crate::gui::types::filters::Filters;
use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::capture::{
    AddressesResolutionState, BackendTrafficMessage, maybe_send_tick, spawn_reverse_dns_pool,
};
use crate::networking::manage_packets::{modify_or_insert_in_map, update_connection_stats};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource, CaptureType};
use crate::networking::types::dropped_packets::DroppedPackets;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::networking::types::message_type::MessageType;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::timestamp::Timestamp;
use async_channel::Sender;
use pcap::{Packet, PacketHeader};
use sniffnet_packet_parser::ParsedPacket;
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::Receiver;

/// Wait for reverse-DNS results while allowing a cancelled capture to stop promptly.
fn wait_for_pending_hosts(
    tx: &Sender<BackendTrafficMessage>,
    resolutions_state: &mut AddressesResolutionState,
) -> Option<Vec<crate::networking::types::host::HostMessage>> {
    let mut pending_hosts = Vec::new();
    while !resolutions_state.addresses_waiting_resolution.is_empty() {
        if tx.is_closed() {
            return None;
        }
        pending_hosts.extend(resolutions_state.new_hosts_to_send());
        thread::sleep(Duration::from_secs(1));
    }
    Some(pending_hosts)
}

/// The calling thread enters a loop in which it waits for network packets
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub fn parse_packets(
    cap_id: usize,
    mut cs: CaptureSource,
    mmdb_readers: &MmdbReaders,
    ip_blacklist: &IpBlacklist,
    capture_context: CaptureContext,
    filters: Filters,
    tx: &Sender<BackendTrafficMessage>,
    freeze_rxs: (Receiver<()>, Receiver<()>),
) {
    let (mut freeze_rx, mut freeze_rx_2) = freeze_rxs;

    let Some(link_type) = capture_context.link_type() else {
        return;
    };
    if !link_type.is_supported() {
        return;
    }

    let (Some(cap), mut savefile) = capture_context.consume() else {
        return;
    };

    let mut info_traffic_msg = InfoTraffic::default();

    let mut resolutions_state = spawn_reverse_dns_pool(mmdb_readers);

    // instant of the first parsed packet plus multiples of 1 second (only used in live captures)
    let mut first_packet_ticks = None;

    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);
    let _ = thread::Builder::new()
        .name("thread_packet_stream".to_string())
        .spawn(move || packet_stream(cap, &pcap_tx, &mut freeze_rx_2, &filters))
        .log_err(location!());

    loop {
        // check if we need to freeze the parsing
        if freeze_rx.try_recv().is_ok() {
            // wait until unfreeze
            let _ = freeze_rx.blocking_recv();
            // reset the first packet ticks
            first_packet_ticks = Some(Instant::now());
        }

        let (packet_res, cap_stats) = pcap_rx
            .recv_timeout(Duration::from_millis(150))
            .unwrap_or((Err(pcap::Error::TimeoutExpired), None));

        if tx.is_closed() {
            return;
        }

        if matches!(cs, CaptureSource::Device(_))
            && maybe_send_tick(
                cap_id,
                &mut info_traffic_msg,
                &mut first_packet_ticks,
                tx,
                &mut resolutions_state,
            )
        {
            // refresh adapter addresses every second
            cs.set_addresses();
        }

        match packet_res {
            Err(e) => {
                if e == pcap::Error::NoMorePackets {
                    // send a message including data from the last interval (only happens in offline captures)
                    let _ = tx.send_blocking(BackendTrafficMessage::TickRun(
                        cap_id,
                        info_traffic_msg,
                        resolutions_state.new_hosts_to_send(),
                        true,
                    ));
                    // Wait for pending reverse-DNS resolutions unless the capture is cancelled.
                    let Some(pending_hosts) = wait_for_pending_hosts(tx, &mut resolutions_state)
                    else {
                        return;
                    };
                    // send one last message including all pending hosts
                    let _ = tx
                        .send_blocking(BackendTrafficMessage::PendingHosts(cap_id, pending_hosts));
                    return;
                }
            }
            Ok(packet) => {
                if let Some(parsed) = ParsedPacket::from_bytes(&packet.data, link_type) {
                    #[allow(clippy::useless_conversion)]
                    let secs = i64::from(packet.header.ts.tv_sec);
                    #[allow(clippy::useless_conversion)]
                    let usecs = i64::from(packet.header.ts.tv_usec);
                    let next_packet_timestamp = Timestamp::new(secs, usecs);

                    if matches!(cs, CaptureSource::File(_)) {
                        maybe_send_tick_import_pcap(
                            cap_id,
                            &mut info_traffic_msg,
                            next_packet_timestamp,
                            tx,
                            &mut resolutions_state,
                        );
                    } else if first_packet_ticks.is_none() {
                        first_packet_ticks = Some(Instant::now());
                    }

                    info_traffic_msg.last_packet_timestamp = next_packet_timestamp;

                    let bytes = parsed.bytes_count() as u128;
                    let mac_addresses = (parsed.link_info.src_mac, parsed.link_info.dst_mac);
                    let message_type = MessageType::from_parsed_packet(&parsed);
                    let vlan_id = parsed.link_info.vlan_id;

                    let key = AddressPortPair::from_parsed_packet(&parsed);

                    // save this packet to PCAP file
                    if let Some(file) = savefile.as_mut() {
                        file.write(&Packet {
                            header: &packet.header,
                            data: &packet.data,
                        });
                    }

                    // update the map
                    let (traffic_direction, service) = modify_or_insert_in_map(
                        &mut info_traffic_msg,
                        &key,
                        cs.get_addresses(),
                        mac_addresses,
                        message_type,
                        vlan_id,
                        1,
                        bytes,
                        ip_blacklist,
                        None,
                        None,
                    );

                    update_connection_stats(
                        &mut info_traffic_msg,
                        &mut resolutions_state,
                        &key,
                        cs.get_addresses(),
                        1,
                        bytes,
                        traffic_direction,
                        service,
                    );

                    // update dropped packets number
                    if let Some(stats) = cap_stats {
                        info_traffic_msg.dropped_packets =
                            Some(DroppedPackets::from_pcap_stats(&stats));
                    }
                }
            }
        }
    }
}

/// Used only by PCAP import
fn maybe_send_tick_import_pcap(
    cap_id: usize,
    info_traffic_msg: &mut InfoTraffic,
    next_packet_timestamp: Timestamp,
    tx: &Sender<BackendTrafficMessage>,
    resolutions_state: &mut AddressesResolutionState,
) {
    if info_traffic_msg.last_packet_timestamp == Timestamp::default() {
        info_traffic_msg.last_packet_timestamp = next_packet_timestamp;
    }
    if info_traffic_msg.last_packet_timestamp.secs() < next_packet_timestamp.secs() {
        let diff_secs =
            next_packet_timestamp.secs() - info_traffic_msg.last_packet_timestamp.secs();
        let _ = tx.send_blocking(BackendTrafficMessage::TickRun(
            cap_id,
            info_traffic_msg.take_but_leave_something(),
            resolutions_state.new_hosts_to_send(),
            false,
        ));
        if diff_secs > 1 {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let _ = tx.send_blocking(BackendTrafficMessage::OfflineGap(
                cap_id,
                diff_secs as u32 - 1,
            ));
        }
    }
}

fn packet_stream(
    mut cap: CaptureType,
    tx: &std::sync::mpsc::SyncSender<(Result<PacketOwned, pcap::Error>, Option<pcap::Stat>)>,
    freeze_rx: &mut Receiver<()>,
    filters: &Filters,
) {
    loop {
        // check if we need to freeze the parsing
        if freeze_rx.try_recv().is_ok() {
            // pause the capture
            cap.pause();
            // wait until unfreeze
            let _ = freeze_rx.blocking_recv();
            // resume the capture
            cap.resume(filters);
        }

        let packet_res = cap.next_packet();
        let packet_owned = packet_res.map(|p| PacketOwned {
            header: *p.header,
            data: p.data.into(),
        });
        if tx.send((packet_owned, cap.stats().ok())).is_err() {
            return;
        }
    }
}

struct PacketOwned {
    header: PacketHeader,
    data: Box<[u8]>,
}

#[cfg(test)]
mod tests {
    use super::wait_for_pending_hosts;
    use crate::networking::capture::AddressesResolutionState;
    use crate::networking::types::data_info::DataInfo;
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Duration;

    #[test]
    fn pending_host_wait_returns_when_capture_channel_closes_during_wait() {
        let (tx, rx) = async_channel::bounded(1);
        let mut state = AddressesResolutionState::new_for_tests();
        state
            .addresses_waiting_resolution
            .insert(IpAddr::V4(Ipv4Addr::LOCALHOST), DataInfo::default());
        let waiter = std::thread::spawn(move || wait_for_pending_hosts(&tx, &mut state).is_none());

        // Let the worker enter its wait before simulating capture cancellation.
        std::thread::sleep(Duration::from_millis(50));
        drop(rx);

        assert!(waiter.join().unwrap());
    }

    #[test]
    fn closing_full_capture_channel_unblocks_blocked_backend_send() {
        let (tx, rx) = async_channel::bounded(1);
        tx.send_blocking(crate::networking::capture::BackendTrafficMessage::OfflineGap(0, 1))
            .unwrap();

        let blocked_sender = std::thread::spawn(move || {
            tx.send_blocking(crate::networking::capture::BackendTrafficMessage::OfflineGap(0, 2))
                .is_err()
        });

        // Closing the receiver must release a producer waiting for queue capacity.
        rx.close();
        assert!(blocked_sender.join().unwrap());
    }

    #[test]
    fn pending_host_wait_completes_when_no_resolutions_remain() {
        let (tx, _rx) = async_channel::bounded(1);
        let mut state = AddressesResolutionState::new_for_tests();

        let pending_hosts = wait_for_pending_hosts(&tx, &mut state);

        assert!(pending_hosts.is_some());
        assert!(pending_hosts.unwrap().is_empty());
    }

    #[test]
    fn synthetic_pcap_large_import_preserves_updates_with_slow_consumer() {
        const TIMESTAMP_COUNT: u32 = 2_048;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "sniffnet-offline-large-stress-{}-{nonce}.pcap",
            std::process::id()
        ));
        let timestamps: Vec<u32> = (1..=TIMESTAMP_COUNT).collect();
        write_synthetic_pcap(&path, &timestamps, 1);

        let source = CaptureSource::File(MyPcapImport::new(path.to_string_lossy().into_owned()));
        let context = CaptureContext::new(&source, None, &Filters::default());
        assert!(context.error().is_none(), "large stress PCAP must open");
        let readers = MmdbReaders {
            country: Arc::new(MmdbReader::Empty),
            asn: Arc::new(MmdbReader::Empty),
        };
        let (tx, rx) = capture_update_channel(true);
        let (freeze_tx_1, freeze_rx_1) = tokio::sync::broadcast::channel(1);
        let (freeze_tx_2, freeze_rx_2) = tokio::sync::broadcast::channel(1);
        let worker = std::thread::spawn(move || {
            parse_packets(
                10,
                source,
                &readers,
                &IpBlacklist::default(),
                context,
                Filters::default(),
                &tx,
                (freeze_rx_1, freeze_rx_2),
            );
        });

        let started = Instant::now();
        let deadline = Instant::now() + Duration::from_secs(20);
        let mut tick_count = 0_u32;
        let mut final_timestamp = None;
        let mut saw_final_tick = false;
        let mut saw_pending_hosts = false;
        loop {
            match rx.try_recv() {
                Ok(BackendTrafficMessage::TickRun(10, info, _, is_final)) => {
                    tick_count += 1;
                    if is_final {
                        final_timestamp = Some(info.last_packet_timestamp.secs());
                        saw_final_tick = true;
                    }
                }
                Ok(BackendTrafficMessage::PendingHosts(10, _)) => {
                    saw_pending_hosts = true;
                    break;
                }
                Ok(_) => {}
                Err(async_channel::TryRecvError::Empty) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(error) => panic!("timed out waiting for large stress PCAP import: {error}"),
            }
            // Force repeated backpressure while retaining every UI update.
            std::thread::sleep(Duration::from_millis(1));
        }
        worker.join().unwrap();
        let elapsed = started.elapsed();
        fs::remove_file(path).unwrap();

        assert_eq!(tick_count, TIMESTAMP_COUNT);
        assert_eq!(final_timestamp, Some(TIMESTAMP_COUNT));
        assert!(saw_final_tick, "the final traffic snapshot must be emitted");
        assert!(
            saw_pending_hosts,
            "the large import must finish host delivery"
        );
        assert!(
            elapsed < Duration::from_secs(20),
            "large stress import took {elapsed:?}"
        );
        eprintln!(
            "synthetic PCAP large stress import: {TIMESTAMP_COUNT} timestamp updates, slow consumer, {elapsed:?}"
        );
        let _ = (freeze_tx_1, freeze_tx_2);
    }
}

#[cfg(test)]
mod offline_import_tests {
    use super::parse_packets;
    use crate::gui::types::filters::Filters;
    use crate::mmdb::types::mmdb_reader::{MmdbReader, MmdbReaders};
    use crate::networking::capture::{BackendTrafficMessage, capture_update_channel};
    use crate::networking::types::capture_context::{CaptureContext, CaptureSource, MyPcapImport};
    use crate::networking::types::ip_blacklist::IpBlacklist;
    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::Arc;
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    fn write_u16_le(out: &mut Vec<u8>, value: u16) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u32_le(out: &mut Vec<u8>, value: u32) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn synthetic_udp_frame(source_port: u16, destination_port: u16) -> Vec<u8> {
        let mut frame = Vec::with_capacity(42);
        // Ethernet II header: fixed locally administered MAC addresses, IPv4 payload.
        frame.extend_from_slice(&[0x02, 0, 0, 0, 0, 2, 0x02, 0, 0, 0, 0, 1]);
        frame.extend_from_slice(&[0x08, 0x00]);
        // Minimal IPv4 header (checksum is ignored by the packet parser).
        frame.extend_from_slice(&[
            0x45, 0, 0, 28, 0, 1, 0, 0, 64, 17, 0, 0, 127, 0, 0, 1, 127, 0, 0, 1,
        ]);
        // UDP header with no payload.
        frame.extend_from_slice(&source_port.to_be_bytes());
        frame.extend_from_slice(&destination_port.to_be_bytes());
        frame.extend_from_slice(&[0, 8, 0, 0]);
        frame
    }

    fn write_synthetic_pcap(
        path: &std::path::Path,
        timestamps: &[u32],
        packets_per_timestamp: u16,
    ) {
        let mut bytes = Vec::new();
        // Classic PCAP, little-endian, microsecond timestamps, Ethernet link type.
        bytes.extend_from_slice(&[0xd4, 0xc3, 0xb2, 0xa1]);
        write_u16_le(&mut bytes, 2);
        write_u16_le(&mut bytes, 4);
        write_u32_le(&mut bytes, 0);
        write_u32_le(&mut bytes, 0);
        write_u32_le(&mut bytes, 65_535);
        write_u32_le(&mut bytes, 1);

        let mut packet_count = 0_u16;
        for &seconds in timestamps {
            for _ in 0..packets_per_timestamp {
                let frame = synthetic_udp_frame(20_000 + packet_count, 30_000 + packet_count);
                write_u32_le(&mut bytes, seconds);
                write_u32_le(&mut bytes, u32::from(packet_count) * 100);
                let frame_len = u32::try_from(frame.len()).unwrap();
                write_u32_le(&mut bytes, frame_len);
                write_u32_le(&mut bytes, frame_len);
                bytes.extend_from_slice(&frame);
                packet_count += 1;
            }
        }

        File::create(path).unwrap().write_all(&bytes).unwrap();
    }

    #[test]
    fn synthetic_pcap_import_handles_time_gaps_with_a_slow_ui_consumer() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "sniffnet-offline-import-{}-{nonce}.pcap",
            std::process::id()
        ));
        write_synthetic_pcap(&path, &[1, 2, 10, 11], 3);

        let source = CaptureSource::File(MyPcapImport::new(path.to_string_lossy().into_owned()));
        let context = CaptureContext::new(&source, None, &Filters::default());
        assert!(context.error().is_none(), "synthetic PCAP must open");

        let readers = MmdbReaders {
            country: Arc::new(MmdbReader::Empty),
            asn: Arc::new(MmdbReader::Empty),
        };
        let (tx, rx) = capture_update_channel(true);
        let (freeze_tx_1, freeze_rx_1) = tokio::sync::broadcast::channel(1);
        let (freeze_tx_2, freeze_rx_2) = tokio::sync::broadcast::channel(1);
        let worker = std::thread::spawn(move || {
            parse_packets(
                7,
                source,
                &readers,
                &IpBlacklist::default(),
                context,
                Filters::default(),
                &tx,
                (freeze_rx_1, freeze_rx_2),
            );
        });

        let started = Instant::now();
        let mut tick_count = 0;
        let mut saw_final_tick = false;
        let mut observed_gap = None;
        let mut saw_pending_hosts = false;
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match rx.try_recv() {
                Ok(BackendTrafficMessage::TickRun(7, info, _, is_final)) => {
                    tick_count += 1;
                    saw_final_tick |= is_final;
                    if is_final {
                        assert!(info.last_packet_timestamp.secs() >= 11);
                    }
                }
                Ok(BackendTrafficMessage::OfflineGap(7, seconds)) => {
                    observed_gap = Some(seconds);
                }
                Ok(BackendTrafficMessage::PendingHosts(7, _)) => {
                    saw_pending_hosts = true;
                    break;
                }
                Ok(_) => {}
                Err(async_channel::TryRecvError::Empty) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(error) => panic!("timed out waiting for synthetic PCAP import: {error}"),
            }
            // Simulate a UI consumer that is slower than the packet producer.
            std::thread::sleep(Duration::from_millis(2));
        }
        worker.join().unwrap();
        let elapsed = started.elapsed();
        fs::remove_file(path).unwrap();

        assert_eq!(
            tick_count, 4,
            "one tick per timestamp boundary plus final tick"
        );
        assert_eq!(observed_gap, Some(7), "the 2s-to-10s gap must be preserved");
        assert!(saw_final_tick, "the final traffic snapshot must be emitted");
        assert!(saw_pending_hosts, "the import must finish host delivery");
        assert!(
            elapsed < Duration::from_secs(10),
            "import took too long: {elapsed:?}"
        );
        eprintln!("synthetic PCAP import with slow consumer: {elapsed:?}");
    }

    fn measure_synthetic_import_producer_time(is_offline_import: bool, label: &str) -> Duration {
        const TIMESTAMP_COUNT: u32 = 256;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "sniffnet-channel-compare-{}-{nonce}.pcap",
            std::process::id()
        ));
        let timestamps: Vec<u32> = (1..=TIMESTAMP_COUNT).collect();
        write_synthetic_pcap(&path, &timestamps, 1);

        let source = CaptureSource::File(MyPcapImport::new(path.to_string_lossy().into_owned()));
        let context = CaptureContext::new(&source, None, &Filters::default());
        assert!(context.error().is_none(), "comparison PCAP must open");
        let readers = MmdbReaders {
            country: Arc::new(MmdbReader::Empty),
            asn: Arc::new(MmdbReader::Empty),
        };
        let (tx, rx) = capture_update_channel(is_offline_import);
        let (freeze_tx_1, freeze_rx_1) = tokio::sync::broadcast::channel(1);
        let (freeze_tx_2, freeze_rx_2) = tokio::sync::broadcast::channel(1);
        let (producer_done_tx, producer_done_rx) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || {
            let started = Instant::now();
            parse_packets(
                9,
                source,
                &readers,
                &IpBlacklist::default(),
                context,
                Filters::default(),
                &tx,
                (freeze_rx_1, freeze_rx_2),
            );
            producer_done_tx.send(started.elapsed()).unwrap();
        });

        let consumer_started = Instant::now();
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut tick_count = 0_u32;
        let mut saw_pending_hosts = false;
        loop {
            match rx.try_recv() {
                Ok(BackendTrafficMessage::TickRun(9, _, _, _)) => tick_count += 1,
                Ok(BackendTrafficMessage::PendingHosts(9, _)) => {
                    saw_pending_hosts = true;
                    break;
                }
                Ok(_) => {}
                Err(async_channel::TryRecvError::Empty) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(error) => panic!("timed out waiting for {label} comparison import: {error}"),
            }
            // Same deliberately slow UI-consumer behavior for both channel modes.
            std::thread::sleep(Duration::from_millis(1));
        }
        let consumer_elapsed = consumer_started.elapsed();
        let producer_elapsed = producer_done_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("producer should finish after final messages are consumed");
        worker.join().unwrap();
        fs::remove_file(path).unwrap();

        assert_eq!(tick_count, TIMESTAMP_COUNT);
        assert!(saw_pending_hosts);
        eprintln!(
            "synthetic PCAP channel comparison ({label}): producer={producer_elapsed:?}, consumer-drain={consumer_elapsed:?}, updates={tick_count}"
        );
        let _ = (freeze_tx_1, freeze_tx_2);
        producer_elapsed
    }

    #[test]
    fn synthetic_pcap_compares_bounded_and_unbounded_producer_time() {
        // Informational measurement only: CI scheduling is noisy, so do not assert a speed ratio.
        let bounded = measure_synthetic_import_producer_time(true, "bounded/offline");
        let unbounded = measure_synthetic_import_producer_time(false, "unbounded/live-style");
        eprintln!(
            "synthetic PCAP producer-time comparison: bounded={bounded:?}, unbounded={unbounded:?}"
        );
    }

    #[test]
    fn synthetic_pcap_import_stress_keeps_up_with_many_ui_updates() {
        const TIMESTAMP_COUNT: u32 = 256;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "sniffnet-offline-stress-{}-{nonce}.pcap",
            std::process::id()
        ));
        let timestamps: Vec<u32> = (1..=TIMESTAMP_COUNT).collect();
        write_synthetic_pcap(&path, &timestamps, 1);

        let source = CaptureSource::File(MyPcapImport::new(path.to_string_lossy().into_owned()));
        let context = CaptureContext::new(&source, None, &Filters::default());
        assert!(context.error().is_none(), "stress PCAP must open");
        let readers = MmdbReaders {
            country: Arc::new(MmdbReader::Empty),
            asn: Arc::new(MmdbReader::Empty),
        };
        let (tx, rx) = capture_update_channel(true);
        let (freeze_tx_1, freeze_rx_1) = tokio::sync::broadcast::channel(1);
        let (freeze_tx_2, freeze_rx_2) = tokio::sync::broadcast::channel(1);
        let worker = std::thread::spawn(move || {
            parse_packets(
                8,
                source,
                &readers,
                &IpBlacklist::default(),
                context,
                Filters::default(),
                &tx,
                (freeze_rx_1, freeze_rx_2),
            );
        });

        let started = Instant::now();
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut tick_count = 0_u32;
        let mut saw_pending_hosts = false;
        loop {
            match rx.try_recv() {
                Ok(BackendTrafficMessage::TickRun(8, _, _, _)) => tick_count += 1,
                Ok(BackendTrafficMessage::PendingHosts(8, _)) => {
                    saw_pending_hosts = true;
                    break;
                }
                Ok(_) => {}
                Err(async_channel::TryRecvError::Empty) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(error) => panic!("timed out waiting for stress PCAP import: {error}"),
            }
            // Deliberately slower than the producer, so the bounded channel fills repeatedly.
            std::thread::sleep(Duration::from_millis(1));
        }
        worker.join().unwrap();
        let elapsed = started.elapsed();
        fs::remove_file(path).unwrap();

        assert_eq!(tick_count, TIMESTAMP_COUNT);
        assert!(saw_pending_hosts);
        assert!(
            elapsed < Duration::from_secs(10),
            "stress import took {elapsed:?}"
        );
        eprintln!(
            "synthetic PCAP stress import: {TIMESTAMP_COUNT} timestamp updates, slow consumer, {elapsed:?}"
        );
        let _ = (freeze_tx_1, freeze_tx_2);
    }
}
