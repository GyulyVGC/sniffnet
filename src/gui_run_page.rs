use std::cmp::min;
use std::fmt::format;
use iced::{alignment, Alignment, Button, Column, Container, Length, Renderer, Row, Scrollable, Svg, Text};
use iced::alignment::Horizontal;
use crate::app::Message;
use crate::{FONT_SIZE_TITLE, get_app_count_string, icon_sun_moon, Sniffer};
use crate::address_port_pair::AddressPortPair;
use crate::info_address_port_pair::InfoAddressPortPair;
use crate::style::icon;

pub fn run_page(sniffer: &mut Sniffer) -> Column<Message> {

    let logo = Svg::from_path("./resources/sniffnet_logo.svg");

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

    let button_reset = Button::new(
        &mut sniffer.reset,
        icon('\u{f177}')
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(80))
        .style(sniffer.style)
        .on_press(Message::Reset);

    let header = Row::new()
        .height(Length::FillPortion(3))
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(Container::new(button_reset).width(Length::FillPortion(1)).align_x(Horizontal::Center))
        .push(Container::new(logo).width(Length::FillPortion(6)).align_x(Horizontal::Center))
        .push(Container::new(button_style).width(Length::FillPortion(1)).align_x(Horizontal::Center));

    let button_report = Button::new(
        &mut sniffer.report,
        Text::new("Open full report")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .padding(10)
        .height(Length::Units(85))
        .width(Length::Units(75))
        .style(sniffer.style)
        .on_press(Message::OpenReport);

    let sniffer_lock = sniffer.info_traffic.lock().unwrap();

    let mut col_packets = Column::new()
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center)
        .spacing(20)
        .push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()))
        .push(iced::Text::new(sniffer_lock.all_packets.to_string()));
    if sniffer_lock.tot_received_packets + sniffer_lock.tot_sent_packets > 0 {
        col_packets = col_packets
            .push(iced::Text::new("Packets count per application protocol"))
            .push(iced::Text::new(get_app_count_string(sniffer_lock.app_protocols.clone(), sniffer_lock.tot_received_packets + sniffer_lock.tot_sent_packets)));
    }

    let mut row_report = Row::new()
        .align_items(Alignment::Center);
    if sniffer_lock.map.len() > 0 {
        let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = sniffer_lock.map.iter().collect();
        sorted_vec.sort_by(|&(_, a), &(_, b)|
            b.final_timestamp.cmp(&a.final_timestamp));
        let n_entry = min(sorted_vec.len(), 10);
        let mut col_report = Column::new()
            .padding(10);
        col_report = col_report
            .push(iced::Text::new("Latest connections\n").size(FONT_SIZE_TITLE))
            .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------"))
            .push(iced::Text::new("|     Src IP address      | Src port |     Dst IP address      | Dst port | Layer 4 | Layer 7 |   Packets  |   Bytes    |"))
            .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------"));
        for i in 0..n_entry {
            let key_val = sorted_vec.get(i).unwrap();
            col_report = col_report.push(iced::Text::new(format!("{}{}", key_val.0, key_val.1.print_without_timestamps())));
        }
        col_report = col_report
            .push(iced::Text::new("-------------------------------------------------------------------------------------------------------------------------"));
        let col_open_report = Column::new()
            .push(button_report);
        row_report = row_report
            .push(col_report)
            .push(col_open_report);
    }

    let body = Column::new().height(Length::FillPortion(9))
        .align_items(Alignment::Center)
        .spacing(10)
        .push(col_packets)
        .push(row_report);

    Column::new()
        .push(header)
        .push(body)

}