//! Module defining the `RunTimeData` struct, useful to to generate charts and to display statistics about network traffic
//!
use std::collections::{HashMap, HashSet, VecDeque};

use crate::enums::logged_notification::LoggedNotification;
use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::AppProtocol;

/// Struct containing useful data to generate charts and to display statistics about network traffic
pub struct RunTimeData {
    /// Total number of bytes (filtered and not filtered)
    pub all_bytes: u128,
    /// Sent bytes filtered and their time occurrence
    pub sent_bytes: VecDeque<(u32, i64)>,
    /// Received bytes filtered and their time occurrence
    pub received_bytes: VecDeque<(u32, i64)>,
    /// Total number of packets (filtered and not filtered)
    pub all_packets: u128,
    /// Sent packets filtered and their time occurrence
    pub sent_packets: VecDeque<(u32, i64)>,
    /// Received packets filtered and their time occurrence
    pub received_packets: VecDeque<(u32, i64)>,
    /// Application protocol with the respective number of filtered packets
    pub app_protocols: HashMap<AppProtocol, u128>,
    /// Connection entries to be displayed in report column
    pub report_vec: Vec<(AddressPortPair, InfoAddressPortPair)>,
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
    /// Minimum number of sent bytes per time interval (computed on last 30 intervals)
    pub min_sent_bytes: i64,
    /// Minimum number of received bytes per time interval (computed on last 30 intervals)
    pub max_received_bytes: i64,
    /// Minimum number of sent packets per time interval (computed on last 30 intervals)
    pub min_sent_packets: i64,
    /// Minimum number of received packets per time interval (computed on last 30 intervals)
    pub max_received_packets: i64,
    /// Collection of favorite connections that exchanged data in the last interval
    pub favorites_last_interval: HashSet<usize>,
    /// Log of the received notifications
    pub logged_notifications: VecDeque<LoggedNotification>,
    /// Current time interval number
    pub ticks: u32,
}

impl RunTimeData {
    /// Constructs a new `ChartsData` element.
    pub fn new() -> Self {
        RunTimeData {
            all_bytes: 0,
            sent_bytes: VecDeque::default(),
            received_bytes: VecDeque::default(),
            all_packets: 0,
            sent_packets: VecDeque::default(),
            received_packets: VecDeque::default(),
            app_protocols: HashMap::default(),
            report_vec: Vec::default(),
            tot_sent_bytes: 0,
            tot_received_bytes: 0,
            tot_sent_packets: 0,
            tot_received_packets: 0,
            tot_sent_bytes_prev: 0,
            tot_received_bytes_prev: 0,
            tot_sent_packets_prev: 0,
            tot_received_packets_prev: 0,
            min_sent_bytes: 0,
            max_received_bytes: 0,
            min_sent_packets: 0,
            max_received_packets: 0,
            favorites_last_interval: HashSet::new(),
            logged_notifications: VecDeque::default(),
            ticks: 0,
        }
    }
}
