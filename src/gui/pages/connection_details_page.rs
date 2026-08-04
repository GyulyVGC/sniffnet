use std::net::IpAddr;

use crate::countries::country_utils::{get_computer_tooltip, get_flag_tooltip};
use crate::gui::components::button::button_hide;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_TITLE, TOOLTIP_DELAY};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::gui::types::timing_events::TimingEvents;
use crate::networking::manage_packets::{
    get_address_to_lookup, get_traffic_type, is_local_connection, is_my_address,
};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::arp_type::ArpType;
use crate::networking::types::bogon::is_bogon;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::icmp_type::IcmpType;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::latency::LatencyStatus;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::networking::types::traffic_type::TrafficType;
use crate::translations::translations::{
    address_translation, incoming_translation, outgoing_translation, packets_translation,
    protocol_translation,
};
use crate::translations::translations_2::{
    connection_details_translation, destination_translation, fqdn_translation,
    mac_address_translation, socket_address_translation, source_translation,
    transmitted_data_translation,
};
use crate::translations::translations_3::{
    copy_translation, messages_translation, service_translation,
};
use crate::translations::translations_5::program_translation;
use crate::translations::translations_6::latency_translation;
use crate::utils::formatted_strings::{get_formatted_timestamp, get_socket_address};
use crate::utils::types::icon::Icon;
use crate::{Language, Protocol, Sniffer, StyleType};
use iced::alignment::Vertical;
use iced::widget::scrollable::Direction;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Row, Space, Text, Tooltip};
use iced::widget::{Scrollable, button};
use iced::{Alignment, Element, Length, Padding};

pub fn connection_details_page(
    sniffer: &Sniffer,
    key: AddressPortPair,
) -> Container<'_, Message, StyleType> {
    Container::new(page_content(sniffer, &key))
}

fn page_content<'a>(sniffer: &Sniffer, key: &AddressPortPair) -> Container<'a, Message, StyleType> {
    let Settings {
        language,
        color_gradient,
        ..
    } = sniffer.conf.settings;

    let info_traffic = &sniffer.info_traffic;
    let default_val = InfoAddressPortPair::default();
    let val = info_traffic.map.get(key).unwrap_or(&default_val);
    let address_to_lookup = get_address_to_lookup(key, val.traffic_direction);
    let host_option = sniffer.addresses_resolved.get(&address_to_lookup);
    let default_host = Host::default();
    let host_info_option = info_traffic
        .hosts
        .get(host_option.map_or(&default_host, |(_, h)| h))
        .copied();

    let header_and_content = Column::new()
        .width(Length::Fill)
        .push(page_header(color_gradient, language));

    let mut source_caption = Row::new().align_y(Alignment::Center).spacing(10).push(
        Text::new(source_translation(language))
            .size(FONT_SIZE_TITLE)
            .class(TextType::Title),
    );
    let mut dest_caption = Row::new().align_y(Alignment::Center).spacing(10).push(
        Text::new(destination_translation(language))
            .size(FONT_SIZE_TITLE)
            .class(TextType::Title),
    );
    let mut host_info_col = Column::new();
    if let Some((r_dns, host)) = host_option {
        host_info_col = get_host_info_col(r_dns, host, language);
        let host_info = host_info_option.unwrap_or_default();
        let flag = get_flag_tooltip(host.country, &host_info, language, false, 1.0);
        let computer = get_local_tooltip(sniffer, &address_to_lookup, key);
        if address_to_lookup.eq(&key.source) {
            source_caption = source_caption.push(flag);
            dest_caption = dest_caption.push(computer);
        } else {
            dest_caption = dest_caption.push(flag);
            source_caption = source_caption.push(computer);
        }
    }

    let mut source_col = get_src_or_dest_col(
        source_caption,
        &key.source,
        key.sport,
        val.mac_address1.as_ref(),
        language,
        &sniffer.timing_events,
    );
    let mut dest_col = get_src_or_dest_col(
        dest_caption,
        &key.dest,
        key.dport,
        val.mac_address2.as_ref(),
        language,
        &sniffer.timing_events,
    );

    if address_to_lookup.eq(&key.source) {
        source_col = source_col.push(host_info_col);
    } else {
        dest_col = dest_col.push(host_info_col);
    }

    let col_info = col_info(sniffer, key, val, address_to_lookup);

    let content = assemble_widgets(col_info, source_col, dest_col);

    Container::new(header_and_content.push(content))
        .width(1000)
        .height(500)
        .class(ContainerType::Modal)
}

fn page_header<'a>(
    color_gradient: GradientType,
    language: Language,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Row::new()
            .push(Space::new().width(Length::Fill))
            .push(
                Text::new(connection_details_translation(language))
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .align_x(Alignment::Center),
            )
            .push(
                Container::new(button_hide(Message::HideModal, language))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            ),
    )
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .height(40.0)
    .width(Length::Fill)
    .class(ContainerType::Gradient(color_gradient))
}

