use std::collections::{HashMap, VecDeque};
use crate::AppProtocol;

// #[derive(Clone)]
pub struct RunTimeData {
    pub all_bytes: u128,
    pub sent_bytes: VecDeque<(u128, i128)>,
    pub received_bytes: VecDeque<(u128, i128)>,
    pub all_packets: u128,
    pub sent_packets: VecDeque<(u128, i128)>,
    pub received_packets: VecDeque<(u128, i128)>,
    pub app_protocols: HashMap<AppProtocol, u128>,
    pub tot_sent_bytes: i128,
    pub tot_received_bytes: i128,
    pub tot_sent_packets: i128,
    pub tot_received_packets: i128,
    pub tot_sent_bytes_prev: i128,
    pub tot_received_bytes_prev: i128,
    pub tot_sent_packets_prev: i128,
    pub tot_received_packets_prev: i128,
    pub min_sent_bytes: i128,
    pub max_received_bytes: i128,
    pub min_sent_packets: i128,
    pub max_received_packets: i128,
    pub ticks: u128
}


impl RunTimeData {

    /// Constructs a new ChartsData element.
    pub fn new () -> Self {
        RunTimeData {
            all_bytes: 0,
            sent_bytes: Default::default(),
            received_bytes: Default::default(),
            all_packets: 0,
            sent_packets: Default::default(),
            received_packets: Default::default(),
            app_protocols: Default::default(),
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
            ticks: 0
        }
    }

}