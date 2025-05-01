//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: chart, detailed connections data
//! and overall statistics about the filtered traffic.

use crate::chart::types::donut_chart::donut_chart;
use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_pictures::FLAGS_WIDTH_BIG;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::FONT_SIZE_TITLE;
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::palette_extension::PaletteExtension;
use crate::gui::types::message::Message;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::filters::Filters;
use crate::networking::types::host::Host;
use crate::networking::types::my_device::MyDevice;
use crate::report::get_report_entries::{get_host_entries, get_service_entries};
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::translations::translations::{
    active_filters_translation, error_translation, incoming_translation,
    network_adapter_translation, no_addresses_translation, none_translation, outgoing_translation,
    some_observed_translation, traffic_rate_translation, waiting_translation,
};
use crate::translations::translations_2::{
    data_representation_translation, dropped_translation, host_translation,
    only_top_30_items_translation,
};
use crate::translations::translations_3::{service_translation, unsupported_link_type_translation};
use crate::translations::translations_4::excluded_translation;
use crate::utils::formatted_strings::get_active_filters_string;
use crate::utils::types::icon::Icon;
use crate::{ByteMultiple, ChartType, ConfigSettings, Language, RunningPage, StyleType};
use iced::Length::{Fill, FillPortion};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{
    Button, Column, Container, Row, Rule, Scrollable, Space, Text, Tooltip, button,
    horizontal_space, lazy, vertical_space,
};
use iced::{Alignment, Font, Length, Padding, Shrink};
use std::fmt::Write;

/// Computes the body of gui overview page
pub fn overview_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let mut body = Column::new();
    let mut tab_and_body = Column::new().height(Length::Fill);

    if let Some(error) = sniffer.pcap_error.as_ref() {
        // pcap threw an ERROR!
        body = body_pcap_error(error, &sniffer.waiting, language, font);
    } else {
        // NO pcap error detected
        let observed = sniffer.runtime_data.all_packets;
        let filtered = sniffer.runtime_data.tot_out_packets + sniffer.runtime_data.tot_in_packets;
        let dropped = sniffer.runtime_data.dropped_packets;
        let total = observed + u128::from(dropped);

        match (observed, filtered) {
            (0, 0) => {
                //no packets observed at all
                body = body_no_packets(&sniffer.device, font, language, &sniffer.waiting);
            }
            (observed, 0) => {
                //no packets have been filtered but some have been observed
                body =
                    body_no_observed(&sniffer.filters, observed, font, language, &sniffer.waiting);
            }
            (_observed, filtered) => {
                //observed > filtered > 0 || observed = filtered > 0
                let tabs = get_pages_tabs(
                    RunningPage::Overview,
                    font,
                    font_headers,
                    language,
                    sniffer.unread_notifications,
                );
                tab_and_body = tab_and_body.push(tabs);

                let container_chart = container_chart(sniffer, font);

                let container_info = lazy(
                    (total, style, language, sniffer.traffic_chart.chart_type),
                    move |_| lazy_col_info(sniffer),
                );

                let num_favorites = sniffer.info_traffic.lock().unwrap().favorite_hosts.len();
                let container_report = lazy(
                    (
                        filtered,
                        num_favorites,
                        style,
                        language,
                        sniffer.traffic_chart.chart_type,
                        sniffer.host_sort_type,
                        sniffer.service_sort_type,
                    ),
                    move |_| lazy_row_report(sniffer),
                );

                body = body
                    .width(Length::Fill)
                    .padding(10)
                    .spacing(10)
                    .align_x(Alignment::Center)
                    .push(
                        Row::new()
                            .height(280)
                            .spacing(10)
                            .push(container_info)
                            .push(container_chart),
                    )
                    .push(container_report);
            }
        }
    }

    Container::new(Column::new().push(tab_and_body.push(body))).height(Length::Fill)
}

