use iced::alignment::Horizontal;
use iced::widget::{
    Button, Checkbox, Column, Container, PickList, Row, Scrollable, Text, TextInput, Tooltip,
};
use iced::{alignment, Alignment, Font, Length};
use iced_lazy::lazy;
use iced_native::widget::scrollable::Properties;
use iced_native::widget::tooltip::Position;
use iced_native::widget::{button, horizontal_space, vertical_space, Rule};

use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::styles::style_constants::{get_font, FONT_SIZE_TITLE, ICONS, SARASA_MONO_SC_BOLD};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::networking::types::search_parameters::{FilterInputType, SearchParameters};
use crate::report::get_report_entries::get_searched_entries;
use crate::translations::translations::application_protocol_translation;
use crate::translations::translations_2::{
    administrative_entity_translation, country_translation, domain_name_translation,
    no_search_results_translation, only_show_favorites_translation, search_filters_translation,
    showing_results_translation, sort_by_translation,
};
use crate::utils::formatted_strings::{get_connection_color, get_open_report_tooltip};
use crate::{Language, ReportSortType, RunningPage, Sniffer, StyleType};

/// Computes the body of gui inspect page
pub fn inspect_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);

    let mut body = Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(Length::Fill);

    let tabs = get_pages_tabs(
        [
            RunningPage::Overview,
            RunningPage::Inspect,
            RunningPage::Notifications,
        ],
        &["d ", "5 ", "7 "],
        &[
            Message::ChangeRunningPage(RunningPage::Overview),
            Message::TickInit,
            Message::ChangeRunningPage(RunningPage::Notifications),
        ],
        RunningPage::Inspect,
        sniffer.style,
        sniffer.language,
        sniffer.unread_notifications,
    );

    tab_and_body = tab_and_body.push(tabs);

    let sort_active_str = sniffer
        .report_sort_type
        .get_picklist_label(sniffer.language);
    let sort_list_str: Vec<String> = ReportSortType::all_strings(sniffer.language);
    let picklist_sort = PickList::new(
        sort_list_str.clone(),
        Some(sort_active_str),
        move |selected_str| {
            if selected_str == *sort_list_str.get(0).unwrap_or(&String::new()) {
                Message::ReportSortSelection(ReportSortType::MostRecent)
            } else if selected_str == *sort_list_str.get(1).unwrap_or(&String::new()) {
                Message::ReportSortSelection(ReportSortType::MostBytes)
            } else {
                Message::ReportSortSelection(ReportSortType::MostPackets)
            }
        },
    )
    .padding([3, 7])
    .font(font)
    .style(StyleTuple(sniffer.style, ElementType::Standard));

    let report = lazy(
        (
            sniffer.runtime_data.tot_sent_packets + sniffer.runtime_data.tot_received_packets,
            sniffer.style,
            sniffer.language,
            sniffer.report_sort_type,
            sniffer.search.clone(),
            sniffer.page_number,
        ),
        move |_| lazy_report(sniffer),
    );

    body = body
        .push(
            Container::new(
                Row::new()
                    .push(filters_col(
                        &sniffer.search,
                        sniffer.style,
                        sniffer.language,
                    ))
                    .push(
                        Rule::vertical(25).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                            StyleTuple(sniffer.style, ElementType::Standard),
                        )),
                    )
                    .push(
                        Column::new()
                            .spacing(10)
                            .push(
                                Text::new(sort_by_translation(sniffer.language))
                                    .font(font)
                                    .size(FONT_SIZE_TITLE),
                            )
                            .push(picklist_sort),
                    ),
            )
            .height(Length::Fixed(160.0))
            .padding(10)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(sniffer.style, ElementType::BorderedRound),
            )),
        )
        .push(report);

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(Length::Fill)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn lazy_report(sniffer: &Sniffer) -> Row<'static, Message> {
    let font = get_font(sniffer.style);

    let (search_results, results_number) = get_searched_entries(sniffer);

    let mut col_report = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center);
    col_report = col_report
        .push(Text::new("       Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets     Bytes   Country").font(font))
        .push(Rule::horizontal(20).style(<StyleTuple as Into<iced::theme::Rule>>::into(StyleTuple(
            sniffer.style,
            ElementType::Standard,
        ))))
    ;
    let mut scroll_report = Column::new();
    let start_entry_num = (sniffer.page_number - 1) * 20 + 1;
    let end_entry_num = start_entry_num + search_results.len() - 1;
    for (key, val, flag) in search_results {
        let entry_color = get_connection_color(val.traffic_direction, sniffer.style);
        let entry_row = Row::new()
            .align_items(Alignment::Center)
            .push(
                Text::new(format!("  {}{}", key.print_gui(), val.print_gui()))
                    .style(iced::theme::Text::Color(entry_color))
                    .font(SARASA_MONO_SC_BOLD),
            )
            .push(flag)
            .push(Text::new("  "));

        scroll_report = scroll_report.push(
            button(entry_row)
                .padding(2)
                .on_press(Message::ShowModal(MyModal::ConnectionDetails(val.index)))
                .style(StyleTuple(sniffer.style, ElementType::Neutral).into()),
        );
    }
    if results_number > 0 {
        col_report = col_report
            .push(
                Scrollable::new(scroll_report)
                    .height(Length::FillPortion(15))
                    .width(Length::Fill)
                    .horizontal_scroll(Properties::new())
                    .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                        StyleTuple(sniffer.style, ElementType::Standard),
                    )),
            )
            .push(
                Rule::horizontal(20).style(<StyleTuple as Into<iced::theme::Rule>>::into(
                    StyleTuple(sniffer.style, ElementType::Standard),
                )),
            )
            .push(
                Row::new()
                    .height(Length::FillPortion(2))
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .push(if sniffer.page_number > 1 {
                        Container::new(get_button_change_page(sniffer.style, false).width(25.0))
                    } else {
                        Container::new(horizontal_space(25.0))
                    })
                    .push(
                        Text::new(showing_results_translation(
                            sniffer.language,
                            start_entry_num,
                            end_entry_num,
                            results_number,
                        ))
                        .font(font),
                    )
                    .push(if sniffer.page_number < (results_number + 20 - 1) / 20 {
                        Container::new(get_button_change_page(sniffer.style, true).width(25.0))
                    } else {
                        Container::new(horizontal_space(25.0))
                    }),
            );
    } else {
        col_report = col_report.push(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .align_items(Alignment::Center)
                .push(vertical_space(Length::FillPortion(1)))
                .push(Text::new('V'.to_string()).font(ICONS).size(60))
                .push(vertical_space(Length::Fixed(15.0)))
                .push(Text::new(no_search_results_translation(sniffer.language)).font(font))
                .push(vertical_space(Length::FillPortion(2))),
        );
    }

    Row::new()
        .spacing(15)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(horizontal_space(Length::FillPortion(1)))
        .push(
            Container::new(col_report)
                .padding([10, 7, 7, 7])
                .width(Length::Fixed(1035.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(sniffer.style, ElementType::BorderedRound),
                )),
        )
        .push(
            Container::new(get_button_open_report(
                sniffer.style,
                sniffer.language,
                font,
            ))
            .width(Length::FillPortion(1)),
        )
}

