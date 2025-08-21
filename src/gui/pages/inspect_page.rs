use std::cmp::min;

use iced::widget::scrollable::Direction;
use iced::widget::text::LineHeight;
use iced::widget::text_input::Side;
use iced::widget::tooltip::Position;
use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput};
use iced::widget::{
    ComboBox, Rule, Space, Toggler, Tooltip, button, combo_box, horizontal_space, text_input,
    vertical_space,
};
use iced::{Alignment, Font, Length, Padding, Pixels, alignment};

use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::overview_page::{get_bars, get_bars_length};
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, ICONS};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host_data_states::HostStates;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::get_report_entries::get_searched_entries;
use crate::report::types::report_col::ReportCol;
use crate::report::types::search_parameters::{FilterInputType, SearchParameters};
use crate::translations::translations_2::{
    administrative_entity_translation, country_translation, domain_name_translation,
    no_search_results_translation, only_show_favorites_translation, showing_results_translation,
};
use crate::translations::translations_3::filter_by_host_translation;
use crate::utils::types::icon::Icon;
use crate::{Language, ReportSortType, RunningPage, Sniffer, StyleType};

/// Computes the body of gui inspect page
pub fn inspect_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings {
        style, language, ..
    } = sniffer.conf.settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let mut body = Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_x(Alignment::Center);

    let mut tab_and_body = Column::new().height(Length::Fill);

    let tabs = get_pages_tabs(
        RunningPage::Inspect,
        font,
        font_headers,
        language,
        sniffer.unread_notifications,
    );

    tab_and_body = tab_and_body.push(tabs);

    let report = report(sniffer);

    let col_report = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Alignment::Start)
        .push(report_header_row(
            language,
            &sniffer.search,
            font,
            sniffer.conf.report_sort_type,
            sniffer.traffic_chart.data_repr,
        ))
        .push(Space::with_height(4))
        .push(Rule::horizontal(5))
        .push(report);

    body = body
        .push(
            Container::new(host_filters_col(
                &sniffer.search,
                &sniffer.host_data_states.states,
                font,
                language,
            ))
            .padding(10)
            .class(ContainerType::BorderedRound),
        )
        .push(
            Container::new(col_report)
                .align_y(Alignment::Center)
                .align_x(Alignment::Center)
                .padding(Padding::new(7.0).top(10).bottom(3))
                .width(947)
                .class(ContainerType::BorderedRound),
        );

    Container::new(Column::new().push(tab_and_body.push(body))).height(Length::Fill)
}

fn report<'a>(sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let Settings {
        style, language, ..
    } = sniffer.conf.settings;
    let data_repr = sniffer.traffic_chart.data_repr;
    let font = style.get_extension().font;

    let (search_results, results_number, agglomerate) = get_searched_entries(sniffer);

    let mut ret_val = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Alignment::Start);

    let mut scroll_report = Column::new().align_x(Alignment::Start);
    let start_entry_num = (sniffer.page_number.saturating_sub(1)) * 20 + 1;
    let end_entry_num = start_entry_num + search_results.len() - 1;
    for report_entry in search_results {
        scroll_report = scroll_report.push(
            button(row_report_entry(
                &report_entry.0,
                &report_entry.1,
                data_repr,
                font,
            ))
            .padding(2)
            .on_press(Message::ShowModal(MyModal::ConnectionDetails(
                report_entry.0,
            )))
            .class(ButtonType::Neutral),
        );
    }
    if results_number > 0 {
        ret_val = ret_val
            .push(
                Scrollable::with_direction(
                    scroll_report,
                    Direction::Vertical(ScrollbarType::properties()),
                )
                .height(Length::Fill)
                .width(Length::Fill),
            )
            .push(Rule::horizontal(5))
            .push(get_agglomerates_row(
                font,
                agglomerate,
                sniffer.traffic_chart.data_repr,
            ))
            .push(Rule::horizontal(5))
            .push(get_change_page_row(
                font,
                language,
                sniffer.page_number,
                start_entry_num,
                end_entry_num,
                results_number,
            ));
    } else {
        ret_val = ret_val.push(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .align_x(Alignment::Center)
                .push(vertical_space())
                .push(Icon::Funnel.to_text().size(60))
                .push(Space::with_height(15))
                .push(Text::new(no_search_results_translation(language)).font(font))
                .push(Space::with_height(Length::FillPortion(2))),
        );
    }

    ret_val
}

