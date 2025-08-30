use std::cmp::min;
use std::net::IpAddr;

use iced::widget::{Column, Container, Row, Rule, Space, Text, vertical_space};
use iced::{Alignment, Font, Length};

use crate::chart::types::donut_chart::donut_chart;
use crate::countries::country_utils::get_flag_tooltip;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::{Host, ThumbnailHost};
use crate::networking::types::info_traffic::InfoTraffic;
use crate::report::get_report_entries::{get_host_entries, get_service_entries};
use crate::report::types::sort_type::SortType;
use crate::translations::types::language::Language;

const MAX_ENTRIES: usize = 4;
const MAX_CHARS_HOST: usize = 26;
const MAX_CHARS_SERVICE: usize = 13;

/// Computes the body of the thumbnail view
pub fn thumbnail_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings { style, .. } = sniffer.conf.settings;
    let font = style.get_extension().font;

    let tot_packets = sniffer
        .info_traffic
        .tot_data_info
        .tot_data(DataRepr::Packets);

    if tot_packets == 0 {
        return Container::new(
            Column::new()
                .push(vertical_space())
                .push(Text::new(&sniffer.dots_pulse.0).font(font).size(50))
                .push(Space::with_height(Length::FillPortion(2))),
        )
        .width(Length::Fill)
        .align_x(Alignment::Center);
    }

    let info_traffic = &sniffer.info_traffic;
    let data_repr = sniffer.traffic_chart.data_repr;

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
            font,
            sniffer.thumbnail,
        ))
        // .push(Rule::vertical(10))
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
            font,
            sniffer.conf.host_sort_type,
        ))
        .push(Rule::vertical(10))
        .push(service_col(
            info_traffic,
            data_repr,
            font,
            sniffer.conf.service_sort_type,
        ));

    let content = Column::new()
        .push(charts)
        // .push(Container::new(Rule::horizontal(10)).padding([0, 5]))
        .push(report);

    Container::new(content)
}

fn host_col<'a>(
    info_traffic: &InfoTraffic,
    data_repr: DataRepr,
    font: Font,
    sort_type: SortType,
) -> Column<'a, Message, StyleType> {
    let mut host_col = Column::new()
        .padding([0, 5])
        .spacing(3)
        .width(Length::FillPortion(2));
    let hosts = get_host_entries(info_traffic, data_repr, sort_type);
    let mut thumbnail_hosts = Vec::new();

    for (host, data_info_host) in &hosts {
        let text = host_text(host);
        let country = host.country;
        let thumbnail_host = ThumbnailHost {
            country,
            text: text.clone(),
        };

        if thumbnail_hosts.contains(&thumbnail_host) {
            continue;
        }

        thumbnail_hosts.push(thumbnail_host);

        let flag = get_flag_tooltip(country, data_info_host, Language::default(), font, true);
        let host_row = Row::new()
            .align_y(Alignment::Center)
            .spacing(5)
            .push(flag)
            .push(Text::new(text).font(font).size(FONT_SIZE_FOOTER));
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
    font: Font,
    sort_type: SortType,
) -> Column<'a, Message, StyleType> {
    let mut service_col = Column::new().padding([0, 5]).spacing(3).width(Length::Fill);
    let services = get_service_entries(info_traffic, data_repr, sort_type);
    let n_entry = min(services.len(), MAX_ENTRIES);
    for (service, _) in services.get(..n_entry).unwrap_or_default() {
        service_col = service_col.push(
            Text::new(clip_text(&service.to_string(), MAX_CHARS_SERVICE))
                .font(font)
                .size(FONT_SIZE_FOOTER),
        );
    }
    service_col
}

fn host_text(host: &Host) -> String {
    let domain = &host.domain;
    let asn = &host.asn.name;

    let text = if asn.is_empty() || (!domain.trim().is_empty() && domain.parse::<IpAddr>().is_err())
    {
        domain
    } else {
        asn
    };

    clip_text(text, MAX_CHARS_HOST)
}

fn clip_text(text: &str, max_chars: usize) -> String {
    let text = text.trim();
    let chars = text.chars().collect::<Vec<char>>();
    let tot_len = chars.len();
    let slice_len = min(max_chars, tot_len);

    let suspensions = if tot_len > max_chars { "…" } else { "" };
    let slice = if tot_len > max_chars {
        &chars[..slice_len - 2]
    } else {
        &chars[..slice_len]
    }
    .iter()
    .collect::<String>();

    [slice.trim(), suspensions].concat()
}

#[cfg(test)]
mod tests {
    use crate::gui::pages::thumbnail_page::{
        MAX_CHARS_HOST, MAX_CHARS_SERVICE, clip_text, host_text,
    };
    use crate::networking::types::asn::Asn;
    use crate::networking::types::host::Host;

    fn host_for_tests(domain: &str, asn: &str) -> Host {
        Host {
            domain: domain.to_string(),
            asn: Asn {
                name: asn.to_string(),
                code: "512".to_string(),
            },
            country: Default::default(),
        }
    }

    #[test]
    fn test_clip_text() {
        assert_eq!(
            clip_text("iphone-di-doofenshmirtz.local", MAX_CHARS_HOST),
            "iphone-di-doofenshmirtz.…"
        );
        assert_eq!(clip_text("github.com", MAX_CHARS_HOST), "github.com");

        assert_eq!(clip_text("https6789012", MAX_CHARS_SERVICE), "https6789012");
        assert_eq!(
            clip_text("https67890123", MAX_CHARS_SERVICE),
            "https67890123"
        );
        assert_eq!(
            clip_text("https678901234", MAX_CHARS_SERVICE),
            "https678901…"
        );
        assert_eq!(
            clip_text("https6789012345", MAX_CHARS_SERVICE),
            "https678901…"
        );

        assert_eq!(
            clip_text("protocol with space", MAX_CHARS_SERVICE),
            "protocol wi…"
        );
        assert_eq!(
            clip_text("protocol90 23456", MAX_CHARS_SERVICE),
            "protocol90…"
        );

        assert_eq!(
            clip_text("      \n\t    sniffnet.net       ", MAX_CHARS_HOST),
            "sniffnet.net"
        );
        assert_eq!(
            clip_text("        protocol90 23456    \n      ", MAX_CHARS_SERVICE),
            "protocol90…"
        );
        assert_eq!(
            clip_text("        protocol90 23456          ", MAX_CHARS_HOST),
            "protocol90 23456"
        );
    }

    #[test]
    fn test_host_text() {
        let host = host_for_tests("iphone-di-doofenshmirtz.local", "AS1234");
        assert_eq!(host_text(&host), "iphone-di-doofenshmirtz.…");

        let host = host_for_tests("", "");
        assert_eq!(host_text(&host), "");

        let host = host_for_tests("192.168.1.113", "AS1234");
        assert_eq!(host_text(&host), "AS1234");

        let host = host_for_tests("192.168.1.113", "");
        assert_eq!(host_text(&host), "192.168.1.113");

        let host = host_for_tests("", "FASTLY");
        assert_eq!(host_text(&host), "FASTLY");

        let host = host_for_tests("::", "GOOGLE");
        assert_eq!(host_text(&host), "GOOGLE");

        let host = host_for_tests("::f", "AKAMAI-TECHNOLOGIES-INCORPORATED");
        assert_eq!(host_text(&host), "AKAMAI-TECHNOLOGIES-INCO…");

        let host = host_for_tests("::g", "GOOGLE");
        assert_eq!(host_text(&host), "::g");

        let host = host_for_tests(" ", "GOOGLE");
        assert_eq!(host_text(&host), "GOOGLE");
    }
}
