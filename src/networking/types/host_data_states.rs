use std::collections::BTreeSet;
use std::net::IpAddr;

use iced::widget::combo_box;

use crate::countries::types::country::Country;
use crate::networking::types::host::Host;
use crate::report::types::search_parameters::SearchParameters;

/// Struct to contain all the sets of data related to network hosts
///
/// It also stores combobox states for the host-related filters
#[derive(Default)]
pub struct HostDataStates {
    pub data: HostData,
    pub states: HostStates,
}

impl HostDataStates {
    pub fn update_states(&mut self, search: &SearchParameters) {
        let states = &mut self.states;
        let data = &mut self.data;

        if data.domains.1 {
            states.domains = combo_box::State::with_selection(
                data.domains.0.iter().cloned().collect(),
                Some(&search.domain),
            );
            data.domains.1 = false;
        }

        if data.asns.1 {
            states.asns = combo_box::State::with_selection(
                data.asns.0.iter().cloned().collect(),
                Some(&search.as_name),
            );
            data.asns.1 = false;
        }

        if data.countries.1 {
            states.countries = combo_box::State::with_selection(
                data.countries.0.iter().cloned().collect(),
                Some(&search.country),
            );
            data.countries.1 = false;
        }
    }
}

#[derive(Default)]
pub struct HostData {
    pub domains: (BTreeSet<String>, bool),
    pub asns: (BTreeSet<String>, bool),
    pub countries: (BTreeSet<String>, bool),
}

impl HostData {
    pub fn update(&mut self, host: &Host) {
        if !host.domain.is_empty() && host.domain.parse::<IpAddr>().is_err() {
            self.domains.1 = self.domains.0.insert(host.domain.clone()) || self.domains.1;
        }

        if !host.asn.name.is_empty() {
            self.asns.1 = self.asns.0.insert(host.asn.name.clone()) || self.asns.1;
        }

        if host.country != Country::ZZ {
            self.countries.1 =
                self.countries.0.insert(host.country.to_string()) || self.countries.1;
        }
    }
}

#[derive(Default)]
pub struct HostStates {
    pub domains: combo_box::State<String>,
    pub asns: combo_box::State<String>,
    pub countries: combo_box::State<String>,
}