fn report_header_row(
    language: Language,
    search_params: &SearchParameters,
    font: Font,
    sort_type: ReportSortType,
    data_repr: DataRepr,
) -> Row<'_, Message, StyleType> {
    let mut ret_val = Row::new().padding([0, 2]).align_y(Alignment::Center);
    for report_col in ReportCol::ALL {
        let (title_display, title_small_display, tooltip_val) =
            title_report_col_display(&report_col, data_repr, language);
        let title_row = Row::new()
            .align_y(Alignment::End)
            .push(Text::new(title_display).font(font))
            .push(
                Text::new(title_small_display)
                    .font(font)
                    .size(FONT_SIZE_FOOTER),
            );
        let tooltip_style = if tooltip_val.is_empty() {
            ContainerType::Standard
        } else {
            ContainerType::Tooltip
        };
        let title_tooltip = Tooltip::new(
            title_row,
            Text::new(tooltip_val).font(font),
            Position::FollowCursor,
        )
        .class(tooltip_style);

        let mut col_header = Column::new()
            .align_x(Alignment::Center)
            .width(report_col.get_width())
            .height(56)
            .push(title_tooltip);
        if report_col == ReportCol::Data {
            col_header = col_header.push(sort_arrows(sort_type));
        } else {
            col_header = col_header.push(
                Container::new(filter_input(
                    report_col.get_filter_input_type(),
                    search_params.clone(),
                    font,
                ))
                .height(Length::Fill)
                .align_y(Alignment::Center),
            );
        }
        ret_val = ret_val.push(col_header);
    }
    ret_val
}

fn title_report_col_display(
    report_col: &ReportCol,
    data_repr: DataRepr,
    language: Language,
) -> (String, String, String) {
    let max_chars = report_col.get_max_chars(Some(language));
    let title = report_col.get_title(language, data_repr);
    let title_direction_info = report_col.get_title_direction_info(language);
    let chars_title = title.chars().collect::<Vec<char>>();
    let chars_title_direction_info = title_direction_info.chars().collect::<Vec<char>>();
    if chars_title.len() + chars_title_direction_info.len() <= max_chars {
        (title, title_direction_info, String::new())
    } else if chars_title.len() >= max_chars - 4 {
        (
            chars_title[..min(max_chars - 2, chars_title.len())]
                .iter()
                .collect::<String>(),
            String::from("…"),
            [title, title_direction_info].concat(),
        )
    } else {
        // title length is < max_chars - 4, but with direction info the whole thing is too long
        (
            title.clone(),
            [
                &chars_title_direction_info[..max_chars - chars_title.len() - 2]
                    .iter()
                    .collect::<String>(),
                "…",
            ]
            .concat(),
            [title, title_direction_info].concat(),
        )
    }
}

fn sort_arrows<'a>(active_sort_type: ReportSortType) -> Container<'a, Message, StyleType> {
    Container::new(
        button(
            active_sort_type
                .icon()
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .class(active_sort_type.button_type())
        .on_press(Message::ReportSortSelection(active_sort_type.next_sort())),
    )
    .align_y(Alignment::Center)
    .height(Length::Fill)
}

fn row_report_entry<'a>(
    key: &AddressPortPair,
    val: &InfoAddressPortPair,
    data_repr: DataRepr,
    font: Font,
) -> Row<'a, Message, StyleType> {
    let text_type = if val.traffic_direction == TrafficDirection::Outgoing {
        TextType::Outgoing
    } else {
        TextType::Incoming
    };

    let mut ret_val = Row::new().align_y(Alignment::Center);

    for report_col in ReportCol::ALL {
        let max_chars = report_col.get_max_chars(None);
        let col_value = report_col.get_value(key, val, data_repr);
        ret_val = ret_val.push(
            Container::new(
                Text::new(if col_value.len() <= max_chars {
                    col_value
                } else {
                    [&col_value[..max_chars - 2], "…"].concat()
                })
                .font(font)
                .class(text_type),
            )
            .align_x(Alignment::Center)
            .width(report_col.get_width()),
        );
    }
    ret_val
}