fn body_no_packets<'a>(
    device: &MyDevice,
    font: Font,
    language: Language,
    waiting: &str,
) -> Column<'a, Message, StyleType> {
    let link_type = device.link_type;
    let mut adapter_info = device.name.clone();
    let _ = write!(
        adapter_info,
        "\n{}",
        link_type.full_print_on_one_line(language)
    );
    let (icon_text, nothing_to_see_text) = if !link_type.is_supported() {
        (
            Icon::Warning.to_text().size(60),
            unsupported_link_type_translation(language, &adapter_info)
                .align_x(Alignment::Center)
                .font(font),
        )
    } else if device.addresses.lock().unwrap().is_empty() {
        (
            Icon::Warning.to_text().size(60),
            no_addresses_translation(language, &adapter_info)
                .align_x(Alignment::Center)
                .font(font),
        )
    } else {
        (
            Icon::get_hourglass(waiting.len()).size(60),
            waiting_translation(language, &adapter_info)
                .align_x(Alignment::Center)
                .font(font),
        )
    };

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_x(Alignment::Center)
        .push(vertical_space())
        .push(icon_text)
        .push(Space::with_height(15))
        .push(nothing_to_see_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn body_no_observed<'a>(
    filters: &Filters,
    observed: u128,
    font: Font,
    language: Language,
    waiting: &str,
) -> Column<'a, Message, StyleType> {
    let tot_packets_text = some_observed_translation(language, observed)
        .align_x(Alignment::Center)
        .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_x(Alignment::Center)
        .push(vertical_space())
        .push(Icon::Funnel.to_text().size(60))
        .push(get_active_filters_col(filters, language, font))
        .push(Rule::horizontal(20))
        .push(tot_packets_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn body_pcap_error<'a>(
    pcap_error: &'a str,
    waiting: &'a str,
    language: Language,
    font: Font,
) -> Column<'a, Message, StyleType> {
    let error_text = error_translation(language, pcap_error)
        .align_x(Alignment::Center)
        .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_x(Alignment::Center)
        .push(vertical_space())
        .push(Icon::Error.to_text().size(60))
        .push(Space::with_height(15))
        .push(error_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn lazy_row_report<'a>(sniffer: &Sniffer) -> Container<'a, Message, StyleType> {
    let col_host = col_host(840.0, sniffer);
    let col_service = col_service(250.0, sniffer);

    let row_report = Row::new()
        .padding(Padding::new(10.0).top(0).bottom(5))
        .push(col_host)
        .push(
            Column::new()
                .padding(Padding::ZERO.top(10).bottom(5))
                .push(Rule::vertical(40)),
        )
        .push(col_service);

    Container::new(row_report)
        .height(Shrink)
        .class(ContainerType::BorderedRound)
}

fn col_host<'a>(width: f32, sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut scroll_host = Column::new().width(width).align_x(Alignment::Center);
    let entries = get_host_entries(&sniffer.info_traffic, chart_type, sniffer.host_sort_type);
    let first_entry_data_info = entries
        .iter()
        .map(|(_, d)| d.data_info)
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, chart_type))
        .unwrap_or_default();

    for (host, data_info_host) in &entries {
        let (incoming_bar_len, outgoing_bar_len) = get_bars_length(
            width * 0.86,
            chart_type,
            &first_entry_data_info,
            &data_info_host.data_info,
        );

        let star_button = get_star_button(data_info_host.is_favorite, host.clone());

        let host_bar = Column::new()
            .width(width)
            .spacing(1)
            .push(
                Row::new()
                    .push(Text::new(host.domain.clone()).font(font))
                    .push(
                        Text::new(if host.asn.name.is_empty() {
                            String::new()
                        } else {
                            format!(" - {}", host.asn.name)
                        })
                        .font(font),
                    )
                    .push(horizontal_space())
                    .push(
                        Text::new(if chart_type.eq(&ChartType::Packets) {
                            data_info_host.data_info.tot_packets().to_string()
                        } else {
                            ByteMultiple::formatted_string(data_info_host.data_info.tot_bytes())
                        })
                        .font(font),
                    ),
            )
            .push(get_bars(incoming_bar_len, outgoing_bar_len));

        let content = Row::new()
            .align_y(Alignment::Center)
            .spacing(5)
            .push(star_button)
            .push(get_flag_tooltip(
                host.country,
                data_info_host,
                language,
                font,
                false,
            ))
            .push(host_bar);

        scroll_host = scroll_host.push(
            button(content)
                .padding(Padding::new(5.0).right(15).left(10))
                .on_press(Message::Search(SearchParameters::new_host_search(host)))
                .class(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_host = scroll_host.push(Space::with_height(25)).push(
            Text::new(only_top_30_items_translation(language))
                .font(font)
                .align_x(Alignment::Center),
        );
    }

    Column::new()
        .width(width + 11.0)
        .push(
            Row::new()
                .height(45)
                .align_y(Alignment::Center)
                .push(
                    Text::new(host_translation(language))
                        .font(font)
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(horizontal_space())
                .push(sort_arrows(
                    sniffer.host_sort_type,
                    Message::HostSortSelection,
                )),
        )
        .push(
            Scrollable::with_direction(
                scroll_host,
                Direction::Vertical(ScrollbarType::properties()),
            )
            .width(Length::Fill),
        )
}

fn col_service<'a>(width: f32, sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut scroll_service = Column::new().width(width).align_x(Alignment::Center);
    let entries = get_service_entries(&sniffer.info_traffic, chart_type, sniffer.service_sort_type);
    let first_entry_data_info = entries
        .iter()
        .map(|&(_, d)| d)
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, chart_type))
        .unwrap_or_default();

    for (service, data_info) in &entries {
        let (incoming_bar_len, outgoing_bar_len) =
            get_bars_length(width * 0.88, chart_type, &first_entry_data_info, data_info);

        let content = Column::new()
            .spacing(1)
            .width(width)
            .push(
                Row::new()
                    .push(Text::new(service.to_string()).font(font))
                    .push(horizontal_space())
                    .push(
                        Text::new(if chart_type.eq(&ChartType::Packets) {
                            data_info.tot_packets().to_string()
                        } else {
                            ByteMultiple::formatted_string(data_info.tot_bytes())
                        })
                        .font(font),
                    ),
            )
            .push(get_bars(incoming_bar_len, outgoing_bar_len));

        scroll_service = scroll_service.push(
            button(content)
                .padding(Padding::new(5.0).right(15).bottom(8).left(10))
                .on_press(Message::Search(SearchParameters::new_service_search(
                    service,
                )))
                .class(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_service = scroll_service.push(Space::with_height(25)).push(
            Text::new(only_top_30_items_translation(language))
                .font(font)
                .align_x(Alignment::Center),
        );
    }

    Column::new()
        .width(width + 11.0)
        .push(
            Row::new()
                .height(45)
                .align_y(Alignment::Center)
                .push(
                    Text::new(service_translation(language))
                        .font(font)
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(horizontal_space())
                .push(sort_arrows(
                    sniffer.service_sort_type,
                    Message::ServiceSortSelection,
                )),
        )
        .push(
            Scrollable::with_direction(
                scroll_service,
                Direction::Vertical(ScrollbarType::properties()),
            )
            .width(Length::Fill),
        )
}

fn lazy_col_info<'a>(sniffer: &Sniffer) -> Container<'a, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let PaletteExtension { font, .. } = style.get_extension();

    let col_device = col_device(language, font, &sniffer.device);

    let col_data_representation =
        col_data_representation(language, font, sniffer.traffic_chart.chart_type);

    let donut_row = donut_row(language, font, sniffer);

    let content = Column::new()
        .align_x(Alignment::Center)
        .padding([5, 10])
        .push(
            Row::new()
                .height(Length::Fill)
                .push(
                    Scrollable::with_direction(
                        col_device,
                        Direction::Horizontal(ScrollbarType::properties()),
                    )
                    .width(Length::Fill),
                )
                .push(Container::new(Rule::vertical(25)).height(Length::Shrink))
                .push(col_data_representation.width(Length::Fill)),
        )
        .push(Rule::horizontal(15))
        .push(donut_row.height(Length::Fill));

    Container::new(content)
        .width(400)
        .padding(Padding::new(5.0).top(10))
        .align_x(Alignment::Center)
        .class(ContainerType::BorderedRound)
}

fn container_chart(sniffer: &Sniffer, font: Font) -> Container<Message, StyleType> {
    let ConfigSettings { language, .. } = sniffer.configs.lock().unwrap().settings;
    let traffic_chart = &sniffer.traffic_chart;

    Container::new(
        Column::new()
            .align_x(Alignment::Center)
            .push(
                Row::new().padding([10, 0]).align_y(Alignment::Center).push(
                    traffic_rate_translation(language)
                        .font(font)
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                ),
            )
            .push(traffic_chart.view()),
    )
    .width(Fill)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .class(ContainerType::BorderedRound)
}

fn col_device<'a>(
    language: Language,
    font: Font,
    device: &MyDevice,
) -> Column<'a, Message, StyleType> {
    let link_type = device.link_type;
    #[cfg(not(target_os = "windows"))]
    let adapter_info = &device.name;
    #[cfg(target_os = "windows")]
    let adapter_info = device.desc.as_ref().unwrap_or(&device.name);

    Column::new()
        .height(Length::Fill)
        .spacing(10)
        .push(TextType::highlighted_subtitle_with_desc(
            network_adapter_translation(language),
            adapter_info,
            font,
        ))
        .push(link_type.link_type_col(language, font))
}

