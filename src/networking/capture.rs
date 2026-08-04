//! Entry point for starting a capture backend.
//!
//! Picks the implementation that matches the opened capture context — the IPFIX
//! collector for a bound UDP socket, the pcap pipeline for everything else — so
//! callers only have to ask for "the capture thread".

use async_channel::Sender;
use std::thread;
use tokio::sync::broadcast::Receiver;

use crate::gui::types::filters::Filters;
use crate::location;
use crate::mmdb::types::mmdb_reader::MmdbReaders;
use crate::networking::ipfix::collect::collect_ipfix;
use crate::networking::parse_packets::{BackendTrafficMessage, parse_packets};
use crate::networking::types::capture_context::{CaptureContext, CaptureSource};
use crate::networking::types::ip_blacklist::IpBlacklist;
use crate::utils::error_logger::{ErrorLogger, Location};

/// Spawns the backend thread that feeds `tx` with traffic updates.
#[allow(clippy::too_many_arguments)]
pub fn spawn_capture_thread(
    cap_id: usize,
    capture_source: CaptureSource,
    capture_context: CaptureContext,
    mmdb_readers: MmdbReaders,
    ip_blacklist: IpBlacklist,
    filters: Filters,
    tx: Sender<BackendTrafficMessage>,
    freeze_rxs: (Receiver<()>, Receiver<()>),
) {
    match capture_context {
        CaptureContext::Ipfix(socket) => {
            let _ = thread::Builder::new()
                .name("thread_collect_ipfix".to_string())
                .spawn(move || {
                    collect_ipfix(
                        cap_id,
                        socket,
                        &mmdb_readers,
                        &ip_blacklist,
                        &tx,
                        freeze_rxs,
                    );
                })
                .log_err(location!());
        }
        capture_context => {
            let _ = thread::Builder::new()
                .name("thread_parse_packets".to_string())
                .spawn(move || {
                    parse_packets(
                        cap_id,
                        capture_source,
                        &mmdb_readers,
                        &ip_blacklist,
                        capture_context,
                        filters,
                        &tx,
                        freeze_rxs,
                    );
                })
                .log_err(location!());
        }
    }
}