fn filters_col(
    search_params: &SearchParameters,
    style: StyleType,
    language: Language,
) -> Column<'static, Message> {
    let font = get_font(style);
    let search_params2 = search_params.clone();

    Column::new()
        .spacing(3)
        .push(
            Text::new(search_filters_translation(language))
                .font(font)
                .size(FONT_SIZE_TITLE),
        )
        .push(vertical_space(Length::Fixed(10.0)))
        .push(
            Container::new(
                Checkbox::new(
                    only_show_favorites_translation(language),
                    search_params.only_favorites,
                    move |toggled| {
                        Message::Search(SearchParameters {
                            only_favorites: toggled,
                            ..search_params2.clone()
                        })
                    },
                )
                .spacing(5)
                .size(18)
                .font(font)
                .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
                    StyleTuple(style, ElementType::Badge),
                )),
            )
            .padding([5, 8])
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(
                    style,
                    if search_params.only_favorites {
                        ElementType::Badge
                    } else {
                        ElementType::Neutral
                    },
                ),
            )),
        )
        .push(
            Row::new()
                .align_items(Alignment::Center)
                .spacing(10)
                .push(filter_input(
                    FilterInputType::App,
                    &search_params.app,
                    application_protocol_translation(language),
                    60.0,
                    search_params.clone(),
                    font,
                    style,
                ))
                .push(filter_input(
                    FilterInputType::Country,
                    &search_params.country,
                    country_translation(language),
                    30.0,
                    search_params.clone(),
                    font,
                    style,
                )),
        )
        .push(
            Row::new()
                .align_items(Alignment::Center)
                .spacing(10)
                .push(filter_input(
                    FilterInputType::Domain,
                    &search_params.domain,
                    domain_name_translation(language),
                    120.0,
                    search_params.clone(),
                    font,
                    style,
                ))
                .push(filter_input(
                    FilterInputType::AS,
                    &search_params.as_name.clone(),
                    administrative_entity_translation(language),
                    120.0,
                    search_params.clone(),
                    font,
                    style,
                )),
        )
}

