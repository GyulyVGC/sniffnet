//! Module defining the run page of the application.
//!
//! It contains elements to display traffic statistics: chart, detailed connections data
//! and overall statistics about the traffic.

use crate::chart::types::donut_chart::donut_chart;
use crate::countries::country_utils::get_flag_tooltip;
use crate::countries::flags_pictures::{FLAGS_HEIGHT_BIG, FLAGS_WIDTH_BIG};
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::pages::initial_page::get_addresses_row;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_TITLE, TOOLTIP_DELAY};
use crate::gui::styles::text::TextType;
use crate::gui::types::conf::Conf;
use crate::gui::types::filters::Filters;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::program_lookup::ProgramLookup;
use crate::report::get_report_entries::{
    get_host_entries, get_program_entries, get_service_entries,
};
use crate::report::types::search_parameters::SearchParameters;
use crate::report::types::sort_type::SortType;
use crate::translations::translations::{
    active_filters_translation, incoming_translation, none_translation, outgoing_translation,
    traffic_rate_translation,
};
use crate::translations::translations_2::{
    data_representation_translation, dropped_translation, host_translation,
    only_top_30_items_translation,
};
use crate::translations::translations_3::service_translation;
use crate::translations::translations_5::program_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, RunningPage, StyleType};
use iced::Length::Fill;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Button, Column, Container, Row, Scrollable, Space, Text, Tooltip, button};
use iced::{Alignment, Element, Length, Padding};

/// Computes the body of gui overview page
pub fn overview_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;

    let mut body = Column::new();
    let mut tab_and_body = Column::new().height(Length::Fill);

    // some packets are there!
    let tabs = get_pages_tabs(
        RunningPage::Overview,
        language,
        sniffer.unread_notifications,
    );
    tab_and_body = tab_and_body.push(tabs);

    let container_chart = container_chart(sniffer);

    let container_info = col_info(sniffer);

    let container_report = row_report(sniffer);

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

    Container::new(Column::new().push(tab_and_body.push(body))).height(Length::Fill)
}

fn row_report<'a>(sniffer: &Sniffer) -> Row<'a, Message, StyleType> {
    let col_host = col_host(sniffer);
    let col_service = col_service(sniffer);
    let container_program = sniffer.program_lookup.as_ref().map(|program_lookup| {
        Container::new(col_program(&sniffer.conf, program_lookup))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .padding(Padding::new(10.0).top(0).bottom(5))
            .class(ContainerType::BorderedRound)
    });

    Row::new()
        .spacing(10)
        .push(
            Container::new(col_host)
                .width(Length::FillPortion(5))
                .height(Length::Fill)
                .padding(Padding::new(10.0).top(0).bottom(5))
                .class(ContainerType::BorderedRound),
        )
        .push(
            Container::new(col_service)
                .width(Length::FillPortion(2))
                .height(Length::Fill)
                .padding(Padding::new(10.0).top(0).bottom(5))
                .class(ContainerType::BorderedRound),
        )
        .push(container_program)
}

