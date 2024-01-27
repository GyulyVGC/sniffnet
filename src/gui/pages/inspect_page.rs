use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::text_input::Side;
use iced::widget::tooltip::Position;
use iced::widget::{button, horizontal_space, text_input, vertical_space, Rule, Toggler, Tooltip};
use iced::widget::{lazy, Button, Checkbox, Column, Container, Row, Scrollable, Text, TextInput};
use iced::{alignment, Alignment, Font, Length, Renderer};

use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, ICONS};
use crate::gui::styles::text::TextType;
use crate::gui::styles::text_input::TextInputType;
use crate::gui::types::message::Message;
use crate::networking::types::address_port_pair::AddressPortPair;
use crate::networking::types::info_address_port_pair::InfoAddressPortPair;
use crate::networking::types::search_parameters::{FilterInputType, SearchParameters};
use crate::networking::types::traffic_direction::TrafficDirection;
use crate::report::get_report_entries::get_searched_entries;
use crate::report::types::report_col::ReportCol;
use crate::translations::translations_2::{
    administrative_entity_translation, country_translation, domain_name_translation,
    no_search_results_translation, only_show_favorites_translation, showing_results_translation,
};
use crate::translations::translations_3::filter_by_host_translation;
use crate::utils::types::icon::Icon;
use crate::{ConfigSettings, Language, ReportSortType, RunningPage, Sniffer, StyleType};

/// Computes the body of gui inspect page
pub fn inspect_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

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
            Container::new(host_filters_col(&sniffer.search, font, language))
                .padding(10)
                .style(ContainerType::BorderedRound),
        )
        .push(report);

    Container::new(Column::new().push(tab_and_body.push(body))).height(Length::Fill)
}

fn lazy_report(sniffer: &Sniffer) -> Container<'static, Message, Renderer<StyleType>> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;

    let (search_results, results_number) = get_searched_entries(sniffer);

    let mut col_report = Column::new()
        .height(Length::Fill)
        .width(Length::Fill)
        .align_items(Alignment::Start)
        .push(report_header_row(
            language,
            &sniffer.search,
            font,
            sniffer.report_sort_type,
        ))
        .push(Rule::horizontal(5));

    let mut scroll_report = Column::new().align_items(Alignment::Start);
    let start_entry_num = (sniffer.page_number - 1) * 20 + 1;
    let end_entry_num = start_entry_num + search_results.len() - 1;
    for report_entry in search_results {
        scroll_report = scroll_report.push(
            button(row_report_entry(&report_entry.0, &report_entry.1, font))
                .padding(2)
                .on_press(Message::ShowModal(MyModal::ConnectionDetails(
                    report_entry.0,
                )))
                .style(ButtonType::Neutral),
        );
    }
    if results_number > 0 {
        col_report = col_report
            .push(
                Scrollable::new(scroll_report)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .direction(Direction::Vertical(ScrollbarType::properties())),
            )
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

fn report_header_row(
    language: Language,
    search_params: &SearchParameters,
    font: Font,
    sort_type: ReportSortType,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Row::new().align_items(Alignment::Center);
    for report_col in ReportCol::ALL {
        let width = report_col.get_width();
        let max_chars = report_col.get_max_chars() as usize;
        let full_title = report_col.get_title(language);
        let chars = full_title.chars().collect::<Vec<char>>();
        let title_tooltip = if chars.len() <= max_chars {
            Tooltip::new(
                Text::new(full_title)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .font(font),
                "",
                Position::FollowCursor,
            )
            .font(font)
            .style(ContainerType::Neutral)
        } else {
            let reduced_title = &chars[..max_chars - 3].iter().collect::<String>();
            Tooltip::new(
                Text::new([reduced_title.trim(), "..."].concat())
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .font(font),
                full_title,
                Position::FollowCursor,
            )
            .font(font)
            .style(ContainerType::Tooltip)
        };
        let mut col_header = Column::new()
            .align_items(Alignment::Center)
            .width(Length::Fixed(width))
            .height(Length::Fixed(60.0))
            .push(title_tooltip);
        if report_col != ReportCol::Packets && report_col != ReportCol::Bytes {
            col_header = col_header.push(filter_input(
                report_col.get_filter_input_type(),
                width,
                search_params.clone(),
                font,
            ));
        } else {
            col_header = col_header.push(sort_arrows(sort_type, &report_col));
        }
        ret_val = ret_val.push(col_header);
    }
    ret_val
}

fn sort_arrows(
    active_sort_type: ReportSortType,
    report_col: &ReportCol,
) -> Container<'static, Message, Renderer<StyleType>> {
    Container::new(
        button(
            active_sort_type
                .icon(report_col)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .style(active_sort_type.button_type(report_col))
        .on_press(Message::ReportSortSelection(
            active_sort_type.next_sort(report_col),
        )),
    )
    .align_y(Vertical::Center)
    .height(Length::Fill)
}

fn row_report_entry(
    key: &AddressPortPair,
    val: &InfoAddressPortPair,
    font: Font,
) -> Row<'static, Message, Renderer<StyleType>> {
    let text_type = if val.traffic_direction == TrafficDirection::Outgoing {
        TextType::Outgoing
    } else {
        TextType::Incoming
    };

    let mut ret_val = Row::new().align_items(Alignment::Center);

    for report_col in ReportCol::ALL {
        let max_chars = report_col.get_max_chars() as usize;
        let col_value = report_col.get_value(key, val);
        ret_val = ret_val.push(
            Container::new(
                Text::new(if col_value.len() <= max_chars {
                    col_value
                } else {
                    [&col_value[..max_chars - 3], "..."].concat()
                })
                .font(font)
                .style(text_type),
            )
            .align_x(Horizontal::Center)
            .width(Length::Fixed(report_col.get_width())),
        );
    }
    ret_val
}

