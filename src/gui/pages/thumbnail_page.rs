use crate::configs::types::config_settings::ConfigSettings;
use crate::countries::country_utils::get_flag_tooltip;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::host::Host;
use crate::report::get_report_entries::{get_host_entries, get_service_entries};
use crate::report::types::sort_type::SortType;
use iced::widget::{Column, Container, Row, Rule, Text};
use iced::{Alignment, Font, Length};
use std::cmp::min;
use std::net::IpAddr;

/// Computes the body of the thumbnail view
pub fn thumbnail_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;

    let max_chars_host = 26;
    let max_chars_service = 13;

    let hosts = get_host_entries(
        &sniffer.info_traffic,
        sniffer.traffic_chart.chart_type,
        SortType::Neutral,
        true,
    );
    let mut host_col = Column::new()
        .padding([0, 5])
        .spacing(3)
        .width(Length::FillPortion(2));
    for host in hosts {
        let flag = get_flag_tooltip(host.0.country, &host.1, language, font, true);
        let host_row = Row::new()
            .align_items(Alignment::Center)
            .spacing(5)
            .push(flag)
            .push(host_text(host.0, font, max_chars_host));
        host_col = host_col.push(host_row);
    }

    let services = get_service_entries(
        &sniffer.info_traffic,
        sniffer.traffic_chart.chart_type,
        SortType::Neutral,
        true,
    );
    let mut service_col = Column::new().padding([0, 5]).spacing(3).width(Length::Fill);
    for service in services {
        service_col = service_col.push(
            Text::new(clip_text(service.0.to_string(), max_chars_service))
                .font(font)
                .size(FONT_SIZE_FOOTER),
        );
    }

    let host_service_row = Row::new()
        .padding([5, 0])
        .height(Length::Fill)
        .align_items(Alignment::Start)
        .push(host_col)
        .push(Rule::vertical(10))
        .push(service_col);

    let content = Column::new()
        .push(Container::new(sniffer.traffic_chart.view()).height(Length::Fill))
        .push(host_service_row);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
}

fn host_text(host: Host, font: Font, max_chars: usize) -> Text<'static, StyleType> {
    let domain = host.domain;
    let asn = host.asn.name;

    let text = if asn.is_empty() || (!domain.is_empty() && domain.parse::<IpAddr>().is_err()) {
        domain
    } else {
        asn
    };

    Text::new(clip_text(text, max_chars))
        .font(font)
        .size(FONT_SIZE_FOOTER)
}

fn clip_text(text: String, max_chars: usize) -> String {
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

    [&slice, suspensions].concat()
}