fn filter_input(
    filter_input_type: FilterInputType,
    filter_value: &str,
    caption: &str,
    width: f32,
    search_params: SearchParameters,
    font: Font,
    style: StyleType,
) -> Container<'static, Message> {
    let is_filter_active = !filter_value.is_empty();

    let button_clear = button(
        Text::new("x")
            .font(font)
            .horizontal_alignment(Horizontal::Center)
            .size(15),
    )
    .padding(2)
    .height(Length::Fixed(20.0))
    .width(Length::Fixed(20.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::Search(match filter_input_type {
        FilterInputType::App => SearchParameters {
            app: String::new(),
            ..search_params.clone()
        },
        FilterInputType::Domain => SearchParameters {
            domain: String::new(),
            ..search_params.clone()
        },
        FilterInputType::Country => SearchParameters {
            country: String::new(),
            ..search_params.clone()
        },
        FilterInputType::AS => SearchParameters {
            as_name: String::new(),
            ..search_params.clone()
        },
    }));

    let input = TextInput::new("-", filter_value)
        .on_input(move |new_value| {
            Message::Search(match filter_input_type {
                FilterInputType::App => SearchParameters {
                    app: new_value.trim().to_string(),
                    ..search_params.clone()
                },
                FilterInputType::Domain => SearchParameters {
                    domain: new_value.trim().to_string(),
                    ..search_params.clone()
                },
                FilterInputType::Country => SearchParameters {
                    country: new_value.trim().to_string(),
                    ..search_params.clone()
                },
                FilterInputType::AS => SearchParameters {
                    as_name: new_value.trim().to_string(),
                    ..search_params.clone()
                },
            })
        })
        .padding([0, 5])
        .font(font)
        .width(Length::Fixed(width))
        .style(<StyleTuple as Into<iced::theme::TextInput>>::into(
            StyleTuple(
                style,
                if is_filter_active {
                    ElementType::Badge
                } else {
                    ElementType::Standard
                },
            ),
        ));

    let mut content = Row::new()
        .spacing(5)
        .push(Text::new(format!("{caption}:")).font(font))
        .push(input);

    if is_filter_active {
        content = content.push(button_clear);
    }

    Container::new(content)
        .padding(5)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(
                style,
                if is_filter_active {
                    ElementType::Badge
                } else {
                    ElementType::Neutral
                },
            ),
        ))
}

fn get_button_change_page(style: StyleType, increment: bool) -> Button<'static, Message> {
    button(
        Text::new(if increment { "j" } else { "i" })
            .size(10.0)
            .font(ICONS)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(5)
    .height(Length::Fixed(25.0))
    .width(Length::Fixed(25.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::UpdatePageNumber(increment))
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
