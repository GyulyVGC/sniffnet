use std::net::IpAddr;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::tooltip::Position;
use iced::widget::{button, horizontal_space, lazy, vertical_space, Rule};
use iced::widget::{Column, Container, Row, Text, Tooltip};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

use crate::countries::country_utils::{get_computer_tooltip, get_flag_tooltip};
use crate::countries::flags_pictures::FLAGS_WIDTH_BIG;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::gui::types::timing_events::TimingEvents;
use crate::networking::manage_packets::{get_address_to_lookup, get_traffic_type, is_my_address};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::host::Host;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::translations::translations::{
    application_protocol_translation, hide_translation, incoming_translation, outgoing_translation,
    packets_translation, transport_protocol_translation,
};
use crate::translations::translations_2::{
    administrative_entity_translation, connection_details_translation, destination_translation,
    fqdn_translation, mac_address_translation, socket_address_translation, source_translation,
    transmitted_data_translation,
};
use crate::translations::translations_3::copy_translation;
use crate::utils::formatted_strings::{get_formatted_bytes_string_with_b, get_socket_address};
use crate::utils::types::icon::Icon;
use crate::{Language, Sniffer, StyleType};

pub fn connection_details_page(
    sniffer: &Sniffer,
    key: AddressPortPair,
) -> Container<Message, Renderer<StyleType>> {
    Container::new(lazy(
        (
            sniffer.runtime_data.tot_sent_packets + sniffer.runtime_data.tot_received_packets,
            sniffer.timing_events.was_just_copy_ip(&key.address1),
            sniffer.timing_events.was_just_copy_ip(&key.address2),
        ),
        move |_| page_content(sniffer, &key),
    ))
}

fn page_content(
    sniffer: &Sniffer,
    key: &AddressPortPair,
) -> Container<'static, Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let color_gradient = sniffer.settings.color_gradient;
    let font = get_font(style);
    let font_headers = get_font_headers(style);

    let info_traffic_lock = sniffer
        .info_traffic
        .lock()
        .expect("Error acquiring mutex\n\r");
    let val = info_traffic_lock.map.get(key).unwrap().clone();
    let address_to_lookup = get_address_to_lookup(key, val.traffic_direction);
    let host_option = info_traffic_lock
        .addresses_resolved
        .get(&address_to_lookup)
        .cloned();
    let host_info_option = info_traffic_lock
        .hosts
        .get(&host_option.clone().unwrap_or_default().1)
        .cloned();
    drop(info_traffic_lock);

    let header_and_content = Column::new().width(Length::Fill).push(page_header(
        font,
        font_headers,
        color_gradient,
        language,
    ));

    let mut source_caption = Row::new().align_items(Alignment::Center).spacing(10).push(
        Text::new(source_translation(language))
            .font(font)
            .size(FONT_SIZE_TITLE)
            .style(TextType::Title),
    );
    let mut dest_caption = Row::new().align_items(Alignment::Center).spacing(10).push(
        Text::new(destination_translation(language))
            .font(font)
            .size(FONT_SIZE_TITLE)
            .style(TextType::Title),
    );
    let mut host_info_col = Column::new();
    if let Some((r_dns, host)) = host_option {
        host_info_col = get_host_info_col(&r_dns, &host, font, language);
        let host_info = host_info_option.unwrap_or_default();
        let flag = get_flag_tooltip(
            host.country,
            FLAGS_WIDTH_BIG,
            host_info.is_local,
            host_info.traffic_type,
            language,
            font,
        );
        let computer = get_local_tooltip(sniffer, &address_to_lookup, key);
        if address_to_lookup.eq(&key.address1) {
            source_caption = source_caption.push(flag);
            dest_caption = dest_caption.push(computer);
        } else {
            dest_caption = dest_caption.push(flag);
            source_caption = source_caption.push(computer);
        }
    }

    let mut source_col = get_src_or_dest_col(
        source_caption,
        &key.address1,
        key.port1,
        &val.mac_address1,
        font,
        language,
        &sniffer.timing_events,
    );
    let mut dest_col = get_src_or_dest_col(
        dest_caption,
        &key.address2,
        key.port2,
        &val.mac_address2,
        font,
        language,
        &sniffer.timing_events,
    );

    if address_to_lookup.eq(&key.address1) {
        source_col = source_col.push(host_info_col);
    } else {
        dest_col = dest_col.push(host_info_col);
    }

    let col_info = col_info(key, &val, font, language);

    let content = assemble_widgets(col_info, source_col, dest_col);

    Container::new(header_and_content.push(content))
        .width(Length::Fixed(1000.0))
        .height(Length::Fixed(500.0))
        .style(ContainerType::Modal)
}

fn page_header(
    font: Font,
    font_headers: Font,
    color_gradient: GradientType,
    language: Language,
) -> Container<'static, Message, Renderer<StyleType>> {
    let tooltip = hide_translation(language).to_string();
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new(connection_details_translation(language))
                    .font(font_headers)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    Tooltip::new(
                        button(
                            Text::new("×")
                                .font(font)
                                .vertical_alignment(Vertical::Center)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Fixed(20.0))
                        .width(Fixed(20.0))
                        .on_press(Message::HideModal),
                        tooltip,
                        Position::Right,
                    )
                    .font(font)
                    .style(ContainerType::Tooltip),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Fixed(40.0))
    .width(Length::Fill)
    .style(ContainerType::Gradient(color_gradient))
}

