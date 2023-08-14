use std::cmp::{min, Ordering};
use std::sync::{Arc, Mutex};

use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_pictures::FLAGS_WIDTH_SMALL;
use crate::gui::styles::style_constants::get_font;
use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::report::types::report_entry::ReportEntry;
use crate::{AppProtocol, ChartType, InfoTraffic, ReportSortType, Sniffer};

/// Returns the elements which satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints
pub fn get_searched_entries(sniffer: &Sniffer) -> (Vec<ReportEntry>, usize) {
    let info_traffic_lock = sniffer.info_traffic.lock().unwrap();
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic_lock
        .map
        .iter()
        .filter(|(key, value)| {
            let address_to_lookup = &get_address_to_lookup(key, value.traffic_direction);
            let r_dns_host = info_traffic_lock.addresses_resolved.get(address_to_lookup);

            let searched_domain = &*sniffer.search.domain.to_lowercase();
            let searched_country = &*sniffer.search.country.to_lowercase();
            let searched_as_name = &*sniffer.search.as_name.to_lowercase();
            let searched_only_fav = sniffer.search.only_favorites;
            // if a host-related filter is active and this address has not been resolved yet => false
            if r_dns_host.is_none()
                && (!searched_domain.is_empty()
                    || !searched_country.is_empty()
                    || !searched_as_name.is_empty()
                    || searched_only_fav)
            {
                return false;
            }
            // check application protocol filter
            let searched_app = &*sniffer.search.app.to_lowercase();
            let app = format!("{:?}", value.app_protocol).to_lowercase();
            if !searched_app.is_empty() && app.ne(searched_app) {
                return false;
            }
            // check domain filter
            if !searched_domain.is_empty() {
                let domain = r_dns_host.unwrap().0.to_lowercase();
                if !domain.contains(searched_domain) {
                    return false;
                }
            }
            // check country filter
            if !searched_country.is_empty() {
                let country = r_dns_host.unwrap().1.country.to_string().to_lowercase();
                if !country.starts_with(searched_country) {
                    return false;
                }
            }
            // check Autonomous System name filter
            if !searched_as_name.is_empty() {
                let asn_name = r_dns_host.unwrap().1.asn.name.to_lowercase();
                if !asn_name.contains(searched_as_name) {
                    return false;
                }
            }
            // check favorites filter
            if searched_only_fav
                && !info_traffic_lock
                    .hosts
                    .get(&r_dns_host.unwrap().1)
                    .unwrap()
                    .is_favorite
            {
                return false;
            }
            // if arrived at this point all filters are satisfied => return true
            true
        })
        .collect();
    all_results.sort_by(|&(_, a), &(_, b)| match sniffer.report_sort_type {
        ReportSortType::MostRecent => b.final_timestamp.cmp(&a.final_timestamp),
        ReportSortType::MostBytes => b.transmitted_bytes.cmp(&a.transmitted_bytes),
        ReportSortType::MostPackets => b.transmitted_packets.cmp(&a.transmitted_packets),
    });

    let upper_bound = min(sniffer.page_number * 20, all_results.len());

    (
        all_results
            .get((sniffer.page_number - 1) * 20..upper_bound)
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
                    host.country,
                    FLAGS_WIDTH_SMALL,
                    host_info.is_local,
                    host_info.traffic_type,
                    sniffer.language,
                    get_font(sniffer.style),
                );
                ReportEntry {
                    key: key_val.0.clone(),
                    val: key_val.1.clone(),
                    tooltip: flag,
                }
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

    sorted_vec.iter().map(|e| (*e.0, *e.1)).collect()
}
