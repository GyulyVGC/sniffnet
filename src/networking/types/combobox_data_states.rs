use std::collections::BTreeSet;
use std::net::IpAddr;

use crate::countries::types::country::Country;
use crate::networking::types::host::Host;
use crate::report::types::search_parameters::SearchParameters;
use iced::widget::combo_box;
use listeners::Process;

/// Struct to contain all the sets of data related to network hosts and programs
///
/// It also stores combobox states for the host-related filters
#[derive(Default)]
pub struct ComboboxDataStates {
    pub data: ComboboxData,
    pub states: ComboboxStates,
}

impl ComboboxDataStates {
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

        if data.programs.1 {
            states.programs = combo_box::State::with_selection(
                data.programs.0.iter().cloned().collect(),
                Some(&search.program),
            );
            data.programs.1 = false;
        }
    }
}

#[derive(Default)]
pub struct ComboboxData {
    pub domains: (BTreeSet<String>, bool),
    pub asns: (BTreeSet<String>, bool),
    pub countries: (BTreeSet<String>, bool),
    pub programs: (BTreeSet<String>, bool),
}

impl ComboboxData {
    pub fn update_host(&mut self, host: &Host) {
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

    pub fn update_program(&mut self, program: Option<&Process>) {
        if let Some(program) = program
            && !program.name.is_empty()
        {
            self.programs.1 = self.programs.0.insert(program.name.clone()) || self.programs.1;
        }
    }
}

#[derive(Default)]
pub struct ComboboxStates {
    pub domains: combo_box::State<String>,
    pub asns: combo_box::State<String>,
    pub countries: combo_box::State<String>,
    pub programs: combo_box::State<String>,
}
