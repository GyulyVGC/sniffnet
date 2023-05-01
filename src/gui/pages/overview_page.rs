//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: chart, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, vertical_space, Column, Container, Row, Scrollable, Text};
use iced::Length::{Fill, FillPortion};
use iced::{Alignment, Font, Length};
use iced_lazy::lazy;
use iced_native::widget::{horizontal_space, Rule};
use pcap::Device;
use thousands::Separable;

use crate::gui::components::radio::chart_radios;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::styles::style_constants::{get_font, FONT_SIZE_TITLE, ICONS};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::filters::Filters;
use crate::networking::types::search_parameters::SearchParameters;
use crate::report::get_report_entries::{get_app_entries, get_host_entries};
use crate::translations::translations::{
    application_protocol_translation, bytes_chart_translation, error_translation,
    filtered_bytes_no_percentage_translation, filtered_bytes_translation,
    filtered_packets_translation, network_adapter_translation, no_addresses_translation,
    none_translation, packets_chart_translation, some_observed_translation,
    traffic_rate_translation, waiting_translation,
};
use crate::translations::translations_2::{
    data_representation_translation, dropped_packets_translation, host_translation,
    of_total_translation, only_top_30_hosts_translation,
};
use crate::utils::countries::{get_flag_tooltip, FLAGS_WIDTH_BIG};
use crate::utils::formatted_strings::{
    get_active_filters_string, get_formatted_bytes_string, get_percentage_string,
};
use crate::{AppProtocol, ChartType, Language, RunningPage};

/// Computes the body of gui overview page
pub fn overview_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let mut body = Column::new();
    let mut tab_and_body = Column::new().height(Length::Fill);

    if sniffer.pcap_error.is_none() {
        // NO pcap error detected
        let observed = sniffer.runtime_data.all_packets;
        let filtered =
            sniffer.runtime_data.tot_sent_packets + sniffer.runtime_data.tot_received_packets;
        let dropped = sniffer.runtime_data.dropped_packets;
        let total = observed + dropped as u128;

        match (observed, filtered) {
            (0, 0) => {
                //no packets observed at all
                body = body_no_packets(&sniffer.device, font, sniffer.language, &sniffer.waiting);
            }
            (observed, 0) => {
                //no packets have been filtered but some have been observed
                body = body_no_observed(
                    &sniffer.filters,
                    observed,
                    font,
                    sniffer.language,
                    &sniffer.waiting,
                );
            }
            (_observed, filtered) => {
                //observed > filtered > 0 || observed = filtered > 0
                let tabs = get_pages_tabs(
                    [
                        RunningPage::Overview,
                        RunningPage::Inspect,
                        RunningPage::Notifications,
                    ],
                    &["d ", "5 ", "7 "],
                    &[
                        Message::TickInit,
                        Message::ChangeRunningPage(RunningPage::Inspect),
                        Message::ChangeRunningPage(RunningPage::Notifications),
                    ],
                    RunningPage::Overview,
                    sniffer.style,
                    sniffer.language,
                    sniffer.unread_notifications,
                );
                tab_and_body = tab_and_body.push(tabs);

                let mut chart_info_string = String::from("(");
                chart_info_string.push_str(
                    if sniffer.traffic_chart.chart_type.eq(&ChartType::Packets) {
                        packets_chart_translation(sniffer.language)
                    } else {
                        bytes_chart_translation(sniffer.language)
                    },
                );
                chart_info_string.push(')');
                let col_chart = Container::new(
                    Column::new()
                        .align_items(Alignment::Center)
                        .push(
                            Row::new()
                                .padding([10, 0, 15, 0])
                                .spacing(10)
                                .align_items(Alignment::Center)
                                .push(
                                    traffic_rate_translation(sniffer.language)
                                        .font(font)
                                        .size(FONT_SIZE_TITLE),
                                )
                                .push(Text::new(chart_info_string).font(font)),
                        )
                        .push(sniffer.traffic_chart.view()),
                )
                .width(Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(sniffer.style, ElementType::BorderedRound),
                ));

                let col_info = lazy(
                    (
                        total,
                        sniffer.style,
                        sniffer.language,
                        sniffer.traffic_chart.chart_type,
                    ),
                    move |_| lazy_col_info(total, filtered, dropped, sniffer),
                );

                let active_radio_report = sniffer.report_type;
                let num_favorites = sniffer.info_traffic.lock().unwrap().favorite_hosts.len();
                let row_report = lazy(
                    (
                        filtered,
                        active_radio_report,
                        num_favorites,
                        sniffer.style,
                        sniffer.language,
                        sniffer.traffic_chart.chart_type,
                    ),
                    move |_| lazy_row_report(sniffer),
                );

                body = body
                    .width(Length::Fill)
                    .padding(10)
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(
                        Row::new()
                            .spacing(10)
                            .height(FillPortion(5))
                            .push(
                                Container::new(col_info)
                                    .width(Length::Fixed(400.0))
                                    .padding([10, 5, 5, 5])
                                    .height(Length::Fill)
                                    .align_x(Horizontal::Center)
                                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                                        StyleTuple(sniffer.style, ElementType::BorderedRound),
                                    )),
                            )
                            .push(col_chart),
                    )
                    .push(
                        Container::new(row_report)
                            .align_x(Horizontal::Center)
                            .height(FillPortion(4)),
                    );
            }
        }
    } else {
        // pcap threw an ERROR!
        body = body_pcap_error(
            sniffer.pcap_error.as_ref().unwrap(),
            &sniffer.waiting,
            sniffer.language,
            font,
        );
    }

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(Length::Fill)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn body_no_packets(
    device: &Device,
    font: Font,
    language: Language,
    waiting: &str,
) -> Column<'static, Message> {
    let adapter_name = device.name.clone();
    let (icon_text, nothing_to_see_text) = if device.addresses.is_empty() {
        (
            Text::new('T'.to_string()).font(ICONS).size(60),
            no_addresses_translation(language, &adapter_name)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
    } else {
        (
            Text::new(waiting.len().to_string()).font(ICONS).size(60),
            waiting_translation(language, &adapter_name)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
    };

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space(FillPortion(1)))
        .push(icon_text)
        .push(vertical_space(Length::Fixed(15.0)))
        .push(nothing_to_see_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(vertical_space(FillPortion(2)))
}

fn body_no_observed(
    filters: &Filters,
    observed: u128,
    font: Font,
    language: Language,
    waiting: &str,
) -> Column<'static, Message> {
    let tot_packets_text = some_observed_translation(
        language,
        &observed.separate_with_spaces(),
        &get_active_filters_string(filters, language),
    )
    .horizontal_alignment(Horizontal::Center)
    .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space(FillPortion(1)))
        .push(Text::new('V'.to_string()).font(ICONS).size(60))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(tot_packets_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(vertical_space(FillPortion(2)))
}

