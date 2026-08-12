//! `pcap`-based capture backend

use crate::gui::types::filters::Filters;
use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::capture::{
    AddressesResolutionState, BackendTrafficMessage, maybe_send_tick, spawn_reverse_dns_pool,
};
use crate::networking::manage_packets::{
    analyze_headers, modify_or_insert_in_map, update_connection_stats,
};
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::capture_context::{CaptureContext, CaptureSource, CaptureType};
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::networking::types::my_link_type::MyLinkType;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::types::timestamp::Timestamp;
use async_channel::Sender;
use etherparse::{EtherType, LaxPacketHeaders};
use pcap::{Packet, PacketHeader};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::Receiver;

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

    let my_link_type = capture_context.my_link_type();
    if !my_link_type.is_supported() {
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
                if let Some(headers) = get_sniffable_headers(&packet.data, my_link_type) {
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

                    let mut exchanged_bytes = 0;
                    let mut mac_addresses = (None, None);
                    let mut icmp_type = IcmpType::default();
                    let mut arp_type = ArpType::default();

                    let key_option = analyze_headers(
                        headers,
                        &mut mac_addresses,
                        &mut exchanged_bytes,
                        &mut icmp_type,
                        &mut arp_type,
                    );

                    let Some(key) = key_option else {
                        continue;
                    };

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
                        Some(icmp_type),
                        arp_type,
                        1,
                        exchanged_bytes,
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
                        exchanged_bytes,
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

pub(super) fn get_sniffable_headers(
    packet: &[u8],
    my_link_type: MyLinkType,
) -> Option<LaxPacketHeaders<'_>> {
    match my_link_type {
        MyLinkType::Ethernet(_) | MyLinkType::Unsupported(_) | MyLinkType::NotYetAssigned => {
            LaxPacketHeaders::from_ethernet(packet).ok()
        }
        MyLinkType::RawIp(_) | MyLinkType::IPv4(_) | MyLinkType::IPv6(_) => {
            LaxPacketHeaders::from_ip(packet).ok()
        }
        MyLinkType::LinuxSll(_) => from_linux_sll(packet, true),
        MyLinkType::LinuxSll2(_) => from_linux_sll(packet, false),
        MyLinkType::Null(_) | MyLinkType::Loop(_) => from_null(packet),
    }
}

fn from_null(packet: &[u8]) -> Option<LaxPacketHeaders<'_>> {
    if packet.len() <= 4 {
        return None;
    }

    let is_valid_af_inet = {
        // based on https://wiki.wireshark.org/NullLoopback.md (2023-12-31)
        fn matches(value: u32) -> bool {
            match value {
                // 2 = IPv4 on all platforms
                // 24, 28, or 30 = IPv6 depending on platform
                2 | 24 | 28 | 30 => true,
                _ => false,
            }
        }
        let h = &packet[..4];
        let b = [h[0], h[1], h[2], h[3]];
        // check both big endian and little endian representations
        // as some OS'es use native endianness and others use big endian
        matches(u32::from_le_bytes(b)) || matches(u32::from_be_bytes(b))
    };

    if is_valid_af_inet {
        LaxPacketHeaders::from_ip(&packet[4..]).ok()
    } else {
        None
    }
}

// TODO: do this with etherparse once they support Linux SLL2
fn from_linux_sll(packet: &[u8], is_v1: bool) -> Option<LaxPacketHeaders<'_>> {
    let header_len = if is_v1 { 16 } else { 20 };
    if packet.len() <= header_len {
        return None;
    }

    let protocol_type = u16::from_be_bytes(if is_v1 {
        [packet[14], packet[15]]
    } else {
        [packet[0], packet[1]]
    });
    let payload = &packet[header_len..];

    Some(LaxPacketHeaders::from_ether_type(
        EtherType(protocol_type),
        payload,
    ))
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
