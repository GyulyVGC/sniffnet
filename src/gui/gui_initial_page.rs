//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use iced::{alignment, Alignment, Button, Column, Container, Length, PickList, Radio, Row, Scrollable, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use pcap::Device;

use crate::{AppProtocol, Mode, TransProtocol};
use crate::gui::app::Message;
use crate::gui::style::{APP_VERSION, COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon_sun_moon, ICONS, logo_glyph};
use crate::structs::sniffer::Sniffer;

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &mut Sniffer) -> Column<Message> {
    let font = if sniffer.style == Mode::Day { COURIER_PRIME_BOLD } else { COURIER_PRIME };
    let font_footer = if sniffer.style == Mode::Day { COURIER_PRIME_ITALIC } else { COURIER_PRIME_BOLD_ITALIC };
    let headers_style = if sniffer.style == Mode::Day { Mode::HeadersDay } else { Mode::HeadersNight };
    let logo = logo_glyph().size(100);

    let button_style = Button::new(
        &mut sniffer.mode,
        icon_sun_moon(sniffer.style)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(60))
        .style(sniffer.style)
        .on_press(Message::Style);

    let header = Container::new(Row::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(Container::new(Row::new()).width(Length::FillPortion(1)).width(Length::FillPortion(1)).align_x(Horizontal::Center))
        .push(Container::new(Row::new().height(Length::Fill).align_items(Alignment::Center).push(logo)).width(Length::FillPortion(6)).height(Length::Fill).align_y(Vertical::Center).align_x(Horizontal::Center))
        .push(Container::new(button_style).width(Length::FillPortion(1)).align_x(Horizontal::Center)))
        .height(Length::FillPortion(HEIGHT_HEADER))
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .style(headers_style);

    let button_start = Button::new(
        &mut sniffer.start,
        Text::new("Run!").font(font).size(FONT_SIZE_TITLE).vertical_alignment(alignment::Vertical::Center).horizontal_alignment(alignment::Horizontal::Center),
    )
        .padding(10)
        .height(Length::Units(80))
        .width(Length::Units(160))
        .style(sniffer.style)
        .on_press(Message::Start);

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
            1 => { dev_str.push_str("\nAddress:"); }
            _ => { dev_str.push_str("\nAddresses:"); }
        }

        for addr in dev.addresses {
            let address_string = addr.addr.to_string();
            dev_str.push_str(&format!("\n    {}", address_string));
        }
        dev_str.push_str("\n ");
        dev_str_list.push((name, dev_str));
    }

    let col_adapter = Column::new()
        .padding(10)
        .spacing(5)
        .height(Length::Fill)
        .width(Length::FillPortion(4))
        .push(Text::new("Select network adapter to inspect").font(font).size(FONT_SIZE_TITLE))
        .push(dev_str_list.iter().fold(
            Scrollable::new(&mut sniffer.scroll_adapters).style(sniffer.style).padding(13).spacing(5),
            |scroll_adapters, adapter| {
                scroll_adapters.push(Container::new(Radio::new(
                    &adapter.0,
                    &adapter.1,
                    Some(&sniffer.device.clone().lock().unwrap().name),
                    |name| Message::AdapterSelection(name.to_string()),
                ).font(font).size(15).width(Length::Fill).style(sniffer.style)).padding(10).style(Mode::BorderedRound))
            },
        ));

    let col_space = Column::new()
        .padding(20)
        .spacing(10)
        .width(Length::FillPortion(1));

    let filtri = sniffer.filters.lock().unwrap();
    let ip_active = &*filtri.ip;
    let col_ip_radio = Column::new().spacing(10)
        .push(Text::new("IP version").font(font).size(FONT_SIZE_SUBTITLE))
        .push(Radio::new(
            "ipv4",
            "IPv4",
            Some(ip_active),
            |version| Message::IpVersionSelection(version.to_string()),
        ).width(Length::Fill).font(font).size(15).style(sniffer.style))
        .push(Radio::new(
            "ipv6",
            "IPv6",
            Some(ip_active),
            |version| Message::IpVersionSelection(version.to_string()),
        ).width(Length::Fill).font(font).size(15).style(sniffer.style))
        .push(Radio::new(
            "no filter",
            "both",
            Some(ip_active),
            |version| Message::IpVersionSelection(version.to_string()),
        ).width(Length::Fill).font(font).size(15).style(sniffer.style));
    let col_ip = Column::new()
        .spacing(10)
        .width(Length::FillPortion(1))
        .push(col_ip_radio);

    let transport_active = filtri.transport;
    let col_transport_radio = Column::new().spacing(10)
        .push(Text::new("Transport protocol").font(font).size(FONT_SIZE_SUBTITLE))
        .push(Radio::new(
            TransProtocol::TCP,
            "TCP",
            Some(transport_active),
            Message::TransportProtocolSelection,
        ).width(Length::Fill).font(font).size(15).style(sniffer.style))
        .push(Radio::new(
            TransProtocol::UDP,
            "UDP",
            Some(transport_active),
            Message::TransportProtocolSelection,
        ).width(Length::Fill).font(font).size(15).style(sniffer.style))
        .push(Radio::new(
            TransProtocol::Other,
            "both",
            Some(transport_active),
            Message::TransportProtocolSelection,
        ).width(Length::Fill).font(font).size(15).style(sniffer.style));
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
        Message::AppProtocolSelection,
    )
        .font(font)
        .placeholder("Select application protocol")
        .style(sniffer.style);
    let col_app = Column::new()
        .width(Length::FillPortion(2))
        .spacing(10)
        .push(iced::Text::new("Application protocol").font(font).size(FONT_SIZE_SUBTITLE))
        .push(picklist_app);

    let filters = Column::new().width(Length::FillPortion(6)).padding(10).spacing(15)
        .push(Row::new().push(Text::new("Select filters to be applied on network traffic").font(font).size(FONT_SIZE_TITLE)))
        .push(Row::new().height(Length::FillPortion(3)).push(col_ip).push(col_transport).push(col_app));

    let body = Row::new().height(Length::FillPortion(HEIGHT_BODY))
        .push(col_adapter)
        .push(col_space)
        .push(filters);

    let button_github = Button::new(
        &mut sniffer.git,
        Text::new('H'.to_string()).font(ICONS).size(24)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .height(Length::Units(35))
        .width(Length::Units(35))
        .style(sniffer.style)
        .on_press(Message::OpenGithub);
    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(Text::new(format!("Sniffnet {} - by Giuliano Bellini ", APP_VERSION)).size(FONT_SIZE_FOOTER).font(font_footer))
        .push(button_github)
        .push(Text::new("  ").font(font));
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