fn body_pcap_error(
    pcap_error: &str,
    waiting: &str,
    language: Language,
    font: Font,
) -> Column<'static, Message> {
    // let err_string = pcap_error.clone().unwrap();
    let error_text = error_translation(language, pcap_error)
        .horizontal_alignment(Horizontal::Center)
        .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space(FillPortion(1)))
        .push(Text::new('U'.to_string()).font(ICONS).size(60))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(error_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(vertical_space(FillPortion(2)))
}

fn lazy_row_report(sniffer: &Sniffer) -> Row<'static, Message> {
    let mut row_report = Row::new()
        .padding(10)
        .height(Length::Fill)
        .width(Length::Fill);

    let col_host = col_host(840.0, sniffer);
    let col_app = col_app(250.0, sniffer);

    // if sniffer.report_type.eq(&ReportType::Favorites) && num_favorites == 0 {
    //     col_report = col_report.push(
    //         no_favorites_translation(sniffer.language)
    //             .font(font)
    //             .height(Length::Fill)
    //             .width(Length::Fill)
    //             .horizontal_alignment(Horizontal::Center)
    //             .vertical_alignment(Vertical::Center),
    //     );
    // } else {
    //     col_report = col_report
    //             .push(Text::new("       Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets      Bytes   Country").font(font))
    //             .push(Text::new("--------------------------------------------------------------------------------------------------------------------------").font(font))
    //         ;
    //     let mut scroll_report = Column::new();
    //     for key_val in get_report_entries(&sniffer.info_traffic.clone(), sniffer.report_type) {
    //         let entry_color = get_connection_color(key_val.1.traffic_type, sniffer.style);
    //         let mut entry_row = Row::new().align_items(Alignment::Center).push(
    //             Text::new(format!(
    //                 "  {}{}",
    //                 key_val.0.print_gui(),
    //                 key_val.1.print_gui()
    //             ))
    //             .style(iced::theme::Text::Color(entry_color))
    //             .font(SARASA_MONO_SC_BOLD),
    //         );
    //         if key_val.1.country.is_empty() {
    //             entry_row = entry_row
    //                 .push(
    //                     Text::new("?")
    //                         .width(Length::Fixed(FLAGS_WIDTH))
    //                         .style(iced::theme::Text::Color(entry_color))
    //                         .font(SARASA_MONO_SC_BOLD),
    //                 )
    //                 .push(Text::new("    "));
    //         } else {
    //             entry_row = entry_row
    //                 .push(get_flag_from_country_code(&key_val.1.country))
    //                 .push(Text::new("  "));
    //         }
    //         entry_row = entry_row
    //             .push(
    //                 button(
    //                     Text::new('X'.to_string())
    //                         .font(ICONS)
    //                         .size(14)
    //                         .horizontal_alignment(Horizontal::Center)
    //                         .vertical_alignment(Vertical::Center),
    //                 )
    //                 .padding(0)
    //                 .height(Length::Fixed(16.0))
    //                 .width(Length::Fixed(16.0))
    //                 .style(
    //                     StyleTuple(
    //                         sniffer.style,
    //                         if key_val.1.is_favorite {
    //                             ElementType::Starred
    //                         } else {
    //                             ElementType::Neutral
    //                         },
    //                     )
    //                     .into(),
    //                 )
    //                 .on_press(Message::AddOrRemoveFavorite(
    //                     key_val.1.index,
    //                     !key_val.1.is_favorite,
    //                 )),
    //             )
    //             .push(Text::new("  ").font(font));
    //         scroll_report = scroll_report.push(entry_row);
    //     }
    //     col_report = col_report.push(Container::new(
    //     ));
    // };
    row_report = row_report
        .push(col_host)
        .push(
            Rule::vertical(40).style(<StyleTuple as Into<iced::theme::Rule>>::into(StyleTuple(
                sniffer.style,
                ElementType::Standard,
            ))),
        )
        .push(col_app);

    Row::new().push(
        Container::new(row_report)
            .height(Length::Fill)
            .width(Length::Fixed(1170.0))
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(sniffer.style, ElementType::BorderedRound),
            )),
    )
}

