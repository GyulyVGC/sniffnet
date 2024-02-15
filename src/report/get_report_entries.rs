use std::cmp::min;
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
            .map(|&(key, val)| (key.to_owned(), val.to_owned()))
            .collect(),
        all_results.len(),
    )
}

pub fn get_host_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    chart_type: ChartType,
    sort_type: SortType,
) -> Vec<(Host, DataInfoHost)> {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut sorted_vec: Vec<(&Host, &DataInfoHost)> = info_traffic_lock.hosts.iter().collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.data_info.compare(&b.data_info, sort_type, chart_type));

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(host, data_info_host)| (host.to_owned(), data_info_host.to_owned()))
        .collect()
}

pub fn get_service_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    chart_type: ChartType,
    sort_type: SortType,
) -> Vec<(Service, DataInfo)> {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut sorted_vec: Vec<(&Service, &DataInfo)> = info_traffic_lock
        .services
        .iter()
        .filter(|(service, _)| service != &&Service::NotApplicable)
        .collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.compare(b, sort_type, chart_type));

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(service, data_info)| (*service, *data_info))
        .collect()
}
