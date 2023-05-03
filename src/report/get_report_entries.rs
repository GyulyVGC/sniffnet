use std::cmp::{min, Ordering};
use std::sync::{Arc, Mutex};

use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::search_parameters::SearchParameters;
use crate::{AppProtocol, ChartType, InfoTraffic, ReportSortType};

/// Returns the indexes of the elements which satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints
pub fn get_searched_entries(
    info_traffic: &Arc<Mutex<InfoTraffic>>,
    search_parameters: &SearchParameters,
    report_sort_type: ReportSortType,
    page_number: usize,
) -> (Vec<(AddressPortPair, InfoAddressPortPair)>, usize) {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic_lock
        .map
        .iter()
        .filter(|(_key, value)| {
            let mut boolean_flags = Vec::new();
            // check application protocol filter
            if let Some(app) = &search_parameters.app {
                boolean_flags.push(value.app_protocol.eq(app));
            }
            // check domain filter
            if let Some(domain) = &search_parameters.domain {
                if !value.r_dns_already_resolved() {
                    return false;
                }
                boolean_flags.push(value.r_dns.as_ref().unwrap().ends_with(domain));
            }
            // check country filter
            if let Some(country) = &search_parameters.country {
                boolean_flags.push(value.country.eq(country));
            }
            // check Autonomous System name filter
            if let Some(as_name) = &search_parameters.as_name {
                boolean_flags.push(value.asn.name.eq(as_name));
            }

            if boolean_flags.is_empty() {
                return true;
            }
            return boolean_flags.iter().all(|flag| *flag);
        })
        .collect();
    all_results.sort_by(|&(_, a), &(_, b)| match report_sort_type {
        ReportSortType::MostRecent => b.final_timestamp.cmp(&a.final_timestamp),
        ReportSortType::MostBytes => b.transmitted_bytes.cmp(&a.transmitted_bytes),
        ReportSortType::MostPackets => b.transmitted_packets.cmp(&a.transmitted_packets),
    });

    let upper_bound = min(page_number * 10, all_results.len());

    (
        all_results
            .get((page_number - 1) * 10..upper_bound)
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
) -> Vec<(AppProtocol, DataInfo)> {
    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut sorted_vec: Vec<(&AppProtocol, &DataInfo)> =
        info_traffic_lock.app_protocols.iter().collect();

    sorted_vec.sort_by(|&(p1, a), &(p2, b)| {
        if p1.eq(&AppProtocol::Other) {
            Ordering::Greater
        } else if p2.eq(&AppProtocol::Other) {
            Ordering::Less
        } else {
            match chart_type {
                ChartType::Packets => b.tot_packets().cmp(&a.tot_packets()),
                ChartType::Bytes => b.tot_bytes().cmp(&a.tot_bytes()),
            }
        }
    });

    sorted_vec.iter().map(|e| (*e.0, e.1.clone())).collect()
}
