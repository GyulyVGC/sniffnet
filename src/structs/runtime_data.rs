//! Module defining the `RunTimeData` struct, useful to to generate charts and to display statistics about network traffic
//!
use std::collections::{HashMap, HashSet, VecDeque};

use crate::enums::logged_notification::LoggedNotification;
use crate::AppProtocol;

/// Struct containing useful data to generate charts and to display statistics about network traffic
pub struct RunTimeData {
    /// Total number of bytes (filtered and not filtered)
    pub all_bytes: u128,
    /// Total number of packets (filtered and not filtered)
    pub all_packets: u128,
    /// Application protocol with the respective number of filtered packets
    pub app_protocols: HashMap<AppProtocol, u128>,
    /// Total sent bytes filtered
    pub tot_sent_bytes: u128,
    /// Total received bytes filtered
    pub tot_received_bytes: u128,
    /// Total sent packets filtered
    pub tot_sent_packets: u128,
    /// Total received packets filtered
    pub tot_received_packets: u128,
    /// Total sent bytes filtered before the current time interval
    pub tot_sent_bytes_prev: u128,
    /// Total received bytes filtered before the current time interval
    pub tot_received_bytes_prev: u128,
    /// Total sent packets filtered before the current time interval
    pub tot_sent_packets_prev: u128,
    /// Total received packets filtered before the current time interval
    pub tot_received_packets_prev: u128,
    /// Collection of favorite connections that exchanged data in the last interval
    pub favorites_last_interval: HashSet<usize>,
    /// Log of the received notifications
    pub logged_notifications: VecDeque<LoggedNotification>,
}

impl RunTimeData {
    /// Constructs a new `ChartsData` element.
    pub fn new() -> Self {
        RunTimeData {
            all_bytes: 0,
            all_packets: 0,
            app_protocols: HashMap::default(),
            tot_sent_bytes: 0,
            tot_received_bytes: 0,
            tot_sent_packets: 0,
            tot_received_packets: 0,
            tot_sent_bytes_prev: 0,
            tot_received_bytes_prev: 0,
            tot_sent_packets_prev: 0,
            tot_received_packets_prev: 0,
            favorites_last_interval: HashSet::new(),
            logged_notifications: VecDeque::default(),
        }
    }
}
