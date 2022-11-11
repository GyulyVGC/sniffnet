use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use iced::Color;
use thousands::Separable;

use crate::{AppProtocol, Filters, RunTimeData, TransProtocol};
use crate::gui::style::{SPECIAL_DAY, SPECIAL_NIGHT};
use crate::structs::address_port_pair::TrafficType;

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

/// Computes the String representing the percentage of filtered bytes/packets
pub fn get_percentage_string(observed: u128, filtered: i128) -> String {
    if format!("{:.1}", 100.0 * (filtered) as f32 / observed as f32).eq("0.0") {
        "<0.1%".to_string()
    } else {
        format!("{:.1}%", 100.0 * (filtered) as f32 / observed as f32)
    }
}

/// Computes the String representing the active filters
pub fn get_active_filters_string(filters: Arc<Mutex<Filters>>) -> String {
    let filters_lock = filters.lock().unwrap();
    if filters_lock.ip == "no filter"
        && filters_lock.application.eq(&AppProtocol::Other)
        && filters_lock.transport.eq(&TransProtocol::Other) {
        "Active filters:\n   none".to_string()
    } else {
        let mut ret_val = "Active filters:".to_string();
        if filters_lock.ip != "no filter" {
            ret_val.push_str(&format!("\n   {}", filters_lock.ip.replace("ip", "IP")));
        }
        if filters_lock.transport.ne(&TransProtocol::Other) {
            ret_val.push_str(&format!("\n   {}", filters_lock.transport));
        }
        if filters_lock.application.ne(&AppProtocol::Other) {
            ret_val.push_str(&format!("\n   {}", filters_lock.application));
        }
        ret_val
    }
}

/// Computes the String representing the active filters, without line breaks
pub fn get_active_filters_string_nobr(filters: Arc<Mutex<Filters>>) -> String {
    let filters_lock = filters.lock().unwrap();
    let mut ret_val = "Active filters:".to_string();
    if filters_lock.ip != "no filter" {
        ret_val.push_str(&format!(" {}", filters_lock.ip.replace("ip", "IP")));
    }
    if filters_lock.transport.ne(&TransProtocol::Other) {
        ret_val.push_str(&format!(" {}", filters_lock.transport));
    }
    if filters_lock.application.ne(&AppProtocol::Other) {
        ret_val.push_str(&format!(" {}", filters_lock.application));
    }
    ret_val
}

/// Returns the color to be used for a specific connection of the relevant connections table in gui run page
pub fn get_connection_color(traffic_type: TrafficType) -> Color {
    if traffic_type == TrafficType::Incoming
        || traffic_type == TrafficType::Multicast {
        SPECIAL_NIGHT
    } else {
        SPECIAL_DAY
    }
}

/// Given the map of app layer protocols with the relative sniffed packets count,
/// the function generates the corresponding String
/// to be displayed in gui run page.
///
/// # Arguments
///
/// * `app_count` - Map of app layer protocols with the relative sniffed packets count
///
/// * `tot_packets` - Total number of sniffed packets
pub fn get_app_count_string(app_count: HashMap<AppProtocol, u128>, tot_packets: u128) -> String {
    let mut ret_val = "".to_string();

    let mut sorted_app_count: Vec<(&AppProtocol, &u128)> = app_count.iter().collect();
    sorted_app_count.sort_by(|&(p1, a), &(p2, b)| {
        if p1.eq(&AppProtocol::Other) {
            Ordering::Greater
        } else if p2.eq(&AppProtocol::Other) {
            Ordering::Less
        } else {
            b.cmp(a)
        }
    });

    //compute the length of the longest packet count string, used to align text
    let mut longest_num;
    longest_num = sorted_app_count.get(0).unwrap().1.separate_with_spaces().len();
    match app_count.get(&AppProtocol::Other) {
        None => {}
        Some(x) => {
            if x.separate_with_spaces().len() > longest_num {
                longest_num = x.separate_with_spaces().len();
            }
        }
    }

    for entry in sorted_app_count {
        let app_proto_string = format!("{:?}", entry.0);

        let num_string = entry.1.separate_with_spaces().to_string();

        let percentage_string =
            if format!("{:.1}", 100.0 * (*entry.1) as f32 / tot_packets as f32).eq("0.0") {
                "(<0.1%)".to_string()
            } else {
                format!("({:.1}%)", 100.0 * (*entry.1) as f32 / tot_packets as f32)
            };

        //to align digits
        let spaces_string_1 = " ".to_string()
            .repeat(9 + longest_num - num_string.len() - app_proto_string.len());
        let spaces_string_2 = " ".to_string()
            .repeat(10 - percentage_string.len());

        ret_val.push_str(&format!("   {}:{}{}{}{}  \n",
                                  app_proto_string,
                                  spaces_string_1,
                                  num_string,
                                  spaces_string_2,
                                  percentage_string));
    }
    ret_val
}