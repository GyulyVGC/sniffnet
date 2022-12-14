//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use iced::widget::{button, Button, Column, Container, PickList, Radio, Row, Scrollable, Text};
use iced::Length::FillPortion;
use iced::{alignment, Alignment, Font, Length};
use pcap::Device;

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::sniffer::Sniffer;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::style_constants::{get_font, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, HEIGHT_BODY};
use crate::{AppProtocol, IpVersion, StyleType, TransProtocol};

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let col_adapter = get_col_adapter(sniffer, font);

    let col_space = Column::new().width(FillPortion(1));

    let ip_active = sniffer.filters.ip;
    let col_ip_radio = Column::new()
        .spacing(10)
        .push(Text::new("IP version").font(font).size(FONT_SIZE_SUBTITLE))
        .push(
            Radio::new(
                IpVersion::IPv4,
                "IPv4",
                Some(ip_active),
                Message::IpVersionSelection,
            )
            .width(Length::Fill)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced_style::theme::Radio>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
        .push(
            Radio::new(
                IpVersion::IPv6,
                "IPv6",
                Some(ip_active),
                Message::IpVersionSelection,
            )
            .width(Length::Fill)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced_style::theme::Radio>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
        .push(
            Radio::new(
                IpVersion::Other,
                "both",
                Some(ip_active),
                Message::IpVersionSelection,
            )
            .width(Length::Fill)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced_style::theme::Radio>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        );
    let col_ip = Column::new()
        .spacing(10)
        .width(FillPortion(1))
        .push(col_ip_radio);

    let transport_active = sniffer.filters.transport;
    let col_transport_radio = Column::new()
        .spacing(10)
        .push(
            Text::new("Transport protocol")
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(
            Radio::new(
                TransProtocol::TCP,
                "TCP",
                Some(transport_active),
                Message::TransportProtocolSelection,
            )
            .width(Length::Fill)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced_style::theme::Radio>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
        .push(
            Radio::new(
                TransProtocol::UDP,
                "UDP",
                Some(transport_active),
                Message::TransportProtocolSelection,
            )
            .width(Length::Fill)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced_style::theme::Radio>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
        .push(
            Radio::new(
                TransProtocol::Other,
                "both",
                Some(transport_active),
                Message::TransportProtocolSelection,
            )
            .width(Length::Fill)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced_style::theme::Radio>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        );
    let col_transport = Column::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .width(FillPortion(2))
        .push(col_transport_radio)
        .push(Row::new().height(FillPortion(2)))
        .push(get_button_start(sniffer.style, font))
        .push(Row::new().height(FillPortion(1)));

    let app_active = sniffer.filters.application;
    let picklist_app = PickList::new(
        &AppProtocol::ALL[..],
        Some(app_active),
        Message::AppProtocolSelection,
    )
    .font(font)
    .style(<StyleTuple as Into<iced::theme::PickList>>::into(
        StyleTuple(sniffer.style, ElementType::Standard),
    ));
    let col_app = Column::new()
        .width(FillPortion(2))
        .spacing(10)
        .push(
            Text::new("Application protocol")
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(picklist_app);

    let filters = Column::new()
        .width(FillPortion(6))
        .padding(10)
        .spacing(15)
        .push(
            Row::new().push(
                Text::new("Select filters to be applied on network traffic")
                    .font(font)
                    .size(FONT_SIZE_TITLE),
            ),
        )
        .push(
            Row::new()
                .height(FillPortion(3))
                .push(col_ip)
                .push(col_transport)
                .push(col_app),
        );

    let body = Row::new().push(col_adapter).push(col_space).push(filters);

    Container::new(body)
        .height(FillPortion(HEIGHT_BODY))
        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

pub fn get_button_start(style: StyleType, font: Font) -> Button<'static, Message> {
    button(
        Text::new("Run!")
            .font(font)
            .size(FONT_SIZE_TITLE)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(10)
    .height(Length::Units(80))
    .width(Length::Units(160))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::Start)
}

fn get_col_adapter(sniffer: &Sniffer, font: Font) -> Column<Message> {
    let mut dev_str_list = vec![];
    for dev in Device::list().expect("Error retrieving device list\r\n") {
        let mut dev_str = "\n".to_string();
        let name = dev.name;
        match dev.desc {
            None => {
                dev_str.push_str(&name);
            }
            Some(description) => {
                #[cfg(not(target_os = "windows"))]
                dev_str.push_str(&format!("{}\n", name));
                dev_str.push_str(&description);
            }
        }
        let num_addresses = dev.addresses.len();
        match num_addresses {
            0 => {}
            1 => {
                dev_str.push_str("\nAddress:");
            }
            _ => {
                dev_str.push_str("\nAddresses:");
            }
        }

        for addr in dev.addresses {
            let address_string = addr.addr.to_string();
            dev_str.push_str(&format!("\n    {}", address_string));
        }
        dev_str.push_str("\n ");
        dev_str_list.push((name, dev_str));
    }

    Column::new()
        .padding(10)
        .spacing(5)
        .height(Length::Fill)
        .width(FillPortion(4))
        .push(
            Text::new("Select network adapter to inspect")
                .font(font)
                .size(FONT_SIZE_TITLE),
        )
        .push(
            Scrollable::new(dev_str_list.iter().fold(
                Column::new().padding(13).spacing(5),
                |scroll_adapters, adapter| {
                    let name = &adapter.0;
                    scroll_adapters.push(
                        Container::new(
                            Radio::new(name, &adapter.1, Some(&sniffer.device.name), |name| {
                                Message::AdapterSelection(name.to_string())
                            })
                            .font(font)
                            .size(15)
                            .width(Length::Fill)
                            .style(<StyleTuple as Into<
                                iced_style::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        )
                        .padding(10)
                        .style(<StyleTuple as Into<
                            iced_style::theme::Container,
                        >>::into(
                            StyleTuple(sniffer.style, ElementType::BorderedRound),
                        )),
                    )
                },
            ))
            .style(<StyleTuple as Into<iced_style::theme::Scrollable>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
}
