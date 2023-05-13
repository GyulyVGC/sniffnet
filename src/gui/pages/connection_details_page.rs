use std::net::IpAddr;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Row, Text, Tooltip};
use iced::Length::Fixed;
use iced::{Alignment, Length};
use iced_lazy::lazy;
use iced_native::widget::tooltip::Position;
use iced_native::widget::{button, horizontal_space, vertical_space, Rule};

use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_TITLE, ICONS};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::networking::manage_packets::{get_address_to_lookup, get_traffic_type, is_my_address};
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::translations::translations::{
    application_protocol_translation, bytes_translation, hide_translation, incoming_translation,
    outgoing_translation, packets_translation, transport_protocol_translation,
};
use crate::translations::translations_2::{
    administrative_entity_translation, connection_details_translation, destination_translation,
    fqdn_translation, mac_address_translation, socket_address_translation, source_translation,
    transmitted_data_translation,
};
use crate::utils::countries::{get_computer_tooltip, get_flag_tooltip, FLAGS_WIDTH_BIG};
use crate::utils::formatted_strings::{get_formatted_bytes_string, get_socket_address};
use crate::{Language, Sniffer, StyleType};

pub fn connection_details_page(sniffer: &Sniffer, connection_index: usize) -> Container<Message> {
    Container::new(lazy(
        sniffer.runtime_data.tot_sent_packets + sniffer.runtime_data.tot_received_packets,
        move |_| page_content(sniffer, connection_index),
    ))
}

