//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: chart, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{
    button, horizontal_space, lazy, vertical_space, Button, Column, Container, Row, Rule,
    Scrollable, Space, Text, Tooltip,
};
use iced::Length::{Fill, FillPortion};
use iced::{Alignment, Font, Length};

use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_pictures::FLAGS_WIDTH_BIG;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::FONT_SIZE_TITLE;
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::palette_extension::PaletteExtension;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::filters::Filters;
use crate::networking::types::host::Host;
use crate::networking::types::my_device::MyDevice;
use crate::report::get_report_entries::{get_host_entries, get_service_entries};
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::translations::translations::{
    active_filters_translation, bytes_chart_translation, error_translation,
    filtered_bytes_translation, filtered_packets_translation, network_adapter_translation,
    no_addresses_translation, none_translation, of_total_translation, packets_chart_translation,
    some_observed_translation, traffic_rate_translation, waiting_translation,
};
use crate::translations::translations_2::{
    data_representation_translation, dropped_packets_translation, host_translation,
    only_top_30_items_translation,
};
use crate::translations::translations_3::{service_translation, unsupported_link_type_translation};
use crate::utils::formatted_strings::{get_active_filters_string, get_percentage_string};
use crate::utils::types::icon::Icon;
use crate::{ByteMultiple, ChartType, ConfigSettings, Language, RunningPage, StyleType};

/// Computes the body of gui overview page
pub fn overview_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let mut body = Column::new();
    let mut tab_and_body = Column::new().height(Length::Fill);

    if sniffer.pcap_error.is_none() {
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
                body = body_no_observed(
                    &sniffer.filters,
                    observed,
                    font,
                    font_headers,
                    language,
                    &sniffer.waiting,
                );
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
                    move |_| lazy_col_info(total, filtered, dropped, sniffer),
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
                    .align_items(Alignment::Center)
                    .push(
                        Row::new()
                            .spacing(10)
                            .height(FillPortion(5))
                            .push(container_info)
                            .push(container_chart),
                    )
                    .push(container_report);
            }
        }
    } else {
        // pcap threw an ERROR!
        body = body_pcap_error(
            sniffer.pcap_error.as_ref().unwrap(),
            &sniffer.waiting,
            language,
            font,
        );
    }

    Container::new(Column::new().push(tab_and_body.push(body))).height(Length::Fill)
}