fn col_data_representation<'a>(
    language: Language,
    font: Font,
    chart_type: ChartType,
) -> Column<'a, Message, StyleType> {
    let mut ret_val = Column::new().spacing(5).push(
        Text::new(format!("{}:", data_representation_translation(language)))
            .class(TextType::Subtitle)
            .font(font),
    );

    for option in ChartType::ALL {
        let is_active = chart_type.eq(&option);
        ret_val = ret_val.push(
            Button::new(
                Text::new(option.get_label(language).to_owned())
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .font(font),
            )
            .width(Length::Fill)
            .height(33)
            .class(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::ChartSelection(option)),
        );
    }
    ret_val
}

fn donut_row<'a>(
    language: Language,
    font: Font,
    sniffer: &Sniffer,
) -> Container<'a, Message, StyleType> {
    let chart_type = sniffer.traffic_chart.chart_type;
    let filters = &sniffer.filters;

    let (in_data, out_data, filtered_out, dropped) = if chart_type.eq(&ChartType::Bytes) {
        (
            sniffer.runtime_data.tot_in_bytes,
            sniffer.runtime_data.tot_out_bytes,
            sniffer.runtime_data.all_bytes
                - sniffer.runtime_data.tot_out_bytes
                - sniffer.runtime_data.tot_in_bytes,
            // assume that the dropped packets have the same size as the average packet
            u128::from(sniffer.runtime_data.dropped_packets) * sniffer.runtime_data.all_bytes
                / sniffer.runtime_data.all_packets,
        )
    } else {
        (
            sniffer.runtime_data.tot_in_packets,
            sniffer.runtime_data.tot_out_packets,
            sniffer.runtime_data.all_packets
                - sniffer.runtime_data.tot_out_packets
                - sniffer.runtime_data.tot_in_packets,
            u128::from(sniffer.runtime_data.dropped_packets),
        )
    };

    let legend_entry_filtered = if filters.none_active() {
        None
    } else {
        Some(donut_legend_entry(
            filtered_out,
            chart_type,
            RuleType::FilteredOut,
            filters,
            font,
            language,
        ))
    };

    let legend_col = Column::new()
        .spacing(5)
        .push(donut_legend_entry(
            in_data,
            chart_type,
            RuleType::Incoming,
            filters,
            font,
            language,
        ))
        .push(donut_legend_entry(
            out_data,
            chart_type,
            RuleType::Outgoing,
            filters,
            font,
            language,
        ))
        .push_maybe(legend_entry_filtered)
        .push(donut_legend_entry(
            dropped,
            chart_type,
            RuleType::Dropped,
            filters,
            font,
            language,
        ));

    let donut_row = Row::new()
        .align_y(Vertical::Center)
        .spacing(20)
        .push(donut_chart(
            chart_type,
            in_data,
            out_data,
            filtered_out,
            dropped,
            font,
        ))
        .push(legend_col);

    Container::new(donut_row)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn donut_legend_entry<'a>(
    value: u128,
    chart_type: ChartType,
    rule_type: RuleType,
    filters: &Filters,
    font: Font,
    language: Language,
) -> Row<'a, Message, StyleType> {
    let value_text = if chart_type.eq(&ChartType::Bytes) {
        ByteMultiple::formatted_string(value)
    } else {
        value.to_string()
    };

    let label = match rule_type {
        RuleType::Incoming => incoming_translation(language),
        RuleType::Outgoing => outgoing_translation(language),
        RuleType::FilteredOut => excluded_translation(language),
        RuleType::Dropped => dropped_translation(language),
        _ => "",
    };

    let tooltip = if matches!(rule_type, RuleType::FilteredOut) {
        Some(get_active_filters_tooltip(filters, language, font))
    } else {
        None
    };

    Row::new()
        .spacing(10)
        .align_y(Alignment::Center)
        .push(
            Row::new()
                .width(10)
                .push(Rule::horizontal(1).class(rule_type)),
        )
        .push(Text::new(format!("{label}: {value_text}")).font(font))
        .push_maybe(tooltip)
}

