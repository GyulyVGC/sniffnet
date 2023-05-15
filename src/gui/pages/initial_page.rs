//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use iced::widget::{
    button, horizontal_space, vertical_space, Button, Column, Container, PickList, Row, Scrollable,
    Text, Tooltip,
};
use iced::Length::FillPortion;
use iced::{alignment, Alignment, Font, Length};
use iced_native::widget::tooltip::Position;
use pcap::Device;

use crate::gui::components::radio::{ip_version_radios, transport_protocol_radios};
use crate::gui::styles::style_constants::{get_font, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, ICONS};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::translations::translations::{
    address_translation, addresses_translation, all_translation, application_protocol_translation,
    choose_adapters_translation, select_filters_translation, start_translation,
};
use crate::{AppProtocol, Language, StyleType};

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let col_adapter = get_col_adapter(sniffer, font);

    let ip_active = sniffer.filters.ip;
    let col_ip_radio = ip_version_radios(ip_active, font, sniffer.style, sniffer.language);
    let col_ip = Column::new()
        .spacing(10)
        .width(FillPortion(1))
        .push(col_ip_radio);

    let transport_active = sniffer.filters.transport;
    let col_transport_radio =
        transport_protocol_radios(transport_active, font, sniffer.style, sniffer.language);
    let col_transport = Column::new()
        .align_items(Alignment::Center)
        .spacing(10)
        .width(FillPortion(2))
        .push(col_transport_radio)
        .push(vertical_space(FillPortion(2)))
        .push(button_start(sniffer.style, sniffer.language))
        .push(vertical_space(FillPortion(1)));

    let app_active = if sniffer.filters.application.ne(&AppProtocol::Other) {
        Some(sniffer.filters.application)
    } else {
        None
    };
    let picklist_app = PickList::new(
        if app_active.is_some() {
            &AppProtocol::ALL[..]
        } else {
            &AppProtocol::ALL[1..]
        },
        app_active,
        Message::AppProtocolSelection,
    )
    .padding([3, 7])
    .placeholder(all_translation(sniffer.language))
    .font(font)
    .style(StyleTuple(sniffer.style, ElementType::Standard));
    let col_app = Column::new()
        .width(FillPortion(1))
        .spacing(10)
        .push(
            Text::new(application_protocol_translation(sniffer.language))
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
                select_filters_translation(sniffer.language)
                    .font(font)
                    .size(FONT_SIZE_TITLE),
            ),
        )
        .push(
            Row::new()
                .spacing(10)
                .height(FillPortion(3))
                .push(col_ip)
                .push(col_transport)
                .push(col_app),
        );

    let body = Column::new().push(vertical_space(Length::Fixed(5.0))).push(
        Row::new()
            .push(col_adapter)
            .push(horizontal_space(Length::Fixed(30.0)))
            .push(filters),
    );

    Container::new(body).height(Length::Fill).style(
        <StyleTuple as Into<iced::theme::Container>>::into(StyleTuple(
            sniffer.style,
            ElementType::Standard,
        )),
    )
}

fn button_start(style: StyleType, language: Language) -> Tooltip<'static, Message> {
    let content = button(
        Text::new("S")
            .font(ICONS)
            .size(FONT_SIZE_TITLE)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(80.0))
    .width(Length::Fixed(160.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::Start);

    let tooltip = start_translation(language).to_string();
    //tooltip.push_str(" [⏎]");
    Tooltip::new(content, tooltip, Position::Top)
        .gap(5)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}

fn get_col_adapter(sniffer: &Sniffer, font: Font) -> Column<Message> {
    let mut dev_str_list = vec![];
    for dev in Device::list().expect("Error retrieving device list\r\n") {
        let mut dev_str = String::new();
        let name = dev.name;
        match dev.desc {
            None => {
                dev_str.push_str(&name);
            }
            Some(description) => {
                #[cfg(not(target_os = "windows"))]
                dev_str.push_str(&format!("{name}\n"));
                dev_str.push_str(&description);
            }
        }
        let num_addresses = dev.addresses.len();
        match num_addresses {
            0 => {}
            1 => {
                dev_str.push_str(&format!("\n{}:", address_translation(sniffer.language)));
            }
            _ => {
                dev_str.push_str(&format!("\n{}:", addresses_translation(sniffer.language)));
            }
        }

        for addr in dev.addresses {
            let address_string = addr.addr.to_string();
            dev_str.push_str(&format!("\n   {address_string}"));
        }
        dev_str_list.push((name, dev_str));
    }

    Column::new()
        .padding(10)
        .spacing(5)
        .height(Length::Fill)
        .width(FillPortion(4))
        .push(
            choose_adapters_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_TITLE),
        )
        .push(
            Scrollable::new(dev_str_list.iter().fold(
                Column::new().padding(13).spacing(5),
                |scroll_adapters, adapter| {
                    let name = adapter.0.clone();
                    let description = adapter.1.clone();
                    scroll_adapters.push(
                        Button::new(Text::new(description).font(font))
                            .padding([20, 30])
                            .width(Length::Fill)
                            .style(
                                StyleTuple(
                                    sniffer.style,
                                    if name == sniffer.device.name {
                                        ElementType::BorderedRoundSelected
                                    } else {
                                        ElementType::BorderedRound
                                    },
                                )
                                .into(),
                            )
                            .on_press(Message::AdapterSelection(name)),
                    )
                },
            ))
            .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        )
}
