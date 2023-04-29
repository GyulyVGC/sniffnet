use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Row, Text, Tooltip};
use iced::Length::Fixed;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;
use iced_native::widget::{button, horizontal_space, vertical_space};

use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_TITLE};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::translations::translations::hide_translation;
use crate::translations::translations_2::connection_details_translation;
use crate::utils::formatted_strings::get_formatted_bytes_string;
use crate::{Language, Sniffer, StyleType};

pub fn connection_details_page(sniffer: &Sniffer, connection_index: usize) -> Container<Message> {
    let font = get_font(sniffer.style);

    let info_traffic_lock = sniffer
        .info_traffic
        .lock()
        .expect("Error acquiring mutex\n\r");
    let key_val: (&AddressPortPair, &InfoAddressPortPair) =
        info_traffic_lock.map.get_index(connection_index).unwrap();
    let key = key_val.0.clone();
    let val = key_val.1.clone();
    drop(info_traffic_lock);

    let header_and_content = Column::new()
        .width(Length::Fill)
        .push(page_header(sniffer.style, sniffer.language));
    let mut content = Column::new()
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Start)
        .width(Length::Fill)
        .push(
            Text::new(format!(
                "Data exchanged: {} bytes ({} packets).",
                get_formatted_bytes_string(val.transmitted_bytes).trim(),
                val.transmitted_packets,
            ))
            .font(font),
        )
        .push(
            Text::new(format!(
                "Data transmission observed from {} to {}.",
                val.initial_timestamp.to_string().get(11..19).unwrap(),
                val.final_timestamp.to_string().get(11..19).unwrap()
            ))
            .font(font),
        )
        .push(vertical_space(Length::Fixed(15.0)))
        .push(Text::new("Source").font(font).size(FONT_SIZE_TITLE))
        .push(Text::new(format!("Socket address: {} {}", key.address1, key.port1)).font(font))
        .push(Text::new(format!("MAC address: {}", val.mac_address1)).font(font))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(Text::new("Destination").font(font).size(FONT_SIZE_TITLE))
        .push(Text::new(format!("Socket address: {} {}", key.address2, key.port2)).font(font))
        .push(Text::new(format!("MAC address: {}", val.mac_address2)).font(font))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(Text::new(val.asn.name.clone()).font(font))
        .push(Text::new(format!("{}", val.asn.number)).font(font));

    if let Some(r_dns) = val.r_dns {
        if !r_dns.is_empty() {
            content = content.push(Text::new(format!("{}", r_dns)).font(font));
        }
    }

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
