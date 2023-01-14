//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: charts, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Properties;
use iced::widget::{button, Column, Container, Radio, Row, Scrollable, Text};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use thousands::Separable;
//use dns_lookup::lookup_addr;

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::components::tabs::get_tabs;
use crate::structs::sniffer::Sniffer;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::countries::get_flag;
use crate::utility::get_formatted_strings::{
    get_active_filters_string, get_active_filters_string_nobr, get_app_count_string,
    get_connection_color, get_formatted_bytes_string, get_percentage_string,
};
use crate::utility::style_constants::{
    get_font, COURIER_PRIME_BOLD, FONT_SIZE_SUBTITLE, HEIGHT_BODY, ICONS,
};
use crate::{AppProtocol, ChartType, ReportType};

/// Computes the body of gui run page
pub fn run_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let mut body = Column::new()
        .width(Length::Fill)
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(FillPortion(HEIGHT_BODY));

    if sniffer.pcap_error.is_none() {
        // NO pcap error detected

        let tabs = get_tabs(
            &["Overview", "Inspect", "Notifications"],
            &[Message::TickInit, Message::Reset, Message::Reset],
            "Overview",
            sniffer.style,
        );

        let observed = sniffer.runtime_data.borrow().all_packets;
        let filtered = sniffer.runtime_data.borrow().tot_sent_packets
            + sniffer.runtime_data.borrow().tot_received_packets;
        let observed_bytes = sniffer.runtime_data.borrow().all_bytes;
        let filtered_bytes = sniffer.runtime_data.borrow().tot_sent_bytes
            + sniffer.runtime_data.borrow().tot_received_bytes;
        let app_protocols = sniffer.runtime_data.borrow().app_protocols.clone();
        let filtered_bytes_string = get_formatted_bytes_string(filtered_bytes as u128);

        match (observed, filtered) {
            (0, 0) => {
                //no packets observed at all

                let adapter_name = sniffer.device.name.clone();
                let (icon_text, nothing_to_see_text) = if sniffer.device.addresses.is_empty() {
                    (Text::new('T'.to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                                              Network adapter: {adapter_name}\n\n\
                                                              If you are sure you are connected to the internet, try choosing a different adapter.")).font(font))
                } else {
                    (Text::new(sniffer.waiting.len().to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                                              Network adapter: {adapter_name}\n\n\
                                                              Are you sure you are connected to the internet and you have selected the right adapter?")).font(font))
                };
                body = body
                    .push(Row::new().height(FillPortion(1)))
                    .push(icon_text)
                    .push(nothing_to_see_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(Row::new().height(FillPortion(2)));
            }

            (observed, 0) => {
                //no packets have been filtered but some have been observed

                let tot_packets_text = Text::new(format!("Total intercepted packets: {}\n\n\
                                                    Filtered packets: 0\n\n\
                                                    Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{}",
                                                         observed.separate_with_spaces(), get_active_filters_string_nobr(&sniffer.filters.clone()))).font(font);

                body = body
                    .push(Row::new().height(FillPortion(1)))
                    .push(Text::new('V'.to_string()).font(ICONS).size(60))
                    .push(tot_packets_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(Row::new().height(FillPortion(2)));
            }

            (observed, filtered) => {
                //observed > filtered > 0 || observed = filtered > 0

                tab_and_body = tab_and_body.push(tabs);

                let active_radio_chart = sniffer.traffic_chart.chart_type;
                let row_radio_chart =
                    Row::new()
                        .padding(15)
                        .spacing(10)
                        .push(
                            Text::new("Traffic rate:    ")
                                .size(FONT_SIZE_SUBTITLE)
                                .font(font),
                        )
                        .push(
                            Radio::new(
                                ChartType::Packets,
                                "packets per second",
                                Some(active_radio_chart),
                                Message::ChartSelection,
                            )
                            .width(Length::Units(220))
                            .font(font)
                            .size(15)
                            .style(<StyleTuple as Into<
                                iced::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        )
                        .push(
                            Radio::new(
                                ChartType::Bytes,
                                "bytes per second",
                                Some(active_radio_chart),
                                Message::ChartSelection,
                            )
                            .width(Length::Units(220))
                            .font(font)
                            .size(15)
                            .style(<StyleTuple as Into<
                                iced::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        );

                let col_chart = Container::new(
                    Column::new()
                        .push(row_radio_chart)
                        .push(sniffer.traffic_chart.view()),
                )
                .width(FillPortion(2))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(sniffer.style, ElementType::BorderedRound),
                ));

                let mut col_packets = Column::new()
                    //.padding(10)
                    //.push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()).font(font))
                    //.push(iced::Text::new(confy::get_configuration_file_path("sniffnet", None).unwrap().to_string_lossy()).font(font))
                    //.push(Text::new(lookup_addr(&"8.8.8.8".parse().unwrap()).unwrap()).font(font))
                    .push(Text::new(get_active_filters_string(&sniffer.filters.clone())).font(font))
                    .push(Text::new(" "))
                    .push(
                        Text::new(format!(
                            "Filtered packets:\n   {} ({} of the total)",
                            filtered.separate_with_spaces(),
                            get_percentage_string(observed, filtered)
                        ))
                        .font(font),
                    )
                    .push(Text::new(" "))
                    .push(
                        Text::new(format!(
                            "Filtered bytes:\n   {} ({} of the total)",
                            filtered_bytes_string,
                            get_percentage_string(observed_bytes, filtered_bytes)
                        ))
                        .font(font),
                    );
                if sniffer.filters.application.eq(&AppProtocol::Other) {
                    col_packets = col_packets
                        .push(Text::new(" "))
                        .push(Text::new("Filtered packets per application protocol:").font(font))
                        .push(
                            Scrollable::new(
                                Text::new(get_app_count_string(&app_protocols, filtered as u128))
                                    .font(font),
                            )
                            .style(<StyleTuple as Into<
                                iced::theme::Scrollable,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        );
                }

                let active_radio_report = sniffer.report_type;
                let row_radio_report =
                    Row::new()
                        .padding(10)
                        .push(
                            Text::new("Relevant connections:    ")
                                .size(FONT_SIZE_SUBTITLE)
                                .font(font),
                        )
                        .push(
                            Radio::new(
                                ReportType::MostRecent,
                                "most recent",
                                Some(active_radio_report),
                                Message::ReportSelection,
                            )
                            .width(Length::Units(200))
                            .font(font)
                            .size(15)
                            .style(<StyleTuple as Into<
                                iced::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        )
                        .push(
                            Radio::new(
                                ReportType::MostPackets,
                                "most packets",
                                Some(active_radio_report),
                                Message::ReportSelection,
                            )
                            .width(Length::Units(200))
                            .font(font)
                            .size(15)
                            .style(<StyleTuple as Into<
                                iced::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        )
                        .push(
                            Radio::new(
                                ReportType::MostBytes,
                                "most bytes",
                                Some(active_radio_report),
                                Message::ReportSelection,
                            )
                            .width(Length::Units(200))
                            .font(font)
                            .size(15)
                            .style(<StyleTuple as Into<
                                iced::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        )
                        .push(
                            Radio::new(
                                ReportType::Favorites,
                                "favorites",
                                Some(active_radio_report),
                                Message::ReportSelection,
                            )
                            .width(Length::Units(200))
                            .font(font)
                            .size(15)
                            .style(<StyleTuple as Into<
                                iced::theme::Radio,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        );

                let mut col_report = Column::new()
                    .height(Length::Fill)
                    .push(row_radio_report)
                    .push(Text::new("     Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets      Bytes   Country").font(font))
                    .push(Text::new("------------------------------------------------------------------------------------------------------------------------").font(font))
                    ;
                let mut scroll_report = Column::new();
                for key_val in &sniffer.runtime_data.borrow().report_vec {
                    let entry_color = get_connection_color(key_val.1.traffic_type, sniffer.style);
                    let flag = get_flag(&key_val.1.country);
                    let entry_row = Row::new()
                        .push(
                            Text::new(format!(
                                "{}{}",
                                key_val.0.print_gui(),
                                key_val.1.print_gui()
                            ))
                            .style(iced::theme::Text::Color(entry_color))
                            .font(COURIER_PRIME_BOLD),
                        )
                        .push(flag)
                        .push(Text::new("  ").font(font))
                        .push(
                            button(
                                Text::new('X'.to_string())
                                    .font(ICONS)
                                    .size(14)
                                    .horizontal_alignment(Horizontal::Center)
                                    .vertical_alignment(Vertical::Center),
                            )
                            .padding(0)
                            .height(Length::Units(16))
                            .width(Length::Units(16))
                            .style(
                                StyleTuple(
                                    sniffer.style,
                                    if key_val.1.is_favorite {
                                        ElementType::Starred
                                    } else {
                                        ElementType::NotStarred
                                    },
                                )
                                .into(),
                            )
                            .on_press(if key_val.1.is_favorite {
                                Message::UnSaveConnection(key_val.1.index)
                            } else {
                                Message::SaveConnection(key_val.1.index)
                            }),
                        )
                        .push(Text::new("   ").font(font));
                    scroll_report = scroll_report.push(entry_row);
                }
                col_report = col_report.push(
                    Scrollable::new(scroll_report)
                        .horizontal_scroll(Properties::new())
                        .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                            StyleTuple(sniffer.style, ElementType::Standard),
                        )),
                );

                let row_report = Row::new().push(
                    Container::new(col_report)
                        .padding(5)
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .style(<StyleTuple as Into<iced::theme::Container>>::into(
                            StyleTuple(sniffer.style, ElementType::BorderedRound),
                        )),
                );

                body = body
                    .push(
                        Row::new()
                            .spacing(5)
                            .height(FillPortion(3))
                            .push(col_chart)
                            .push(
                                Container::new(col_packets)
                                    .width(FillPortion(1))
                                    .padding(10)
                                    .height(Length::Fill)
                                    .align_x(Horizontal::Center)
                                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                                        StyleTuple(sniffer.style, ElementType::BorderedRound),
                                    )),
                            ),
                    )
                    .push(
                        Column::new()
                            .align_items(Alignment::Center)
                            .height(FillPortion(2))
                            .width(Length::Fill)
                            .push(row_report),
                    );
            }
        }
    } else {
        // pcap threw an ERROR!
        let err_string = sniffer.pcap_error.clone().unwrap();

        let error_text = Text::new(format!(
            "An error occurred! \n\n\
                                                    {}",
            err_string
        ))
        .font(font);

        body = body
            .push(Row::new().height(FillPortion(1)))
            .push(Text::new('U'.to_string()).font(ICONS).size(60))
            .push(error_text)
            .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
            .push(Row::new().height(FillPortion(2)));
    }

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(FillPortion(HEIGHT_BODY))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