fn col_host<'a>(sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;
    let data_repr = sniffer.conf.data_repr;

    let mut scroll_host = Column::new()
        .padding(Padding::ZERO.right(11.0))
        .align_x(Alignment::Center);
    let entries = get_host_entries(
        &sniffer.info_traffic,
        data_repr,
        sniffer.conf.host_sort_type,
    );
    let first_entry_data_info = entries
        .iter()
        .map(|(_, d)| d.data_info)
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, data_repr))
        .unwrap_or_default();

    for (host, data_info_host) in &entries {
        let star_button = get_star_button(data_info_host.is_favorite, host.clone());

        let host_bar = host_bar(
            host,
            data_info_host,
            data_repr,
            first_entry_data_info,
            language,
        );

        let content = Row::new()
            .align_y(Alignment::Center)
            .spacing(5)
            .push(star_button)
            .push(host_bar);

        scroll_host = scroll_host.push(
            button(content)
                .padding(Padding::new(5.0).right(15).left(10))
                .on_press(Message::Search(SearchParameters::new_host_search(host)))
                .class(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_host = scroll_host
            .push(Space::new().height(25))
            .push(Text::new(only_top_30_items_translation(language)).align_x(Alignment::Center));
    }

    Column::new()
        .push(
            Row::new()
                .height(45)
                .align_y(Alignment::Center)
                .push(
                    Text::new(host_translation(language))
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(Space::new().width(Length::Fill))
                .push(sort_arrows(
                    sniffer.conf.host_sort_type,
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

fn col_service<'a>(sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;
    let data_repr = sniffer.conf.data_repr;

    let mut scroll_service = Column::new()
        .padding(Padding::ZERO.right(11.0))
        .align_x(Alignment::Center);
    let entries = get_service_entries(
        &sniffer.info_traffic,
        data_repr,
        sniffer.conf.service_sort_type,
    );
    let first_entry_data_info = entries
        .iter()
        .map(|&(_, d)| d)
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, data_repr))
        .unwrap_or_default();

    for (service, data_info) in &entries {
        let content = simple_bar(
            service.to_string(),
            data_info,
            data_repr,
            first_entry_data_info,
        );

        scroll_service = scroll_service.push(
            button(content)
                .padding(Padding::new(5.0).right(15).left(10))
                .on_press(Message::Search(SearchParameters::new_service_search(
                    service,
                )))
                .class(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_service = scroll_service
            .push(Space::new().height(25))
            .push(Text::new(only_top_30_items_translation(language)).align_x(Alignment::Center));
    }

    Column::new()
        .push(
            Row::new()
                .height(45)
                .align_y(Alignment::Center)
                .push(
                    Text::new(service_translation(language))
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(Space::new().width(Length::Fill))
                .push(sort_arrows(
                    sniffer.conf.service_sort_type,
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

fn col_program<'a>(conf: &Conf, program_lookup: &ProgramLookup) -> Column<'a, Message, StyleType> {
    let Settings { language, .. } = conf.settings;
    let data_repr = conf.data_repr;

    let mut scroll_program = Column::new()
        .padding(Padding::ZERO.right(11.0))
        .align_x(Alignment::Center);
    let entries = get_program_entries(program_lookup, data_repr, conf.program_sort_type);
    let first_entry_data_info = entries
        .iter()
        .map(|&(_, d)| d)
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, data_repr))
        .unwrap_or_default();

    for (program, data_info) in &entries {
        let content = simple_bar(
            program.to_string(),
            data_info,
            data_repr,
            first_entry_data_info,
        );

        scroll_program = scroll_program.push(
            button(content)
                .padding(Padding::new(5.0).right(15).left(10))
                .on_press(Message::Search(SearchParameters::new_program_search(
                    program,
                )))
                .class(ButtonType::Neutral),
        );
    }

    if entries.len() >= 30 {
        scroll_program = scroll_program
            .push(Space::new().height(25))
            .push(Text::new(only_top_30_items_translation(language)).align_x(Alignment::Center));
    }

    Column::new()
        .push(
            Row::new()
                .height(45)
                .align_y(Alignment::Center)
                .push(
                    Text::new(program_translation(language))
                        .class(TextType::Title)
                        .size(FONT_SIZE_TITLE),
                )
                .push(Space::new().width(Length::Fill))
                .push(sort_arrows(
                    conf.program_sort_type,
                    Message::ProgramSortSelection,
                )),
        )
        .push(
            Scrollable::with_direction(
                scroll_program,
                Direction::Vertical(ScrollbarType::properties()),
            )
            .width(Length::Fill),
        )
}

pub fn host_bar<'a>(
    host: &Host,
    data_info_host: &DataInfoHost,
    data_repr: DataRepr,
    first_entry_data_info: DataInfo,
    language: Language,
) -> Row<'a, Message, StyleType> {
    let (incoming_bar_len, outgoing_bar_len) =
        get_bars_length(data_repr, &first_entry_data_info, &data_info_host.data_info);

    Row::new()
        .height(FLAGS_HEIGHT_BIG)
        .align_y(Alignment::Center)
        .spacing(5)
        .push(get_flag_tooltip(
            host.country,
            data_info_host,
            language,
            false,
        ))
        .push(
            Column::new()
                .spacing(1)
                .push(
                    Row::new()
                        .push(Text::new(host.to_entry_string()))
                        .push(Space::new().width(Length::Fill))
                        .push(Text::new(data_repr.formatted_string(
                            data_info_host.data_info.tot_data(data_repr),
                        ))),
                )
                .push(get_bars(incoming_bar_len, outgoing_bar_len)),
        )
}

pub fn simple_bar<'a>(
    item: String,
    data_info: &DataInfo,
    data_repr: DataRepr,
    first_entry_data_info: DataInfo,
) -> Row<'a, Message, StyleType> {
    let (incoming_bar_len, outgoing_bar_len) =
        get_bars_length(data_repr, &first_entry_data_info, data_info);

    Row::new()
        .height(FLAGS_HEIGHT_BIG)
        .align_y(Alignment::Center)
        .spacing(5)
        .push(
            Column::new()
                .spacing(1)
                .push(
                    Row::new()
                        .push(Text::new(item))
                        .push(Space::new().width(Length::Fill))
                        .push(Text::new(
                            data_repr.formatted_string(data_info.tot_data(data_repr)),
                        )),
                )
                .push(get_bars(incoming_bar_len, outgoing_bar_len)),
        )
}

fn col_info(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;

    let col_device = col_device(language, &sniffer.capture_source, &sniffer.conf.filters);

    let col_data_representation = col_data_representation(language, sniffer.conf.data_repr);

    let donut_row = donut_row(language, sniffer);

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
                .push(RuleType::Standard.vertical(25))
                .push(col_data_representation.width(Length::Fill)),
        )
        .push(RuleType::Standard.horizontal(15))
        .push(donut_row.height(Length::Fill));

    Container::new(content)
        .width(400)
        .padding(Padding::new(5.0).top(10))
        .align_x(Alignment::Center)
        .class(ContainerType::BorderedRound)
}

fn container_chart(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings { language, .. } = sniffer.conf.settings;
    let traffic_chart = &sniffer.traffic_chart;

    Container::new(
        Column::new()
            .align_x(Alignment::Center)
            .push(
                Row::new().padding([10, 0]).align_y(Alignment::Center).push(
                    traffic_rate_translation(language)
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

pub(crate) fn col_device<'a>(
    language: Language,
    cs: &'a CaptureSource,
    filters: &'a Filters,
) -> Column<'a, Message, StyleType> {
    let link_type = cs.get_link_type();
    #[cfg(not(target_os = "windows"))]
    let cs_info = cs.get_name();
    #[cfg(target_os = "windows")]
    let cs_info = cs.get_desc().unwrap_or(cs.get_name());

    let filters_desc: Element<Message, StyleType> = if filters.is_some_filter_active() {
        Row::new()
            .spacing(10)
            .push(Text::new("BPF"))
            .push(get_info_tooltip(Text::new(filters.bpf()).into()))
            .into()
    } else {
        Text::new(none_translation(language)).into()
    };

    Column::new()
        .height(Length::Fill)
        .spacing(10)
        .push(
            Column::new()
                .push(Text::new(format!("{}:", cs.title(language))).class(TextType::Subtitle))
                .push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new(format!("   {}", &cs_info)))
                        .push(get_info_tooltip(
                            Column::new()
                                .spacing(10)
                                .push(Text::new(link_type.full_print_on_one_line(language)))
                                .push(get_addresses_row(link_type, cs.get_addresses()))
                                .into(),
                        )),
                ),
        )
        .push(
            Column::new()
                .push(
                    Text::new(format!("{}:", active_filters_translation(language)))
                        .class(TextType::Subtitle),
                )
                .push(
                    Row::new()
                        .push(Text::new("   ".to_string()))
                        .push(filters_desc),
                ),
        )
}

fn col_data_representation<'a>(
    language: Language,
    data_repr: DataRepr,
) -> Column<'a, Message, StyleType> {
    let mut ret_val = Column::new().spacing(5).push(
        Text::new(format!("{}:", data_representation_translation(language)))
            .class(TextType::Subtitle),
    );

    let [bits, bytes, packets] = DataRepr::ALL.map(|option| {
        let is_active = data_repr.eq(&option);
        Button::new(
            Text::new(option.get_label(language).to_owned())
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .height(33)
        .class(if is_active {
            ButtonType::BorderedRoundSelected
        } else {
            ButtonType::BorderedRound
        })
        .on_press(Message::DataReprSelection(option))
    });

    ret_val = ret_val
        .push(Row::new().spacing(5).push(bits).push(bytes))
        .push(packets);

    ret_val
}

fn donut_row(language: Language, sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let data_repr = sniffer.conf.data_repr;

    let (in_data, out_data, dropped) = sniffer.info_traffic.get_thumbnail_data(data_repr);

    let legend_col = Column::new()
        .spacing(5)
        .push(donut_legend_entry(
            in_data,
            data_repr,
            RuleType::Incoming,
            language,
        ))
        .push(donut_legend_entry(
            out_data,
            data_repr,
            RuleType::Outgoing,
            language,
        ))
        .push(donut_legend_entry(
            dropped,
            data_repr,
            RuleType::Dropped,
            language,
        ));

    let donut_row = Row::new()
        .align_y(Vertical::Center)
        .spacing(20)
        .push(donut_chart(
            data_repr,
            in_data,
            out_data,
            dropped,
            sniffer.thumbnail,
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
    data_repr: DataRepr,
    rule_type: RuleType,
    language: Language,
) -> Row<'a, Message, StyleType> {
    let value_text = data_repr.formatted_string(value);

    let label = match rule_type {
        RuleType::Incoming => incoming_translation(language),
        RuleType::Outgoing => outgoing_translation(language),
        RuleType::Dropped => dropped_translation(language),
        _ => "",
    };

    Row::new()
        .spacing(10)
        .align_y(Alignment::Center)
        .push(Row::new().width(10).push(rule_type.horizontal(5)))
        .push(Text::new(format!("{label}: {value_text}")))
}

const MIN_BARS_LENGTH: f32 = 4.0;

pub fn get_bars_length(
    data_repr: DataRepr,
    first_entry: &DataInfo,
    data_info: &DataInfo,
) -> (u16, u16) {
    let in_val = data_info.incoming_data(data_repr);
    let out_val = data_info.outgoing_data(data_repr);
    let first_entry_tot_val = first_entry.tot_data(data_repr);

    let tot_val = in_val + out_val;
    if tot_val == 0 {
        return (0, 0);
    }

    #[allow(clippy::cast_precision_loss)]
    let tot_len = 100.0 * tot_val as f32 / first_entry_tot_val as f32;
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

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    (in_len.round() as u16, out_len.round() as u16)
}

pub fn get_bars<'a>(in_len: u16, out_len: u16) -> Row<'a, Message, StyleType> {
    Row::new()
        .push(if in_len > 0 {
            Row::new()
                .width(Length::FillPortion(in_len))
                .push(RuleType::Incoming.horizontal(5))
        } else {
            Row::new()
        })
        .push(if out_len > 0 {
            Row::new()
                .width(Length::FillPortion(out_len))
                .push(RuleType::Outgoing.horizontal(5))
        } else {
            Row::new()
        })
        .push(if in_len + out_len < 100 {
            Row::new().width(Length::FillPortion(100 - in_len - out_len))
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
    .height(FLAGS_HEIGHT_BIG)
    .width(FLAGS_WIDTH_BIG)
    .class(if is_favorite {
        ButtonType::Starred
    } else {
        ButtonType::NotStarred
    })
    .on_press(Message::AddOrRemoveFavorite(host, !is_favorite))
}

fn get_info_tooltip(tooltip_content: Element<Message, StyleType>) -> Tooltip<Message, StyleType> {
    Tooltip::new(
        Container::new(
            Text::new("i")
                .size(FONT_SIZE_FOOTER)
                .line_height(LineHeight::Relative(1.0)),
        )
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .height(20)
        .width(20)
        .class(ContainerType::BadgeInfo),
        tooltip_content,
        Position::FollowCursor,
    )
    .class(ContainerType::Tooltip)
    .delay(TOOLTIP_DELAY)
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
    use crate::gui::pages::overview_page::{MIN_BARS_LENGTH, get_bars_length};
    use crate::networking::types::data_info::DataInfo;
    use crate::networking::types::data_representation::DataRepr;

    #[test]
    fn test_get_bars_length_simple() {
        let first_entry = DataInfo::new_for_tests(50, 50, 150, 50);
        let data_info = DataInfo::new_for_tests(25, 55, 165, 30);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (25, 55)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (83, 15)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bits, &first_entry, &data_info),
            (83, 15)
        );
    }

    #[test]
    fn test_get_bars_length_normalize_small_values() {
        let first_entry = DataInfo::new_for_tests(50, 50, 150, 50);
        let mut data_info = DataInfo::new_for_tests(2, 1, 1, 0);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16 / 2, MIN_BARS_LENGTH as u16 / 2)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16, 0)
        );

        data_info = DataInfo::new_for_tests(0, 3, 0, 2);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (0, MIN_BARS_LENGTH as u16)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (0, MIN_BARS_LENGTH as u16)
        );
    }

    #[test]
    fn test_get_bars_length_normalize_very_small_values() {
        let first_entry =
            DataInfo::new_for_tests(u128::MAX / 2, u128::MAX / 2, u128::MAX / 2, u128::MAX / 2);
        let mut data_info = DataInfo::new_for_tests(1, 1, 1, 1);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16 / 2, MIN_BARS_LENGTH as u16 / 2)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16 / 2, MIN_BARS_LENGTH as u16 / 2)
        );

        data_info = DataInfo::new_for_tests(0, 1, 0, 1);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (0, MIN_BARS_LENGTH as u16)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (0, MIN_BARS_LENGTH as u16)
        );

        data_info = DataInfo::new_for_tests(1, 0, 1, 0);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16, 0)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16, 0)
        );
    }

    #[test]
    fn test_get_bars_length_complex() {
        let first_entry = DataInfo::new_for_tests(48, 7, 2, 12);

        let mut data_info = DataInfo::new_for_tests(0, 9, 0, 10);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (0, 16)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (0, 71)
        );
        data_info = DataInfo::new_for_tests(9, 0, 13, 0);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (16, 0)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (93, 0)
        );

        data_info = DataInfo::new_for_tests(4, 5, 6, 7);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (7, 9)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (43, 50)
        );
        data_info = DataInfo::new_for_tests(5, 4, 7, 6);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (9, 7)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (50, 43)
        );

        data_info = DataInfo::new_for_tests(1, 8, 1, 12);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16 / 2, 14)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (7, 86)
        );
        data_info = DataInfo::new_for_tests(8, 1, 12, 1);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (14, MIN_BARS_LENGTH as u16 / 2)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (86, 7)
        );

        data_info = DataInfo::new_for_tests(6, 1, 10, 1);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (11, MIN_BARS_LENGTH as u16 / 2)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (71, 7)
        );
        data_info = DataInfo::new_for_tests(1, 6, 1, 9);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (MIN_BARS_LENGTH as u16 / 2, 11,)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (7, 64)
        );

        data_info = DataInfo::new_for_tests(1, 6, 5, 5);
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (36, 36)
        );

        data_info = DataInfo::new_for_tests(0, 0, 0, 0);
        assert_eq!(
            get_bars_length(DataRepr::Packets, &first_entry, &data_info),
            (0, 0)
        );
        assert_eq!(
            get_bars_length(DataRepr::Bytes, &first_entry, &data_info),
            (0, 0)
        );
    }
}
