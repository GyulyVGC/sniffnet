use crate::Sniffer;
use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::report::types::REPORT_ENTRIES_PER_PAGE;
use std::cmp::min;

pub fn paginate<T: Clone>(
    all_results: &[T],
    page_number: usize,
) -> (Vec<T>, usize) {
    let upper_bound = min(page_number * REPORT_ENTRIES_PER_PAGE, all_results.len());
    let page = all_results
        .get((page_number.saturating_sub(1)) * REPORT_ENTRIES_PER_PAGE..upper_bound)
        .unwrap_or_default()
        .to_vec();
    (page, all_results.len())
}

pub fn get_searched_entries(
    sniffer: &Sniffer,
) -> (
    Vec<(&AddressPortPair, &InfoAddressPortPair)>,
    usize,
    DataInfo,
) {
    let mut agglomerate = DataInfo::default();
    let info_traffic = &sniffer.info_traffic;
    let favorites = &sniffer.conf.favorites;
    let mut all_results: Vec<(&AddressPortPair, &InfoAddressPortPair)> = info_traffic
        .map
        .iter()
        .filter(|(key, value)| {
            let address_to_lookup = &get_address_to_lookup(key, value.traffic_direction);
            let r_dns_host = sniffer.addresses_resolved.get(address_to_lookup);
            let is_favorite_host = if let Some(e) = r_dns_host {
                favorites.contains_host(&e.1)
            } else {
                false
            };
            let is_favorite_service = favorites.contains_service(&value.service);
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

    let (page, total) = paginate(&all_results, sniffer.page_number);

    (page, total, agglomerate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paginate_first_page_full() {
        let data: Vec<i32> = (0..75).collect();
        let (page, total) = paginate(&data, 1);
        assert_eq!(page.len(), REPORT_ENTRIES_PER_PAGE);
        assert_eq!(total, 75);
        assert_eq!(page[0], 0);
        assert_eq!(page[REPORT_ENTRIES_PER_PAGE - 1], REPORT_ENTRIES_PER_PAGE as i32 - 1);
    }

    #[test]
    fn test_paginate_last_page_partial() {
        let data: Vec<i32> = (0..75).collect();
        let (page, total) = paginate(&data, 3);
        assert_eq!(page.len(), 15);
        assert_eq!(total, 75);
        assert_eq!(page[0], 60);
        assert_eq!(page[14], 74);
    }

    #[test]
    fn test_paginate_page_zero_returns_empty() {
        let data: Vec<i32> = (0..10).collect();
        let (page, total) = paginate(&data, 0);
        assert!(page.is_empty());
        assert_eq!(total, 10);
    }

    #[test]
    fn test_paginate_beyond_last_page_returns_empty() {
        let data: Vec<i32> = (0..30).collect();
        let (page, total) = paginate(&data, 100);
        assert!(page.is_empty());
        assert_eq!(total, 30);
    }

    #[test]
    fn test_paginate_empty_data() {
        let data: Vec<i32> = vec![];
        let (page, total) = paginate(&data, 1);
        assert!(page.is_empty());
        assert_eq!(total, 0);
    }

    #[test]
    fn test_paginate_exact_multiple() {
        let data: Vec<i32> = (0..60).collect();
        let (page, total) = paginate(&data, 2);
        assert_eq!(page.len(), REPORT_ENTRIES_PER_PAGE);
        assert_eq!(total, 60);
        assert_eq!(page[0], 30);
        assert_eq!(page[REPORT_ENTRIES_PER_PAGE - 1], 59);
    }

    #[test]
    fn test_paginate_single_element() {
        let data = vec![42];
        let (page, total) = paginate(&data, 1);
        assert_eq!(page, vec![42]);
        assert_eq!(total, 1);
    }

    #[test]
    fn test_paginate_page_past_end_with_partial_last() {
        let data: Vec<i32> = (0..45).collect();
        let (page, total) = paginate(&data, 3);
        assert!(page.is_empty());
        assert_eq!(total, 45);
    }
}