fn col_host(width: f32, sniffer: &Sniffer) -> Column<'static, Message> {
    let font = get_font(sniffer.style);
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut col_host = Column::new()
        .width(Length::Fixed(width + 11.0))
        .push(
            Text::new(host_translation(sniffer.language))
                .font(font)
                .size(FONT_SIZE_TITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)));

    let mut scroll_host = Column::new()
        .width(Length::Fixed(width))
        .align_items(Alignment::Center);
    let entries = get_host_entries(&sniffer.info_traffic, chart_type);

    for (host, data_info_host) in &entries {
        let (mut incoming_bar_len, mut outgoing_bar_len) = get_bars_length(
            width * 0.86,
            chart_type,
            entries.get(0).unwrap().1.data_info.clone(),
            &data_info_host.data_info,
        );

        let star_button = button(
            Text::new('g'.to_string())
                .font(ICONS)
                .size(20)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .padding(0)
        .height(Length::Fixed(FLAGS_WIDTH_BIG * 0.75))
        .width(Length::Fixed(FLAGS_WIDTH_BIG))
        .style(
            StyleTuple(
                sniffer.style,
                if data_info_host.is_favorite {
                    ElementType::Starred
                } else {
                    ElementType::NotStarred
                },
            )
            .into(),
        )
        .on_press(Message::AddOrRemoveFavorite(
            host.clone(),
            !data_info_host.is_favorite,
        ));

        // normalize smaller values
        if incoming_bar_len > 0.0 && incoming_bar_len < 3.0 {
            incoming_bar_len = 3.0;
        }
        if outgoing_bar_len > 0.0 && outgoing_bar_len < 3.0 {
            outgoing_bar_len = 3.0;
        }

        let host_bar = Column::new()
            .width(Length::Fixed(width))
            .spacing(1)
            .push(
                Row::new()
                    .push(Text::new(host.domain.clone()))
                    .push(Text::new(if host.asn.name.is_empty() {
                        String::new()
                    } else {
                        format!(" - {}", host.asn.name)
                    }))
                    .push(horizontal_space(Length::FillPortion(1)))
                    .push(Text::new(if chart_type.eq(&ChartType::Packets) {
                        data_info_host.data_info.tot_packets().to_string()
                    } else {
                        let mut bytes_string =
                            get_formatted_bytes_string(data_info_host.data_info.tot_bytes())
                                .replace("  ", " ");
                        bytes_string.push('B');
                        bytes_string
                    })),
            )
            .push(
                Row::new()
                    .push(
                        Row::new()
                            .padding(0)
                            .width(Length::Fixed(incoming_bar_len))
                            .push(Rule::horizontal(1).style(<StyleTuple as Into<
                                iced::theme::Rule,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Incoming),
                            ))),
                    )
                    .push(
                        Row::new()
                            .padding(0)
                            .width(Length::Fixed(outgoing_bar_len))
                            .push(Rule::horizontal(1).style(<StyleTuple as Into<
                                iced::theme::Rule,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Outgoing),
                            ))),
                    ),
            );

        let content = Row::new()
            .align_items(Alignment::Center)
            .spacing(5)
            .push(star_button)
            .push(get_flag_tooltip(
                &host.country,
                FLAGS_WIDTH_BIG,
                data_info_host.is_local,
                data_info_host.traffic_type,
                sniffer.language,
                sniffer.style,
            ))
            .push(host_bar);

        scroll_host = scroll_host.push(
            button(content)
                .padding([5, 15, 5, 10])
                .on_press(Message::Search(SearchParameters {
                    app: None,
                    domain: Some(host.domain.clone()),
                    country: Some(host.country.clone()),
                    as_name: Some(host.asn.name.clone()),
                }))
                .style(StyleTuple(sniffer.style, ElementType::Neutral).into()),
        );
    }

    if entries.len() == 30 {
        scroll_host = scroll_host.push(vertical_space(Length::Fixed(25.0))).push(
            Text::new(only_top_30_hosts_translation(sniffer.language))
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        );
    }

    col_host = col_host.push(
        Scrollable::new(Container::new(scroll_host).width(Length::Fill)).style(
            <StyleTuple as Into<iced::theme::Scrollable>>::into(StyleTuple(
                sniffer.style,
                ElementType::Standard,
            )),
        ),
    );

    col_host
}

