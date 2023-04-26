//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: chart, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Properties;
use iced::widget::{button, vertical_space, Column, Container, Row, Scrollable, Text, Tooltip};
use iced::Length::{Fill, FillPortion};
use iced::{alignment, Alignment, Font, Length};
use iced_lazy::lazy;
use iced_native::widget::tooltip::Position;
use pcap::Device;
use thousands::Separable;

use crate::gui::components::radio::{chart_radios, report_radios};
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::styles::style_constants::{get_font, ICONS, SARASA_MONO_SC_BOLD};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::filters::Filters;
use crate::report::get_report_entries::get_report_entries;
use crate::translations::translations::{
    error_translation, filtered_application_translation, filtered_bytes_translation,
    filtered_packets_translation, no_addresses_translation, no_favorites_translation,
    some_observed_translation, waiting_translation,
};
use crate::utils::countries::{get_flag_from_country_code, FLAGS_WIDTH};
use crate::utils::formatted_strings::{
    get_active_filters_string, get_active_filters_string_nobr, get_app_count_string,
    get_connection_color, get_formatted_bytes_string, get_open_report_tooltip,
    get_percentage_string,
};
use crate::{AppProtocol, Language, ReportType, RunningPage, StyleType};

//use dns_lookup::lookup_addr;

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
            (observed, filtered) => {
                //observed > filtered > 0 || observed = filtered > 0
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
                    sniffer.unread_notifications,
                );
                tab_and_body = tab_and_body.push(tabs);

                let row_radio_chart = chart_radios(
                    sniffer.traffic_chart.chart_type,
                    font,
                    sniffer.style,
                    sniffer.language,
                );
                let col_chart = Container::new(
                    Column::new()
                        .push(row_radio_chart)
                        .push(sniffer.traffic_chart.view()),
                )
                .width(Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(sniffer.style, ElementType::BorderedRound),
                ));

                let col_packets = lazy((observed, sniffer.style, sniffer.language), move |_| {
                    lazy_col_packets(observed, filtered, sniffer)
                });

                let active_radio_report = sniffer.report_type;
                let num_favorites = sniffer
                    .info_traffic
                    .lock()
                    .unwrap()
                    .favorite_connections
                    .len();
                let row_report = lazy(
                    (
                        filtered,
                        active_radio_report,
                        num_favorites,
                        sniffer.style,
                        sniffer.language,
                    ),
                    move |_| lazy_row_report(active_radio_report, num_favorites, sniffer),
                );

                body = body
                    .width(Length::Fill)
                    .padding(10)
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(
                        Row::new()
                            .spacing(10)
                            .height(FillPortion(3))
                            .push(col_chart)
                            .push(
                                Container::new(col_packets)
                                    .width(Length::Fixed(400.0))
                                    .padding([10, 5, 5, 5])
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
                                .push(get_button_open_report(
                                    sniffer.style,
                                    sniffer.language,
                                    font,
                                )),
                        )
                        .align_x(Horizontal::Center)
                        .height(FillPortion(2)),
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
        &get_active_filters_string_nobr(filters, language),
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

fn lazy_row_report(
    active_radio_report: ReportType,
    num_favorites: usize,
    sniffer: &Sniffer,
) -> Row<'static, Message> {
    let font = get_font(sniffer.style);
    let row_radio_report =
        report_radios(active_radio_report, font, sniffer.style, sniffer.language);
    let mut col_report = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .push(row_radio_report);

    if sniffer.report_type.eq(&ReportType::Favorites) && num_favorites == 0 {
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
                .push(Text::new("       Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets      Bytes   Country").font(font))
                .push(Text::new("--------------------------------------------------------------------------------------------------------------------------").font(font))
            ;
        let mut scroll_report = Column::new();
        // let info_traffic_lock = sniffer.info_traffic.lock().unwrap();
        for key_val in get_report_entries(&sniffer.info_traffic.clone(), sniffer.report_type) {
            let entry_color = get_connection_color(key_val.1.traffic_type, sniffer.style);
            let mut entry_row = Row::new().align_items(Alignment::Center).push(
                Text::new(format!(
                    "  {}{}",
                    key_val.0.print_gui(),
                    key_val.1.print_gui()
                ))
                .style(iced::theme::Text::Color(entry_color))
                .font(SARASA_MONO_SC_BOLD),
            );
            if key_val.1.country.is_empty() {
                entry_row = entry_row
                    .push(
                        Text::new("?")
                            .width(Length::Fixed(FLAGS_WIDTH))
                            .style(iced::theme::Text::Color(entry_color))
                            .font(SARASA_MONO_SC_BOLD),
                    )
                    .push(Text::new("    "));
            } else {
                entry_row = entry_row
                    .push(get_flag_from_country_code(&key_val.1.country))
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
                    .on_press(Message::AddOrRemoveFavorite(
                        key_val.1.index,
                        !key_val.1.is_favorite,
                    )),
                )
                .push(Text::new("  ").font(font));
            scroll_report = scroll_report.push(entry_row);
        }
        // drop(info_traffic_lock);
        col_report = col_report.push(Container::new(
            Scrollable::new(scroll_report)
                .horizontal_scroll(Properties::new())
                .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                    StyleTuple(sniffer.style, ElementType::Standard),
                )),
        ));
    };
    Row::new().push(
        Container::new(col_report)
            .padding([0, 5, 5, 5])
            .height(Length::Fill)
            .width(Length::Fixed(1080.0))
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(sniffer.style, ElementType::BorderedRound),
            )),
    )
}

fn lazy_col_packets(observed: u128, filtered: u128, sniffer: &Sniffer) -> Column<'static, Message> {
    let font = get_font(sniffer.style);
    let filtered_bytes =
        sniffer.runtime_data.tot_sent_bytes + sniffer.runtime_data.tot_received_bytes;
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
                &get_formatted_bytes_string(filtered_bytes),
                &get_percentage_string(sniffer.runtime_data.all_bytes, filtered_bytes),
            )
            .font(font),
        );
    if sniffer.filters.application.eq(&AppProtocol::Other) {
        col_packets = col_packets
            .push(Text::new(" "))
            .push(filtered_application_translation(sniffer.language).font(font))
            .push(
                Scrollable::new(
                    Text::new(get_app_count_string(
                        &sniffer.info_traffic.lock().unwrap().app_protocols,
                        filtered,
                    ))
                    .font(font),
                )
                .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                    StyleTuple(sniffer.style, ElementType::Standard),
                )),
            );
    }
    col_packets
}

fn get_button_open_report(
    style: StyleType,
    language: Language,
    font: Font,
) -> Tooltip<'static, Message> {
    let content = button(
        Text::new('8'.to_string())
            .font(ICONS)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(50.0))
    .width(Length::Fixed(75.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::OpenReport);

    Tooltip::new(content, get_open_report_tooltip(language), Position::Top)
        .gap(5)
        .font(font)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
