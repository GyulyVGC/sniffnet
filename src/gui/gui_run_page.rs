//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: charts, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, Column, Container, Radio, Row, Scrollable, Text};
use iced::Length::FillPortion;
use iced::{alignment, Alignment, Length};
use plotters::style::RGBColor;
use thousands::Separable;

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::colors::to_rgb_color;
use crate::structs::sniffer::Sniffer;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::countries::get_flag;
use crate::utility::get_formatted_strings::{
    get_active_filters_string, get_active_filters_string_nobr, get_app_count_string,
    get_connection_color, get_formatted_bytes_string, get_percentage_string, APP_VERSION,
};
use crate::utility::style_constants::{
    COURIER_PRIME, COURIER_PRIME_BOLD, COURIER_PRIME_BOLD_ITALIC, COURIER_PRIME_ITALIC,
    FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, HEIGHT_BODY, HEIGHT_FOOTER, HEIGHT_HEADER, ICONS,
};
use crate::{get_colors, AppProtocol, ChartType, ReportType};

/// Computes the body of gui run page
pub fn run_page(sniffer: &Sniffer) -> Column<Message> {
    let font = match to_rgb_color(get_colors(sniffer.style).text_body) {
        RGBColor(255, 255, 255) => COURIER_PRIME,
        _ => COURIER_PRIME_BOLD,
    };
    let font_footer = match to_rgb_color(get_colors(sniffer.style).text_headers) {
        RGBColor(255, 255, 255) => COURIER_PRIME_ITALIC,
        _ => COURIER_PRIME_BOLD_ITALIC,
    };
    let logo = Text::new('A'.to_string())
        .font(ICONS)
        .horizontal_alignment(Horizontal::Center)
        .size(95);

    let button_style = button(
        Text::new('K'.to_string())
            .font(ICONS)
            .width(Length::Units(25))
            .horizontal_alignment(Horizontal::Center)
            .size(20),
    )
    .padding(10)
    .height(Length::Units(40))
    .width(Length::Units(60))
    .style(StyleTuple(sniffer.style, ElementType::Standard).into())
    .on_press(Message::Style);

    let button_reset = button(
        Text::new('C'.to_string())
            .font(ICONS)
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(10)
    .height(Length::Units(40))
    .width(Length::Units(60))
        .style(StyleTuple(sniffer.style, ElementType::Standard).into())
    .on_press(Message::Reset);

    let button_overview = button(
        Text::new("Overview")
            .font(font)
            .size(FONT_SIZE_SUBTITLE)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .height(Length::Units(30))
    .width(Length::FillPortion(1))
        .style(StyleTuple(sniffer.style, ElementType::TabActive).into())
    .on_press(Message::TickInit); //do nothing, just update the page

    let button_inspect = button(
        Text::new("Inspect")
            .font(font)
            .size(FONT_SIZE_SUBTITLE)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .height(Length::Units(30))
    .width(Length::FillPortion(1))
        .style(StyleTuple(sniffer.style, ElementType::TabInactive).into())
    .on_press(Message::Reset);

    let button_settings = button(
        Text::new("Settings")
            .font(font)
            .size(FONT_SIZE_SUBTITLE)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .height(Length::Units(30))
    .width(Length::FillPortion(1))
        .style(StyleTuple(sniffer.style, ElementType::TabInactive).into())
    .on_press(Message::Reset);

    let header = Container::new(
        Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Container::new(button_reset)
                    .width(Length::FillPortion(1))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            )
            .push(
                Container::new(Row::new().align_items(Alignment::Center).push(logo))
                    .width(Length::FillPortion(6))
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            )
            .push(
                Container::new(button_style)
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            ),
    )
    .height(Length::FillPortion(HEIGHT_HEADER))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
        StyleTuple(sniffer.style, ElementType::Headers),
    ));

    // let _button_report = Button::new(
    //     Text::new("Open full report")
    //         .font(font)
    //         .horizontal_alignment(alignment::Horizontal::Center)
    //         .vertical_alignment(alignment::Vertical::Center),
    // )
    // .padding(10)
    // .height(Length::Units(35))
    // .width(Length::Units(200))
    // // .style(StyleTuple(sniffer.style, ElementType::Standard))
    // .on_press(Message::OpenReport);

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

    let mut tab_body = Column::new().height(Length::FillPortion(HEIGHT_BODY));

    if sniffer.pcap_error.lock().unwrap().is_none() {
        // NO pcap error detected

        match (observed, filtered) {
            (0, 0) => {
                //no packets observed at all

                let adapter_name = &*sniffer.device.clone().lock().unwrap().name.clone();
                let (icon_text, nothing_to_see_text) = if !sniffer
                    .device
                    .lock()
                    .unwrap()
                    .addresses
                    .is_empty()
                {
                    (Text::new(sniffer.waiting.len().to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                                              Network adapter: {}\n\n\
                                                              Are you sure you are connected to the internet and you have selected the right adapter?", adapter_name)).font(font))
                } else {
                    (Text::new('T'.to_string()).font(ICONS).size(60),
                     Text::new(format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                                              Network adapter: {}\n\n\
                                                              If you are sure you are connected to the internet, try choosing a different adapter.", adapter_name)).font(font))
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

                let tabs = Row::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .push(button_overview)
                    .push(button_inspect)
                    .push(button_settings);

                tab_body = tab_body.push(tabs);

                let active_radio_chart = sniffer.traffic_chart.chart_type;
                let row_radio_chart = Row::new()
                    .padding(15)
                    .spacing(10)
                    .push(
                        Text::new("Plotted data:    ")
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
                                Text::new(get_app_count_string(app_protocols, filtered as u128))
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
                    let entry_color = get_connection_color(key_val.1.traffic_type, &sniffer.style);
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
                col_report = col_report.push(Scrollable::new(scroll_report)
                    .style(<StyleTuple as Into<iced_style::theme::Scrollable>>::into(
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

        // if sniffer.waiting.len() > 2 {
        //     sniffer.waiting = "".to_string();
        // }
        // sniffer.waiting = ".".repeat(sniffer.waiting.len() + 1);

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

    let button_github = button(
        Text::new('H'.to_string())
            .font(ICONS)
            .size(24)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .height(Length::Units(35))
    .width(Length::Units(35))
        .style(StyleTuple(sniffer.style, ElementType::Standard).into())
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
        .style(<StyleTuple as Into<iced_style::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Headers),
        ));

    Column::new()
        .push(header)
        .push(tab_body.push(body))
        .push(footer)
}