fn col_info<'a>(
    sniffer: &Sniffer,
    key: &AddressPortPair,
    val: &InfoAddressPortPair,
    latency_target: IpAddr,
) -> Column<'a, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;
    let data_repr = sniffer.conf.data_repr;
    let is_unicast = get_traffic_type(
        &latency_target,
        sniffer.capture_source.get_addresses(),
        val.traffic_direction,
    ) == TrafficType::Unicast;
    let measure_latency = matches!(sniffer.capture_source, CaptureSource::Device(_)) && is_unicast;
    let is_icmp = key.protocol.eq(&Protocol::ICMP);
    let is_arp = key.protocol.eq(&Protocol::ARP);

    let mut ret_val = Column::new()
        .spacing(10)
        .padding(Padding::new(20.0).right(10).left(40))
        .width(Length::FillPortion(2))
        .push(Space::new().height(Length::Fill))
        .push(
            Row::new()
                .spacing(8)
                .align_y(Vertical::Center)
                .push(Icon::Clock.to_text())
                .push(Text::new(format!(
                    "{}\n{}",
                    get_formatted_timestamp(val.initial_timestamp),
                    get_formatted_timestamp(val.final_timestamp)
                ))),
        )
        .push(TextType::highlighted_subtitle_with_desc(
            protocol_translation(language),
            &key.protocol.to_string(),
        ));

    if !is_icmp && !is_arp {
        ret_val = ret_val
            .push(TextType::highlighted_subtitle_with_desc(
                service_translation(language),
                &val.service.to_string(),
            ))
            .push(TextType::highlighted_subtitle_with_desc(
                program_translation(language),
                &val.program.to_string(),
            ));
    }

    ret_val = ret_val.push(TextType::highlighted_subtitle_with_desc(
        &format!(
            "{} ({})",
            transmitted_data_translation(language),
            if val.traffic_direction.eq(&TrafficDirection::Outgoing) {
                outgoing_translation(language).to_lowercase()
            } else {
                incoming_translation(language).to_lowercase()
            }
        ),
        &(data_repr.formatted_string(val.transmitted_data(data_repr))
            + if data_repr == DataRepr::Packets {
                format!(" {}", packets_translation(language))
            } else {
                String::new()
            }
            .as_ref()),
    ));

    if measure_latency {
        let latency_status = sniffer.latency_statuses.get(&latency_target);
        let hourglass = Icon::get_hourglass(sniffer.dots_pulse.0.len());
        ret_val = ret_val.push(latency_row(
            language,
            latency_target,
            latency_status,
            hourglass,
        ));
    }

    if is_icmp || is_arp {
        ret_val = ret_val.push(
            Column::new()
                .push(
                    Text::new(format!("{}:", messages_translation(language)))
                        .class(TextType::Subtitle),
                )
                .push(Scrollable::with_direction(
                    Column::new()
                        .padding(Padding::ZERO.right(10).bottom(10))
                        .push(Text::new(if is_icmp {
                            IcmpType::pretty_print_types(&val.icmp_types)
                        } else {
                            ArpType::pretty_print_types(&val.arp_types)
                        })),
                    Direction::Both {
                        vertical: ScrollbarType::properties(),
                        horizontal: ScrollbarType::properties(),
                    },
                )),
        );
    }

    ret_val = ret_val.push(Space::new().height(Length::Fill));

    ret_val
}

fn latency_row<'a>(
    language: Language,
    latency_target: IpAddr,
    latency_status: Option<&LatencyStatus>,
    hourglass: Text<'a, StyleType>,
) -> Column<'a, Message, StyleType> {
    let measuring = latency_status.is_some_and(|s| matches!(s, LatencyStatus::Measuring));

    let value: Element<'a, Message, StyleType> = match latency_status {
        Some(LatencyStatus::Failed(error)) => Row::new()
            .push(Text::new("   "))
            .push(get_error_tooltip(error))
            .into(),
        Some(LatencyStatus::Measuring) => Text::new("   ...").into(),
        Some(LatencyStatus::Measured(latency)) => {
            Text::new(format!("   {} ms", latency.as_millis())).into()
        }
        None => Text::new("   -").into(),
    };

    Column::new()
        .push(Text::new(format!("{}:", latency_translation(language))).class(TextType::Subtitle))
        .push(
            Row::new()
                .align_y(Alignment::Center)
                .spacing(10)
                .push(value)
                .push(get_button_ping(latency_target, measuring, hourglass)),
        )
}

fn get_error_tooltip<'a>(error: &str) -> Tooltip<'a, Message, StyleType> {
    Tooltip::new(
        Icon::Error.to_text(),
        Text::new(error.to_string()),
        Position::FollowCursor,
    )
    .class(ContainerType::Tooltip)
    .delay(TOOLTIP_DELAY)
}