fn col_info(
    key: &AddressPortPair,
    val: &InfoAddressPortPair,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    Column::new()
        .spacing(10)
        .padding([0, 0, 0, 40])
        .width(Length::FillPortion(2))
        .push(vertical_space(Length::FillPortion(1)))
        .push(
            Row::new().spacing(5).push(Icon::Clock.to_text()).push(
                Text::new(format!(
                    "{} - {}",
                    val.initial_timestamp.to_string().get(11..19).unwrap(),
                    val.final_timestamp.to_string().get(11..19).unwrap()
                ))
                .font(font),
            ),
        )
        .push(TextType::highlighted_subtitle_with_desc(
            transport_protocol_translation(language),
            &key.protocol.to_string(),
            font,
        ))
        .push(TextType::highlighted_subtitle_with_desc(
            application_protocol_translation(language),
            &val.app_protocol.to_string(),
            font,
        ))
        .push(TextType::highlighted_subtitle_with_desc(
            &format!(
                "{} ({})",
                transmitted_data_translation(language),
                if val.traffic_direction.eq(&TrafficDirection::Outgoing) {
                    outgoing_translation(language).to_lowercase()
                } else {
                    incoming_translation(language).to_lowercase()
                }
            ),
            &format!(
                "{}\n   {} {}",
                get_formatted_bytes_string_with_b(val.transmitted_bytes),
                val.transmitted_packets,
                packets_translation(language)
            ),
            font,
        ))
        .push(vertical_space(Length::FillPortion(1)))
}

fn get_host_info_col(
    r_dns: &str,
    host: &Host,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let mut host_info_col = Column::new().spacing(4);
    if r_dns.parse::<IpAddr>().is_err() || (!host.asn.name.is_empty() && host.asn.number > 0) {
        host_info_col = host_info_col.push(Rule::horizontal(10.0));
    }
    if r_dns.parse::<IpAddr>().is_err() {
        host_info_col = host_info_col.push(TextType::highlighted_subtitle_with_desc(
            fqdn_translation(language),
            r_dns,
            font,
        ));
    }
    if !host.asn.name.is_empty() && host.asn.number > 0 {
        host_info_col = host_info_col.push(TextType::highlighted_subtitle_with_desc(
            administrative_entity_translation(language),
            &format!("{} (ASN {})", host.asn.name, host.asn.number),
            font,
        ));
    }
    host_info_col
}

fn get_local_tooltip(
    sniffer: &Sniffer,
    address_to_lookup: &str,
    key: &AddressPortPair,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;

    let my_interface_addresses = &*sniffer.device.addresses.lock().unwrap();
    get_computer_tooltip(
        is_my_address(
            if address_to_lookup.eq(&key.address1) {
                &key.address2
            } else {
                &key.address1
            },
            my_interface_addresses,
        ),
        get_traffic_type(
            if address_to_lookup.eq(&key.address1) {
                &key.address2
            } else {
                &key.address1
            },
            my_interface_addresses,
            TrafficDirection::Outgoing,
        ),
        language,
        get_font(style),
    )
}

fn get_src_or_dest_col(
    caption: Row<'static, Message, Renderer<StyleType>>,
    ip: &String,
    port: u16,
    mac: &str,
    font: Font,
    language: Language,
    timing_events: &TimingEvents,
) -> Column<'static, Message, Renderer<StyleType>> {
    Column::new()
        .spacing(4)
        .push(
            Container::new(caption)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .push(Rule::horizontal(10.0))
        .push(
            Row::new()
                .spacing(10)
                .align_items(Alignment::End)
                .push(TextType::highlighted_subtitle_with_desc(
                    socket_address_translation(language),
                    &get_socket_address(ip, port),
                    font,
                ))
                .push(get_button_copy(language, font, ip, timing_events)),
        )
        .push(TextType::highlighted_subtitle_with_desc(
            mac_address_translation(language),
            mac,
            font,
        ))
}

fn assemble_widgets(
    col_info: Column<'static, Message, Renderer<StyleType>>,
    source_col: Column<'static, Message, Renderer<StyleType>>,
    dest_col: Column<'static, Message, Renderer<StyleType>>,
) -> Row<'static, Message, Renderer<StyleType>> {
    let [source_container, dest_container] = [source_col, dest_col].map(|col| {
        Container::new(col)
            .padding(7)
            .width(Length::Fill)
            .style(ContainerType::BorderedRound)
    });
    Row::new()
        .padding([0, 10])
        .spacing(10)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .push(col_info)
        .push(
            Column::new()
                .width(Length::FillPortion(3))
                .align_items(Alignment::Center)
                .spacing(5)
                .push(vertical_space(Length::FillPortion(1)))
                .push(source_container)
                .push(Icon::ArrowsDown.to_text())
                .push(dest_container)
                .push(vertical_space(Length::FillPortion(1))),
        )
}

fn get_button_copy(
    language: Language,
    font: Font,
    string: &String,
    timing_events: &TimingEvents,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let icon = if timing_events.was_just_copy_ip(string) {
        Text::new("✔").font(font).size(14)
    } else {
        Icon::Copy.to_text().size(12)
    };

    let content = button(
        icon.horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(0)
    .height(Length::Fixed(25.0))
    .width(Length::Fixed(25.0))
    .on_press(Message::CopyIp(string.clone()));

    Tooltip::new(
        content,
        format!("{} (IP)", copy_translation(language)),
        Position::Right,
    )
    .font(font)
    .style(ContainerType::Tooltip)
}