fn host_filters_col<'a>(
    search_params: &'a SearchParameters,
    host_states: &'a HostStates,
    font: Font,
    language: Language,
) -> Column<'a, Message, StyleType> {
    let search_params2 = search_params.clone();

    let mut title_row = Row::new().spacing(10).align_y(Alignment::Center).push(
        Text::new(filter_by_host_translation(language))
            .font(font)
            .class(TextType::Subtitle)
            .size(FONT_SIZE_SUBTITLE),
    );
    if search_params.is_some_host_filter_active() {
        title_row = title_row.push(button_clear_filter(
            search_params.reset_host_filters(),
            font,
        ));
    }

    let combobox_country = filter_combobox(
        FilterInputType::Country,
        &host_states.countries,
        search_params.clone(),
        font,
    )
    .width(95);

    let combobox_domain = filter_combobox(
        FilterInputType::Domain,
        &host_states.domains,
        search_params.clone(),
        font,
    )
    .width(190);

    let combobox_as_name = filter_combobox(
        FilterInputType::AsName,
        &host_states.asns,
        search_params.clone(),
        font,
    )
    .width(190);

    let container_country = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .push(Text::new(format!("{}:", country_translation(language))).font(font))
        .push(combobox_country);

    let container_domain = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .push(Text::new(format!("{}:", domain_name_translation(language))).font(font))
        .push(combobox_domain);

    let container_as_name = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .push(Text::new(format!("{}:", administrative_entity_translation(language))).font(font))
        .push(combobox_as_name);

    let col1 = Column::new()
        .align_x(Alignment::Start)
        .spacing(5)
        .push(
            Container::new(
                Toggler::new(search_params.only_favorites)
                    .label(only_show_favorites_translation(language).to_owned())
                    .on_toggle(move |toggled| {
                        Message::Search(SearchParameters {
                            only_favorites: toggled,
                            ..search_params2.clone()
                        })
                    })
                    .width(Length::Shrink)
                    .spacing(5)
                    .size(23)
                    .font(font),
            )
            .padding([5, 0]),
        )
        .push(container_domain);

    let col2 = Column::new()
        .align_x(Alignment::Start)
        .spacing(5)
        .push(container_country)
        .push(container_as_name);

    Column::new()
        .align_x(Alignment::Start)
        .push(title_row)
        .push(Space::with_height(10))
        .push(
            Row::new()
                .align_y(Alignment::Center)
                .spacing(30)
                .push(col1)
                .push(col2),
        )
}

fn filter_input<'a>(
    filter_input_type: FilterInputType,
    search_params: SearchParameters,
    font: Font,
) -> Container<'a, Message, StyleType> {
    let filter_value = filter_input_type.current_value(&search_params);
    let is_filter_active = !filter_value.is_empty();

    let button_clear = button_clear_filter(filter_input_type.clear_search(&search_params), font);

    let mut input = TextInput::new("", filter_value)
        .on_input(move |new_value| {
            Message::Search(filter_input_type.new_search(&search_params, new_value))
        })
        .padding([2, 5])
        .size(FONT_SIZE_FOOTER)
        .font(font)
        .width(Length::Fill)
        .class(if is_filter_active {
            TextInputType::Badge
        } else {
            TextInputType::Standard
        });

    if !is_filter_active {
        input = input.icon(text_input::Icon {
            font: ICONS,
            code_point: Icon::Funnel.codepoint(),
            size: Some(Pixels(12.0)),
            spacing: 2.0,
            side: Side::Left,
        });
    }

    let mut content = Row::new().spacing(5).align_y(Alignment::Center).push(input);

    if is_filter_active {
        content = content.push(button_clear);
    }

    Container::new(content)
        .padding(if is_filter_active {
            Padding::new(5.0).left(10)
        } else {
            Padding::new(5.0).right(3).left(3)
        })
        .class(if is_filter_active {
            ContainerType::Badge
        } else {
            ContainerType::Standard
        })
}

