use iced::{alignment, Alignment, Button, Column, Length, Row, Svg, Text};
use crate::app::Message;
use crate::{get_app_count_string, icon_sun_moon, Sniffer};

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

    let header = Row::new()
        .height(Length::FillPortion(3))
        .align_items(Alignment::Center)
        .push(Column::new().width(Length::FillPortion(1)))
        .push(Column::new().width(Length::FillPortion(6)).push(logo))
        .push(Row::new().width(Length::FillPortion(1)).align_items(Alignment::Center).push(button_style));

    let button_reset = Button::new(
        &mut sniffer.reset,
        Text::new("Reset")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(80))
        .style(sniffer.style)
        .on_press(Message::Reset);

    let button_report = Button::new(
        &mut sniffer.report,
        Text::new("Open full report")
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(200))
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
    col_packets = col_packets.push(button_reset).push(button_report);

    let body = Row::new().height(Length::FillPortion(9))
        .push(col_packets);

    Column::new()
        .push(header)
        .push(body)

}