fn body_no_packets(
    device: &MyDevice,
    font: Font,
    language: Language,
    waiting: &str,
) -> Column<'static, Message, StyleType> {
    let link_type = device.link_type;
    let mut adapter_info = device.name.clone();
    adapter_info.push_str(&format!("\n{}", link_type.full_print_on_one_line(language)));
    let (icon_text, nothing_to_see_text) = if !link_type.is_supported() {
        (
            Icon::Warning.to_text().size(60),
            unsupported_link_type_translation(language, &adapter_info)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
    } else if device.addresses.lock().unwrap().is_empty() {
        (
            Icon::Warning.to_text().size(60),
            no_addresses_translation(language, &adapter_info)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
    } else {
        (
            Icon::get_hourglass(waiting.len()).size(60),
            waiting_translation(language, &adapter_info)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
    };

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space())
        .push(icon_text)
        .push(Space::with_height(15))
        .push(nothing_to_see_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn body_no_observed(
    filters: &Filters,
    observed: u128,
    font: Font,
    font_headers: Font,
    language: Language,
    waiting: &str,
) -> Column<'static, Message, StyleType> {
    let tot_packets_text = some_observed_translation(language, observed)
        .horizontal_alignment(Horizontal::Center)
        .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space())
        .push(Icon::Funnel.to_text().size(60))
        .push(get_active_filters_col(
            filters,
            language,
            font,
            font_headers,
            true,
        ))
        .push(Rule::horizontal(20))
        .push(tot_packets_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn body_pcap_error(
    pcap_error: &str,
    waiting: &str,
    language: Language,
    font: Font,
) -> Column<'static, Message, StyleType> {
    // let err_string = pcap_error.clone().unwrap();
    let error_text = error_translation(language, pcap_error)
        .horizontal_alignment(Horizontal::Center)
        .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space())
        .push(Icon::Error.to_text().size(60))
        .push(Space::with_height(15))
        .push(error_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn lazy_row_report(sniffer: &Sniffer) -> Container<'static, Message, StyleType> {
    let col_host = col_host(840.0, sniffer);
    let col_service = col_service(250.0, sniffer);

    let row_report = Row::new()
        .padding([0, 10, 5, 10])
        .push(col_host)
        .push(Rule::vertical(40))
        .push(col_service);

    Container::new(row_report)
        .height(FillPortion(4))
        .style(ContainerType::BorderedRound)
}

fn col_host(width: f32, sniffer: &Sniffer) -> Column<'static, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut scroll_host = Column::new().width(width).align_items(Alignment::Center);
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
            .align_items(Alignment::Center)
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
                .padding([5, 15, 5, 10])
                .on_press(Message::Search(SearchParameters::new_host_search(host)))
                .style(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_host = scroll_host.push(Space::with_height(25)).push(
            Text::new(only_top_30_items_translation(language))
                .font(font)
                .horizontal_alignment(Horizontal::Center),
        );
    }

    Column::new()
        .width(width + 11.0)
        .push(
            Row::new()
                .height(45)
                .align_items(Alignment::Center)
                .push(
                    Text::new(host_translation(language))
                        .font(font)
                        .style(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(horizontal_space())
                .push(sort_arrows(
                    sniffer.host_sort_type,
                    Message::HostSortSelection,
                )),
        )
        .push(
            Scrollable::new(scroll_host)
                .width(Length::Fill)
                .direction(Direction::Vertical(ScrollbarType::properties())),
        )
}

fn col_service(width: f32, sniffer: &Sniffer) -> Column<'static, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut scroll_service = Column::new().width(width).align_items(Alignment::Center);
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
                .padding([5, 15, 8, 10])
                .on_press(Message::Search(SearchParameters::new_service_search(
                    service,
                )))
                .style(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_service = scroll_service.push(Space::with_height(25)).push(
            Text::new(only_top_30_items_translation(language))
                .font(font)
                .horizontal_alignment(Horizontal::Center),
        );
    }

    Column::new()
        .width(width + 11.0)
        .push(
            Row::new()
                .height(45)
                .align_items(Alignment::Center)
                .push(
                    Text::new(service_translation(language))
                        .font(font)
                        .style(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(horizontal_space())
                .push(sort_arrows(
                    sniffer.service_sort_type,
                    Message::ServiceSortSelection,
                )),
        )
        .push(
            Scrollable::new(scroll_service)
                .width(Length::Fill)
                .direction(Direction::Vertical(ScrollbarType::properties())),
        )
}

fn lazy_col_info(
    total: u128,
    filtered: u128,
    dropped: u32,
    sniffer: &Sniffer,
) -> Container<'static, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let PaletteExtension {
        font, font_headers, ..
    } = style.get_extension();

    let col_device = col_device(language, font, &sniffer.device);

    let col_data_representation =
        col_data_representation(language, font, sniffer.traffic_chart.chart_type);

    let col_bytes_packets = col_bytes_packets(
        language,
        dropped,
        total,
        filtered,
        font,
        font_headers,
        sniffer,
    );

    let content = Column::new()
        .align_items(Alignment::Center)
        .padding([5, 10])
        .push(
            Row::new()
                .height(120)
                .push(
                    Scrollable::new(col_device)
                        .width(Length::Fill)
                        .direction(Direction::Horizontal(ScrollbarType::properties())),
                )
                .push(Container::new(Rule::vertical(25)).height(Length::Shrink))
                .push(col_data_representation.width(Length::Fill)),
        )
        .push(Rule::horizontal(15))
        .push(
            Scrollable::new(col_bytes_packets)
                .height(Length::Fill)
                .width(Length::Fill)
                .direction(Direction::Vertical(ScrollbarType::properties())),
        );

    Container::new(content)
        .width(400)
        .padding([10, 5, 5, 5])
        .align_x(Horizontal::Center)
        .style(ContainerType::BorderedRound)
}

fn container_chart(sniffer: &Sniffer, font: Font) -> Container<Message, StyleType> {
    let ConfigSettings { language, .. } = sniffer.configs.lock().unwrap().settings;
    let traffic_chart = &sniffer.traffic_chart;

    let mut chart_info_string = String::from("(");
    chart_info_string.push_str(if traffic_chart.chart_type.eq(&ChartType::Packets) {
        packets_chart_translation(language)
    } else {
        bytes_chart_translation(language)
    });
    chart_info_string.push(')');

    Container::new(
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .padding([10, 0])
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(
                        traffic_rate_translation(language)
                            .font(font)
                            .style(TextType::Title)
                            .size(FONT_SIZE_TITLE),
                    )
                    .push(
                        Text::new(chart_info_string)
                            .style(TextType::Subtitle)
                            .font(font),
                    ),
            )
            .push(traffic_chart.view()),
    )
    .width(Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .style(ContainerType::BorderedRound)
}

fn col_device(
    language: Language,
    font: Font,
    device: &MyDevice,
) -> Column<'static, Message, StyleType> {
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

fn col_data_representation(
    language: Language,
    font: Font,
    chart_type: ChartType,
) -> Column<'static, Message, StyleType> {
    let mut ret_val = Column::new().spacing(5).push(
        Text::new(format!("{}:", data_representation_translation(language)))
            .style(TextType::Subtitle)
            .font(font),
    );

    for option in ChartType::ALL {
        let is_active = chart_type.eq(&option);
        ret_val = ret_val.push(
            Button::new(
                Text::new(option.get_label(language).to_owned())
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center)
                    .font(font),
            )
            .width(Length::Fill)
            .height(33)
            .style(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::ChartSelection(option)),
        );
    }
    ret_val
}

