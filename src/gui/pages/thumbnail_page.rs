use std::cmp::min;

use iced::widget::{Column, Container, Row, Space, Text};
use iced::{Alignment, Length};

use crate::chart::types::donut_chart::donut_chart;
use crate::countries::country_utils::get_flag_tooltip;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::ThumbnailHost;
use crate::networking::types::info_traffic::InfoTraffic;
use crate::report::get_report_entries::{get_host_entries, get_service_entries};
use crate::report::types::sort_type::SortType;
use crate::translations::types::language::Language;
use crate::utils::formatted_strings::clip_text;

const MAX_ENTRIES: usize = 4;
const MAX_CHARS_HOST: usize = 26;
const MAX_CHARS_SERVICE: usize = 13;

/// Computes the body of the thumbnail view
pub fn thumbnail_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let tot_packets = sniffer
        .info_traffic
        .tot_data_info
        .tot_data(DataRepr::Packets);

    if tot_packets == 0 {
        return Container::new(
            Column::new()
                .push(Space::new().height(Length::Fill))
                .push(Text::new(&sniffer.dots_pulse.0).size(50))
                .push(Space::new().height(Length::FillPortion(2))),
        )
        .width(Length::Fill)
        .align_x(Alignment::Center);
    }

    let info_traffic = &sniffer.info_traffic;
    let data_repr = sniffer.conf.data_repr;

    let (in_data, out_data, dropped) = info_traffic.get_thumbnail_data(data_repr);

    let charts = Row::new()
        .padding(5)
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .push(donut_chart(
            data_repr,
            in_data,
            out_data,
            dropped,
            sniffer.thumbnail,
        ))
        .push(
            Container::new(sniffer.traffic_chart.view())
                .height(Length::Fill)
                .width(Length::FillPortion(2)),
        );

    let report = Row::new()
        .padding([5, 0])
        .height(Length::Fill)
        .align_y(Alignment::Start)
        .push(host_col(
            info_traffic,
            data_repr,
            sniffer.conf.host_sort_type,
        ))
        .push(RuleType::Standard.vertical(10))
        .push(service_col(
            info_traffic,
            data_repr,
            sniffer.conf.service_sort_type,
        ));

    let content = Column::new().push(charts).push(report);

    Container::new(content)
}

fn host_col<'a>(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    sort_type: SortType,
) -> Column<'a, Message, StyleType> {
    let mut host_col = Column::new()
        .padding([0, 5])
        .spacing(3)
        .width(Length::FillPortion(2));
    let hosts = get_host_entries(info_traffic, data_repr, sort_type);
    let mut thumbnail_hosts = Vec::new();

    for (host, data_info_host) in &hosts {
        let thumbnail_host = ThumbnailHost::from_host(host, MAX_CHARS_HOST);
        let country = thumbnail_host.country;
        let text = thumbnail_host.text.clone();

        if thumbnail_hosts.contains(&thumbnail_host) {
            continue;
        }

        thumbnail_hosts.push(thumbnail_host);

        let flag = get_flag_tooltip(country, data_info_host, Language::default(), true);
        let host_row = Row::new()
            .align_y(Alignment::Center)
            .spacing(5)
            .push(flag)
            .push(Text::new(text).size(FONT_SIZE_FOOTER));
        host_col = host_col.push(host_row);

        if thumbnail_hosts.len() >= MAX_ENTRIES {
            break;
        }
    }

    host_col
}

fn service_col<'a>(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    sort_type: SortType,
) -> Column<'a, Message, StyleType> {
    let mut service_col = Column::new().padding([0, 5]).spacing(3).width(Length::Fill);
    let services = get_service_entries(info_traffic, data_repr, sort_type);
    let n_entry = min(services.len(), MAX_ENTRIES);
    for (service, _) in services.get(..n_entry).unwrap_or_default() {
        service_col = service_col.push(
            Text::new(clip_text(&service.to_string(), MAX_CHARS_SERVICE)).size(FONT_SIZE_FOOTER),
        );
    }
    service_col
}
