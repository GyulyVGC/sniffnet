//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use iced::{
    alignment, Alignment, Button, Column, Container, Length, PickList, Radio, Row, Scrollable, Text,
};
use pcap::Device;
use plotters::style::RGBColor;

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::style::StyleTuple;
use crate::structs::colors::to_rgb_color;
use crate::structs::sniffer::Sniffer;
use crate::utility::get_formatted_strings::{icon_sun_moon, logo_glyph, APP_VERSION};
use crate::utility::style_constants::{
    COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC,
    FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, HEIGHT_BODY, HEIGHT_FOOTER,
    HEIGHT_HEADER, ICONS,
};
use crate::{get_colors, AppProtocol, IpVersion, TransProtocol};

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &mut Sniffer) -> Column<Message> {
    let font = match to_rgb_color(get_colors(sniffer.style).text_body) {
        RGBColor(255, 255, 255) => COURIER_PRIME,
        _ => COURIER_PRIME_BOLD,
    };
    let font_footer = match to_rgb_color(get_colors(sniffer.style).text_headers) {
        RGBColor(255, 255, 255) => COURIER_PRIME_ITALIC,
        _ => COURIER_PRIME_BOLD_ITALIC,
    };
    let logo = logo_glyph();

    let button_style = Button::new(&mut sniffer.mode, icon_sun_moon())
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(60))
        .style(StyleTuple(sniffer.style, ElementType::Standard))
        .on_press(Message::Style);

    let header = Container::new(
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Container::new(Row::new())
                    .width(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            )
            .push(
                Container::new(
                    Row::new()
                        .height(Length::Fill)
                        .align_items(Alignment::Center)
                        .push(logo),
                )
                .width(Length::FillPortion(6))
                .height(Length::Fill)
                .align_y(Vertical::Center)
                .align_x(Horizontal::Center),
            )
            .push(
                Container::new(button_style)
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            ),
    )
    .height(Length::FillPortion(HEIGHT_HEADER))
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .style(StyleTuple(sniffer.style, ElementType::Headers));

    let button_start = Button::new(
        &mut sniffer.start,
        Text::new("Run!")
            .font(font)
            .size(FONT_SIZE_TITLE)
            .vertical_alignment(alignment::Vertical::Center)
            .horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(10)
    .height(Length::Units(80))
    .width(Length::Units(160))
    .style(StyleTuple(sniffer.style, ElementType::Standard))
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

    let adapter_active = &sniffer.device.lock().unwrap().name.clone();
    let col_adapter = Column::new()
        .padding(10)
        .spacing(5)
        .height(Length::Fill)
        .width(Length::FillPortion(4))
        .push(
            Text::new("Select network adapter to inspect")
                .font(font)
                .size(FONT_SIZE_TITLE),
        )
        .push(
            dev_str_list.iter().fold(
                Scrollable::new(&mut sniffer.scroll_adapters)
                    .style(StyleTuple(sniffer.style, ElementType::Standard))
                    .padding(13)
                    .spacing(5),
                |scroll_adapters, adapter| {
                    let name = &adapter.0;
                    scroll_adapters.push(
                        Container::new(
                            Radio::new(
                                name,
                                &adapter.1,
                                Some(&sniffer.device.clone().lock().unwrap().name),
                                |name| Message::AdapterSelection(name.to_string()),
                            )
                            .font(font)
                            .size(15)
                            .width(Length::Fill)
                            .style(StyleTuple(
                                sniffer.style,
                                if adapter_active.eq(name) {
                                    ElementType::SelectedRadio
                                } else {
                                    ElementType::Standard
                                },
                            )),
                        )
                        .padding(10)
                        .style(StyleTuple(sniffer.style, ElementType::BorderedRound)),
                    )
                },
            ),
        );

    let col_space = Column::new().width(Length::FillPortion(1));

    let filtri = sniffer.filters.lock().unwrap();
    let ip_active = filtri.ip;
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
            .style(StyleTuple(
                sniffer.style,
                if ip_active.eq(&IpVersion::IPv4) {
                    ElementType::SelectedRadio
                } else {
                    ElementType::Standard
                },
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
            .style(StyleTuple(
                sniffer.style,
                if ip_active.eq(&IpVersion::IPv6) {
                    ElementType::SelectedRadio
                } else {
                    ElementType::Standard
                },
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
            .style(StyleTuple(
                sniffer.style,
                if ip_active.eq(&IpVersion::Other) {
                    ElementType::SelectedRadio
                } else {
                    ElementType::Standard
                },
            )),
        );
    let col_ip = Column::new()
        .spacing(10)
        .width(Length::FillPortion(1))
        .push(col_ip_radio);

    let transport_active = filtri.transport;
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
            .style(StyleTuple(
                sniffer.style,
                if transport_active.eq(&TransProtocol::TCP) {
                    ElementType::SelectedRadio
                } else {
                    ElementType::Standard
                },
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
            .style(StyleTuple(
                sniffer.style,
                if transport_active.eq(&TransProtocol::UDP) {
                    ElementType::SelectedRadio
                } else {
                    ElementType::Standard
                },
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
            .style(StyleTuple(
                sniffer.style,
                if transport_active.eq(&TransProtocol::Other) {
                    ElementType::SelectedRadio
                } else {
                    ElementType::Standard
                },
            )),
        );
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
    .style(StyleTuple(sniffer.style, ElementType::Standard));
    let col_app = Column::new()
        .width(Length::FillPortion(2))
        .spacing(10)
        .push(
            iced::Text::new("Application protocol")
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(picklist_app);

    let filters = Column::new()
        .width(Length::FillPortion(6))
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
                .height(Length::FillPortion(3))
                .push(col_ip)
                .push(col_transport)
                .push(col_app),
        );

    let body = Row::new()
        .height(Length::FillPortion(HEIGHT_BODY))
        .push(col_adapter)
        .push(col_space)
        .push(filters);

    let button_github = Button::new(
        &mut sniffer.git,
        Text::new('H'.to_string())
            .font(ICONS)
            .size(24)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .height(Length::Units(35))
    .width(Length::Units(35))
    .style(StyleTuple(sniffer.style, ElementType::Standard))
    .on_press(Message::OpenGithub);
    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(
            Text::new(format!("Sniffnet {} - by Giuliano Bellini ", APP_VERSION))
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        )
        .push(button_github)
        .push(Text::new("  ").font(font));
    let footer = Container::new(footer_row)
        .width(Length::Fill)
        .height(FillPortion(HEIGHT_FOOTER))
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(StyleTuple(sniffer.style, ElementType::Headers));

    Column::new().push(header).push(body).push(footer)
}