fn page_content(sniffer: &Sniffer, connection_index: usize) -> Container<'static, Message> {
    let font = get_font(sniffer.style);

    let info_traffic_lock = sniffer
        .info_traffic
        .lock()
        .expect("Error acquiring mutex\n\r");
    let key_val: (&AddressPortPair, &InfoAddressPortPair) =
        info_traffic_lock.map.get_index(connection_index).unwrap();
    let key = key_val.0.clone();
    let val = key_val.1.clone();
    let address_to_lookup = get_address_to_lookup(&key, val.traffic_direction);
    let host_option = info_traffic_lock
        .addresses_resolved
        .get(&address_to_lookup)
        .cloned();
    let host_info_option = info_traffic_lock
        .hosts
        .get(&host_option.clone().unwrap_or_default().1)
        .cloned();
    drop(info_traffic_lock);

    let header_and_content = Column::new()
        .width(Length::Fill)
        .push(page_header(sniffer.style, sniffer.language));

    let mut source_caption = Row::new().spacing(10).push(
        Text::new(source_translation(sniffer.language))
            .font(font)
            .size(FONT_SIZE_TITLE),
    );
    let mut dest_caption = Row::new().spacing(10).push(
        Text::new(destination_translation(sniffer.language))
            .font(font)
            .size(FONT_SIZE_TITLE),
    );
    let mut host_info_col = Column::new().spacing(4);
    if let Some((r_dns, host)) = host_option {
        let host_info = host_info_option.unwrap_or_default();
        let flag = get_flag_tooltip(
            &host.country,
            FLAGS_WIDTH_BIG,
            host_info.is_local,
            host_info.traffic_type,
            sniffer.language,
            sniffer.style,
        );
        if address_to_lookup.eq(&key.address1) {
            source_caption = source_caption.push(flag);
            let my_interface_addresses = &*sniffer.device.addresses.lock().unwrap();
            let computer = get_computer_tooltip(
                is_my_address(&key.address2, my_interface_addresses),
                get_traffic_type(
                    &key.address2,
                    my_interface_addresses,
                    TrafficDirection::Outgoing,
                ),
                sniffer.language,
                sniffer.style,
            );
            dest_caption = dest_caption.push(computer);
        } else {
            dest_caption = dest_caption.push(flag);
            let my_interface_addresses = &*sniffer.device.addresses.lock().unwrap();
            let computer = get_computer_tooltip(
                is_my_address(&key.address1, my_interface_addresses),
                get_traffic_type(
                    &key.address1,
                    my_interface_addresses,
                    TrafficDirection::Outgoing,
                ),
                sniffer.language,
                sniffer.style,
            );
            source_caption = source_caption.push(computer);
        }
        if r_dns.parse::<IpAddr>().is_err() || (!host.asn.name.is_empty() && host.asn.number > 0) {
            host_info_col =
                host_info_col.push(Rule::horizontal(10.0).style(<StyleTuple as Into<
                    iced::theme::Rule,
                >>::into(
                    StyleTuple(sniffer.style, ElementType::Standard),
                )));
        }
        if r_dns.parse::<IpAddr>().is_err() {
            host_info_col = host_info_col.push(
                Text::new(format!(
                    "{}:\n   {r_dns}",
                    fqdn_translation(sniffer.language)
                ))
                .font(font),
            );
        }
        if !host.asn.name.is_empty() && host.asn.number > 0 {
            host_info_col = host_info_col.push(
                Text::new(format!(
                    "{}:\n   {} (ASN {})",
                    administrative_entity_translation(sniffer.language),
                    host.asn.name,
                    host.asn.number
                ))
                .font(font),
            );
        }
    }

    let mut source_col = Column::new()
        .spacing(5)
        .push(
            Container::new(source_caption)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .push(
            Rule::horizontal(10.0).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
        .push(
            Text::new(format!(
                "{}:\n   {}",
                socket_address_translation(sniffer.language),
                get_socket_address(&key.address1, key.port1)
            ))
            .font(font),
        )
        .push(
            Text::new(format!(
                "{}:\n   {}",
                mac_address_translation(sniffer.language),
                val.mac_address1
            ))
            .font(font),
        );
    let mut dest_col = Column::new()
        .spacing(5)
        .push(
            Container::new(dest_caption)
                .width(Length::Fill)
                .align_x(Horizontal::Center),
        )
        .push(
            Rule::horizontal(10.0).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
        .push(
            Text::new(format!(
                "{}:\n   {}",
                socket_address_translation(sniffer.language),
                get_socket_address(&key.address2, key.port2)
            ))
            .font(font),
        )
        .push(
            Text::new(format!(
                "{}:\n   {}",
                mac_address_translation(sniffer.language),
                val.mac_address2
            ))
            .font(font),
        );

    if address_to_lookup.eq(&key.address1) {
        source_col = source_col.push(host_info_col);
    } else {
        dest_col = dest_col.push(host_info_col);
    }

    let source_container = Container::new(source_col)
        .padding(10)
        .width(Length::Fill)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::BorderedRound),
        ));

    let dest_container = Container::new(dest_col)
        .padding(10)
        .width(Length::Fill)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::BorderedRound),
        ));

    let col_info = Column::new()
        .spacing(10)
        .padding([0, 0, 0, 40])
        .width(Length::FillPortion(2))
        .push(vertical_space(Length::FillPortion(1)))
        .push(
            Row::new().spacing(5).push(Text::new("9").font(ICONS)).push(
                Text::new(format!(
                    "{} - {}",
                    val.initial_timestamp.to_string().get(11..19).unwrap(),
                    val.final_timestamp.to_string().get(11..19).unwrap()
                ))
                .font(font),
            ),
        )
        .push(
            Text::new(format!(
                "{}:\n   {}",
                transport_protocol_translation(sniffer.language),
                key.trans_protocol
            ))
            .font(font),
        )
        .push(
            Text::new(format!(
                "{}:\n   {}",
                application_protocol_translation(sniffer.language),
                val.app_protocol
            ))
            .font(font),
        )
        .push(
            Text::new(format!(
                "{} ({}):\n   {} {}\n   {} {}",
                transmitted_data_translation(sniffer.language),
                if val.traffic_direction.eq(&TrafficDirection::Outgoing) {
                    outgoing_translation(sniffer.language).to_lowercase()
                } else {
                    incoming_translation(sniffer.language).to_lowercase()
                },
                get_formatted_bytes_string(val.transmitted_bytes).trim(),
                bytes_translation(sniffer.language),
                val.transmitted_packets,
                packets_translation(sniffer.language),
            ))
            .font(font),
        )
        .push(vertical_space(Length::FillPortion(1)));

    let content = Row::new()
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
                .push(Text::new(":").font(ICONS))
                .push(dest_container)
                .push(vertical_space(Length::FillPortion(1))),
        );

    Container::new(header_and_content.push(content))
        .width(Length::Fixed(1000.0))
        .height(Length::Fixed(500.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn page_header(style: StyleType, language: Language) -> Container<'static, Message> {
    let font = get_font(style);
    let tooltip = hide_translation(language).to_string();
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new(connection_details_translation(language))
                    .font(get_font_headers(style))
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    Tooltip::new(
                        button(
                            Text::new("x")
                                .font(font)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Fixed(20.0))
                        .width(Fixed(20.0))
                        .style(StyleTuple(style, ElementType::Standard).into())
                        .on_press(Message::HideModal),
                        tooltip,
                        Position::Right,
                    )
                    .font(font)
                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                        StyleTuple(style, ElementType::Tooltip),
                    )),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Fixed(40.0))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}