fn col_app(width: f32, sniffer: &Sniffer) -> Column<'static, Message> {
    let font = get_font(sniffer.style);
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut col_app = Column::new()
        .width(Length::Fixed(width + 11.0))
        .push(
            Text::new(application_protocol_translation(sniffer.language))
                .font(font)
                .size(FONT_SIZE_TITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)));

    let mut scroll_app = Column::new().width(Length::Fixed(width));
    let entries = get_app_entries(&sniffer.info_traffic, chart_type);

    for (app, data_info) in &entries {
        let (mut incoming_bar_len, mut outgoing_bar_len) = get_bars_length(
            width * 0.88,
            chart_type,
            entries.get(0).unwrap().1.clone(),
            data_info,
        );

        // check if Other is longer than the first entry
        if app.eq(&AppProtocol::Other) && incoming_bar_len + outgoing_bar_len > width * 0.88 {
            let incoming_proportion = incoming_bar_len / (incoming_bar_len + outgoing_bar_len);
            incoming_bar_len = width * 0.88 * incoming_proportion;
            outgoing_bar_len = width * 0.88 * (1.0 - incoming_proportion);
        }

        // normalize smaller values
        if incoming_bar_len > 0.0 && incoming_bar_len < 3.0 {
            incoming_bar_len = 3.0;
        }
        if outgoing_bar_len > 0.0 && outgoing_bar_len < 3.0 {
            outgoing_bar_len = 3.0;
        }

        let content = Column::new()
            .spacing(1)
            .width(Length::Fixed(width))
            .push(
                Row::new()
                    .push(Text::new(format!("{:?}", app)))
                    .push(horizontal_space(Length::FillPortion(1)))
                    .push(Text::new(if chart_type.eq(&ChartType::Packets) {
                        data_info.tot_packets().to_string()
                    } else {
                        let mut bytes_string =
                            get_formatted_bytes_string(data_info.tot_bytes()).replace("  ", " ");
                        bytes_string.push('B');
                        bytes_string
                    })),
            )
            .push(
                Row::new()
                    .push(
                        Row::new()
                            .padding(0)
                            .width(Length::Fixed(incoming_bar_len))
                            .push(Rule::horizontal(1).style(<StyleTuple as Into<
                                iced::theme::Rule,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Incoming),
                            ))),
                    )
                    .push(
                        Row::new()
                            .padding(0)
                            .width(Length::Fixed(outgoing_bar_len))
                            .push(Rule::horizontal(1).style(<StyleTuple as Into<
                                iced::theme::Rule,
                            >>::into(
                                StyleTuple(sniffer.style, ElementType::Outgoing),
                            ))),
                    ),
            );

        scroll_app = scroll_app.push(
            button(content)
                .padding([5, 15, 8, 10])
                .on_press(Message::Search(SearchParameters {
                    app: Some(*app),
                    domain: None,
                    country: None,
                    as_name: None,
                }))
                .style(StyleTuple(sniffer.style, ElementType::Neutral).into()),
        );
    }
    col_app = col_app.push(
        Scrollable::new(Container::new(scroll_app).width(Length::Fill)).style(
            <StyleTuple as Into<iced::theme::Scrollable>>::into(StyleTuple(
                sniffer.style,
                ElementType::Standard,
            )),
        ),
    );

    col_app
}

