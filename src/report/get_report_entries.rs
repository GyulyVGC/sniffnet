use std::cmp::{min, Ordering};
use std::sync::{Arc, Mutex};

use iced::widget::Tooltip;

use crate::gui::types::message::Message;
use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::utils::countries::{get_flag_tooltip, FLAGS_WIDTH_SMALL};
use crate::{AppProtocol, ChartType, InfoTraffic, ReportSortType, Sniffer};

/// Returns the indexes of the elements which satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints
pub fn get_searched_entries(
    sniffer: &Sniffer,
) -> (
    Vec<(
        AddressPortPair,
        InfoAddressPortPair,
        Tooltip<'static, Message>,
    )>,
    usize,
) {
    let info_traffic = sniffer.info_traffic.clone();
    let search_parameters = &sniffer.search.clone();
    let report_sort_type = sniffer.report_sort_type;
    let page_number = sniffer.page_number;

    let info_traffic_lock = info_traffic.lock().unwrap();
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic_lock
        .map
        .iter()
        .filter(|(key, value)| {
            let mut boolean_flags = Vec::new();
            // retrieve host info
            let address_to_lookup = &get_address_to_lookup(key, value.traffic_direction);
            let r_dns_host = info_traffic_lock.addresses_resolved.get(address_to_lookup);
            // if a host-related filter is active and this address has not been resolved yet => false
            if r_dns_host.is_none()
                && (!search_parameters.domain.is_empty()
                    || !search_parameters.country.is_empty()
                    || !search_parameters.as_name.is_empty()
                    || search_parameters.only_favorites)
            {
                return false;
            }
            // check application protocol filter
            if !search_parameters.app.is_empty() {
                let app_str = format!("{:?}", value.app_protocol);
                boolean_flags.push(
                    app_str
                        .to_lowercase()
                        .eq(&search_parameters.app.to_lowercase()),
                );
            }
            // check domain filter
            if !search_parameters.domain.is_empty() {
                boolean_flags.push(
                    r_dns_host
                        .unwrap()
                        .0
                        .to_lowercase()
                        .contains(&search_parameters.domain.to_lowercase()),
                );
            }
            // check country filter
            if !search_parameters.country.is_empty() {
                boolean_flags.push(
                    r_dns_host
                        .unwrap()
                        .1
                        .country
                        .to_lowercase()
                        .starts_with(&search_parameters.country.to_lowercase()),
                );
            }
            // check Autonomous System name filter
            if !search_parameters.as_name.is_empty() {
                boolean_flags.push(
                    r_dns_host
                        .unwrap()
                        .1
                        .asn
                        .name
                        .to_lowercase()
                        .contains(&search_parameters.as_name.to_lowercase()),
                );
            }
            // check favorites filter
            if search_parameters.only_favorites {
                boolean_flags.push(
                    info_traffic_lock
                        .hosts
                        .get(&r_dns_host.unwrap().1)
                        .unwrap()
                        .is_favorite,
                );
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

    let upper_bound = min(page_number * 20, all_results.len());

    (
        all_results
            .get((page_number - 1) * 20..upper_bound)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|key_val| {
                let address_to_lookup =
                    get_address_to_lookup(key_val.0, key_val.1.traffic_direction);
                let host = info_traffic_lock
                    .addresses_resolved
                    .get(&address_to_lookup)
                    .unwrap_or(&Default::default())
                    .1
                    .clone();
                let default_host_info = &DataInfoHost::default();
                let host_info = info_traffic_lock
                    .hosts
                    .get(&host)
                    .unwrap_or(default_host_info);
                let flag = get_flag_tooltip(
                    &host.country,
                    FLAGS_WIDTH_SMALL,
                    host_info.is_local,
                    host_info.traffic_type,
                    sniffer.language,
                    sniffer.style,
                );
                (key_val.0.clone(), key_val.1.clone(), flag)
            })
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
