use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::{InfoTraffic, ReportType, RunTimeData};
use std::cell::RefMut;
use std::cmp::min;
use std::sync::{Arc, Mutex};

pub fn update_report_data(
    mut runtime_data: RefMut<RunTimeData>,
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    report_type: ReportType,
) {
    let info_traffic_lock = info_traffic.lock().unwrap();
    runtime_data.report_vec = Vec::default();

    if report_type.ne(&ReportType::Favorites) {
        let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> =
            info_traffic_lock.map.iter().collect();

        match report_type {
            ReportType::MostRecent => {
                sorted_vec.sort_by(|&(_, a), &(_, b)| b.final_timestamp.cmp(&a.final_timestamp));
            }
            ReportType::MostPackets => {
                sorted_vec
                    .sort_by(|&(_, a), &(_, b)| b.transmitted_packets.cmp(&a.transmitted_packets));
            }
            ReportType::MostBytes => {
                sorted_vec
                    .sort_by(|&(_, a), &(_, b)| b.transmitted_bytes.cmp(&a.transmitted_bytes));
            }
            ReportType::Favorites => {}
        }

        let n_entry = min(sorted_vec.len(), 15);
        for i in 0..n_entry {
            let key_val = *sorted_vec.get(i).unwrap();
            runtime_data
                .report_vec
                .push((key_val.0.clone(), key_val.1.clone()));
        }
    } else {
        // favorites
        for index in &info_traffic_lock.favorite_connections {
            let key_val = info_traffic_lock.map.get_index(*index).unwrap();
            runtime_data
                .report_vec
                .push((key_val.0.clone(), key_val.1.clone()));
        }
    }
}
