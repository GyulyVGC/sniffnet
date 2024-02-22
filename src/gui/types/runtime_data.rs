//! Module defining the `RunTimeData` struct, useful to to generate chart and to display statistics about network traffic

use std::collections::VecDeque;

use crate::notifications::types::logged_notification::LoggedNotification;

/// Struct containing useful data to display statistics about network traffic and the relative notifications
pub struct RunTimeData {
    /// Total number of bytes (filtered and not filtered)
    pub all_bytes: u128,
    /// Total number of packets (filtered and not filtered)
    pub all_packets: u128,
    /// Total sent bytes filtered
    pub tot_out_bytes: u128,
    /// Total received bytes filtered
    pub tot_in_bytes: u128,
    /// Total sent packets filtered
    pub tot_out_packets: u128,
    /// Total received packets filtered
    pub tot_in_packets: u128,
    /// Number of dropped packets
    pub dropped_packets: u32,
    /// Total sent bytes filtered before the current time interval
    pub tot_out_bytes_prev: u128,
    /// Total received bytes filtered before the current time interval
    pub tot_in_bytes_prev: u128,
    /// Total sent packets filtered before the current time interval
    pub tot_out_packets_prev: u128,
    /// Total received packets filtered before the current time interval
    pub tot_in_packets_prev: u128,
    /// Log of the received notifications
    pub logged_notifications: VecDeque<LoggedNotification>,
    /// The total number of emitted notifications
    pub tot_emitted_notifications: usize,
}

impl RunTimeData {
    /// Constructs a new `ChartsData` element.
    pub fn new() -> Self {
        RunTimeData {
            all_bytes: 0,
            all_packets: 0,
            tot_out_bytes: 0,
            tot_in_bytes: 0,
            tot_out_packets: 0,
            tot_in_packets: 0,
            dropped_packets: 0,
            tot_out_bytes_prev: 0,
            tot_in_bytes_prev: 0,
            tot_out_packets_prev: 0,
            tot_in_packets_prev: 0,
            logged_notifications: VecDeque::default(),
            tot_emitted_notifications: 0,
        }
    }
}
