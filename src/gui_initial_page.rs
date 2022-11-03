use iced::{alignment, Alignment, Button, Column, Container, Length, PickList, Radio, Row, Scrollable, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use pcap::Device;
use crate::app::Message;
use crate::{AppProtocol, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, icon_sun_moon, Mode, Sniffer, TransProtocol};
use crate::style::{COURIER_PRIME_BOLD_ITALIC, FONT_SIZE_FOOTER, FONT_SIZE_SNIFFNET, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon, logo_glyph};

pub fn initial_page(sniffer: &mut Sniffer) -> Column<Message> {
    let headers_style = if sniffer.style == Mode::Day { Mode::HeadersDay } else { Mode::HeadersNight };
    let logo = logo_glyph().size(100);

    let button_style = Button::new(
        &mut sniffer.mode,
        icon_sun_moon(sniffer.style)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(100))
        .style(sniffer.style)
        .on_press(Message::Style);

    let header = Container::new(Row::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(Container::new(Row::new()).width(Length::FillPortion(1)).width(Length::FillPortion(1)).align_x(Horizontal::Center))
        .push(Container::new(Row::new().height(Length::Fill).align_items(Alignment::Center).push(logo).push(Text::new("SNIFFNET").vertical_alignment(Vertical::Center).height(Length::Fill).size(FONT_SIZE_SNIFFNET))).width(Length::FillPortion(6)).height(Length::Fill).align_y(Vertical::Center).align_x(Horizontal::Center))
        .push(Container::new(button_style).width(Length::FillPortion(1)).align_x(Horizontal::Center)))
        .height(Length::FillPortion(HEIGHT_HEADER))
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .style(headers_style);

    let button_start = Button::new(
        &mut sniffer.start,
        Text::new("Run!").size(FONT_SIZE_TITLE).vertical_alignment(alignment::Vertical::Center).horizontal_alignment(alignment::Horizontal::Center),
    )
        .padding(10)
        .height(Length::Units(80))
        .width(Length::Units(160))
        .style(sniffer.style)
        .on_press(Message::Start);

    let mut dev_str_list = vec![];
    for dev in Device::list().expect("Error retrieving device list\r\n") {
        let mut dev_str = String::new();
        match dev.desc {
            None => {
                dev_str.push_str(&format!("{}", dev.name));
            }
            Some(description) => {
                dev_str.push_str(&format!("{}\n{}", dev.name, description));
            }
        }
        let num_addresses = dev.addresses.len();
        match num_addresses {
            0 => {}
            1 => { dev_str.push_str("\nAddress:"); }
            _ => { dev_str.push_str("\nAddresses:"); }
        }

        for addr in dev.addresses {
            let address_string = addr.addr.to_string();
            dev_str.push_str(&format!("\n    {}", address_string));
        }
        dev_str_list.push((dev.name, dev_str));
    }

    let col_adapter = Column::new()
        .padding(10)
        .spacing(5)
        .height(Length::Fill)
        .width(Length::FillPortion(4))
        .push(Text::new("Select network adapter to inspect").size(FONT_SIZE_TITLE))
        .push(dev_str_list.iter().fold(
            Scrollable::new(&mut sniffer.scroll_adapters).style(sniffer.style).padding(13).spacing(5),
            |scroll_adapters, adapter| {
                scroll_adapters.push(Container::new(Radio::new(
                    &adapter.0,
                    &adapter.1,
                    Some(&sniffer.device.clone().lock().unwrap().name),
                    |name| Message::AdapterSelection(name.to_string()),
                ).size(15).width(Length::Fill).style(sniffer.style)).padding(10).style(Mode::BorderedRound))
            },
        ));

    let col_space = Column::new()
        .padding(20)
        .spacing(10)
        .width(Length::FillPortion(1));

    let filtri = sniffer.filters.lock().unwrap();
    let ip_active = &*filtri.ip;
    let col_ip_radio = Column::new().spacing(10)
        .push(Text::new("IP version").size(FONT_SIZE_SUBTITLE))
        .push(Radio::new(
            "ipv4",
            "IPv4",
            Some(ip_active),
            |version| Message::IpVersionSelection(version.to_string()),
        ).size(15).style(sniffer.style))
        .push(Radio::new(
            "ipv6",
            "IPv6",
            Some(ip_active),
            |version| Message::IpVersionSelection(version.to_string()),
        ).size(15).style(sniffer.style))
        .push(Radio::new(
            "no filter",
            "both",
            Some(ip_active),
            |version| Message::IpVersionSelection(version.to_string()),
        ).size(15).style(sniffer.style));
    let col_ip = Column::new()
        .spacing(10)
        .width(Length::FillPortion(1))
        .push(col_ip_radio);

    let transport_active = filtri.transport;
    let col_transport_radio = Column::new().spacing(10)
        .push(Text::new("Transport protocol").size(FONT_SIZE_SUBTITLE))
        .push(Radio::new(
            TransProtocol::TCP,
            "TCP",
            Some(transport_active),
            |protocol| Message::TransportProtocolSelection(protocol),
        ).size(15).style(sniffer.style))
        .push(Radio::new(
            TransProtocol::UDP,
            "UDP",
            Some(transport_active),
            |protocol| Message::TransportProtocolSelection(protocol),
        ).size(15).style(sniffer.style))
        .push(Radio::new(
            TransProtocol::Other,
            "both",
            Some(transport_active),
            |protocol| Message::TransportProtocolSelection(protocol),
        ).size(15).style(sniffer.style));
    let col_transport = Column::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(2))
        .push(col_transport_radio)
        .push(Row::new().height(Length::FillPortion(2)))
        .push(button_start)
        .push(Row::new().height(Length::FillPortion(1)));

    let app_active = filtri.application;
    let picklist_app = PickList::new(
        &mut sniffer.app,
        &AppProtocol::ALL[..],
        Some(app_active),
        |protocol| Message::AppProtocolSelection(protocol),
    )
        .placeholder("Select application protocol")
        .style(sniffer.style);
    let col_app = Column::new()
        .width(Length::FillPortion(2))
        .spacing(10)
        .push(iced::Text::new("Application protocol").size(FONT_SIZE_SUBTITLE))
        .push(picklist_app);

    let filters = Column::new().width(Length::FillPortion(6)).padding(10).spacing(15)
        .push(Row::new().push(Text::new("Select filters to be applied on network traffic").size(FONT_SIZE_TITLE)))
        .push(Row::new().height(Length::FillPortion(3)).push(col_ip).push(col_transport).push(col_app));

    let body = Row::new().height(Length::FillPortion(HEIGHT_BODY))
        .push(col_adapter)
        .push(col_space)
        .push(filters);

    let button_github = Button::new(
        &mut sniffer.git,
        icon('\u{f09b}').size(30)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .height(Length::Units(35))
        .width(Length::Units(35))
        .style(sniffer.style)
        .on_press(Message::OpenGithub);
    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(Text::new("Sniffnet v1.0.0 - by Giuliano Bellini ").size(FONT_SIZE_FOOTER).font(COURIER_PRIME_BOLD_ITALIC))
        .push(button_github)
        .push(Text::new("  "));
    let footer = Container::new(footer_row)
        .width(Length::Fill)
        .height(FillPortion(HEIGHT_FOOTER))
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(headers_style);

    Column::new()
        .spacing(10)
        .push(header)
        .push(body)
        .push(footer)
}