fn lazy_col_info(
    total: u128,
    filtered: u128,
    dropped: u32,
    sniffer: &Sniffer,
) -> Column<'static, Message> {
    let font = get_font(sniffer.style);
    let filtered_bytes =
        sniffer.runtime_data.tot_sent_bytes + sniffer.runtime_data.tot_received_bytes;

    #[cfg(not(target_os = "windows"))]
    let adapter_info = &sniffer.device.name;
    #[cfg(target_os = "windows")]
    let mut adapter_info = &sniffer.device.desc;
    #[cfg(target_os = "windows")]
    if adapter_info.is_empty() {
        adapter_info = &sniffer.device.name;
    }

    let col_device_filters = Column::new()
        .width(Length::FillPortion(1))
        .spacing(15)
        .push(network_adapter_translation(sniffer.language, adapter_info).font(font))
        .push(
            Text::new(get_active_filters_string(
                &sniffer.filters.clone(),
                sniffer.language,
            ))
            .font(font),
        );

    let col_data_representation = Column::new()
        .width(Length::FillPortion(1))
        .push(data_representation_translation(sniffer.language))
        .push(chart_radios(
            sniffer.traffic_chart.chart_type,
            font,
            sniffer.style,
            sniffer.language,
        ));

    let dropped_text = if dropped > 0 {
        let mut temp =
            dropped_packets_translation(sniffer.language, &dropped.separate_with_spaces());
        temp.push_str(&of_total_translation(
            sniffer.language,
            get_percentage_string(total, dropped as u128),
        ));
        temp
    } else {
        dropped_packets_translation(sniffer.language, &none_translation(sniffer.language))
    };
    let col_bytes_packets = Column::new()
        .spacing(15)
        .push(
            if dropped > 0 {
                filtered_bytes_no_percentage_translation(
                    sniffer.language,
                    &get_formatted_bytes_string(filtered_bytes),
                )
            } else {
                filtered_bytes_translation(
                    sniffer.language,
                    &get_formatted_bytes_string(filtered_bytes),
                    &get_percentage_string(sniffer.runtime_data.all_bytes, filtered_bytes),
                )
            }
            .font(font),
        )
        .push(
            filtered_packets_translation(
                sniffer.language,
                &filtered.separate_with_spaces(),
                &get_percentage_string(total, filtered),
            )
            .font(font),
        )
        .push(Text::new(dropped_text).font(font));

    Column::new()
        .align_items(Alignment::Center)
        .padding([5, 10])
        .push(
            Row::new()
                .height(Length::Fixed(120.0))
                .push(col_device_filters)
                .push(
                    Rule::vertical(25).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                        StyleTuple(sniffer.style, ElementType::Standard),
                    )),
                )
                .push(col_data_representation),
        )
        .push(
            Rule::horizontal(25).style(<StyleTuple as Into<iced::theme::Rule>>::into(StyleTuple(
                sniffer.style,
                ElementType::Standard,
            ))),
        )
        .push(
            Scrollable::new(col_bytes_packets)
                .width(Length::Fill)
                .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                    StyleTuple(sniffer.style, ElementType::Standard),
                )),
        )
}

fn get_bars_length(
    tot_width: f32,
    chart_type: ChartType,
    first_entry: DataInfo,
    data_info: &DataInfo,
) -> (f32, f32) {
    match chart_type {
        ChartType::Packets => (
            tot_width * data_info.incoming_packets as f32 / first_entry.tot_packets() as f32,
            tot_width * data_info.outgoing_packets as f32 / first_entry.tot_packets() as f32,
        ),
        ChartType::Bytes => (
            tot_width * data_info.incoming_bytes as f32 / first_entry.tot_bytes() as f32,
            tot_width * data_info.outgoing_bytes as f32 / first_entry.tot_bytes() as f32,
        ),
    }
}
