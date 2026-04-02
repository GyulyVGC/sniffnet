use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_pictures::ICONS_SIZE_BIG;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::conf::Conf;
use crate::gui::types::conf::deserialize_or_default;
use crate::gui::types::message::Message;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::networking::types::program::Program;
use crate::networking::types::program_lookup::ProgramLookup;
use crate::networking::types::service::Service;
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::translations::translations_2::host_translation;
use crate::translations::translations_3::service_translation;
use crate::translations::translations_5::program_translation;
use crate::translations::types::language::Language;
use crate::utils::types::icon::Icon;
use iced::widget::{Button, Container, Space, button};
use iced::{Alignment, Element};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::HashSet;

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Favorites {
    #[serde(deserialize_with = "deserialize_or_default")]
    hosts: HashSet<Host>,
    #[serde(deserialize_with = "deserialize_or_default")]
    services: HashSet<Service>,
    #[serde(deserialize_with = "deserialize_or_default")]
    programs: HashSet<Program>,
}

impl Favorites {
    pub fn contains_host(&self, host: &Host) -> bool {
        self.hosts.contains(host)
    }

    pub fn contains_service(&self, service: &Service) -> bool {
        self.services.contains(service)
    }

    pub fn contains_program(&self, program: &Program) -> bool {
        self.programs.contains(program)
    }

    pub fn insert(&mut self, key: &FavoriteKey) {
        match key {
            FavoriteKey::Host(h) => {
                self.hosts.insert(h.clone());
            }
            FavoriteKey::Service(s) => {
                self.services.insert(*s);
            }
            FavoriteKey::Program(p) => {
                self.programs.insert(p.clone());
            }
        }
    }

    pub fn remove(&mut self, key: &FavoriteKey) {
        match key {
            FavoriteKey::Host(h) => {
                self.hosts.remove(h);
            }
            FavoriteKey::Service(s) => {
                self.services.remove(s);
            }
            FavoriteKey::Program(p) => {
                self.programs.remove(p);
            }
        }
    }

    pub fn hosts(&self) -> &HashSet<Host> {
        &self.hosts
    }

    pub fn services(&self) -> &HashSet<Service> {
        &self.services
    }

    pub fn programs(&self) -> &HashSet<Program> {
        &self.programs
    }
}