fn get_host_info_col<'a>(
    r_dns: &str,
    host: &Host,
    language: Language,
) -> Column<'a, Message, StyleType> {
    let mut host_info_col = Column::new().spacing(4);
    if r_dns.parse::<IpAddr>().is_err() || (!host.asn.name.is_empty() && !host.asn.code.is_empty())
    {
        host_info_col = host_info_col.push(RuleType::Standard.horizontal(10));
    }
    if r_dns.parse::<IpAddr>().is_err() {
        host_info_col = host_info_col.push(TextType::highlighted_subtitle_with_desc(
            fqdn_translation(language),
            r_dns,
        ));
    }
    if !host.asn.name.is_empty() && !host.asn.code.is_empty() {
        host_info_col = host_info_col.push(TextType::highlighted_subtitle_with_desc(
            "ASN",
            &format!("{} ({})", host.asn.name, host.asn.code),
        ));
    }
    host_info_col
}

fn get_local_tooltip<'a>(
    sniffer: &Sniffer,
    address_to_lookup: &IpAddr,
    key: &AddressPortPair,
) -> Tooltip<'a, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;

    let local_address = if address_to_lookup.eq(&key.source) {
        &key.dest
    } else {
        &key.source
    };
    let my_interface_addresses = sniffer.capture_source.get_addresses();
    get_computer_tooltip(
        is_my_address(local_address, my_interface_addresses),
        is_local_connection(local_address, my_interface_addresses),
        is_bogon(local_address),
        get_traffic_type(
            if address_to_lookup.eq(&key.source) {
                &key.dest
            } else {
                &key.source
            },
            my_interface_addresses,
            TrafficDirection::Outgoing,
        ),
        language,
    )
}

fn get_src_or_dest_col<'a>(
    caption: Row<'a, Message, StyleType>,
    ip: &IpAddr,
    port: Option<u16>,
    mac: Option<&String>,
    language: Language,
    timing_events: &TimingEvents,
) -> Column<'a, Message, StyleType> {
    let address_caption = if port.is_some() {
        socket_address_translation(language)
    } else {
        address_translation(language)
    };

    let mac_str = if let Some(val) = mac { val } else { "-" };

    Column::new()
        .spacing(4)
        .push(
            Container::new(caption)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .push(RuleType::Standard.horizontal(10))
        .push(
            Row::new()
                .spacing(10)
                .align_y(Alignment::End)
                .push(TextType::highlighted_subtitle_with_desc(
                    address_caption,
                    &get_socket_address(ip, port),
                ))
                .push(get_button_copy(language, ip, timing_events)),
        )
        .push(TextType::highlighted_subtitle_with_desc(
            mac_address_translation(language),
            mac_str,
        ))
}

fn assemble_widgets<'a>(
    col_info: Column<'a, Message, StyleType>,
    source_col: Column<'a, Message, StyleType>,
    dest_col: Column<'a, Message, StyleType>,
) -> Row<'a, Message, StyleType> {
    let [source_container, dest_container] = [source_col, dest_col].map(|col| {
        Container::new(col)
            .padding(7)
            .width(Length::Fill)
            .class(ContainerType::BorderedRound)
    });
    Row::new()
        .padding([0, 10])
        .spacing(10)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .push(col_info)
        .push(
            Column::new()
                .width(Length::FillPortion(3))
                .align_x(Alignment::Center)
                .spacing(5)
                .push(Space::new().height(Length::Fill))
                .push(source_container)
                .push(Icon::ArrowsDown.to_text())
                .push(dest_container)
                .push(Space::new().height(Length::Fill)),
        )
}

fn get_button_copy<'a>(
    language: Language,
    ip: &IpAddr,
    timing_events: &TimingEvents,
) -> Tooltip<'a, Message, StyleType> {
    let icon = if timing_events.was_just_copy_ip(ip) {
        Text::new("✔").size(14)
    } else {
        Icon::Copy.to_text().size(12)
    };

    let content = button(icon.align_x(Alignment::Center).align_y(Alignment::Center))
        .padding(0)
        .height(25)
        .width(25)
        .on_press(Message::CopyIp(*ip));

    Tooltip::new(
        content,
        Text::new(format!("{} (IP)", copy_translation(language))),
        Position::Right,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
    .delay(TOOLTIP_DELAY)
}

fn get_button_ping(
    ip: IpAddr,
    measuring: bool,
    hourglass: Text<StyleType>,
) -> Tooltip<Message, StyleType> {
    let icon = if measuring {
        hourglass.size(14)
    } else {
        Icon::Ping.to_text().size(16)
    };

    let mut button = button(icon.align_x(Alignment::Center).align_y(Alignment::Center))
        .padding(0)
        .height(25)
        .width(25);

    if !measuring {
        button = button.on_press(Message::MeasureLatency(ip));
    }

    Tooltip::new(button, "Ping", Position::Right)
        .gap(5)
        .class(ContainerType::Tooltip)
        .delay(TOOLTIP_DELAY)
}
