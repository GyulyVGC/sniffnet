use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::{InfoTraffic, ReportType, RunTimeData};
use std::cmp::min;
use std::sync::{Arc, Mutex};

pub fn update_report_data(
    runtime_data: Arc<Mutex<RunTimeData>>,
    info_traffic: Arc<Mutex<InfoTraffic>>,
    report_type: ReportType,
) {
    let mut runtime_data_lock = runtime_data.lock().unwrap();
    let info_traffic_lock = info_traffic.lock().unwrap();

    runtime_data_lock.report_vec = Default::default();
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
            sorted_vec.sort_by(|&(_, a), &(_, b)| b.transmitted_bytes.cmp(&a.transmitted_bytes));
        }
    }

    let n_entry = min(sorted_vec.len(), 15);
    for i in 0..n_entry {
        let key_val = *sorted_vec.get(i).unwrap();
        runtime_data_lock
            .report_vec
            .push((key_val.0.clone(), key_val.1.clone()));
    }
}