const MIN_BARS_LENGTH: f32 = 10.0;

pub fn get_bars_length(
    tot_width: f32,
    chart_type: ChartType,
    first_entry: &DataInfo,
    data_info: &DataInfo,
) -> (f32, f32) {
    let (in_val, out_val, first_entry_tot_val) = match chart_type {
        ChartType::Packets => (
            data_info.incoming_packets(),
            data_info.outgoing_packets(),
            first_entry.tot_packets(),
        ),
        ChartType::Bytes => (
            data_info.incoming_bytes(),
            data_info.outgoing_bytes(),
            first_entry.tot_bytes(),
        ),
    };

    let tot_val = in_val + out_val;
    if tot_val == 0 {
        return (0.0, 0.0);
    }

    #[allow(clippy::cast_precision_loss)]
    let tot_len = tot_width * tot_val as f32 / first_entry_tot_val as f32;
    #[allow(clippy::cast_precision_loss)]
    let (mut in_len, mut out_len) = (
        tot_len * in_val as f32 / tot_val as f32,
        tot_len * out_val as f32 / tot_val as f32,
    );

    if tot_len <= MIN_BARS_LENGTH {
        // normalize small values
        if in_val > 0 {
            if out_val == 0 {
                in_len = MIN_BARS_LENGTH;
            } else {
                in_len = MIN_BARS_LENGTH / 2.0;
            }
        }
        if out_val > 0 {
            if in_val == 0 {
                out_len = MIN_BARS_LENGTH;
            } else {
                out_len = MIN_BARS_LENGTH / 2.0;
            }
        }
    } else {
        // tot_len is longer than minimum
        if in_val > 0 && in_len < MIN_BARS_LENGTH / 2.0 {
            let diff = MIN_BARS_LENGTH / 2.0 - in_len;
            in_len += diff;
            out_len -= diff;
        }
        if out_val > 0 && out_len < MIN_BARS_LENGTH / 2.0 {
            let diff = MIN_BARS_LENGTH / 2.0 - out_len;
            out_len += diff;
            in_len -= diff;
        }
    }

    // cut to 3 significant digits
    in_len = (in_len * 1000.0).round() / 1000.0;
    out_len = (out_len * 1000.0).round() / 1000.0;

    (in_len, out_len)
}

