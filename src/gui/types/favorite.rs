use crate::countries::country_utils::get_flag_tooltip;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::conf::Conf;
use crate::gui::types::message::Message;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_fav::DataInfoFav;
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
use iced::widget::{Button, Container, Tooltip, button};
use iced::{Alignment, Element};
use std::cmp::min;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Favorite {
    Host,
    Service,
    Program,
}

impl Favorite {
    pub fn get_entries(
        self,
        info_traffic: &InfoTraffic,
        program_lookup: Option<&ProgramLookup>,
        data_repr: DataRepr,
        sort_type: SortType,
    ) -> Vec<FavoriteItem> {
        match self {
            Favorite::Host => get_host_entries(info_traffic, data_repr, sort_type),
            Favorite::Service => get_service_entries(info_traffic, data_repr, sort_type),
            Favorite::Program => get_program_entries(program_lookup, data_repr, sort_type),
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
}

#[derive(Clone)]
pub enum FavoriteItem {
    Host((Host, DataInfoHost)),
    Service((Service, DataInfoFav)),
    Program((Program, DataInfoFav)),
}

impl FavoriteItem {
    pub fn data_info(&self) -> DataInfo {
        match self {
            FavoriteItem::Host((_, data_info_host)) => data_info_host.data_info_fav.data_info,
            FavoriteItem::Service((_, data_info_fav)) => data_info_fav.data_info,
            FavoriteItem::Program((_, data_info_fav)) => data_info_fav.data_info,
        }
    }

    fn is_favorite(&self) -> bool {
        match self {
            FavoriteItem::Host((_, data_info_host)) => data_info_host.data_info_fav.is_favorite,
            FavoriteItem::Service((_, data_info_fav)) => data_info_fav.is_favorite,
            FavoriteItem::Program((_, data_info_fav)) => data_info_fav.is_favorite,
        }
    }

    pub fn star_button<'a>(&self) -> Button<'a, Message, StyleType> {
        let is_favorite = self.is_favorite();

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
        thumbnail: bool,
        program_lookup: Option<&'a ProgramLookup>,
    ) -> impl Into<Element<'a, Message, StyleType>> {
        match self {
            FavoriteItem::Host((host, data_info_host)) => Some(get_flag_tooltip(
                host.country,
                data_info_host,
                language,
                thumbnail,
            )),
            FavoriteItem::Service(_) => None::<Tooltip<Message, StyleType>>,
            FavoriteItem::Program((program, _)) => {
                let Some(program_lookup) = program_lookup else {
                    return None::<Tooltip<Message, StyleType>>;
                };
                Some(program_lookup.picon_tooltip(program.icon_key(), program.path()))
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

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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
) -> Vec<FavoriteItem> {
    let mut sorted_vec: Vec<(&Host, &DataInfoHost)> = info_traffic.hosts.iter().collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| {
        a.data_info_fav
            .data_info
            .compare(&b.data_info_fav.data_info, sort_type, data_repr)
    });

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
) -> Vec<FavoriteItem> {
    let mut sorted_vec: Vec<(&Service, &DataInfoFav)> = info_traffic
        .services
        .iter()
        .filter(|(service, _)| service != &&Service::NotApplicable)
        .collect();

    sorted_vec.sort_by(|&(_, a), &(_, b)| a.data_info.compare(&b.data_info, sort_type, data_repr));

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
) -> Vec<FavoriteItem> {
    let Some(program_lookup) = program_lookup else {
        return Vec::new();
    };

    let mut sorted_vec: Vec<(&Program, &DataInfoFav)> = program_lookup
        .programs()
        .iter()
        // Unknown may be inserted, and then all of its data could be reassigned to known programs
        .filter(|(_, d)| d.data_info.tot_data(DataRepr::Packets) > 0)
        .collect();

    sorted_vec.sort_by(|&(p1, a), &(p2, b)| {
        if sort_type == SortType::Neutral && a.data_info.is_within_same_second(&b.data_info) {
            if p1.is_unknown() {
                return std::cmp::Ordering::Greater;
            } else if p2.is_unknown() {
                return std::cmp::Ordering::Less;
            }
        }
        a.data_info.compare(&b.data_info, sort_type, data_repr)
    });

    let n_entry = min(sorted_vec.len(), 30);
    sorted_vec[0..n_entry]
        .iter()
        .map(|&(program, data_info)| FavoriteItem::Program((program.to_owned(), *data_info)))
        .collect()
}
