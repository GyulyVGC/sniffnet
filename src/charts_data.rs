use std::collections::VecDeque;

pub struct ChartsData {
    pub sent_bits: VecDeque<(u128, i128)>,
    pub received_bits: VecDeque<(u128, i128)>,
    pub sent_packets: VecDeque<(u128, i128)>,
    pub received_packets: VecDeque<(u128, i128)>,
    pub tot_sent_bits_prev: i128,
    pub tot_received_bits_prev: i128,
    pub tot_sent_packets_prev: i128,
    pub tot_received_packets_prev: i128,
    pub min_sent_bits: i128,
    pub max_received_bits: i128,
    pub min_sent_packets: i128,
    pub max_received_packets: i128,
    pub ticks: u128
}


impl ChartsData {

    /// Constructs a new ChartsData element.
    pub fn new () -> Self {
        ChartsData {
            sent_bits: Default::default(),
            received_bits: Default::default(),
            sent_packets: Default::default(),
            received_packets: Default::default(),
            tot_sent_bits_prev: 0,
            tot_received_bits_prev: 0,
            tot_sent_packets_prev: 0,
            tot_received_packets_prev: 0,
            min_sent_bits: 0,
            max_received_bits: 0,
            min_sent_packets: 0,
            max_received_packets: 0,
            ticks: 0
        }
    }

}