pub fn get_bars<'a>(in_len: f32, out_len: f32) -> Row<'a, Message, StyleType> {
    Row::new()
        .push(if in_len > 0.0 {
            Row::new()
                .width(in_len)
                .push(Rule::horizontal(1).class(RuleType::Incoming))
        } else {
            Row::new()
        })
        .push(if out_len > 0.0 {
            Row::new()
                .width(out_len)
                .push(Rule::horizontal(1).class(RuleType::Outgoing))
        } else {
            Row::new()
        })
}

fn get_star_button<'a>(is_favorite: bool, host: Host) -> Button<'a, Message, StyleType> {
    button(
        Icon::Star
            .to_text()
            .size(20)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(0)
    .height(FLAGS_WIDTH_BIG * 0.75)
    .width(FLAGS_WIDTH_BIG)
    .class(if is_favorite {
        ButtonType::Starred
    } else {
        ButtonType::NotStarred
    })
    .on_press(Message::AddOrRemoveFavorite(host, !is_favorite))
}

fn get_active_filters_col<'a>(
    filters: &Filters,
    language: Language,
    font: Font,
) -> Column<'a, Message, StyleType> {
    let mut ret_val = Column::new().push(
        Text::new(active_filters_translation(language))
            .font(font)
            .class(TextType::Subtitle),
    );

    if filters.none_active() {
        ret_val = ret_val.push(Text::new(format!("   {}", none_translation(language))).font(font));
    } else {
        let filters_string = get_active_filters_string(filters, language);
        ret_val = ret_val.push(Row::new().push(Text::new(filters_string).font(font)));
    }
    ret_val
}

