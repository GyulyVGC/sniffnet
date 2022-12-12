//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: charts, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Radio, Row, Scrollable, Text};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use thousands::Separable;

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

    let tabs = get_tabs(
        &["Overview", "Inspect", "Prova prova Try try"],
        &[Message::TickInit, Message::Reset, Message::Reset],
        "Overview",
        sniffer.style,
    );

    let runtime_data_lock = sniffer.runtime_data.lock().unwrap();
    let observed = runtime_data_lock.all_packets;
    let filtered = runtime_data_lock.tot_sent_packets + runtime_data_lock.tot_received_packets;
    let observed_bytes = runtime_data_lock.all_bytes;
    let filtered_bytes = runtime_data_lock.tot_sent_bytes + runtime_data_lock.tot_received_bytes;
    let app_protocols = runtime_data_lock.app_protocols.clone();
    drop(runtime_data_lock);
    let filtered_bytes_string = get_formatted_bytes_string(filtered_bytes as u128);

    let mut body = Column::new()
        .width(Length::Fill)
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(Length::FillPortion(HEIGHT_BODY));

    if sniffer.pcap_error.lock().unwrap().is_none() {
        // NO pcap error detected

        match (observed, filtered) {
            (0, 0) => {
                //no packets observed at all

                let adapter_name = &*sniffer.device.clone().lock().unwrap().name.clone();
                let (icon_text, nothing_to_see_text) = if sniffer
                    .device
                    .lock()
                    .unwrap()
                    .addresses
                    .is_empty()
                {
                    (Text::new('T'.to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                                              Network adapter: {}\n\n\
                                                              If you are sure you are connected to the internet, try choosing a different adapter.", adapter_name)).font(font))
                } else {
                    (Text::new(sniffer.waiting.len().to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                                              Network adapter: {}\n\n\
                                                              Are you sure you are connected to the internet and you have selected the right adapter?", adapter_name)).font(font))
                };
                body = body
                    .push(Row::new().height(Length::FillPortion(1)))
                    .push(icon_text)
                    .push(nothing_to_see_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(Row::new().height(Length::FillPortion(2)));
            }

            (observed, 0) => {
                //no packets have been filtered but some have been observed

                let tot_packets_text = Text::new(format!("Total intercepted packets: {}\n\n\
                                                    Filtered packets: 0\n\n\
                                                    Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{}",
                                                         observed.separate_with_spaces(), get_active_filters_string_nobr(sniffer.filters.clone()))).font(font);

                body = body
                    .push(Row::new().height(Length::FillPortion(1)))
                    .push(Text::new('V'.to_string()).font(ICONS).size(60))
                    .push(tot_packets_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(Row::new().height(Length::FillPortion(2)));
            }

            (observed, filtered) => {
                //observed > filtered > 0 || observed = filtered > 0

                tab_and_body = tab_and_body.push(tabs);

                let active_radio_chart = sniffer.traffic_chart.chart_type;
                let row_radio_chart = Row::new()
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
                            iced_style::theme::Radio,
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
                            iced_style::theme::Radio,
                        >>::into(
                            StyleTuple(sniffer.style, ElementType::Standard),
                        )),
                    );

                let col_chart = Container::new(
                    Column::new()
                        .push(row_radio_chart)
                        .push(sniffer.traffic_chart.view()),
                )
                .width(Length::FillPortion(2))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
                    StyleTuple(sniffer.style, ElementType::BorderedRound),
                ));

                let mut col_packets = Column::new()
                    .padding(10)
                    //.push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()).font(font))
                    //.push(iced::Text::new(confy::get_configuration_file_path("sniffnet", None).unwrap().to_string_lossy()).font(font))
                    .push(Text::new(get_active_filters_string(sniffer.filters.clone())).font(font))
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
                if sniffer
                    .filters
                    .lock()
                    .unwrap()
                    .application
                    .eq(&AppProtocol::Other)
                {
                    col_packets = col_packets
                        .push(Text::new(" "))
                        .push(Text::new("Filtered packets per application protocol:").font(font))
                        .push(
                            Scrollable::new(
                                Text::new(get_app_count_string(&app_protocols, filtered as u128))
                                    .font(font),
                            )
                            .style(<StyleTuple as Into<
                                iced_style::theme::Scrollable,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Standard),
                            )),
                        );
                }

                let active_radio_report = sniffer.report_type;
                let row_radio_report = Row::new()
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
                            iced_style::theme::Radio,
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
                            iced_style::theme::Radio,
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
                            iced_style::theme::Radio,
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
                for key_val in sniffer.runtime_data.lock().unwrap().report_vec.iter() {
                    let entry_color = get_connection_color(key_val.1.traffic_type, sniffer.style);
                    let flag = get_flag(&key_val.1.country);
                    scroll_report = scroll_report.push(
                        Row::new()
                            .push(
                                Text::new(format!(
                                    "{}{}",
                                    key_val.0.print_gui(),
                                    key_val.1.print_gui()
                                ))
                                .style(iced_style::theme::Text::Color(entry_color))
                                .font(COURIER_PRIME_BOLD),
                            )
                            .push(flag)
                            .push(Text::new("   ").font(font)),
                    );
                }
                col_report =
                    col_report.push(Scrollable::new(scroll_report).style(<StyleTuple as Into<
                        iced_style::theme::Scrollable,
                    >>::into(
                        StyleTuple(sniffer.style, ElementType::Standard),
                    )));

                let row_report = Row::new().push(
                    Container::new(col_report)
                        .padding(5)
                        .height(Length::Fill)
                        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
                            StyleTuple(sniffer.style, ElementType::BorderedRound),
                        )),
                );

                body = body
                    .push(
                        Row::new()
                            .spacing(5)
                            .height(Length::FillPortion(3))
                            .push(col_chart)
                            .push(
                                Container::new(col_packets)
                                    .width(Length::FillPortion(1))
                                    .padding(10)
                                    .height(Length::Fill)
                                    .style(
                                        <StyleTuple as Into<iced_style::theme::Container>>::into(
                                            StyleTuple(sniffer.style, ElementType::BorderedRound),
                                        ),
                                    ),
                            ),
                    )
                    .push(
                        Column::new()
                            .align_items(Alignment::Center)
                            .height(Length::FillPortion(2))
                            .width(Length::Fill)
                            .push(row_report),
                    );
            }
        }
    } else {
        // pcap threw an ERROR!
        let err_string = sniffer.pcap_error.lock().unwrap().clone().unwrap();

        let error_text = Text::new(format!(
            "An error occurred! \n\n\
                                                    {}",
            err_string
        ))
        .font(font);

        body = body
            .push(Row::new().height(Length::FillPortion(1)))
            .push(Text::new('U'.to_string()).font(ICONS).size(60))
            .push(error_text)
            .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
            .push(Row::new().height(Length::FillPortion(2)));
    }

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(FillPortion(HEIGHT_BODY))
        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
