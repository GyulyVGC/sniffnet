//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: charts, detailed connections data
//! and overall statistics about the filtered traffic.

use std::cmp::min;

use iced::{alignment, Alignment, Button, Column, Container, Length, Radio, Row, Scrollable, Text};
use iced::alignment::{Horizontal, Vertical};
use iced::Length::FillPortion;
use thousands::Separable;

use crate::{AppProtocol, ChartType, ReportType, StyleType};
use crate::enums::message::Message;
use crate::gui::style::{APP_VERSION, COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC, FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, icon_sun_moon, ICONS, logo_glyph};
use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::structs::sniffer::Sniffer;
use crate::utility::get_formatted_strings::{get_active_filters_string, get_active_filters_string_nobr, get_app_count_string, get_connection_color, get_formatted_bytes_string, get_percentage_string};

/// Computes the body of gui run page
pub fn run_page(sniffer: &mut Sniffer) -> Column<Message> {
    let font = if sniffer.style == StyleType::Day { COURIER_PRIME_BOLD } else { COURIER_PRIME };
    let font_footer = if sniffer.style == StyleType::Day { COURIER_PRIME_ITALIC } else { COURIER_PRIME_BOLD_ITALIC };
    let headers_style = if sniffer.style == StyleType::Day { StyleType::HeadersDay } else { StyleType::HeadersNight };
    let logo = logo_glyph().size(90);

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

    let button_reset = Button::new(
        &mut sniffer.reset,
        Text::new("C").font(ICONS)
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .padding(10)
        .height(Length::Units(40))
        .width(Length::Units(60))
        .style(sniffer.style)
        .on_press(Message::Reset);

    let header = Container::new(Row::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(Container::new(button_reset).width(Length::FillPortion(1)).width(Length::FillPortion(1)).align_x(Horizontal::Center))
        .push(Container::new(Row::new().align_items(Alignment::Center).push(logo)).width(Length::FillPortion(6)).height(Length::Fill).align_x(Horizontal::Center).align_y(Vertical::Center))
        .push(Container::new(button_style).width(Length::FillPortion(1)).align_x(Horizontal::Center)))
        .height(Length::FillPortion(HEIGHT_HEADER))
        .width(Length::Fill)
        .style(headers_style);

    let button_report = Button::new(
        &mut sniffer.report,
        Text::new("Open full report").font(font)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .padding(10)
        .height(Length::Units(85))
        .width(Length::Units(75))
        .style(sniffer.style)
        .on_press(Message::OpenReport);

    let runtime_data_lock = sniffer.runtime_data.lock().unwrap();
    let observed = runtime_data_lock.all_packets;
    let filtered = runtime_data_lock.tot_sent_packets + runtime_data_lock.tot_received_packets;
    let observed_bytes = runtime_data_lock.all_bytes;
    let filtered_bytes = runtime_data_lock.tot_sent_bytes + runtime_data_lock.tot_received_bytes;
    let app_protocols = runtime_data_lock.app_protocols.clone();
    drop(runtime_data_lock);
    let filtered_bytes_string = get_formatted_bytes_string(filtered_bytes as u128);

    let mut body = Column::new()
        .height(Length::FillPortion(HEIGHT_BODY))
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center);

    if sniffer.pcap_error.lock().unwrap().is_none() { // NO pcap error detected

        match (observed, filtered) {
            (0, 0) => { //no packets observed at all
                if sniffer.waiting.len() > 2 {
                    sniffer.waiting = "".to_string();
                }
                sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);
                let adapter_name = &*sniffer.device.clone().lock().unwrap().name.clone();
                let (icon_text, nothing_to_see_text) = if !sniffer.device.lock().unwrap().addresses.is_empty() {
                    (Text::new(sniffer.waiting.len().to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                                              Network adapter: {adapter_name}\n\n\
                                                              Are you sure you are connected to the internet and you have selected the right adapter?")).font(font))
                } else {
                    (Text::new("T").font(ICONS).size(60),
                     Text::new(format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                                              Network adapter: {adapter_name}\n\n\
                                                              If you are sure you are connected to the internet, try choosing a different adapter.")).font(font))
                };
                body = body
                    .push(Row::new().height(Length::FillPortion(1)))
                    .push(icon_text)
                    .push(nothing_to_see_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(Row::new().height(Length::FillPortion(2)));
            }

            (observed, 0) => { //no packets have been filtered but some have been observed
                if sniffer.waiting.len() > 2 {
                    sniffer.waiting = "".to_string();
                }
                sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);

                let tot_packets_text = Text::new(format!("Total intercepted packets: {}\n\n\
                                                    Filtered packets: 0\n\n\
                                                    Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{}",
                                                         observed.separate_with_spaces(), get_active_filters_string_nobr(sniffer.filters.clone()))).font(font);

                body = body
                    .push(Row::new().height(Length::FillPortion(1)))
                    .push(Text::new("V").font(ICONS).size(60))
                    .push(tot_packets_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(Row::new().height(Length::FillPortion(2)));
            }

            (observed, filtered) => { //observed > filtered > 0 || observed = filtered > 0

                let active_radio_chart = sniffer.chart_type;
                let row_radio_chart = Row::new().padding(15).spacing(10)
                    .push(Text::new("Plotted data:    ").size(FONT_SIZE_SUBTITLE).font(font))
                    .push(Radio::new(
                        ChartType::Packets,
                        "packets per second",
                        Some(active_radio_chart),
                        Message::ChartSelection,
                    ).width(Length::Units(220)).font(font).size(15).style(sniffer.style))
                    .push(Radio::new(
                        ChartType::Bytes,
                        "bytes per second",
                        Some(active_radio_chart),
                        Message::ChartSelection,
                    ).width(Length::Units(220)).font(font).size(15).style(sniffer.style))
                    ;

                let col_chart = Container::new(
                    Column::new()
                        .push(row_radio_chart)
                        .push(sniffer.traffic_chart.view(sniffer.style, sniffer.chart_type)))
                    .width(Length::FillPortion(2))
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .style(StyleType::BorderedRound);

                let mut col_packets = Column::new()
                    .width(Length::FillPortion(1))
                    .padding(10)
                    //.push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()).font(font))
                    .push(Text::new(get_active_filters_string(sniffer.filters.clone())).font(font))
                    .push(Text::new(" "))
                    .push(Text::new(format!("Filtered packets:\n   {} ({} of the total)",
                                            filtered.separate_with_spaces(), get_percentage_string(observed, filtered))).font(font))
                    .push(Text::new(" "))
                    .push(Text::new(format!("Filtered bytes:\n   {filtered_bytes_string} ({} of the total)",
                                            get_percentage_string(observed_bytes, filtered_bytes))).font(font));
                if sniffer.filters.lock().unwrap().application.eq(&AppProtocol::Other) {
                    col_packets = col_packets
                        .push(Text::new(" "))
                        .push(Text::new("Filtered packets per application protocol:").font(font))
                        .push(Scrollable::new(&mut sniffer.scroll_packets).style(sniffer.style)
                            .push(Text::new(get_app_count_string(app_protocols, filtered as u128)).font(font)));
                }

                let active_radio_report = sniffer.report_type;
                let row_radio_report = Row::new().padding(10)
                    .push(Text::new("Relevant connections:    ").size(FONT_SIZE_SUBTITLE).font(font))
                    .push(Radio::new(
                        ReportType::MostRecent,
                        "most recent",
                        Some(active_radio_report),
                        Message::ReportSelection,
                    )
                        .width(Length::Units(200))
                        .font(font).size(15).style(sniffer.style))
                    .push(Radio::new(
                        ReportType::MostPackets,
                        "most packets",
                        Some(active_radio_report),
                        Message::ReportSelection,
                    )
                        .width(Length::Units(200))
                        .font(font).size(15).style(sniffer.style))
                    .push(Radio::new(
                        ReportType::MostBytes,
                        "most bytes",
                        Some(active_radio_report),
                        Message::ReportSelection,
                    )
                        .width(Length::Units(200))
                        .font(font).size(15).style(sniffer.style))
                    ;

                let sniffer_lock = sniffer.info_traffic.lock().unwrap();
                let mut sorted_vec: Vec<(&AddressPortPair, &InfoAddressPortPair)> = sniffer_lock.map.iter().collect();
                match active_radio_report {
                    ReportType::MostRecent => {
                        sorted_vec.sort_by(|&(_, a), &(_, b)|
                            b.final_timestamp.cmp(&a.final_timestamp));
                    }
                    ReportType::MostPackets => {
                        sorted_vec.sort_by(|&(_, a), &(_, b)|
                            b.transmitted_packets.cmp(&a.transmitted_packets));
                    }
                    ReportType::MostBytes => {
                        sorted_vec.sort_by(|&(_, a), &(_, b)|
                            b.transmitted_bytes.cmp(&a.transmitted_bytes));
                    }
                }
                let n_entry = min(sorted_vec.len(), 15);
                let mut col_report = Column::new()
                    .height(Length::Fill)
                    .push(row_radio_report)
                    .push(Text::new(" "))
                    .push(iced::Text::new("     Src IP address       Src port      Dst IP address       Dst port  Layer 4  Layer 7    Packets      Bytes  ").font(font))
                    .push(iced::Text::new("---------------------------------------------------------------------------------------------------------------").font(font))
                    ;
                let mut scroll_report = Scrollable::new(&mut sniffer.scroll_report).style(sniffer.style);
                for i in 0..n_entry {
                    let key_val = sorted_vec.get(i).unwrap();
                    let entry_color = get_connection_color(key_val.1.traffic_type);
                    scroll_report = scroll_report.push(iced::Text::new(format!("{}{}", key_val.0.print_gui(), key_val.1.print_gui())).color(entry_color).font(COURIER_PRIME_BOLD));
                }
                col_report = col_report.push(scroll_report);
                drop(sniffer_lock);
                let col_open_report = Container::new(button_report)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center);
                let row_report = Row::new()
                    .spacing(10)
                    .height(Length::FillPortion(2))
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .push(Container::new(col_report)
                        .padding(10)
                        .height(Length::Fill)
                        .style(StyleType::BorderedRound))
                    .push(col_open_report);

                body = body
                    .push(Row::new().spacing(10).height(Length::FillPortion(3))
                        .push(col_chart)
                        .push(Container::new(col_packets).padding(10).height(Length::Fill).style(StyleType::BorderedRound)))
                    .push(row_report);
            }
        }
    } else { // pcap threw an ERROR!
        let err_string = sniffer.pcap_error.lock().unwrap().clone().unwrap();

        if sniffer.waiting.len() > 2 {
            sniffer.waiting = "".to_string();
        }
        sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);

        let error_text = Text::new(format!("An error occurred! \n\n\
                                                    {err_string}")).font(font);

        body = body
            .push(Row::new().height(Length::FillPortion(1)))
            .push(Text::new("U").font(ICONS).size(60))
            .push(error_text)
            .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
            .push(Row::new().height(Length::FillPortion(2)));
    }

    let button_github = Button::new(
        &mut sniffer.git,
        Text::new("H").font(ICONS).size(24)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
        .height(Length::Units(35))
        .width(Length::Units(35))
        .style(sniffer.style)
        .on_press(Message::OpenGithub);
    let footer_row = Row::new()
        .align_items(Alignment::Center)
        .push(Text::new(format!("Sniffnet {APP_VERSION} - by Giuliano Bellini ")).size(FONT_SIZE_FOOTER).font(font_footer))
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