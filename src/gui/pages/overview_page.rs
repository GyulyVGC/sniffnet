//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: chart, detailed connections data
//! and overall statistics about the filtered traffic.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::tooltip::Position;
use iced::widget::{
    button, lazy, vertical_space, Button, Column, Container, Row, Scrollable, Text, Tooltip,
};
use iced::widget::{horizontal_space, Rule};
use iced::Length::{Fill, FillPortion, Fixed};
use iced::{Alignment, Font, Length, Renderer};

use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_pictures::FLAGS_WIDTH_BIG;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::gui::types::sniffer::Sniffer;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::filters::Filters;
use crate::networking::types::host::Host;
use crate::networking::types::my_device::MyDevice;
use crate::networking::types::search_parameters::SearchParameters;
use crate::report::get_report_entries::{get_app_entries, get_host_entries};
use crate::translations::translations::{
    active_filters_translation, application_protocol_translation, bytes_chart_translation,
    error_translation, filtered_bytes_translation, filtered_packets_translation,
    network_adapter_translation, no_addresses_translation, none_translation, of_total_translation,
    packets_chart_translation, some_observed_translation, traffic_rate_translation,
    waiting_translation,
};
use crate::translations::translations_2::{
    data_representation_translation, dropped_packets_translation, host_translation,
    only_top_30_hosts_translation,
};
use crate::utils::formatted_strings::{
    get_active_filters_string, get_formatted_bytes_string_with_b, get_percentage_string,
};
use crate::utils::types::icon::Icon;
use crate::{AppProtocol, ChartType, Language, RunningPage, StyleType};

/// Computes the body of gui overview page
pub fn overview_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let font = style.get_font();
    let font_headers = style.get_font_headers();

    let mut body = Column::new();
    let mut tab_and_body = Column::new().height(Length::Fill);

    if sniffer.pcap_error.is_none() {
        // NO pcap error detected
        let observed = sniffer.runtime_data.all_packets;
        let filtered =
            sniffer.runtime_data.tot_sent_packets + sniffer.runtime_data.tot_received_packets;
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
) -> Column<'static, Message, Renderer<StyleType>> {
    let adapter_name = device.name.clone();
    let (icon_text, nothing_to_see_text) = if device.addresses.lock().unwrap().is_empty() {
        (
            Icon::Warning.to_text().size(60),
            no_addresses_translation(language, &adapter_name)
                .horizontal_alignment(Horizontal::Center)
                .font(font),
        )
    } else {
        (
            Icon::get_hourglass(waiting.len()).size(60),
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
) -> Column<'static, Message, Renderer<StyleType>> {
    let tot_packets_text = some_observed_translation(language, observed)
        .horizontal_alignment(Horizontal::Center)
        .font(font);

    Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(vertical_space(FillPortion(1)))
        .push(Icon::Funnel.to_text().size(60))
        .push(get_active_filters_col(filters, language, font, true))
        .push(Rule::horizontal(20))
        .push(tot_packets_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(vertical_space(FillPortion(2)))
}

fn body_pcap_error(
    pcap_error: &str,
    waiting: &str,
    language: Language,
    font: Font,
) -> Column<'static, Message, Renderer<StyleType>> {
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
        .push(Icon::Error.to_text().size(60))
        .push(vertical_space(Length::Fixed(15.0)))
        .push(error_text)
        .push(Text::new(waiting.to_owned()).font(font).size(50))
        .push(vertical_space(FillPortion(2)))
}

fn lazy_row_report(sniffer: &Sniffer) -> Container<'static, Message, Renderer<StyleType>> {
    let mut row_report = Row::new()
        .padding(10)
        .height(Length::Fill)
        .width(Length::Fill);

    let col_host = col_host(840.0, sniffer);
    let col_app = col_app(250.0, sniffer);

    row_report = row_report
        .push(col_host)
        .push(Rule::vertical(40))
        .push(col_app);

    Container::new(row_report)
        .height(FillPortion(4))
        .width(Length::Fixed(1170.0))
        .style(ContainerType::BorderedRound)
        .align_x(Horizontal::Center)
}