#[cfg(test)]
impl<const N: usize> From<[FavoriteKey; N]> for Favorites {
    fn from(keys: [FavoriteKey; N]) -> Self {
        let mut favorites = Favorites::default();
        for key in keys {
            favorites.insert(&key);
        }
        favorites
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Favorite {
    Host,
    Service,
    Program,
}

impl Favorite {
    pub fn get_entries(self, sniffer: &Sniffer) -> Vec<FavoriteItem> {
        let info_traffic = &sniffer.info_traffic;
        let conf = &sniffer.conf;
        let data_repr = conf.data_repr;
        let program_lookup = sniffer.program_lookup.as_ref();

        match self {
            Favorite::Host => get_host_entries(
                info_traffic,
                data_repr,
                conf.host_sort_type,
                conf.host_favorites_filter,
                &conf.favorites.hosts,
            ),
            Favorite::Service => get_service_entries(
                info_traffic,
                data_repr,
                conf.service_sort_type,
                conf.service_favorites_filter,
                &conf.favorites.services,
            ),
            Favorite::Program => get_program_entries(
                program_lookup,
                data_repr,
                conf.program_sort_type,
                conf.program_favorites_filter,
                &conf.favorites.programs,
            ),
        }
    }

    pub fn fill_portion(self) -> iced::Length {
        match self {
            Favorite::Host => iced::Length::FillPortion(2),
            Favorite::Service | Favorite::Program => iced::Length::FillPortion(1),
        }
    }

    pub fn title(self, language: Language) -> &'static str {
        match self {
            Favorite::Host => host_translation(language),
            Favorite::Service => service_translation(language),
            Favorite::Program => program_translation(language),
        }
    }

    pub fn sort_arrows<'a>(self, conf: &Conf) -> Container<'a, Message, StyleType> {
        let active_sort_type = match self {
            Favorite::Host => conf.host_sort_type,
            Favorite::Service => conf.service_sort_type,
            Favorite::Program => conf.program_sort_type,
        };

        let message = match self {
            Favorite::Host => Message::HostSortSelection,
            Favorite::Service => Message::ServiceSortSelection,
            Favorite::Program => Message::ProgramSortSelection,
        };

        Container::new(
            button(
                active_sort_type
                    .icon()
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .class(active_sort_type.button_type())
            .on_press(message(active_sort_type.next_sort())),
        )
        .width(50.0)
        .align_x(Alignment::Center)
    }

    pub fn star_filter_button<'a>(self, conf: &Conf) -> Button<'a, Message, StyleType> {
        let message = match self {
            Favorite::Host => Message::HostFavoritesFilterToggle,
            Favorite::Service => Message::ServiceFavoritesFilterToggle,
            Favorite::Program => Message::ProgramFavoritesFilterToggle,
        };

        let is_active = match self {
            Favorite::Host => conf.host_favorites_filter,
            Favorite::Service => conf.service_favorites_filter,
            Favorite::Program => conf.program_favorites_filter,
        };

        let (icon, class) = if is_active {
            (Icon::StarFull, ButtonType::SortArrowActive)
        } else {
            (Icon::Funnel, ButtonType::SortArrows)
        };

        button(
            icon.to_text()
                .size(16)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .padding(0)
        .height(25)
        .width(25)
        .class(class)
        .on_press(message)
    }
}

#[derive(Clone)]
pub enum FavoriteItem {
    Host((Host, DataInfoHost)),
    Service((Service, DataInfo)),
    Program((Program, DataInfo)),
}

impl FavoriteItem {
    pub fn data_info(&self) -> DataInfo {
        match self {
            FavoriteItem::Host((_, data_info_host)) => data_info_host.data_info,
            FavoriteItem::Service((_, data_info)) | FavoriteItem::Program((_, data_info)) => {
                *data_info
            }
        }
    }

    pub fn star_button<'a>(&self, favorites: &Favorites) -> Button<'a, Message, StyleType> {
        let is_favorite = match self {
            FavoriteItem::Host((h, _)) => favorites.contains_host(h),
            FavoriteItem::Service((s, _)) => favorites.contains_service(s),
            FavoriteItem::Program((p, _)) => favorites.contains_program(p),
        };

        let (icon, class) = if is_favorite {
            (Icon::StarFull, ButtonType::Starred)
        } else {
            (Icon::StarEmpty, ButtonType::NotStarred)
        };

        button(
            icon.to_text()
                .size(16)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .padding(0)
        .height(25)
        .width(25)
        .class(class)
        .on_press(Message::AddOrRemoveFavorite(
            self.clone().into(),
            !is_favorite,
        ))
    }

    pub fn icon<'a>(
        &self,
        language: Language,
        program_lookup: Option<&'a ProgramLookup>,
        fill_empty: bool,
    ) -> impl Into<Element<'a, Message, StyleType>> {
        match self {
            FavoriteItem::Host((host, data_info_host)) => {
                Some(get_flag_tooltip(host.country, data_info_host, language, false).into())
            }
            FavoriteItem::Service(_) => {
                if fill_empty {
                    Some(Space::new().width(ICONS_SIZE_BIG).into())
                } else {
                    None::<Element<Message, StyleType>>
                }
            }
            FavoriteItem::Program((program, _)) => {
                let program_lookup = program_lookup?;
                Some(
                    program_lookup
                        .picon_tooltip(program.icon_key(), program.path())
                        .into(),
                )
            }
        }
    }

    pub fn to_entry_string(&self) -> String {
        match self {
            FavoriteItem::Host((host, _)) => host.to_entry_string(),
            FavoriteItem::Service((service, _)) => service.to_string(),
            FavoriteItem::Program((program, _)) => program.to_string(),
        }
    }

    pub fn new_entry_search(&self) -> SearchParameters {
        match self {
            FavoriteItem::Host((host, _)) => SearchParameters::new_host_search(host),
            FavoriteItem::Service((service, _)) => SearchParameters::new_service_search(service),
            FavoriteItem::Program((program, _)) => SearchParameters::new_program_search(program),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FavoriteKey {
    Host(Host),
    Service(Service),
    Program(Program),
}

impl From<FavoriteItem> for FavoriteKey {
    fn from(item: FavoriteItem) -> Self {
        match item {
            FavoriteItem::Host((h, _)) => FavoriteKey::Host(h),
            FavoriteItem::Service((s, _)) => FavoriteKey::Service(s),
            FavoriteItem::Program((p, _)) => FavoriteKey::Program(p),
        }
    }
}

// get entries helpers -----------------------------------------------------------------------------

fn get_host_entries(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    sort_type: SortType,
    favorites_filter: bool,
    favorites: &HashSet<Host>,
) -> Vec<FavoriteItem> {
    let default_data_info_host = DataInfoHost::default();
    let mut sorted_vec: Vec<(&Host, &DataInfoHost)> = if favorites_filter {
        favorites
            .iter()
            .map(|host| {
                let data_info_host = info_traffic
                    .hosts
                    .get(host)
                    .unwrap_or(&default_data_info_host);
                (host, data_info_host)
            })
            .collect()
    } else {
        info_traffic.hosts.iter().collect()
    };

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.data_info.compare(&b.data_info, sort_type, data_repr));

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(host, data_info_host)| {
            FavoriteItem::Host((host.to_owned(), data_info_host.to_owned()))
        })
        .collect()
}

fn get_service_entries(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    sort_type: SortType,
    favorites_filter: bool,
    favorites: &HashSet<Service>,
) -> Vec<FavoriteItem> {
    let default_data_info = DataInfo::default();
    let mut sorted_vec: Vec<(&Service, &DataInfo)> = if favorites_filter {
        favorites
            .iter()
            .map(|service| {
                let data_info_host = info_traffic
                    .services
                    .get(service)
                    .unwrap_or(&default_data_info);
                (service, data_info_host)
            })
            .collect()
    } else {
        info_traffic
            .services
            .iter()
            .filter(|(service, _)| service != &&Service::NotApplicable)
            .collect()
    };

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.compare(b, sort_type, data_repr));

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(service, data_info_fav)| FavoriteItem::Service((*service, *data_info_fav)))
        .collect()
}

fn get_program_entries(
    program_lookup: Option<&ProgramLookup>,
    data_repr: DataRepr,
    sort_type: SortType,
    favorites_filter: bool,
    favorites: &HashSet<Program>,
) -> Vec<FavoriteItem> {
    let Some(program_lookup) = program_lookup else {
        return Vec::new();
    };

    let default_data_info = DataInfo::default();
    let mut sorted_vec: Vec<(&Program, &DataInfo)> = if favorites_filter {
        favorites
            .iter()
            .map(|program| {
                let data_info_host = program_lookup
                    .programs()
                    .get(program)
                    .unwrap_or(&default_data_info);
                (program, data_info_host)
            })
            .collect()
    } else {
        program_lookup
            .programs()
            .iter()
            // Unknown may be inserted, and then all of its data could be reassigned to known programs
            .filter(|(_, d)| d.tot_data(DataRepr::Packets) > 0)
            .collect()
    };

    sorted_vec.sort_by(|&(p1, a), &(p2, b)| {
        if sort_type == SortType::Neutral && a.is_within_same_second(b) {
            if p1.is_unknown() {
                return std::cmp::Ordering::Greater;
            } else if p2.is_unknown() {
                return std::cmp::Ordering::Less;
            }
        }
        a.compare(b, sort_type, data_repr)
    });

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(program, data_info)| FavoriteItem::Program((program.to_owned(), *data_info)))
        .collect()
}
