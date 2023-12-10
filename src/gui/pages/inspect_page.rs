use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::{button, horizontal_space, vertical_space, Rule};
use iced::widget::{
    lazy, Button, Checkbox, Column, Container, PickList, Row, Scrollable, Text, TextInput,
};
use iced::{alignment, Alignment, Font, Length, Renderer};

use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{get_font, get_font_headers, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::types::message::Message;
use crate::networking::types::search_parameters::{FilterInputType, SearchParameters};
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::get_report_entries::get_searched_entries;
use crate::translations::translations::application_protocol_translation;
use crate::translations::translations_2::{
    administrative_entity_translation, country_translation, domain_name_translation,
    no_search_results_translation, only_show_favorites_translation, search_filters_translation,
    showing_results_translation, sort_by_translation,
};
use crate::utils::types::icon::Icon;
use crate::{Language, ReportSortType, RunningPage, Sniffer, StyleType};

/// Computes the body of gui inspect page
pub fn inspect_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let font = get_font(style);
    let font_headers = get_font_headers(style);

    let mut body = Column::new()
        .width(Length::Fill)
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new().height(Length::Fill);

    let tabs = get_pages_tabs(
        RunningPage::Inspect,
        font,
        font_headers,
        language,
        sniffer.unread_notifications,
    );

    tab_and_body = tab_and_body.push(tabs);

    let sort_active_str = sniffer.report_sort_type.get_picklist_label(language);
    let sort_list_str: Vec<&str> = ReportSortType::all_strings(language);
    let picklist_sort = PickList::new(
        sort_list_str.clone(),
        Some(sort_active_str),
        move |selected_str| {
            if selected_str == *sort_list_str.first().unwrap_or(&"") {
                Message::ReportSortSelection(ReportSortType::MostRecent)
            } else if selected_str == *sort_list_str.get(1).unwrap_or(&"") {
                Message::ReportSortSelection(ReportSortType::MostBytes)
            } else {
                Message::ReportSortSelection(ReportSortType::MostPackets)
            }
        },
    )
    .padding([3, 7])
    .font(font);

    let report = lazy(
        (
            sniffer.runtime_data.tot_sent_packets + sniffer.runtime_data.tot_received_packets,
            style,
            language,
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
                    .push(filters_col(&sniffer.search, font, language))
                    .push(Rule::vertical(25))
                    .push(
                        Column::new()
                            .spacing(10)
                            .push(
                                Text::new(sort_by_translation(language))
                                    .font(font)
                                    .style(TextType::Title)
                                    .size(FONT_SIZE_TITLE),
                            )
                            .push(picklist_sort),
                    ),
            )
            .height(Length::Fixed(165.0))
            .padding(10)
            .style(ContainerType::BorderedRound),
        )
        .push(report);

    Container::new(Column::new().push(tab_and_body.push(body))).height(Length::Fill)
}

fn lazy_report(sniffer: &Sniffer) -> Container<'static, Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let font = get_font(style);

    let (search_results, results_number) = get_searched_entries(sniffer);

    let mut col_report = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Center);

    let mut scroll_report = Column::new();
    let start_entry_num = (sniffer.page_number - 1) * 20 + 1;
    let end_entry_num = start_entry_num + search_results.len() - 1;
    for report_entry in search_results {
        let entry_text_type = if report_entry.val.traffic_direction == TrafficDirection::Outgoing {
            TextType::Outgoing
        } else {
            TextType::Incoming
        };
        let entry_row = Row::new()
            .align_items(Alignment::Center)
            .push(
                Text::new(format!("  {}{}  ", report_entry.key, report_entry.val))
                    .style(entry_text_type)
                    .font(font),
            )
            .push(report_entry.tooltip)
            .push(Text::new("  "));

        scroll_report = scroll_report.push(
            button(entry_row)
                .padding(2)
                .on_press(Message::ShowModal(MyModal::ConnectionDetails(
                    report_entry.key,
                )))
                .style(ButtonType::Neutral),
        );
    }
    if results_number > 0 {
        col_report = col_report
            .push(Text::new("      Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets     Bytes   Country").vertical_alignment(Vertical::Center).height(Length::FillPortion(2)).font(font))
            .push(Rule::horizontal(5))
            .push(
                Scrollable::new(scroll_report)
                    .height(Length::FillPortion(15))
                    .width(Length::Fill)
                    .direction(Direction::Both {
                        vertical: ScrollbarType::properties(),
                        horizontal: ScrollbarType::properties(),
                    })
            )
            .push(
                Rule::horizontal(5)
            )
            .push(get_change_page_row(
                font,
                language,
                sniffer.page_number,
                start_entry_num,
                end_entry_num,
                results_number,
            ));
    } else {
        col_report = col_report.push(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(20)
                .align_items(Alignment::Center)
                .push(vertical_space(Length::FillPortion(1)))
                .push(Icon::Funnel.to_text().size(60))
                .push(vertical_space(Length::Fixed(15.0)))
                .push(Text::new(no_search_results_translation(language)).font(font))
                .push(vertical_space(Length::FillPortion(2))),
        );
    }

    Container::new(col_report)
        .align_y(Vertical::Center)
        .align_x(Horizontal::Center)
        .padding([10, 7, 7, 7])
        .width(Length::Fixed(1042.0))
        .style(ContainerType::BorderedRound)
}