fn filter_combobox(
    filter_input_type: FilterInputType,
    combo_box_state: &combo_box::State<String>,
    search_params: SearchParameters,
    font: Font,
) -> Container<'_, Message, StyleType> {
    let filter_value = filter_input_type.current_value(&search_params).to_string();
    let is_filter_active = !filter_value.is_empty();

    let button_clear = button_clear_filter(filter_input_type.clear_search(&search_params), font);

    let update_fn =
        move |new_value| Message::Search(filter_input_type.new_search(&search_params, new_value));

    let mut combobox = ComboBox::new(combo_box_state, "", Some(&filter_value), update_fn.clone())
        .on_input(update_fn)
        .padding([2, 5])
        .size(FONT_SIZE_FOOTER)
        .font(font)
        .width(Length::Fill)
        .input_class(if is_filter_active {
            TextInputType::Badge
        } else {
            TextInputType::Standard
        });

    if !is_filter_active {
        combobox = combobox.icon(text_input::Icon {
            font: ICONS,
            code_point: Icon::Funnel.codepoint(),
            size: Some(Pixels(12.0)),
            spacing: 2.0,
            side: Side::Left,
        });
    }

    let mut content = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .push(combobox);

    if is_filter_active {
        content = content.push(button_clear);
    }

    Container::new(content)
        .padding(if is_filter_active {
            Padding::new(5.0).left(10)
        } else {
            Padding::new(5.0).right(3).left(3)
        })
        .class(if is_filter_active {
            ContainerType::Badge
        } else {
            ContainerType::Standard
        })
}

fn get_button_change_page<'a>(increment: bool) -> Button<'a, Message, StyleType> {
    button(
        if increment {
            Icon::ArrowRight
        } else {
            Icon::ArrowLeft
        }
        .to_text()
        .size(8.0)
        .align_x(alignment::Alignment::Center)
        .align_y(alignment::Alignment::Center),
    )
    .padding(2)
    .height(20)
    .width(25)
    .on_press(Message::UpdatePageNumber(increment))
}

fn get_agglomerates_row<'a>(
    font: Font,
    tot: DataInfo,
    data_repr: DataRepr,
) -> Row<'a, Message, StyleType> {
    let (in_length, out_length) = get_bars_length(data_repr, &tot, &tot);
    let bars = get_bars(in_length, out_length).width(ReportCol::FILTER_COLUMNS_WIDTH);

    let data_col = Column::new()
        .align_x(Alignment::Center)
        .width(ReportCol::Data.get_width())
        .push(Text::new(data_repr.formatted_string(tot.tot_data(data_repr))).font(font));

    Row::new()
        .padding([0, 2])
        .height(40)
        .align_y(Alignment::Center)
        .push(bars)
        .push(data_col)
}

fn get_change_page_row<'a>(
    font: Font,
    language: Language,
    page_number: usize,
    start_entry_num: usize,
    end_entry_num: usize,
    results_number: usize,
) -> Row<'a, Message, StyleType> {
    Row::new()
        .height(40)
        .align_y(Alignment::Center)
        .spacing(10)
        .push(horizontal_space())
        .push(if page_number > 1 {
            Container::new(get_button_change_page(false).width(25))
        } else {
            Container::new(Space::with_width(25))
        })
        .push(
            Text::new(showing_results_translation(
                language,
                start_entry_num,
                end_entry_num,
                results_number,
            ))
            .font(font),
        )
        .push(if page_number < results_number.div_ceil(20) {
            Container::new(get_button_change_page(true).width(25))
        } else {
            Container::new(Space::with_width(25))
        })
        .push(horizontal_space())
}

