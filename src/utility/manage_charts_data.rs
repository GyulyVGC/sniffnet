use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::RunTimeData;

/// This function is invoked every second by the application subscription
///
/// It updates data (packets and bytes per second) to be displayed in the charts of gui run page
pub fn update_charts_data(charts_data_mutex: Arc<Mutex<RunTimeData>>) {
    let mut charts_data = charts_data_mutex.lock().unwrap();
    let tot_seconds = charts_data.ticks;
    charts_data.ticks += 1;

    let sent_bytes_entry = charts_data.tot_sent_bytes_prev - charts_data.tot_sent_bytes;
    let received_bytes_entry = charts_data.tot_received_bytes - charts_data.tot_received_bytes_prev;
    let sent_packets_entry = charts_data.tot_sent_packets_prev - charts_data.tot_sent_packets;
    let received_packets_entry = charts_data.tot_received_packets - charts_data.tot_received_packets_prev;

    // update sent bytes traffic data
    if charts_data.sent_bytes.len() >= 30 {
        charts_data.sent_bytes.pop_front();
    }
    charts_data.sent_bytes.push_back((tot_seconds as u128, sent_bytes_entry));
    charts_data.min_sent_bytes = get_min(charts_data.sent_bytes.clone());
    charts_data.tot_sent_bytes_prev = charts_data.tot_sent_bytes;
    // update received bytes traffic data
    if charts_data.received_bytes.len() >= 30 {
        charts_data.received_bytes.pop_front();
    }
    charts_data.received_bytes.push_back((tot_seconds as u128, received_bytes_entry));
    charts_data.max_received_bytes = get_max(charts_data.received_bytes.clone());
    charts_data.tot_received_bytes_prev = charts_data.tot_received_bytes;

    // update sent packets traffic data
    if charts_data.sent_packets.len() >= 30 {
        charts_data.sent_packets.pop_front();
    }
    charts_data.sent_packets.push_back((tot_seconds as u128, sent_packets_entry));
    charts_data.min_sent_packets = get_min(charts_data.sent_packets.clone());
    charts_data.tot_sent_packets_prev = charts_data.tot_sent_packets;
    // update received packets traffic data
    if charts_data.received_packets.len() >= 30 {
        charts_data.received_packets.pop_front();
    }
    charts_data.received_packets.push_back((tot_seconds as u128, received_packets_entry));
    charts_data.max_received_packets = get_max(charts_data.received_packets.clone());
    charts_data.tot_received_packets_prev = charts_data.tot_received_packets;
}

/// Finds the minimum y value to be displayed in charts
fn get_min(deque: VecDeque<(u128, i128)>) -> i128 {
    let mut min = 0;
    for (_, x) in deque.iter() {
        if *x < min {
            min = *x;
        }
    }
    min
}

/// Finds the maximum y value to be displayed in charts
fn get_max(deque: VecDeque<(u128, i128)>) -> i128 {
    let mut max = 0;
    for (_, x) in deque.iter() {
        if *x > max {
            max = *x;
        }
    }
    max
}