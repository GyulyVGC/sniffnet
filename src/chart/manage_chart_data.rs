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

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::chart::manage_chart_data::{get_max, get_min, update_charts_data};
    use crate::{ChartType, RunTimeData, TrafficChart};

    #[test]
    fn test_chart_data_updates() {
        let sent = VecDeque::from([
            (0, -500),
            (1, -1000),
            (2, -1000),
            (3, -1000),
            (4, -1000),
            (5, -1000),
            (6, -1000),
            (7, -1000),
            (8, -1000),
            (9, -1000),
            (10, -1000),
            (11, -1000),
            (12, -1000),
            (13, -1000),
            (14, -1000),
            (15, -1000),
            (16, -1000),
            (17, -1000),
            (18, -1000),
            (19, -1000),
            (20, -1000),
            (21, -1000),
            (22, -1000),
            (23, -1000),
            (24, -1000),
            (25, -1000),
            (26, -1000),
            (27, -1000),
            (28, -1000),
        ]);
        let received = VecDeque::from([
            (0, 1000),
            (1, 21000),
            (2, 21000),
            (3, 21000),
            (4, 21000),
            (5, 21000),
            (6, 21000),
            (7, 21000),
            (8, 21000),
            (9, 21000),
            (10, 21000),
            (11, 21000),
            (12, 21000),
            (13, 21000),
            (14, 21000),
            (15, 21000),
            (16, 21000),
            (17, 21000),
            (18, 21000),
            (19, 21000),
            (20, 21000),
            (21, 21000),
            (22, 21000),
            (23, 21000),
            (24, 21000),
            (25, 21000),
            (26, 21000),
            (27, 21000),
            (28, 21000),
        ]);
        let tot_sent = 1000 * 28 + 500;
        let tot_received = 21000 * 28 + 1000;
        let mut traffic_chart = TrafficChart {
            ticks: 29,
            sent_bytes: sent.clone(),
            received_bytes: received.clone(),
            sent_packets: sent.clone(),
            received_packets: received.clone(),
            min_sent_bytes: -1000,
            max_received_bytes: 21000,
            min_sent_packets: -1000,
            max_received_packets: 21000,
            color_mix: 0.0,
            color_incoming: Default::default(),
            color_outgoing: Default::default(),
            color_font: Default::default(),
            chart_type: ChartType::Packets,
            language: Default::default(),
        };
        let mut runtime_data = RunTimeData {
            all_bytes: 0,
            all_packets: 0,
            tot_sent_bytes: tot_sent + 1111,
            tot_received_bytes: tot_received + 2222,
            tot_sent_packets: tot_sent + 3333,
            tot_received_packets: tot_received + 4444,
            dropped_packets: 0,
            tot_sent_bytes_prev: tot_sent,
            tot_received_bytes_prev: tot_received,
            tot_sent_packets_prev: tot_sent,
            tot_received_packets_prev: tot_received,
            logged_notifications: Default::default(),
            tot_emitted_notifications: 0,
        };

        assert_eq!(get_min(&sent), -1000);
        assert_eq!(get_max(&received), 21000);

        update_charts_data(&mut runtime_data, &mut traffic_chart);

        assert_eq!(get_min(&traffic_chart.sent_packets), -3333);
        assert_eq!(get_max(&traffic_chart.received_bytes), 21000);

        // runtime_data correctly updated?
        assert_eq!(runtime_data.tot_sent_bytes_prev, tot_sent + 1111);
        assert_eq!(runtime_data.tot_received_bytes_prev, tot_received + 2222);
        assert_eq!(runtime_data.tot_sent_packets_prev, tot_sent + 3333);
        assert_eq!(runtime_data.tot_received_packets_prev, tot_received + 4444);

        let mut sent_bytes = sent.clone();
        sent_bytes.push_back((29, -1111));
        let mut received_packets = received.clone();
        received_packets.push_back((29, 4444));
        let mut sent_packets = sent;
        sent_packets.push_back((29, -3333));
        let mut received_bytes = received;
        received_bytes.push_back((29, 2222));

        // traffic_chart correctly updated?
        assert_eq!(traffic_chart.ticks, 30);
        assert_eq!(traffic_chart.min_sent_bytes, -1111);
        assert_eq!(traffic_chart.min_sent_packets, -3333);
        assert_eq!(traffic_chart.max_received_bytes, 21000);
        assert_eq!(traffic_chart.max_received_packets, 21000);
        assert_eq!(traffic_chart.sent_bytes, sent_bytes);
        assert_eq!(traffic_chart.received_packets, received_packets);
        assert_eq!(traffic_chart.sent_packets, sent_packets);
        assert_eq!(traffic_chart.received_bytes, received_bytes);

        runtime_data.tot_sent_bytes += 99;
        runtime_data.tot_received_packets += 990;
        runtime_data.tot_received_bytes += 2;
        update_charts_data(&mut runtime_data, &mut traffic_chart);
        runtime_data.tot_sent_bytes += 77;
        runtime_data.tot_received_packets += 1;
        runtime_data.tot_sent_packets += 220;
        update_charts_data(&mut runtime_data, &mut traffic_chart);

        sent_bytes.pop_front();
        sent_bytes.pop_front();
        sent_bytes.push_back((30, -99));
        sent_bytes.push_back((31, -77));
        received_packets.pop_front();
        received_packets.pop_front();
        received_packets.push_back((30, 990));
        received_packets.push_back((31, 1));
        sent_packets.pop_front();
        sent_packets.pop_front();
        sent_packets.push_back((30, 0));
        sent_packets.push_back((31, -220));
        received_bytes.pop_front();
        received_bytes.pop_front();
        received_bytes.push_back((30, 2));
        received_bytes.push_back((31, 0));

        // traffic_chart correctly updated?
        assert_eq!(traffic_chart.ticks, 32);
        assert_eq!(traffic_chart.min_sent_bytes, -1111);
        assert_eq!(traffic_chart.min_sent_packets, -3333);
        assert_eq!(traffic_chart.max_received_bytes, 21000);
        assert_eq!(traffic_chart.max_received_packets, 21000);
        assert_eq!(traffic_chart.sent_bytes, sent_bytes);
        assert_eq!(traffic_chart.received_packets, received_packets);
        assert_eq!(traffic_chart.sent_packets, sent_packets);
        assert_eq!(traffic_chart.received_bytes, received_bytes);
    }
}
