use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::styles::style_constants::{get_font, FONT_SIZE_FOOTER, ICONS, SARASA_MONO_SC_BOLD};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::networking::types::traffic_type::TrafficType;
use crate::report::get_report_entries::{get_report_entries, get_searched_entries};
use crate::utils::countries::{get_flag_from_country_code, FLAGS_WIDTH};
use crate::utils::formatted_strings::get_connection_color;
use crate::{Language, RunningPage, Sniffer, StyleType};
use dns_lookup::lookup_addr;
use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput, Tooltip};
use iced::{alignment, Alignment, Font, Length};
use iced_native::widget::scrollable::Properties;
use iced_native::widget::tooltip::Position;
use iced_native::widget::{button, horizontal_space};
use std::cmp::min;

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

    let mut col_report = Column::new().height(Length::Fill).width(Length::Fill);
    col_report = col_report
        .push(Text::new("       Src IP address       Src port      Dst IP address       Dst port  Layer4   Layer7     Packets      Bytes   Country").font(font))
        .push(Text::new("--------------------------------------------------------------------------------------------------------------------------").font(font))
    ;
    let mut scroll_report = Column::new();
    let (search_results, results_number) = get_searched_entries(
        &sniffer.info_traffic.clone(),
        sniffer.search.clone(),
        sniffer.page_number,
    );
    for index in &search_results {
        let info_traffic_lock = sniffer.info_traffic.lock().unwrap();
        let key_val = info_traffic_lock.map.get_index(*index).unwrap();
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

        scroll_report = scroll_report.push(
            button(entry_row)
                .padding(2)
                .on_press(Message::ShowModal(MyModal::ConnectionDetails(
                    key_val.1.index,
                )))
                .style(StyleTuple(sniffer.style, ElementType::Neutral).into()),
        );
        drop(info_traffic_lock);
    }
    col_report = col_report.push(Container::new(
        Scrollable::new(scroll_report)
            .horizontal_scroll(Properties::new())
            .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
    ));

    let start_entry_num = (sniffer.page_number - 1) * 15 + 1;
    let end_entry_num = start_entry_num + search_results.len() - 1;
    body = body
        .push(
            Row::new().push(
                Container::new(col_report)
                    .padding([10, 7, 7, 7])
                    .height(Length::Fixed(380.0))
                    .width(Length::Fixed(1050.0))
                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                        StyleTuple(sniffer.style, ElementType::BorderedRound),
                    )),
            ),
        )
        .push(
            Row::new()
                .align_items(Alignment::Center)
                .spacing(10)
                .push(if sniffer.page_number > 1 {
                    Container::new(get_button_change_page(sniffer.style, false).width(30.0))
                } else {
                    Container::new(horizontal_space(30.0))
                })
                .push(Text::new(format!(
                    "Showing {start_entry_num}-{end_entry_num} of {results_number} total results",
                )))
                .push(
                    if sniffer.page_number < f32::ceil(results_number as f32 / 15.0) as usize {
                        Container::new(get_button_change_page(sniffer.style, true).width(30.0))
                    } else {
                        Container::new(horizontal_space(30.0))
                    },
                ),
        );

    Container::new(Column::new().push(tab_and_body.push(body)))
        .height(Length::Fill)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

// fn search_bar(sniffer: &Sniffer) -> Container<'static, Message> {
//     let font = get_font(sniffer.style);
//
//     let text_input = TextInput::new("AAA", &sniffer.search)
//         .on_input(Message::Search)
//         .padding([0, 0, 0, 10])
//         .font(font)
//         .width(Length::Fixed(100.0))
//         .style(<StyleTuple as Into<iced::theme::TextInput>>::into(
//             StyleTuple(sniffer.style, ElementType::Standard),
//         ));
//     Container::new(text_input)
// }

fn get_button_change_page(style: StyleType, increment: bool) -> Button<'static, Message> {
    button(
        Text::new(if increment { "j" } else { "i" })
            .size(12.0)
            .font(ICONS)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .padding(5)
    .height(Length::Fixed(30.0))
    .width(Length::Fixed(30.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::UpdatePageNumber(increment))
}
