use std::collections::VecDeque;

use crate::{RunTimeData, TrafficChart};

/// This function is invoked every second by the application subscription
///
/// It updates data (packets and bytes per second) to be displayed in the chart of gui run page
pub fn update_charts_data(mut runtime_data: &mut RunTimeData, traffic_chart: &mut TrafficChart) {
    let tot_seconds = traffic_chart.ticks;
    traffic_chart.ticks += 1;

    let sent_bytes_entry = runtime_data.tot_sent_bytes - runtime_data.tot_sent_bytes_prev;
    let received_bytes_entry =
        runtime_data.tot_received_bytes - runtime_data.tot_received_bytes_prev;
    let sent_packets_entry = runtime_data.tot_sent_packets - runtime_data.tot_sent_packets_prev;
    let received_packets_entry =
        runtime_data.tot_received_packets - runtime_data.tot_received_packets_prev;

    // update sent bytes traffic data
    if traffic_chart.sent_bytes.len() >= 30 {
        traffic_chart.sent_bytes.pop_front();
    }
    traffic_chart.sent_bytes.push_back((
        tot_seconds,
        -<u128 as TryInto<i64>>::try_into(sent_bytes_entry).unwrap(),
    ));
    traffic_chart.min_sent_bytes = get_min(&traffic_chart.sent_bytes.clone());
    runtime_data.tot_sent_bytes_prev = runtime_data.tot_sent_bytes;
    // update received bytes traffic data
    if traffic_chart.received_bytes.len() >= 30 {
        traffic_chart.received_bytes.pop_front();
    }
    traffic_chart
        .received_bytes
        .push_back((tot_seconds, received_bytes_entry.try_into().unwrap()));
    traffic_chart.max_received_bytes = get_max(&traffic_chart.received_bytes.clone());
    runtime_data.tot_received_bytes_prev = runtime_data.tot_received_bytes;

    // update sent packets traffic data
    if traffic_chart.sent_packets.len() >= 30 {
        traffic_chart.sent_packets.pop_front();
    }
    traffic_chart.sent_packets.push_back((
        tot_seconds,
        -<u128 as TryInto<i64>>::try_into(sent_packets_entry).unwrap(),
    ));
    traffic_chart.min_sent_packets = get_min(&traffic_chart.sent_packets.clone());
    runtime_data.tot_sent_packets_prev = runtime_data.tot_sent_packets;
    // update received packets traffic data
    if traffic_chart.received_packets.len() >= 30 {
        traffic_chart.received_packets.pop_front();
    }
    traffic_chart
        .received_packets
        .push_back((tot_seconds, received_packets_entry.try_into().unwrap()));
    traffic_chart.max_received_packets = get_max(&traffic_chart.received_packets.clone());
    runtime_data.tot_received_packets_prev = runtime_data.tot_received_packets;
}

/// Finds the minimum y value to be displayed in chart
fn get_min(deque: &VecDeque<(u32, i64)>) -> i64 {
    let mut min = 0;
    for (_, x) in deque.iter() {
        if *x < min {
            min = *x;
        }
    }
    min
}

/// Finds the maximum y value to be displayed in chart
fn get_max(deque: &VecDeque<(u32, i64)>) -> i64 {
    let mut max = 0;
    for (_, x) in deque.iter() {
        if *x > max {
            max = *x;
        }
    }
    max
}
