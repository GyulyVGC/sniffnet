use std::cmp::{min, Ordering};
use std::sync::{Arc, Mutex};

use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::report::types::sort_type::SortType;
use crate::{ChartType, InfoTraffic, ReportSortType, Service, Sniffer};

/// Returns the elements which satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints
pub fn get_searched_entries(
    sniffer: &Sniffer,
) -> (Vec<(AddressPortPair, InfoAddressPortPair)>, usize) {
    let info_traffic_lock = sniffer.info_traffic.lock().unwrap();
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic_lock
        .map
        .iter()
        .filter(|(key, value)| {
            let address_to_lookup = &get_address_to_lookup(key, value.traffic_direction);
            let r_dns_host = info_traffic_lock.addresses_resolved.get(address_to_lookup);
            let is_favorite = if let Some(e) = r_dns_host {
                info_traffic_lock.hosts.get(&e.1).unwrap().is_favorite
            } else {
                false
            };
            sniffer
                .search
                .match_entry(key, value, r_dns_host, is_favorite)
        })
        .collect();
    all_results.sort_by(|&(_, a), &(_, b)| match sniffer.report_sort_type {
        ReportSortType {
            byte_sort,
            packet_sort: SortType::Neutral,
        } => match byte_sort {
            SortType::Ascending => a.transmitted_bytes.cmp(&b.transmitted_bytes),
            SortType::Descending => b.transmitted_bytes.cmp(&a.transmitted_bytes),
            SortType::Neutral => b.final_timestamp.cmp(&a.final_timestamp),
        },
        ReportSortType {
            byte_sort: SortType::Neutral,
            packet_sort,
        } => match packet_sort {
            SortType::Ascending => a.transmitted_packets.cmp(&b.transmitted_packets),
            SortType::Descending => b.transmitted_packets.cmp(&a.transmitted_packets),
            SortType::Neutral => b.final_timestamp.cmp(&a.final_timestamp),
        },
        _ => b.final_timestamp.cmp(&a.final_timestamp),
    });

    let upper_bound = min(sniffer.page_number * 20, all_results.len());

    (
        all_results
            .get((sniffer.page_number - 1) * 20..upper_bound)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|key_val| (key_val.0.clone(), key_val.1.clone()))
            .collect(),
        all_results.len(),
    )
}

pub fn get_host_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    chart_type: ChartType,
) -> Vec<(Host, DataInfoHost)> {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut sorted_vec: Vec<(&Host, &DataInfoHost)> = info_traffic_lock.hosts.iter().collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| match chart_type {
        ChartType::Packets => b.data_info.tot_packets().cmp(&a.data_info.tot_packets()),
        ChartType::Bytes => b.data_info.tot_bytes().cmp(&a.data_info.tot_bytes()),
    });

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|e| (e.0.clone(), e.1.clone()))
        .collect()
}

pub fn get_app_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    chart_type: ChartType,
) -> Vec<(Service, DataInfo)> {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut sorted_vec: Vec<(&Service, &DataInfo)> = info_traffic_lock
        .app_protocols
        .iter()
        .filter(|(app_protocol, _)| app_protocol != &&Service::NotApplicable)
        .collect();

    sorted_vec.sort_by(|&(p1, a), &(p2, b)| {
        if p1 == &Service::Unknown {
            Ordering::Greater
        } else if p2 == &Service::Unknown {
            Ordering::Less
        } else {
            match chart_type {
                ChartType::Packets => b.tot_packets().cmp(&a.tot_packets()),
                ChartType::Bytes => b.tot_bytes().cmp(&a.tot_bytes()),
            }
        }
    });

    sorted_vec.iter().map(|e| (e.0.clone(), *e.1)).collect()
}