fn col_host(width: f32, sniffer: &Sniffer) -> Column<'static, Message, Renderer<StyleType>> {
    let language = sniffer.settings.language;
    let font = sniffer.settings.style.get_font();
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut scroll_host = Column::new()
        .width(Length::Fixed(width))
        .align_items(Alignment::Center);
    let entries = get_host_entries(&sniffer.info_traffic, chart_type);

    for (host, data_info_host) in &entries {
        let (incoming_bar_len, outgoing_bar_len) = get_bars_length(
            width * 0.86,
            chart_type,
            &entries.get(0).unwrap().1.data_info.clone(),
            &data_info_host.data_info,
        );

        let star_button = get_star_button(data_info_host.is_favorite, host.clone());

        let host_bar = Column::new()
            .width(Length::Fixed(width))
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
                    .push(horizontal_space(Length::FillPortion(1)))
                    .push(
                        Text::new(if chart_type.eq(&ChartType::Packets) {
                            data_info_host.data_info.tot_packets().to_string()
                        } else {
                            get_formatted_bytes_string_with_b(data_info_host.data_info.tot_bytes())
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
                FLAGS_WIDTH_BIG,
                data_info_host.is_local,
                data_info_host.traffic_type,
                language,
                font,
            ))
            .push(host_bar);

        scroll_host = scroll_host.push(
            button(content)
                .padding([5, 15, 5, 10])
                .on_press(Message::Search(SearchParameters {
                    domain: host.domain.clone(),
                    country: host.country.to_string().clone(),
                    as_name: host.asn.name.clone(),
                    ..SearchParameters::default()
                }))
                .style(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_host = scroll_host.push(vertical_space(Length::Fixed(25.0))).push(
            Text::new(only_top_30_hosts_translation(language))
                .font(font)
                .horizontal_alignment(Horizontal::Center),
        );
    }

    Column::new()
        .width(Length::Fixed(width + 11.0))
        .push(
            Text::new(host_translation(language))
                .font(font)
                .style(TextType::Title)
                .size(FONT_SIZE_TITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Scrollable::new(Container::new(scroll_host).width(Length::Fill))
                .direction(Direction::Vertical(ScrollbarType::properties())),
        )
}

fn col_app(width: f32, sniffer: &Sniffer) -> Column<'static, Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let font = style.get_font();
    let chart_type = sniffer.traffic_chart.chart_type;

    let mut col_app = Column::new()
        .width(Length::Fixed(width + 11.0))
        .push(
            Text::new(application_protocol_translation(language))
                .font(font)
                .style(TextType::Title)
                .size(FONT_SIZE_TITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)));

    let mut scroll_app = Column::new().width(Length::Fixed(width));
    let entries = get_app_entries(&sniffer.info_traffic, chart_type);

    for (app, data_info) in &entries {
        let (mut incoming_bar_len, mut outgoing_bar_len) = get_bars_length(
            width * 0.88,
            chart_type,
            &entries.get(0).unwrap().1.clone(),
            data_info,
        );

        // check if Unknown is longer than the first entry
        if app.eq(&AppProtocol::Unknown) && incoming_bar_len + outgoing_bar_len > width * 0.88 {
            let incoming_proportion = incoming_bar_len / (incoming_bar_len + outgoing_bar_len);
            incoming_bar_len = width * 0.88 * incoming_proportion;
            outgoing_bar_len = width * 0.88 * (1.0 - incoming_proportion);
        }

        let content = Column::new()
            .spacing(1)
            .width(Length::Fixed(width))
            .push(
                Row::new()
                    .push(Text::new(app.to_string()).font(font))
                    .push(horizontal_space(Length::FillPortion(1)))
                    .push(
                        Text::new(if chart_type.eq(&ChartType::Packets) {
                            data_info.tot_packets().to_string()
                        } else {
                            get_formatted_bytes_string_with_b(data_info.tot_bytes())
                        })
                        .font(font),
                    ),
            )
            .push(get_bars(incoming_bar_len, outgoing_bar_len));

        scroll_app = scroll_app.push(
            button(content)
                .padding([5, 15, 8, 10])
                .on_press(Message::Search(SearchParameters {
                    app: app.to_string(),
                    ..SearchParameters::default()
                }))
                .style(ButtonType::Neutral),
        );
    }
    col_app = col_app.push(
        Scrollable::new(Container::new(scroll_app).width(Length::Fill))
            .direction(Direction::Vertical(ScrollbarType::properties())),
    );

    col_app
}

fn lazy_col_info(
    total: u128,
    filtered: u128,
    dropped: u32,
    sniffer: &Sniffer,
) -> Container<'static, Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let font = style.get_font();
    let filtered_bytes =
        sniffer.runtime_data.tot_sent_bytes + sniffer.runtime_data.tot_received_bytes;
    let all_bytes = sniffer.runtime_data.all_bytes;

    let col_device_filters = col_device_filters(language, font, &sniffer.filters, &sniffer.device);

    let col_data_representation =
        col_data_representation(language, font, sniffer.traffic_chart.chart_type);

    let col_bytes_packets = col_bytes_packets(
        language,
        dropped,
        total,
        filtered,
        all_bytes,
        filtered_bytes,
        font,
    );

    let content = Column::new()
        .align_items(Alignment::Center)
        .padding([5, 10])
        .push(
            Row::new()
                .height(Length::Fixed(120.0))
                .push(
                    Scrollable::new(col_device_filters)
                        .width(Length::Fill)
                        .direction(Direction::Vertical(ScrollbarType::properties())),
                )
                .push(Rule::vertical(25))
                .push(col_data_representation),
        )
        .push(Rule::horizontal(25))
        .push(
            Scrollable::new(col_bytes_packets)
                .width(Length::Fill)
                .direction(Direction::Vertical(ScrollbarType::properties())),
        );

    Container::new(content)
        .width(Length::Fixed(400.0))
        .padding([10, 5, 5, 5])
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .style(ContainerType::BorderedRound)
}