fn col_bytes_packets(
    language: Language,
    dropped: u32,
    total: u128,
    filtered: u128,
    font: Font,
    font_headers: Font,
    sniffer: &Sniffer,
) -> Column<'static, Message, StyleType> {
    let filtered_bytes = sniffer.runtime_data.tot_out_bytes + sniffer.runtime_data.tot_in_bytes;
    let all_bytes = sniffer.runtime_data.all_bytes;
    let filters = &sniffer.filters;

    let dropped_val = if dropped > 0 {
        format!(
            "{} {}",
            dropped,
            of_total_translation(language, &get_percentage_string(total, u128::from(dropped)))
        )
    } else {
        none_translation(language).to_string()
    };
    let bytes_value = if dropped > 0 {
        ByteMultiple::formatted_string(filtered_bytes)
    } else {
        format!(
            "{} {}",
            &ByteMultiple::formatted_string(filtered_bytes),
            of_total_translation(language, &get_percentage_string(all_bytes, filtered_bytes))
        )
    };

    Column::new()
        .spacing(10)
        .push(get_active_filters_col(
            filters,
            language,
            font,
            font_headers,
            false,
        ))
        .push(TextType::highlighted_subtitle_with_desc(
            filtered_bytes_translation(language),
            &bytes_value,
            font,
        ))
        .push(TextType::highlighted_subtitle_with_desc(
            filtered_packets_translation(language),
            &format!(
                "{} {}",
                filtered,
                of_total_translation(language, &get_percentage_string(total, filtered))
            ),
            font,
        ))
        .push(TextType::highlighted_subtitle_with_desc(
            dropped_packets_translation(language),
            &dropped_val,
            font,
        ))
}

const MIN_BARS_LENGTH: f32 = 10.0;

fn get_bars_length(
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

fn get_bars(in_len: f32, out_len: f32) -> Row<'static, Message, StyleType> {
    Row::new()
        .push(if in_len > 0.0 {
            Row::new()
                .width(in_len)
                .push(Rule::horizontal(1).style(RuleType::Incoming))
        } else {
            Row::new()
        })
        .push(if out_len > 0.0 {
            Row::new()
                .width(out_len)
                .push(Rule::horizontal(1).style(RuleType::Outgoing))
        } else {
            Row::new()
        })
}

fn get_star_button(is_favorite: bool, host: Host) -> Button<'static, Message, StyleType> {
    button(
        Icon::Star
            .to_text()
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(0)
    .height(FLAGS_WIDTH_BIG * 0.75)
    .width(FLAGS_WIDTH_BIG)
    .style(if is_favorite {
        ButtonType::Starred
    } else {
        ButtonType::NotStarred
    })
    .on_press(Message::AddOrRemoveFavorite(host, !is_favorite))
}

fn get_active_filters_col(
    filters: &Filters,
    language: Language,
    font: Font,
    font_headers: Font,
    show: bool,
) -> Column<'static, Message, StyleType> {
    let mut ret_val = Column::new().push(
        Text::new(format!("{}:", active_filters_translation(language),))
            .font(font)
            .style(TextType::Subtitle),
    );

    if filters.none_active() {
        ret_val = ret_val.push(Text::new(format!("   {}", none_translation(language))).font(font));
    } else {
        let filters_string = get_active_filters_string(filters, language);
        ret_val = ret_val.push(if show {
            Row::new().push(Text::new(filters_string).font(font))
        } else {
            Row::new().padding([0, 0, 0, 20]).push(
                Tooltip::new(
                    Container::new(
                        Text::new("i")
                            .font(font_headers)
                            .size(15)
                            .line_height(LineHeight::Relative(1.0)),
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .height(20)
                    .width(20)
                    .style(ContainerType::Highlighted),
                    Text::new(filters_string).font(font),
                    Position::FollowCursor,
                )
                .style(ContainerType::Tooltip),
            )
        });
    }
    ret_val
}

fn sort_arrows(
    active_sort_type: SortType,
    message: fn(SortType) -> Message,
) -> Container<'static, Message, StyleType> {
    Container::new(
        button(
            active_sort_type
                .icon()
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .style(active_sort_type.button_type())
        .on_press(message(active_sort_type.next_sort())),
    )
    .width(60.0)
    .align_x(Horizontal::Center)
}

#[cfg(test)]
mod tests {
    use crate::chart::types::chart_type::ChartType;
    use crate::gui::pages::overview_page::{get_bars_length, MIN_BARS_LENGTH};
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
