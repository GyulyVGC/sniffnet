use std::cmp::min;

use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::report::types::sort_type::SortType;
use crate::{InfoTraffic, Service, Sniffer};

/// Return the elements that satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints,
/// with their packets, in-bytes, and out-bytes count
pub fn get_searched_entries(
    sniffer: &Sniffer,
) -> (Vec<(AddressPortPair, InfoAddressPortPair)>, usize, DataInfo) {
    let mut agglomerate = DataInfo::default();
    let info_traffic = &sniffer.info_traffic;
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic
        .map
        .iter()
        .filter(|(key, value)| {
            let address_to_lookup = &get_address_to_lookup(key, value.traffic_direction);
            let r_dns_host = sniffer.addresses_resolved.get(address_to_lookup);
            let is_favorite = if let Some(e) = r_dns_host {
                info_traffic
                    .hosts
                    .get(&e.1)
                    .unwrap_or(&DataInfoHost::default())
                    .is_favorite
            } else {
                false
            };
            sniffer
                .search
                .match_entry(key, value, r_dns_host, is_favorite)
        })
        .map(|(key, val)| {
            agglomerate.add_packets(
                val.transmitted_packets,
                val.transmitted_bytes,
                val.traffic_direction,
            );
            (key, val)
        })
        .collect();

    all_results.sort_by(|&(_, a), &(_, b)| {
        a.compare(
            b,
            sniffer.conf.report_sort_type.data_sort,
            sniffer.traffic_chart.data_repr,
        )
    });

    let upper_bound = min(sniffer.page_number * 20, all_results.len());

    (
        all_results
            .get((sniffer.page_number.saturating_sub(1)) * 20..upper_bound)
            .unwrap_or_default()
            .iter()
            .map(|&(key, val)| (key.to_owned(), val.to_owned()))
            .collect(),
        all_results.len(),
        agglomerate,
    )
}

pub fn get_host_entries(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    sort_type: SortType,
) -> Vec<(Host, DataInfoHost)> {
    let mut sorted_vec: Vec<(&Host, &DataInfoHost)> = info_traffic.hosts.iter().collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.data_info.compare(&b.data_info, sort_type, data_repr));

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(host, data_info_host)| (host.to_owned(), data_info_host.to_owned()))
        .collect()
}

pub fn get_service_entries(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    sort_type: SortType,
) -> Vec<(Service, DataInfo)> {
    let mut sorted_vec: Vec<(&Service, &DataInfo)> = info_traffic
        .services
        .iter()
        .filter(|(service, _)| service != &&Service::NotApplicable)
        .collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.compare(b, sort_type, data_repr));

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(service, data_info)| (*service, *data_info))
        .collect()
}