fn container_chart(sniffer: &Sniffer, font: Font) -> Container<Message, Renderer<StyleType>> {
    let traffic_chart = &sniffer.traffic_chart;
    let language = sniffer.settings.language;

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
                    .padding([10, 0, 15, 0])
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

fn col_device_filters(
    language: Language,
    font: Font,
    filters: &Filters,
    device: &MyDevice,
) -> Column<'static, Message, Renderer<StyleType>> {
    #[cfg(not(target_os = "windows"))]
    let adapter_info = &device.name;
    #[cfg(target_os = "windows")]
    let adapter_name = &device.name;
    #[cfg(target_os = "windows")]
    let adapter_info = device.desc.as_ref().unwrap_or(adapter_name);

    Column::new()
        .width(Length::FillPortion(1))
        .push(TextType::highlighted_subtitle_with_desc(
            network_adapter_translation(language),
            adapter_info,
            font,
        ))
        .push(vertical_space(15))
        .push(get_active_filters_col(filters, language, font, false))
}

fn col_data_representation(
    language: Language,
    font: Font,
    chart_type: ChartType,
) -> Column<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Column::new().spacing(5).width(Length::FillPortion(1)).push(
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
            .height(Length::Fixed(33.0))
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
    all_bytes: u128,
    filtered_bytes: u128,
    font: Font,
) -> Column<'static, Message, Renderer<StyleType>> {
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
        get_formatted_bytes_string_with_b(filtered_bytes)
    } else {
        format!(
            "{} {}",
            &get_formatted_bytes_string_with_b(filtered_bytes),
            of_total_translation(language, &get_percentage_string(all_bytes, filtered_bytes))
        )
    };

    Column::new()
        .spacing(15)
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

fn get_bars_length(
    tot_width: f32,
    chart_type: ChartType,
    first_entry: &DataInfo,
    data_info: &DataInfo,
) -> (f32, f32) {
    #[allow(clippy::cast_precision_loss)]
    let (mut incoming_bar_len, mut outgoing_bar_len) = match chart_type {
        ChartType::Packets => (
            tot_width * data_info.incoming_packets as f32 / first_entry.tot_packets() as f32,
            tot_width * data_info.outgoing_packets as f32 / first_entry.tot_packets() as f32,
        ),
        ChartType::Bytes => (
            tot_width * data_info.incoming_bytes as f32 / first_entry.tot_bytes() as f32,
            tot_width * data_info.outgoing_bytes as f32 / first_entry.tot_bytes() as f32,
        ),
    };

    // normalize smaller values
    if incoming_bar_len > 0.0 && incoming_bar_len < 3.0 {
        incoming_bar_len = 3.0;
    }
    if outgoing_bar_len > 0.0 && outgoing_bar_len < 3.0 {
        outgoing_bar_len = 3.0;
    }

    (incoming_bar_len, outgoing_bar_len)
}

fn get_bars(in_len: f32, out_len: f32) -> Row<'static, Message, Renderer<StyleType>> {
    Row::new()
        .push(if in_len > 0.0 {
            Row::new()
                .width(Length::Fixed(in_len))
                .push(Rule::horizontal(1).style(RuleType::Incoming))
        } else {
            Row::new()
        })
        .push(if out_len > 0.0 {
            Row::new()
                .width(Length::Fixed(out_len))
                .push(Rule::horizontal(1).style(RuleType::Outgoing))
        } else {
            Row::new()
        })
}

fn get_star_button(is_favorite: bool, host: Host) -> Button<'static, Message, Renderer<StyleType>> {
    button(
        Icon::Star
            .to_text()
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(0)
    .height(Length::Fixed(FLAGS_WIDTH_BIG * 0.75))
    .width(Length::Fixed(FLAGS_WIDTH_BIG))
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
    show: bool,
) -> Column<'static, Message, Renderer<StyleType>> {
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
                    button(
                        Text::new("i")
                            .font(font)
                            .vertical_alignment(Vertical::Center)
                            .horizontal_alignment(Horizontal::Center)
                            .size(15),
                    )
                    .padding(2)
                    .height(Fixed(20.0))
                    .width(Fixed(20.0)),
                    filters_string,
                    Position::FollowCursor,
                )
                .font(font)
                .style(ContainerType::Tooltip),
            )
        });
    }
    ret_val
}