fn get_active_filters_tooltip<'a>(
    filters: &Filters,
    language: Language,
    font: Font,
) -> Tooltip<'a, Message, StyleType> {
    let filters_string = get_active_filters_string(filters, language);

    let mut ret_val = Column::new().push(
        Text::new(active_filters_translation(language))
            .font(font)
            .class(TextType::Subtitle),
    );

    ret_val = ret_val.push(Row::new().push(Text::new(filters_string).font(font)));

    let tooltip = Tooltip::new(
        Container::new(
            Text::new("i")
                .font(font)
                .size(15)
                .line_height(LineHeight::Relative(1.0)),
        )
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .height(20)
        .width(20)
        .class(ContainerType::BadgeInfo),
        ret_val,
        Position::FollowCursor,
    )
    .class(ContainerType::Tooltip);

    tooltip
}

fn sort_arrows<'a>(
    active_sort_type: SortType,
    message: fn(SortType) -> Message,
) -> Container<'a, Message, StyleType> {
    Container::new(
        button(
            active_sort_type
                .icon()
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .class(active_sort_type.button_type())
        .on_press(message(active_sort_type.next_sort())),
    )
    .width(60.0)
    .align_x(Alignment::Center)
}

#[cfg(test)]
mod tests {
    use crate::chart::types::chart_type::ChartType;
    use crate::gui::pages::overview_page::{MIN_BARS_LENGTH, get_bars_length};
    use crate::networking::types::data_info::DataInfo;

    #[test]
    fn test_get_bars_length_simple() {
        let first_entry = DataInfo::new_for_tests(50, 50, 150, 50);
        let data_info = DataInfo::new_for_tests(25, 55, 165, 30);
        assert_eq!(
            get_bars_length(200.0, ChartType::Packets, &first_entry, &data_info),
            (50.0, 110.0)
        );
        assert_eq!(
            get_bars_length(200.0, ChartType::Bytes, &first_entry, &data_info),
            (165.0, 30.0)
        );
    }

    #[test]
    fn test_get_bars_length_normalize_small_values() {
        let first_entry = DataInfo::new_for_tests(50, 50, 150, 50);
        let mut data_info = DataInfo::new_for_tests(2, 1, 1, 0);
        assert_eq!(
            get_bars_length(200.0, ChartType::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, MIN_BARS_LENGTH / 2.0)
        );
        assert_eq!(
            get_bars_length(200.0, ChartType::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH, 0.0)
        );

        data_info = DataInfo::new_for_tests(0, 3, 0, 2);
        assert_eq!(
            get_bars_length(200.0, ChartType::Packets, &first_entry, &data_info),
            (0.0, MIN_BARS_LENGTH)
        );
        assert_eq!(
            get_bars_length(200.0, ChartType::Bytes, &first_entry, &data_info),
            (0.0, MIN_BARS_LENGTH)
        );
    }

