/// Used to express the search filters applied to GUI inspect page
#[derive(Clone, Debug, Default, Hash)]
pub struct SearchParameters {
    /// IP address (source)
    pub address_src: String,
    /// Transport port (source)
    pub port_src: String,
    /// IP address (destination)
    pub address_dst: String,
    /// Transport port (destination)
    pub port_dst: String,
    /// Protocol
    pub proto: String,
    /// Application protocol
    pub app_proto: String,
    /// Country
    pub country: String,
    /// Domain
    pub domain: String,
    /// Autonomous System name
    pub as_name: String,
    /// Whether to display only favorites
    pub only_favorites: bool,
}

impl SearchParameters {
    pub fn is_some_host_filter_active(&self) -> bool {
        self.only_favorites
            || !self.country.is_empty()
            || !self.as_name.is_empty()
            || !self.domain.is_empty()
    }

    pub fn reset_host_filters(&self) -> Self {
        Self {
            country: String::new(),
            domain: String::new(),
            as_name: String::new(),
            only_favorites: false,
            ..self.clone()
        }
    }
}

#[derive(Copy, Clone)]
pub enum FilterInputType {
    AddressSrc,
    PortSrc,
    AddressDst,
    PortDst,
    Proto,
    AppProto,
    Country,
    Domain,
    AsName,
}

impl FilterInputType {
    pub fn current_value(self, search_params: &SearchParameters) -> &str {
        match self {
            FilterInputType::AddressSrc => &search_params.address_src,
            FilterInputType::PortSrc => &search_params.port_src,
            FilterInputType::AddressDst => &search_params.address_dst,
            FilterInputType::PortDst => &search_params.port_dst,
            FilterInputType::Proto => &search_params.proto,
            FilterInputType::AppProto => &search_params.app_proto,
            FilterInputType::Country => &search_params.country,
            FilterInputType::Domain => &search_params.domain,
            FilterInputType::AsName => &search_params.as_name,
        }
    }

    pub fn clear_search(&self, search_params: &SearchParameters) -> SearchParameters {
        match self {
            FilterInputType::AddressSrc => SearchParameters {
                address_src: String::new(),
                ..search_params.clone()
            },
            FilterInputType::PortSrc => SearchParameters {
                port_src: String::new(),
                ..search_params.clone()
            },
            FilterInputType::AddressDst => SearchParameters {
                address_dst: String::new(),
                ..search_params.clone()
            },
            FilterInputType::PortDst => SearchParameters {
                port_dst: String::new(),
                ..search_params.clone()
            },
            FilterInputType::Proto => SearchParameters {
                proto: String::new(),
                ..search_params.clone()
            },
            FilterInputType::AppProto => SearchParameters {
                app_proto: String::new(),
                ..search_params.clone()
            },
            FilterInputType::Domain => SearchParameters {
                domain: String::new(),
                ..search_params.clone()
            },
            FilterInputType::Country => SearchParameters {
                country: String::new(),
                ..search_params.clone()
            },
            FilterInputType::AsName => SearchParameters {
                as_name: String::new(),
                ..search_params.clone()
            },
        }
    }

    pub fn new_search(
        &self,
        search_params: &SearchParameters,
        new_value: String,
    ) -> SearchParameters {
        match self {
            FilterInputType::AddressSrc => SearchParameters {
                address_src: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::PortSrc => SearchParameters {
                port_src: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::AddressDst => SearchParameters {
                address_dst: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::PortDst => SearchParameters {
                port_dst: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::Proto => SearchParameters {
                proto: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::AppProto => SearchParameters {
                app_proto: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::Domain => SearchParameters {
                domain: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::Country => SearchParameters {
                country: new_value.trim().to_string(),
                ..search_params.clone()
            },
            FilterInputType::AsName => SearchParameters {
                as_name: new_value,
                ..search_params.clone()
            },
        }
    }
}
