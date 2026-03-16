use crate::Sniffer;
use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use std::cmp::min;

/// Return the elements that satisfy the search constraints and belong to the given page,
/// and the total number of elements which satisfy the search constraints,
/// with their packets, in-bytes, and out-bytes count
pub fn get_searched_entries(
    sniffer: &Sniffer,
) -> (
    Vec<(&AddressPortPair, &InfoAddressPortPair)>,
    usize,
    DataInfo,
) {
    let mut agglomerate = DataInfo::default();
    let info_traffic = &sniffer.info_traffic;
    let favorites = &sniffer.conf.settings.favorites;
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic
        .map
        .iter()
        .filter(|(key, value)| {
            let address_to_lookup = &get_address_to_lookup(key, value.traffic_direction);
            let r_dns_host = sniffer.addresses_resolved.get(address_to_lookup);
            // is this a favorite host?
            let is_favorite_host = if let Some(e) = r_dns_host {
                favorites.contains_host(&e.1)
            } else {
                false
            };
            // is this a favorite service?
            let is_favorite_service = favorites.contains_service(&value.service);
            // is this a favorite program?
            let is_favorite_program = if sniffer.program_lookup.is_some() {
                favorites.contains_program(&value.program)
            } else {
                false
            };
            let is_favorite = is_favorite_host || is_favorite_service || is_favorite_program;
            sniffer
                .search
                .match_entry(key, value, r_dns_host, is_favorite)
        })
        .map(|(key, val)| {
            agglomerate.refresh(val.data_info());
            (key, val)
        })
        .collect();

    all_results.sort_by(|&(_, a), &(_, b)| {
        a.compare(b, sniffer.conf.report_sort_type, sniffer.conf.data_repr)
    });

    let upper_bound = min(sniffer.page_number * 30, all_results.len());

    (
        all_results
            .get((sniffer.page_number.saturating_sub(1)) * 30..upper_bound)
            .unwrap_or_default()
            .to_vec(),
        all_results.len(),
        agglomerate,
    )
}
