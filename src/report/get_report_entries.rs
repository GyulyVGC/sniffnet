use std::cmp::min;
use std::sync::{Arc, Mutex};

use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::{InfoTraffic, ReportType};

pub fn get_report_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    report_type: ReportType,
) -> Vec<(AddressPortPair, InfoAddressPortPair)> {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = Vec::new();
    match report_type {
        ReportType::MostRecent => {
            sorted_vec = info_traffic_lock.map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)| b.final_timestamp.cmp(&a.final_timestamp));
        }
        ReportType::MostPackets => {
            sorted_vec = info_traffic_lock.map.iter().collect();
            sorted_vec
                .sort_by(|&(_, a), &(_, b)| b.transmitted_packets.cmp(&a.transmitted_packets));
        }
        ReportType::MostBytes => {
            sorted_vec = info_traffic_lock.map.iter().collect();
            sorted_vec.sort_by(|&(_, a), &(_, b)| b.transmitted_bytes.cmp(&a.transmitted_bytes));
        }
        ReportType::Favorites => {
            for index in &info_traffic_lock.favorite_connections {
                let key_val = info_traffic_lock.map.get_index(*index).unwrap();
                sorted_vec.push((key_val.0, key_val.1));
            }
        }
    }

    let n_entry = min(sorted_vec.len(), 15);
    sorted_vec[0..n_entry]
        .iter()
        .map(|e| (e.0.clone(), e.1.clone()))
        .collect()
}

/// Returns the indexes of the elements which satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints
pub fn get_searched_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    search: String,
    page_number: usize,
) -> (Vec<usize>, usize) {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let all_results: Vec<usize> = info_traffic_lock
        .map
        .iter()
        .filter(|(key, value)| value.country == search)
        .map(|(key, value)| value.index)
        .collect();
    let upper_bound = min(page_number * 15, all_results.len());

    (
        all_results[(page_number - 1) * 15..upper_bound].to_vec(),
        all_results.len(),
    )
}