    #[test]
    fn test_get_bars_length_normalize_very_small_values() {
        let first_entry =
            DataInfo::new_for_tests(u128::MAX / 2, u128::MAX / 2, u128::MAX / 2, u128::MAX / 2);
        let mut data_info = DataInfo::new_for_tests(1, 1, 1, 1);
        assert_eq!(
            get_bars_length(200.0, ChartType::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, MIN_BARS_LENGTH / 2.0)
        );
        assert_eq!(
            get_bars_length(200.0, ChartType::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, MIN_BARS_LENGTH / 2.0)
        );

        data_info = DataInfo::new_for_tests(0, 1, 0, 1);
        assert_eq!(
            get_bars_length(200.0, ChartType::Packets, &first_entry, &data_info),
            (0.0, MIN_BARS_LENGTH)
        );
        assert_eq!(
            get_bars_length(200.0, ChartType::Bytes, &first_entry, &data_info),
            (0.0, MIN_BARS_LENGTH)
        );

        data_info = DataInfo::new_for_tests(1, 0, 1, 0);
        assert_eq!(
            get_bars_length(200.0, ChartType::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH, 0.0)
        );
        assert_eq!(
            get_bars_length(200.0, ChartType::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH, 0.0)
        );
    }

    #[test]
    fn test_get_bars_length_complex() {
        let first_entry = DataInfo::new_for_tests(350, 50, 12, 88);

        let mut data_info = DataInfo::new_for_tests(0, 9, 0, 10);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (0.0, 16.245)
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (0.0, MIN_BARS_LENGTH)
        );
        data_info = DataInfo::new_for_tests(9, 0, 13, 0);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (16.245, 0.0)
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (13.0, 0.0)
        );

        data_info = DataInfo::new_for_tests(4, 5, 6, 7);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (
                (1000.0_f32 * 16.245 * 4.0 / 9.0).round() / 1000.0,
                (1000.0_f32 * 16.245 * 5.0 / 9.0).round() / 1000.0
            )
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (6.0, 7.0)
        );
        data_info = DataInfo::new_for_tests(5, 4, 7, 6);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (
                (1000.0_f32 * 16.245 * 5.0 / 9.0).round() / 1000.0,
                (1000.0_f32 * 16.245 * 4.0 / 9.0).round() / 1000.0
            )
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (7.0, 6.0)
        );

        data_info = DataInfo::new_for_tests(1, 8, 1, 12);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, 11.245)
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, 8.0)
        );
        data_info = DataInfo::new_for_tests(8, 1, 12, 1);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (11.245, MIN_BARS_LENGTH / 2.0)
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (8.0, MIN_BARS_LENGTH / 2.0)
        );

        data_info = DataInfo::new_for_tests(6, 1, 10, 1);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (
                16.245 * 7.0 / 9.0 - MIN_BARS_LENGTH / 2.0,
                MIN_BARS_LENGTH / 2.0
            )
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (6.0, MIN_BARS_LENGTH / 2.0)
        );
        data_info = DataInfo::new_for_tests(1, 6, 1, 9);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (
                MIN_BARS_LENGTH / 2.0,
                16.245 * 7.0 / 9.0 - MIN_BARS_LENGTH / 2.0,
            )
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, MIN_BARS_LENGTH / 2.0)
        );

        data_info = DataInfo::new_for_tests(1, 6, 5, 5);
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH / 2.0, MIN_BARS_LENGTH / 2.0)
        );

        data_info = DataInfo::new_for_tests(0, 0, 0, 0);
        assert_eq!(
            get_bars_length(722.0, ChartType::Packets, &first_entry, &data_info),
            (0.0, 0.0,)
        );
        assert_eq!(
            get_bars_length(100.0, ChartType::Bytes, &first_entry, &data_info),
            (0.0, 0.0)
        );
    }
}
