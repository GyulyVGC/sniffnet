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
use sniffnet_packet_parser::{LinkType, ParsedPacket};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::Receiver;

const RECAPTURE_DELAY: Duration = Duration::from_millis(500);

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

    let has_savefile = savefile.is_some();
    let device = match &cs {
        CaptureSource::Device(dev) => Some(dev.clone()),
        CaptureSource::File(_) | CaptureSource::Ipfix(_) => None,
    };

    let (pcap_tx, pcap_rx) = std::sync::mpsc::sync_channel(10_000);
    let _ = thread::Builder::new()
        .name("thread_packet_stream".to_string())
        .spawn(move || {
            packet_stream(
                cap,
                device,
                has_savefile,
                &pcap_tx,
                &mut freeze_rx_2,
                &filters,
            );
        })
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
                    // wait until there is still some IP address waiting for resolution
                    let mut pending_hosts = Vec::new();
                    while !resolutions_state.addresses_waiting_resolution.is_empty() {
                        pending_hosts.extend(resolutions_state.new_hosts_to_send());
                        thread::sleep(Duration::from_secs(1));
                    }
                    // send one last message including all pending hosts
                    let _ = tx
                        .send_blocking(BackendTrafficMessage::PendingHosts(cap_id, pending_hosts));
                    return;
                }
            }
            Ok(packet) => {
                if let Some(parsed) = ParsedPacket::from_bytes(&packet.data, packet.link_type) {
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

trait PacketSource {
    type Packet;

    fn next_packet_owned(&mut self) -> Result<Self::Packet, pcap::Error>;
    fn stats(&mut self) -> Option<pcap::Stat>;
    fn is_live(&self) -> bool;
    fn pause(&mut self);
    fn resume(&mut self, filters: &Filters) -> Result<(), pcap::Error>;
}

impl PacketSource for CaptureType {
    type Packet = PacketOwned;

    fn next_packet_owned(&mut self) -> Result<Self::Packet, pcap::Error> {
        let link_type = self.link_type();
        self.next_packet().map(|p| PacketOwned {
            header: *p.header,
            data: p.data.into(),
            link_type,
        })
    }

    fn stats(&mut self) -> Option<pcap::Stat> {
        Self::stats(self).ok()
    }

    fn is_live(&self) -> bool {
        matches!(self, Self::Live(_))
    }

    fn pause(&mut self) {
        Self::pause(self);
    }

    fn resume(&mut self, filters: &Filters) -> Result<(), pcap::Error> {
        Self::resume(self, filters)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn packet_stream(
    cap: CaptureType,
    device: Option<crate::networking::types::my_device::MyDevice>,
    has_savefile: bool,
    tx: &std::sync::mpsc::SyncSender<(Result<PacketOwned, pcap::Error>, Option<pcap::Stat>)>,
    freeze_rx: &mut Receiver<()>,
    filters: &Filters,
) {
    let reopen = || {
        let dev = device.as_ref()?;
        CaptureType::from_device_with_filters(dev, has_savefile, filters)
            .ok()
            .filter(|c| c.link_type().is_supported())
    };
    packet_stream_loop(Some(cap), tx, freeze_rx, filters, reopen, RECAPTURE_DELAY);
}

fn packet_stream_loop<P, S: PacketSource<Packet = P>>(
    mut cap: Option<S>,
    tx: &std::sync::mpsc::SyncSender<(Result<P, pcap::Error>, Option<pcap::Stat>)>,
    freeze_rx: &mut Receiver<()>,
    filters: &Filters,
    mut reopen: impl FnMut() -> Option<S>,
    retry_delay: Duration,
) {
    loop {
        // check if we need to freeze the parsing
        if freeze_rx.try_recv().is_ok() {
            // pause the capture
            if let Some(active_cap) = cap.as_mut() {
                active_cap.pause();
            }
            // wait until unfreeze
            if freeze_rx.blocking_recv().is_err() {
                return;
            }
            // resume the capture
            if let Some(active_cap) = cap.as_mut()
                && active_cap.resume(filters).is_err()
            {
                cap = None;
            }
        }

        let Some(active_cap) = cap.as_mut() else {
            thread::sleep(retry_delay);
            // keep parse_packets responsive and exit promptly if the receiver was dropped
            if tx.send((Err(pcap::Error::TimeoutExpired), None)).is_err() {
                return;
            }
            cap = reopen();
            continue;
        };

        match active_cap.next_packet_owned() {
            Ok(packet_owned) => {
                let stats = active_cap.stats();
                if tx.send((Ok(packet_owned), stats)).is_err() {
                    return;
                }
            }
            Err(pcap::Error::TimeoutExpired) => {
                let stats = active_cap.stats();
                if tx.send((Err(pcap::Error::TimeoutExpired), stats)).is_err() {
                    return;
                }
            }
            Err(pcap::Error::NoMorePackets) => {
                let stats = active_cap.stats();
                let _ = tx.send((Err(pcap::Error::NoMorePackets), stats));
                return;
            }
            Err(_) => {
                if !active_cap.is_live() {
                    let _ = tx.send((Err(pcap::Error::NoMorePackets), None));
                    return;
                }
                // Live interface error (e.g., interface went down / disappeared):
                // drop the stale capture handle so the next iteration waits and re-instantiates it.
                cap = None;
            }
        }
    }
}

struct PacketOwned {
    header: PacketHeader,
    data: Box<[u8]>,
    link_type: LinkType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockPacketSource {
        responses: VecDeque<Result<u8, pcap::Error>>,
        live: bool,
        resume_err: bool,
        next_calls: Arc<AtomicUsize>,
    }

    impl PacketSource for MockPacketSource {
        type Packet = u8;

        fn next_packet_owned(&mut self) -> Result<Self::Packet, pcap::Error> {
            self.next_calls.fetch_add(1, Ordering::SeqCst);
            self.responses
                .pop_front()
                .unwrap_or(Err(pcap::Error::PcapError(
                    "The interface disappeared".to_string(),
                )))
        }

        fn stats(&mut self) -> Option<pcap::Stat> {
            None
        }

        fn is_live(&self) -> bool {
            self.live
        }

        fn pause(&mut self) {}

        fn resume(&mut self, _filters: &Filters) -> Result<(), pcap::Error> {
            if self.resume_err {
                Err(pcap::Error::PcapError(
                    "The interface disappeared".to_string(),
                ))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_packet_stream_reopens_after_interface_disappeared() {
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        let (_freeze_tx, mut freeze_rx) = tokio::sync::broadcast::channel(4);
        let filters = Filters::default();

        let first_calls = Arc::new(AtomicUsize::new(0));
        let second_calls = Arc::new(AtomicUsize::new(0));
        let reopen_attempts = Arc::new(AtomicUsize::new(0));

        let initial_cap = MockPacketSource {
            responses: VecDeque::from([
                Ok(1),
                Err(pcap::Error::PcapError(
                    "The interface disappeared".to_string(),
                )),
            ]),
            live: true,
            resume_err: false,
            next_calls: first_calls.clone(),
        };

        let second_calls_clone = second_calls.clone();
        let reopen_attempts_clone = reopen_attempts.clone();
        let reopen = move || {
            let attempt = reopen_attempts_clone.fetch_add(1, Ordering::SeqCst);
            if attempt == 0 {
                // Simulate interface still being down on the first reopen attempt
                None
            } else {
                Some(MockPacketSource {
                    responses: VecDeque::from([Ok(2), Err(pcap::Error::NoMorePackets)]),
                    live: true,
                    resume_err: false,
                    next_calls: second_calls_clone.clone(),
                })
            }
        };

        packet_stream_loop(
            Some(initial_cap),
            &tx,
            &mut freeze_rx,
            &filters,
            reopen,
            Duration::from_millis(5),
        );

        assert_eq!(first_calls.load(Ordering::SeqCst), 2);
        assert_eq!(reopen_attempts.load(Ordering::SeqCst), 2);
        assert_eq!(second_calls.load(Ordering::SeqCst), 2);

        let msgs: Vec<_> = rx.try_iter().map(|(res, _)| res).collect();
        assert_eq!(
            msgs,
            vec![
                Ok(1),
                Err(pcap::Error::TimeoutExpired),
                Err(pcap::Error::TimeoutExpired),
                Ok(2),
                Err(pcap::Error::NoMorePackets),
            ]
        );
    }

    #[test]
    fn test_packet_stream_stops_when_receiver_dropped_while_interface_down() {
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        let (_freeze_tx, mut freeze_rx) = tokio::sync::broadcast::channel(4);
        let filters = Filters::default();

        let calls = Arc::new(AtomicUsize::new(0));
        let initial_cap = MockPacketSource {
            responses: VecDeque::from([Err(pcap::Error::PcapError(
                "The interface disappeared".to_string(),
            ))]),
            live: true,
            resume_err: false,
            next_calls: calls.clone(),
        };

        drop(rx);

        packet_stream_loop(
            Some(initial_cap),
            &tx,
            &mut freeze_rx,
            &filters,
            || None,
            Duration::from_millis(5),
        );

        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_packet_stream_offline_error_stops_without_spinning() {
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        let (_freeze_tx, mut freeze_rx) = tokio::sync::broadcast::channel(4);
        let filters = Filters::default();

        let calls = Arc::new(AtomicUsize::new(0));
        let offline_cap = MockPacketSource {
            responses: VecDeque::from([Err(pcap::Error::PcapError(
                "truncated dump file".to_string(),
            ))]),
            live: false,
            resume_err: false,
            next_calls: calls.clone(),
        };

        packet_stream_loop(
            Some(offline_cap),
            &tx,
            &mut freeze_rx,
            &filters,
            || None,
            Duration::from_millis(5),
        );

        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert!(matches!(
            rx.try_recv(),
            Ok((Err(pcap::Error::NoMorePackets), None))
        ));
    }

    #[test]
    fn test_packet_stream_freeze_resume_failure_reopens() {
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        let (freeze_tx, mut freeze_rx) = tokio::sync::broadcast::channel(4);
        let filters = Filters::default();

        let _ = freeze_tx.send(());
        let _ = freeze_tx.send(());

        let first_calls = Arc::new(AtomicUsize::new(0));
        let initial_cap = MockPacketSource {
            responses: VecDeque::from([Ok(9)]),
            live: true,
            resume_err: true,
            next_calls: first_calls.clone(),
        };

        let reopen = || {
            Some(MockPacketSource {
                responses: VecDeque::from([Ok(3), Err(pcap::Error::NoMorePackets)]),
                live: true,
                resume_err: false,
                next_calls: Arc::new(AtomicUsize::new(0)),
            })
        };

        packet_stream_loop(
            Some(initial_cap),
            &tx,
            &mut freeze_rx,
            &filters,
            reopen,
            Duration::from_millis(5),
        );

        assert_eq!(first_calls.load(Ordering::SeqCst), 0);
        let msgs: Vec<_> = rx.try_iter().map(|(res, _)| res).collect();
        assert_eq!(
            msgs,
            vec![
                Err(pcap::Error::TimeoutExpired),
                Ok(3),
                Err(pcap::Error::NoMorePackets),
            ]
        );
    }
}
