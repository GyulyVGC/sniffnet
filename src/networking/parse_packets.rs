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
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::timestamp::Timestamp;
use async_channel::Sender;
use pcap::{Packet, PacketHeader};
use sniffnet_packet_parser::ParsedPacket;
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::Receiver;

/// The calling thread enters a loop in which it waits for network packets
#[allow(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::similar_names
)]
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
                    let icmp_type = parsed.transport_info.icmp_type;
                    let arp_type = parsed.net_info.arp_type;
                    let igmp_type = parsed.transport_info.igmp_type;

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
                        icmp_type,
                        arp_type,
                        igmp_type,
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
                        info_traffic_msg.dropped_packets = Some(stats.dropped);
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
