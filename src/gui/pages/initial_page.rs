//! Module defining the initial page of the application.
//!
//! It contains elements to select network adapter and traffic filters.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::text::Shaping;
use iced::widget::tooltip::Position;
use iced::widget::{
    button, horizontal_space, vertical_space, Button, Column, Container, Row, Rule, Scrollable,
    Text, TextInput, Tooltip,
};
use iced::Length::FillPortion;
use iced::{alignment, Font, Length, Renderer};
use pcap::Device;
use std::collections::HashSet;

use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{get_font, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::filters::Filters;
use crate::networking::types::ip_collection::IpCollection;
use crate::networking::types::port_collection::PortCollection;
use crate::translations::translations::{
    address_translation, addresses_translation, choose_adapters_translation,
    ip_version_translation, select_filters_translation, start_translation,
    transport_protocol_translation,
};
use crate::translations::translations_3::port_translation;
use crate::utils::types::icon::Icon;
use crate::{IpVersion, Language, StyleType, TransProtocol};

/// Computes the body of gui initial page
pub fn initial_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let color_gradient = sniffer.settings.color_gradient;
    let font = get_font(style);

    let col_adapter = get_col_adapter(sniffer, font);

    let ip_active = &sniffer.filters.ip;
    let col_ip_buttons = col_ip_buttons(ip_active, font, language);

    let transport_active = &sniffer.filters.transport;
    let col_transport_buttons = col_transport_buttons(transport_active, font, language);

    let address_active = &sniffer.filters.address_str;
    let col_address_filter = col_address_input(address_active, font, language);

    let port_active = &sniffer.filters.port_str;
    let col_port_filter = col_port_input(port_active, font, language);

    let filters_pane = Column::new()
        .width(FillPortion(6))
        .padding(10)
        .spacing(15)
        .push(
            select_filters_translation(language)
                .font(font)
                .style(TextType::Title)
                .size(FONT_SIZE_TITLE),
        )
        .push(
            Row::new()
                .spacing(20)
                .push(col_ip_buttons)
                .push(col_transport_buttons),
        )
        .push(
            Row::new()
                .spacing(20)
                .push(col_address_filter)
                .push(col_port_filter),
        )
        .push(Rule::horizontal(40))
        .push(
            Container::new(button_start(
                font,
                language,
                color_gradient,
                &sniffer.filters,
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Center)
            .align_x(Horizontal::Center),
        );

    let body = Column::new().push(vertical_space(Length::Fixed(5.0))).push(
        Row::new()
            .push(col_adapter)
            .push(horizontal_space(Length::Fixed(30.0)))
            .push(filters_pane),
    );

    Container::new(body).height(Length::Fill)
}

fn col_ip_buttons(
    active_ip_filters: &HashSet<IpVersion>,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let mut buttons_row = Row::new().spacing(5).padding([0, 0, 0, 5]);
    for option in IpVersion::ALL {
        let is_active = active_ip_filters.contains(&option);
        let check_symbol = if is_active { "✔" } else { "✘" };
        buttons_row = buttons_row.push(
            Button::new(
                Text::new(format!("{option} {check_symbol}"))
                    .shaping(Shaping::Advanced)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
                    .font(font),
            )
            .width(Length::Fixed(90.0))
            .height(Length::Fixed(35.0))
            .style(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::IpVersionSelection(option, !is_active)),
        );
    }

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            ip_version_translation(language)
                .font(font)
                .style(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(buttons_row)
}

fn col_transport_buttons(
    active_transport_filters: &HashSet<TransProtocol>,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let mut buttons_row = Row::new().spacing(5).padding([0, 0, 0, 5]);
    for option in TransProtocol::ALL {
        let is_active = active_transport_filters.contains(&option);
        let check_symbol = if is_active { "✔" } else { "✘" };
        buttons_row = buttons_row.push(
            Button::new(
                Text::new(format!("{option} {check_symbol}"))
                    .shaping(Shaping::Advanced)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
                    .font(font),
            )
            .width(Length::Fixed(90.0))
            .height(Length::Fixed(35.0))
            .style(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::TransportProtocolSelection(option, !is_active)),
        );
    }

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(transport_protocol_translation(language))
                .font(font)
                .style(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(buttons_row)
}

fn col_address_input(
    value: &String,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let is_error = if value.is_empty() {
        false
    } else {
        IpCollection::new(value).is_none()
    };
    let input_row = Row::new().padding([0, 0, 0, 5]).push(
        TextInput::new(IpCollection::PLACEHOLDER_STR, value)
            .padding([2, 5])
            .on_input(Message::AddressFilter)
            .font(font)
            .width(Length::Fixed(310.0))
            .style(if is_error {
                TextInputType::Error
            } else {
                TextInputType::Standard
            }),
    );

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(address_translation(language))
                .font(font)
                .style(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(input_row)
}

fn col_port_input(
    value: &String,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let is_error = if value.is_empty() {
        false
    } else {
        PortCollection::new(value).is_none()
    };
    let input_row = Row::new().padding([0, 0, 0, 5]).push(
        TextInput::new(PortCollection::PLACEHOLDER_STR, value)
            .padding([2, 5])
            .on_input(Message::PortFilter)
            .font(font)
            .width(Length::Fixed(180.0))
            .style(if is_error {
                TextInputType::Error
            } else {
                TextInputType::Standard
            }),
    );

    Column::new()
        .width(Length::Fill)
        .spacing(7)
        .push(
            Text::new(port_translation(language))
                .font(font)
                .style(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(input_row)
}

fn button_start(
    font: Font,
    language: Language,
    color_gradient: GradientType,
    filters: &Filters,
) -> Tooltip<'static, Message, Renderer<StyleType>> {
    let mut content = button(
        Icon::Rocket
            .to_text()
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(80.0))
    .width(Length::Fixed(160.0))
    .style(ButtonType::Gradient(color_gradient));

    if filters.are_valid() {
        content = content.on_press(Message::Start);
    }

    let tooltip = start_translation(language).to_string();
    //tooltip.push_str(" [⏎]");
    Tooltip::new(content, tooltip, Position::Top)
        .gap(5)
        .font(font)
        .style(ContainerType::Tooltip)
}

fn get_col_adapter(sniffer: &Sniffer, font: Font) -> Column<Message, Renderer<StyleType>> {
    let language = sniffer.settings.language;

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
                dev_str.push_str(&format!("\n{}:", address_translation(language)));
            }
            _ => {
                dev_str.push_str(&format!("\n{}:", addresses_translation(language)));
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
            choose_adapters_translation(language)
                .font(font)
                .style(TextType::Title)
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
                            .style(if name == sniffer.device.name {
                                ButtonType::BorderedRoundSelected
                            } else {
                                ButtonType::BorderedRound
                            })
                            .on_press(Message::AdapterSelection(name)),
                    )
                },
            ))
            .direction(Direction::Vertical(ScrollbarType::properties())),
        )
}
