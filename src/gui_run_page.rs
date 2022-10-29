use std::cmp::min;
use iced::{alignment, Alignment, Button, Column, Container, Length, Row, Svg, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use thousands::Separable;
use crate::app::Message;
use crate::{FONT_SIZE_TITLE, get_app_count_string, icon_sun_moon, Mode, Sniffer};
use crate::address_port_pair::AddressPortPair;
use crate::info_address_port_pair::InfoAddressPortPair;
use crate::style::{COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon};

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
        .height(Length::FillPortion(HEIGHT_HEADER))
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
        .push(Text::new(format!("Total intercepted packets: {}",
                                sniffer_lock.all_packets.separate_with_spaces())))
        .push(Text::new(format!("Filtered packets: {}",
                                (sniffer_lock.tot_received_packets + sniffer_lock.tot_sent_packets).separate_with_spaces())));
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

    let body = Column::new().height(Length::FillPortion(HEIGHT_BODY))
        .align_items(Alignment::Center)
        .spacing(10)
        .push(col_packets)
        .push(row_report);

    let button_github = Button::new(
        &mut sniffer.git,
        icon('\u{f09b}').size(30)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .height(Length::Units(35))
        .width(Length::Units(35))
        .style(sniffer.style)
        .on_press(Message::OpenGithub);
    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(Text::new("Sniffnet v1.0.0 - by Giuliano Bellini ").size(FONT_SIZE_FOOTER).font(COURIER_PRIME_ITALIC))
        .push(button_github)
        .push(Text::new("  "));
    let footer = Container::new(footer_row)
        .width(Length::Fill)
        .height(FillPortion(HEIGHT_FOOTER))
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .style(Mode::Bordered);

    Column::new()
        .push(header)
        .push(body)
        .push(footer)

}