fn host_filters_col(
    search_params: &SearchParameters,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let search_params2 = search_params.clone();

    let mut title_row = Row::new().spacing(10).align_items(Alignment::Center).push(
        Text::new(filter_by_host_translation(language))
            .font(font)
            .style(TextType::Subtitle)
            .size(FONT_SIZE_SUBTITLE),
    );
    if search_params.is_some_host_filter_active() {
        title_row = title_row.push(button_clear_filter(
            search_params.reset_host_filters(),
            font,
        ));
    }

    let input_country = filter_input(FilterInputType::Country, 80.0, search_params.clone(), font);
    let input_domain = filter_input(FilterInputType::Domain, 180.0, search_params.clone(), font);
    let input_as_name = filter_input(FilterInputType::AsName, 180.0, search_params.clone(), font);

    let container_country = Row::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .push(Text::new(format!("{}:", country_translation(language))).font(font))
        .push(input_country);

    let container_domain = Row::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .push(Text::new(format!("{}:", domain_name_translation(language))).font(font))
        .push(input_domain);

    let container_as_name = Row::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .push(Text::new(format!("{}:", administrative_entity_translation(language))).font(font))
        .push(input_as_name);

    let col1 = Column::new()
        .align_items(Alignment::Start)
        .spacing(5)
        .push(
            Container::new(
                Toggler::new(
                    only_show_favorites_translation(language).to_owned(),
                    search_params.only_favorites,
                    move |toggled| {
                        Message::Search(SearchParameters {
                            only_favorites: toggled,
                            ..search_params2.clone()
                        })
                    },
                )
                .width(Length::Shrink)
                .spacing(5)
                .size(23)
                .font(font),
            )
            .padding([5, 0]),
        )
        .push(container_country);

    let col2 = Column::new()
        .align_items(Alignment::Start)
        .spacing(5)
        .push(container_domain)
        .push(container_as_name);

    Column::new()
        .align_items(Alignment::Start)
        .push(title_row)
        .push(vertical_space(10))
        .push(
            Row::new()
                .align_items(Alignment::Center)
                .spacing(30)
                .push(col1)
                .push(col2),
        )
}

fn filter_input(
    filter_input_type: FilterInputType,
    width: f32,
    search_params: SearchParameters,
    font: Font,
) -> Container<'static, Message, Renderer<StyleType>> {
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
        .width(Length::Fixed(if is_filter_active {
            width - 45.0
        } else {
            width
        }))
        .style(if is_filter_active {
            TextInputType::Badge
        } else {
            TextInputType::Standard
        });

    if !is_filter_active {
        input = input.icon(text_input::Icon {
            font: ICONS,
            code_point: Icon::Funnel.codepoint(),
            size: Some(12.0),
            spacing: 2.0,
            side: Side::Left,
        });
    }

    let mut content = Row::new()
        .height(Length::Fill)
        .spacing(5)
        .align_items(Alignment::Center)
        .push(input);

    if is_filter_active {
        content = content.push(button_clear);
    }

    Container::new(content)
        .padding(if is_filter_active {
            [5, 5, 5, 10]
        } else {
            [5, 3, 5, 3]
        })
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
        .height(Length::Fixed(40.0))
        .align_items(Alignment::Center)
        .spacing(10)
        .push(horizontal_space(Length::Fill))
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
        .push(horizontal_space(Length::Fill))
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