fn filters_col(
    search_params: &SearchParameters,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let search_params2 = search_params.clone();

    let mut title_row = Row::new().spacing(10).align_items(Alignment::Center).push(
        Text::new(search_filters_translation(language))
            .font(font)
            .style(TextType::Title)
            .size(FONT_SIZE_TITLE),
    );
    if search_params.is_some_filter_active() {
        title_row = title_row.push(button_clear_filter(SearchParameters::default(), font));
    }

    Column::new()
        .spacing(3)
        .push(title_row)
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
                .font(font),
            )
            .padding([5, 8])
            .style(if search_params.only_favorites {
                ContainerType::Badge
            } else {
                ContainerType::Neutral
            }),
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
                ))
                .push(filter_input(
                    FilterInputType::Country,
                    &search_params.country,
                    country_translation(language),
                    30.0,
                    search_params.clone(),
                    font,
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
                ))
                .push(filter_input(
                    FilterInputType::AS,
                    &search_params.as_name.clone(),
                    administrative_entity_translation(language),
                    120.0,
                    search_params.clone(),
                    font,
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
) -> Container<'static, Message, Renderer<StyleType>> {
    let is_filter_active = !filter_value.is_empty();

    let button_clear = button_clear_filter(
        match filter_input_type {
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
        },
        font,
    );

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
        .style(if is_filter_active {
            TextInputType::Badge
        } else {
            TextInputType::Standard
        });

    let mut content = Row::new()
        .spacing(5)
        .push(Text::new(format!("{caption}:")).font(font))
        .push(input);

    if is_filter_active {
        content = content.push(button_clear);
    }

    Container::new(content)
        .padding(5)
        .style(if is_filter_active {
            ContainerType::Badge
        } else {
            ContainerType::Neutral
        })
}

fn get_button_change_page(increment: bool) -> Button<'static, Message, Renderer<StyleType>> {
    button(
        if increment {
            Icon::ArrowRight
        } else {
            Icon::ArrowLeft
        }
        .to_text()
        .size(8.0)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(2)
    .height(Length::Fixed(20.0))
    .width(Length::Fixed(25.0))
    .on_press(Message::UpdatePageNumber(increment))
}

fn get_change_page_row(
    font: Font,
    language: Language,
    page_number: usize,
    start_entry_num: usize,
    end_entry_num: usize,
    results_number: usize,
) -> Row<'static, Message, Renderer<StyleType>> {
    Row::new()
        .height(Length::FillPortion(2))
        .align_items(Alignment::Center)
        .spacing(10)
        .push(if page_number > 1 {
            Container::new(get_button_change_page(false).width(25.0))
        } else {
            Container::new(horizontal_space(25.0))
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
        .push(if page_number < (results_number + 20 - 1) / 20 {
            Container::new(get_button_change_page(true).width(25.0))
        } else {
            Container::new(horizontal_space(25.0))
        })
}

fn button_clear_filter(
    new_search_parameters: SearchParameters,
    font: Font,
) -> Button<'static, Message, Renderer<StyleType>> {
    button(
        Text::new("×")
            .font(font)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .size(15),
    )
    .padding(2)
    .height(Length::Fixed(20.0))
    .width(Length::Fixed(20.0))
    .on_press(Message::Search(new_search_parameters))
}
