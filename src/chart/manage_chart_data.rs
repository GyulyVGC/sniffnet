use splines::{Interpolation, Key, Spline};

use crate::{RunTimeData, TrafficChart};

/// This function is invoked every second by the application subscription
///
/// It updates data (packets and bytes per second) to be displayed in the chart of gui run page
pub fn update_charts_data(runtime_data: &mut RunTimeData, traffic_chart: &mut TrafficChart) {
    #[allow(clippy::cast_precision_loss)]
    let tot_seconds = traffic_chart.ticks as f32;
    traffic_chart.ticks += 1;

    #[allow(clippy::cast_precision_loss)]
    let out_bytes_entry =
        -1.0 * (runtime_data.tot_out_bytes - runtime_data.tot_out_bytes_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let in_bytes_entry = (runtime_data.tot_in_bytes - runtime_data.tot_in_bytes_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let out_packets_entry =
        -1.0 * (runtime_data.tot_out_packets - runtime_data.tot_out_packets_prev) as f32;
    #[allow(clippy::cast_precision_loss)]
    let in_packets_entry = (runtime_data.tot_in_packets - runtime_data.tot_in_packets_prev) as f32;

    let out_bytes_key = Key::new(tot_seconds, out_bytes_entry, Interpolation::Cosine);
    let in_bytes_key = Key::new(tot_seconds, in_bytes_entry, Interpolation::Cosine);
    let out_packets_key = Key::new(tot_seconds, out_packets_entry, Interpolation::Cosine);
    let in_packets_key = Key::new(tot_seconds, in_packets_entry, Interpolation::Cosine);

    // update sent bytes traffic data
    update_spline(&mut traffic_chart.out_bytes, out_bytes_key);
    traffic_chart.min_bytes = get_min(&traffic_chart.out_bytes);
    runtime_data.tot_out_bytes_prev = runtime_data.tot_out_bytes;

    // update received bytes traffic data
    update_spline(&mut traffic_chart.in_bytes, in_bytes_key);
    traffic_chart.max_bytes = get_max(&traffic_chart.in_bytes);
    runtime_data.tot_in_bytes_prev = runtime_data.tot_in_bytes;

    // update sent packets traffic data
    update_spline(&mut traffic_chart.out_packets, out_packets_key);
    traffic_chart.min_packets = get_min(&traffic_chart.out_packets);
    runtime_data.tot_out_packets_prev = runtime_data.tot_out_packets;

    // update received packets traffic data
    update_spline(&mut traffic_chart.in_packets, in_packets_key);
    traffic_chart.max_packets = get_max(&traffic_chart.in_packets);
    runtime_data.tot_in_packets_prev = runtime_data.tot_in_packets;
}

fn update_spline(spline: &mut Spline<f32, f32>, new_key: Key<f32, f32>) {
    if spline.len() >= 30 {
        spline.remove(0);
    }
    spline.add(new_key);
}

/// Finds the minimum y value to be displayed in chart.
fn get_min(spline: &Spline<f32, f32>) -> f32 {
    let mut min = 0.0;
    for key in spline {
        if key.value < min {
            min = key.value;
        }
    }
    min
}

/// Finds the maximum y value to be displayed in chart.
fn get_max(spline: &Spline<f32, f32>) -> f32 {
    let mut max = 0.0;
    for key in spline {
        if key.value > max {
            max = key.value;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use splines::{Interpolation, Key, Spline};

    use crate::chart::manage_chart_data::{get_max, get_min, update_charts_data};
    use crate::{ChartType, Language, RunTimeData, StyleType, TrafficChart};

    fn spline_from_vec(vec: Vec<(i32, i32)>) -> Spline<f32, f32> {
        Spline::from_vec(
            vec.iter()
                .map(|&(x, y)| Key::new(x as f32, y as f32, Interpolation::Cosine))
                .collect::<Vec<Key<f32, f32>>>(),
        )
    }

    #[test]
    fn test_chart_data_updates() {
        let sent_vec = vec![
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
        ];
        let sent = spline_from_vec(sent_vec);
        let received_vec = vec![
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
        ];
        let received = spline_from_vec(received_vec);
        let tot_sent = 1000 * 28 + 500;
        let tot_received = 21000 * 28 + 1000;
        let mut traffic_chart = TrafficChart {
            ticks: 29,
            out_bytes: sent.clone(),
            in_bytes: received.clone(),
            out_packets: sent.clone(),
            in_packets: received.clone(),
            min_bytes: -1000.0,
            max_bytes: 21000.0,
            min_packets: -1000.0,
            max_packets: 21000.0,
            language: Language::default(),
            chart_type: ChartType::Packets,
            style: StyleType::default(),
            thumbnail: false,
        };
        let mut runtime_data = RunTimeData {
            all_bytes: 0,
            all_packets: 0,
            tot_out_bytes: tot_sent + 1111,
            tot_in_bytes: tot_received + 2222,
            tot_out_packets: tot_sent + 3333,
            tot_in_packets: tot_received + 4444,
            dropped_packets: 0,
            tot_out_bytes_prev: tot_sent,
            tot_in_bytes_prev: tot_received,
            tot_out_packets_prev: tot_sent,
            tot_in_packets_prev: tot_received,
            logged_notifications: Default::default(),
            tot_emitted_notifications: 0,
        };

        assert_eq!(get_min(&sent), -1000.0);
        assert_eq!(get_max(&received), 21000.0);

        update_charts_data(&mut runtime_data, &mut traffic_chart);

        assert_eq!(get_min(&traffic_chart.out_packets), -3333.0);
        assert_eq!(get_max(&traffic_chart.in_bytes), 21000.0);

        // runtime_data correctly updated?
        assert_eq!(runtime_data.tot_out_bytes_prev, tot_sent + 1111);
        assert_eq!(runtime_data.tot_in_bytes_prev, tot_received + 2222);
        assert_eq!(runtime_data.tot_out_packets_prev, tot_sent + 3333);
        assert_eq!(runtime_data.tot_in_packets_prev, tot_received + 4444);

        let mut sent_bytes = sent.clone();
        sent_bytes.add(Key::new(29.0, -1111.0, Interpolation::Cosine));
        let mut received_packets = received.clone();
        received_packets.add(Key::new(29.0, 4444.0, Interpolation::Cosine));
        let mut sent_packets = sent;
        sent_packets.add(Key::new(29.0, -3333.0, Interpolation::Cosine));
        let mut received_bytes = received;
        received_bytes.add(Key::new(29.0, 2222.0, Interpolation::Cosine));

        // traffic_chart correctly updated?
        assert_eq!(traffic_chart.ticks, 30);
        assert_eq!(traffic_chart.min_bytes, -1111.0);
        assert_eq!(traffic_chart.min_packets, -3333.0);
        assert_eq!(traffic_chart.max_bytes, 21000.0);
        assert_eq!(traffic_chart.max_packets, 21000.0);
        assert_eq!(traffic_chart.out_bytes.keys(), sent_bytes.keys());
        assert_eq!(traffic_chart.in_packets.keys(), received_packets.keys());
        assert_eq!(traffic_chart.out_packets.keys(), sent_packets.keys());
        assert_eq!(traffic_chart.in_bytes.keys(), received_bytes.keys());

        runtime_data.tot_out_bytes += 99;
        runtime_data.tot_in_packets += 990;
        runtime_data.tot_in_bytes += 2;
        update_charts_data(&mut runtime_data, &mut traffic_chart);
        runtime_data.tot_out_bytes += 77;
        runtime_data.tot_in_packets += 1;
        runtime_data.tot_out_packets += 220;
        update_charts_data(&mut runtime_data, &mut traffic_chart);

        sent_bytes.remove(0);
        sent_bytes.remove(0);
        sent_bytes.add(Key::new(30.0, -99.0, Interpolation::Cosine));
        sent_bytes.add(Key::new(31.0, -77.0, Interpolation::Cosine));
        received_packets.remove(0);
        received_packets.remove(0);
        received_packets.add(Key::new(30.0, 990.0, Interpolation::Cosine));
        received_packets.add(Key::new(31.0, 1.0, Interpolation::Cosine));
        sent_packets.remove(0);
        sent_packets.remove(0);
        sent_packets.add(Key::new(30.0, 0.0, Interpolation::Cosine));
        sent_packets.add(Key::new(31.0, -220.0, Interpolation::Cosine));
        received_bytes.remove(0);
        received_bytes.remove(0);
        received_bytes.add(Key::new(30.0, 2.0, Interpolation::Cosine));
        received_bytes.add(Key::new(31.0, 0.0, Interpolation::Cosine));

        // traffic_chart correctly updated?
        assert_eq!(traffic_chart.ticks, 32);
        assert_eq!(traffic_chart.min_bytes, -1111.0);
        assert_eq!(traffic_chart.min_packets, -3333.0);
        assert_eq!(traffic_chart.max_bytes, 21000.0);
        assert_eq!(traffic_chart.max_packets, 21000.0);
        assert_eq!(traffic_chart.out_bytes.keys(), sent_bytes.keys());
        assert_eq!(traffic_chart.in_packets.keys(), received_packets.keys());
        assert_eq!(traffic_chart.out_packets.keys(), sent_packets.keys());
        assert_eq!(traffic_chart.in_bytes.keys(), received_bytes.keys());
    }
}
