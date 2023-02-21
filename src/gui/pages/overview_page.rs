//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: charts, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Properties;
use iced::widget::{button, vertical_space, Column, Container, Row, Scrollable, Text, Tooltip};
use iced::Length::FillPortion;
use iced::{alignment, Alignment, Length};
use iced_native::widget::tooltip::Position;
use thousands::Separable;
//use dns_lookup::lookup_addr;

use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::gui::components::radio::{chart_radios, report_radios};
use crate::gui::components::tab::get_pages_tabs;
use crate::structs::sniffer::Sniffer;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::countries::{get_flag, FLAGS_WIDTH};
use crate::utility::get_formatted_strings::{
    get_active_filters_string, get_active_filters_string_nobr, get_app_count_string,
    get_connection_color, get_formatted_bytes_string, get_percentage_string, get_report_path,
};
use crate::utility::style_constants::{get_font, HEIGHT_BODY, ICONS, INCONSOLATA_BOLD};
use crate::utility::translations::{
    error_translation, filtered_application_translation, filtered_bytes_translation,
    filtered_packets_translation, no_addresses_translation, no_favorites_translation,
    open_report_translation, some_observed_translation, waiting_translation,
};
use crate::{AppProtocol, ReportType, RunningPage};

/// Computes the body of gui run page
pub fn overview_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let mut body = Column::new()
        .width(Length::Fill)
        .padding(5)
        .spacing(5)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(FillPortion(HEIGHT_BODY));

    if sniffer.pcap_error.is_none() {
        // NO pcap error detected

        let tabs = get_pages_tabs(
            [
                RunningPage::Overview,
                //RunningPage::Inspect,
                RunningPage::Notifications,
            ],
            &["d ", "7 "],
            &[
                Message::TickInit,
                //Message::ChangeRunningPage(RunningPage::Inspect),
                Message::ChangeRunningPage(RunningPage::Notifications),
            ],
            RunningPage::Overview,
            sniffer.style,
            sniffer.language,
        );

        let observed = sniffer.runtime_data.borrow().all_packets;
        let filtered = sniffer.runtime_data.borrow().tot_sent_packets
            + sniffer.runtime_data.borrow().tot_received_packets;
        let observed_bytes = sniffer.runtime_data.borrow().all_bytes;
        let filtered_bytes = sniffer.runtime_data.borrow().tot_sent_bytes
            + sniffer.runtime_data.borrow().tot_received_bytes;
        let app_protocols = sniffer.runtime_data.borrow().app_protocols.clone();
        let filtered_bytes_string = get_formatted_bytes_string(filtered_bytes);

        match (observed, filtered) {
            (0, 0) => {
                //no packets observed at all

                let adapter_name = sniffer.device.name.clone();
                let (icon_text, nothing_to_see_text) = if sniffer.device.addresses.is_empty() {
                    (
                        Text::new('T'.to_string()).font(ICONS).size(60),
                        no_addresses_translation(sniffer.language, &adapter_name)
                            .horizontal_alignment(Horizontal::Center)
                            .font(font),
                    )
                } else {
                    (
                        Text::new(sniffer.waiting.len().to_string())
                            .font(ICONS)
                            .size(60),
                        waiting_translation(sniffer.language, &adapter_name)
                            .horizontal_alignment(Horizontal::Center)
                            .font(font),
                    )
                };
                body = body
                    .push(vertical_space(FillPortion(1)))
                    .push(icon_text)
                    .push(vertical_space(Length::Fixed(15.0)))
                    .push(nothing_to_see_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(vertical_space(FillPortion(2)));
            }

            (observed, 0) => {
                //no packets have been filtered but some have been observed

                let tot_packets_text = some_observed_translation(
                    sniffer.language,
                    &observed.separate_with_spaces(),
                    &get_active_filters_string_nobr(&sniffer.filters.clone(), sniffer.language),
                )
                .horizontal_alignment(Horizontal::Center)
                .font(font);

                body = body
                    .push(vertical_space(FillPortion(1)))
                    .push(Text::new('V'.to_string()).font(ICONS).size(60))
                    .push(vertical_space(Length::Fixed(15.0)))
                    .push(tot_packets_text)
                    .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
                    .push(vertical_space(FillPortion(2)));
            }

            (observed, filtered) => {
                //observed > filtered > 0 || observed = filtered > 0

                tab_and_body = tab_and_body.push(tabs);

                let active_radio_chart = sniffer.traffic_chart.chart_type;
                let row_radio_chart =
                    chart_radios(active_radio_chart, font, sniffer.style, sniffer.language);
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
                    //.push(iced::Text::new(std::env::current_dir().unwrap().to_str().unwrap()).font(font))
                    //.push(iced::Text::new(confy::get_configuration_file_path("sniffnet", None).unwrap().to_string_lossy()).font(font))
                    //.push(Text::new(lookup_addr(&"8.8.8.8".parse().unwrap()).unwrap()).font(font))
                    .push(
                        Text::new(get_active_filters_string(
                            &sniffer.filters.clone(),
                            sniffer.language,
                        ))
                        .font(font),
                    )
                    .push(Text::new(" "))
                    .push(
                        filtered_packets_translation(
                            sniffer.language,
                            &filtered.separate_with_spaces(),
                            &get_percentage_string(observed, filtered),
                        )
                        .font(font),
                    )
                    .push(Text::new(" "))
                    .push(
                        filtered_bytes_translation(
                            sniffer.language,
                            &filtered_bytes_string,
                            &get_percentage_string(observed_bytes, filtered_bytes),
                        )
                        .font(font),
                    );
                if sniffer.filters.application.eq(&AppProtocol::Other) {
                    col_packets = col_packets
                        .push(Text::new(" "))
                        .push(filtered_application_translation(sniffer.language).font(font))
                        .push(
                            Scrollable::new(
                                Text::new(get_app_count_string(&app_protocols, filtered))
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
                    report_radios(active_radio_report, font, sniffer.style, sniffer.language);

                let mut col_report = Column::new()
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .push(row_radio_report);

                if sniffer.report_type.eq(&ReportType::Favorites)
                    && sniffer.runtime_data.borrow().report_vec.is_empty()
                {
                    col_report = col_report.push(
                        no_favorites_translation(sniffer.language)
                            .font(font)
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Center)
                            .vertical_alignment(Vertical::Center),
                    );
                } else {
                    col_report = col_report
                        .push(Text::new("     Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets      Bytes   Country").font(font))
                        .push(Text::new("------------------------------------------------------------------------------------------------------------------------").font(font))
                    ;
                    let mut scroll_report = Column::new();
                    for key_val in &sniffer.runtime_data.borrow().report_vec {
                        let entry_color =
                            get_connection_color(key_val.1.traffic_type, sniffer.style);
                        let mut entry_row = Row::new().align_items(Alignment::Center).push(
                            Text::new(format!(
                                "{}{}",
                                key_val.0.print_gui(),
                                key_val.1.print_gui()
                            ))
                            .style(iced::theme::Text::Color(entry_color))
                            .font(INCONSOLATA_BOLD),
                        );
                        if key_val.1.country.is_empty() {
                            entry_row = entry_row
                                .push(
                                    Text::new("?")
                                        .width(Length::Fixed(FLAGS_WIDTH))
                                        .style(iced::theme::Text::Color(entry_color))
                                        .font(INCONSOLATA_BOLD),
                                )
                                .push(Text::new("    "));
                        } else {
                            entry_row = entry_row
                                .push(get_flag(&key_val.1.country))
                                .push(Text::new("  "));
                        }
                        entry_row = entry_row
                            .push(
                                button(
                                    Text::new('X'.to_string())
                                        .font(ICONS)
                                        .size(14)
                                        .horizontal_alignment(Horizontal::Center)
                                        .vertical_alignment(Vertical::Center),
                                )
                                .padding(0)
                                .height(Length::Fixed(16.0))
                                .width(Length::Fixed(16.0))
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
                                .on_press(
                                    if key_val.1.is_favorite {
                                        Message::UnSaveConnection(key_val.1.index)
                                    } else {
                                        Message::SaveConnection(key_val.1.index)
                                    },
                                ),
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
                };

                let row_report = Row::new().push(
                    Container::new(col_report)
                        .padding(5)
                        .height(Length::Fill)
                        .width(Length::Fixed(1100.0))
                        .style(<StyleTuple as Into<iced::theme::Container>>::into(
                            StyleTuple(sniffer.style, ElementType::BorderedRound),
                        )),
                );

                let open_report_translation = open_report_translation(sniffer.language);
                let report_path = get_report_path().to_string_lossy().to_string();
                let open_report_tooltip = format!(
                    "{:^len$}\n{report_path}",
                    open_report_translation,
                    len = report_path.len()
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
                        Container::new(
                            Row::new()
                                .spacing(15)
                                .align_items(Alignment::Center)
                                .width(Length::Fill)
                                .push(row_report)
                                .push(
                                    Tooltip::new(
                                        button(
                                            Text::new('8'.to_string())
                                                .font(ICONS)
                                                .horizontal_alignment(alignment::Horizontal::Center)
                                                .vertical_alignment(alignment::Vertical::Center),
                                        )
                                        .padding(10)
                                        .height(Length::Fixed(50.0))
                                        .width(Length::Fixed(75.0))
                                        .style(
                                            StyleTuple(sniffer.style, ElementType::Standard).into(),
                                        )
                                        .on_press(Message::OpenReport),
                                        open_report_tooltip,
                                        Position::Top,
                                    )
                                    .gap(5)
                                    .font(font)
                                    .style(
                                        <StyleTuple as Into<iced::theme::Container>>::into(
                                            StyleTuple(sniffer.style, ElementType::Tooltip),
                                        ),
                                    ),
                                ),
                        )
                        .align_x(Horizontal::Center)
                        .height(FillPortion(2)),
                    );
            }
        }
    } else {
        // pcap threw an ERROR!
        let err_string = sniffer.pcap_error.clone().unwrap();

        let error_text = error_translation(sniffer.language, &err_string)
            .horizontal_alignment(Horizontal::Center)
            .font(font);

        body = body
            .push(vertical_space(FillPortion(1)))
            .push(Text::new('U'.to_string()).font(ICONS).size(60))
            .push(vertical_space(Length::Fixed(15.0)))
            .push(error_text)
            .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
            .push(vertical_space(FillPortion(2)));
    }

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(FillPortion(HEIGHT_BODY))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}