fn button_clear_filter<'a>(
    new_search_parameters: SearchParameters,
    font: Font,
) -> Button<'a, Message, StyleType> {
    button(
        Text::new("×")
            .font(font)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .size(15)
            .line_height(LineHeight::Relative(1.0)),
    )
    .padding(2)
    .height(20)
    .width(20)
    .on_press(Message::Search(new_search_parameters))
}

#[cfg(test)]
mod tests {
    use crate::gui::pages::inspect_page::title_report_col_display;
    use crate::networking::types::data_representation::DataRepr;
    use crate::report::types::report_col::ReportCol;
    use crate::translations::types::language::Language;

    #[test]
    fn test_table_titles_display_and_tooltip_values_for_each_language() {
        // check glyph len when adding new language...
        assert_eq!(Language::ALL.len(), 22);
        for report_col in ReportCol::ALL {
            for data_repr in DataRepr::ALL {
                for language in Language::ALL {
                    let (title, title_small, tooltip_val) =
                        title_report_col_display(&report_col, data_repr, language);
                    let title_chars = title.chars().collect::<Vec<char>>();
                    let title_small_chars = title_small.chars().collect::<Vec<char>>();
                    let max_chars = report_col.get_max_chars(Some(language));
                    if tooltip_val.is_empty() {
                        // all is entirely displayed
                        assert!(title_chars.len() + title_small_chars.len() <= max_chars);
                        assert_eq!(title, report_col.get_title(language, data_repr));
                        assert_eq!(title_small, report_col.get_title_direction_info(language));
                    } else {
                        // tooltip is the full concatenation
                        assert_eq!(
                            tooltip_val,
                            [
                                report_col.get_title(language, data_repr),
                                report_col.get_title_direction_info(language)
                            ]
                            .concat()
                        );
                        if report_col.get_title_direction_info(language).len() == 0 {
                            // displayed values have max len -1 (they include "…" that counts for 2 units)
                            assert_eq!(title_chars.len() + title_small_chars.len(), max_chars - 1);
                        } else {
                            match title_chars.len() {
                                x if x == max_chars - 4 || x == max_chars - 3 => {
                                    assert_eq!(title_small_chars.len(), 1)
                                }
                                _ => assert_eq!(
                                    title_chars.len() + title_small_chars.len(),
                                    max_chars - 1
                                ),
                            }
                        }
                        if title != report_col.get_title(language, data_repr) {
                            // first title part is not full, so second one is suspensions
                            assert_eq!(title_small, "…");
                            // check len wrt max
                            assert!(title_chars.len() >= max_chars - 4);
                            // first title part is max - 2 chars of full self
                            assert_eq!(
                                title,
                                report_col
                                    .get_title(language, data_repr)
                                    .chars()
                                    .collect::<Vec<char>>()[..max_chars - 2]
                                    .iter()
                                    .collect::<String>()
                            );
                        } else {
                            // first part is untouched
                            // second title part is max - title.len - 2 chars of full self, plus suspensions
                            let mut second_part = [
                                &report_col
                                    .get_title_direction_info(language)
                                    .chars()
                                    .collect::<Vec<char>>()[..max_chars - 2 - title_chars.len()]
                                    .iter()
                                    .collect::<String>(),
                                "…",
                            ]
                            .concat();
                            if second_part == String::from(" (…")
                                || second_part == String::from(" …")
                            {
                                second_part = String::from("…");
                            }
                            assert_eq!(title_small, second_part);
                            // second part never terminates with "(…"
                            assert!(!title_small.ends_with("(…"));
                            // second part never terminates with " …"
                            assert!(!title_small.ends_with(" …"));
                        }
                    }
                }
            }
        }
    }
}
