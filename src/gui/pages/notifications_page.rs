use iced::Length::FillPortion;
use iced::widget::scrollable::Direction;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Row, Scrollable, Text, Tooltip, horizontal_space};
use iced::widget::{Space, button, vertical_space};
use iced::{Alignment, Font, Length, Padding};

use crate::chart::types::chart_type::ChartType;
use crate::countries::flags_pictures::FLAGS_WIDTH_BIG;
use crate::gui::components::header::get_button_settings;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::overview_page::{get_bars, get_bars_length, host_bar};
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::networking::types::data_info::DataInfo;
use crate::notifications::types::logged_notification::{
    DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
};
use crate::report::types::sort_type::SortType;
use crate::translations::translations::{
    bytes_exceeded_translation, clear_all_translation, favorite_transmitted_translation,
    no_notifications_received_translation, no_notifications_set_translation,
    only_last_30_translation, packets_exceeded_translation, per_second_translation,
    threshold_translation,
};
use crate::utils::types::icon::Icon;
use crate::{ByteMultiple, ConfigSettings, Language, RunningPage, Sniffer, StyleType};

/// Computes the body of gui notifications page
pub fn notifications_page(sniffer: &Sniffer) -> Container<Message, StyleType> {
    let ConfigSettings {
        style,
        language,
        notifications,
        ..
    } = sniffer.configs.settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let mut tab_and_body = Column::new()
        .align_x(Alignment::Center)
        .height(Length::Fill);

    let tabs = get_pages_tabs(
        RunningPage::Notifications,
        font,
        font_headers,
        language,
        sniffer.unread_notifications,
    );

    tab_and_body = tab_and_body.push(tabs);

    if notifications.packets_notification.threshold.is_none()
        && notifications.bytes_notification.threshold.is_none()
        && !notifications.favorite_notification.notify_on_favorite
        && sniffer.logged_notifications.0.is_empty()
    {
        let body = body_no_notifications_set(font, language);
        tab_and_body = tab_and_body.push(body);
    } else if sniffer.logged_notifications.0.is_empty() {
        let body = body_no_notifications_received(font, language, &sniffer.dots_pulse.0);
        tab_and_body = tab_and_body.push(body);
    } else {
        let logged_notifications = logged_notifications(sniffer);
        let body_row = Row::new()
            .spacing(10)
            .padding(Padding::new(10.0).bottom(0))
            .push(
                Container::new(if sniffer.logged_notifications.0.len() < 30 {
                    Text::new("")
                } else {
                    Text::new(only_last_30_translation(language)).font(font)
                })
                .width(150)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
            )
            .push(Scrollable::with_direction(
                logged_notifications,
                Direction::Vertical(ScrollbarType::properties()),
            ))
            .push(
                Container::new(get_button_clear_all(font, language))
                    .width(150)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            );
        tab_and_body = tab_and_body.push(body_row);
    }

    Container::new(Column::new().push(tab_and_body)).height(Length::Fill)
}

fn body_no_notifications_set<'a>(font: Font, language: Language) -> Column<'a, Message, StyleType> {
    Column::new()
        .padding(5)
        .spacing(5)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(vertical_space())
        .push(
            no_notifications_set_translation(language)
                .align_x(Alignment::Center)
                .font(font),
        )
        .push(get_button_settings(
            font,
            language,
            SettingsPage::Notifications,
        ))
        .push(Space::with_height(FillPortion(2)))
}

fn body_no_notifications_received(
    font: Font,
    language: Language,
    dots: &str,
) -> Column<Message, StyleType> {
    Column::new()
        .padding(5)
        .spacing(5)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(vertical_space())
        .push(
            no_notifications_received_translation(language)
                .align_x(Alignment::Center)
                .font(font),
        )
        .push(Text::new(dots.to_owned()).font(font).size(50))
        .push(Space::with_height(FillPortion(2)))
}

fn data_notification_log<'a>(
    logged_notification: DataThresholdExceeded,
    first_entry_data_info: DataInfo,
    language: Language,
    font: Font,
) -> Container<'a, Message, StyleType> {
    let chart_type = logged_notification.chart_type;
    let data_string = if chart_type == ChartType::Bytes {
        ByteMultiple::formatted_string(logged_notification.threshold.into())
    } else {
        logged_notification.threshold.to_string()
    };
    let icon = if chart_type == ChartType::Bytes {
        Icon::BytesThreshold
    } else {
        Icon::PacketsThreshold
    }
    .to_text()
    .size(80)
    .line_height(LineHeight::Relative(1.0));
    let threshold_str = format!(
        "{}: {data_string} {}",
        threshold_translation(language),
        per_second_translation(language)
    );
    let content = Row::new()
        .align_y(Alignment::Center)
        .spacing(30)
        .push(icon)
        .push(
            Column::new()
                .spacing(7)
                .width(250)
                .push(
                    Row::new()
                        .spacing(8)
                        .push(Icon::Clock.to_text())
                        .push(Text::new(logged_notification.timestamp.clone()).font(font)),
                )
                .push(
                    Text::new(if chart_type == ChartType::Bytes {
                        bytes_exceeded_translation(language)
                    } else {
                        packets_exceeded_translation(language)
                    })
                    .class(TextType::Title)
                    .font(font),
                )
                .push(
                    Text::new(threshold_str)
                        .class(TextType::Subtitle)
                        .size(FONT_SIZE_FOOTER)
                        .font(font),
                ),
        )
        .push(threshold_bar(
            &logged_notification,
            chart_type,
            first_entry_data_info,
            font,
        ));
    let content_and_extra = Column::new()
        .push(content)
        .push(if logged_notification.is_expanded {
            Row::new().push(get_button_clear_all(font, language))
        } else {
            Row::new()
        });
    Container::new(content_and_extra)
        .width(Length::Fill)
        .padding(15)
        .class(ContainerType::BorderedRound)
}

fn favorite_notification_log<'a>(
    logged_notification: FavoriteTransmitted,
    first_entry_data_info: DataInfo,
    chart_type: ChartType,
    language: Language,
    font: Font,
) -> Container<'a, Message, StyleType> {
    let host_bar = host_bar(
        &logged_notification.host,
        &logged_notification.data_info_host,
        chart_type,
        first_entry_data_info,
        font,
        language,
    );

    let content = Row::new()
        .spacing(30)
        .align_y(Alignment::Center)
        .push(
            Icon::Star
                .to_text()
                .size(80)
                .class(TextType::Starred)
                .line_height(LineHeight::Relative(1.0)),
        )
        .push(
            Column::new()
                .width(250)
                .spacing(7)
                .push(
                    Row::new()
                        .spacing(8)
                        .push(Icon::Clock.to_text())
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(
                    Text::new(favorite_transmitted_translation(language))
                        .class(TextType::Title)
                        .font(font),
                ),
        )
        .push(host_bar);

    Container::new(content)
        .width(Length::Fill)
        .padding(15)
        .class(ContainerType::BorderedRound)
}

fn get_button_clear_all<'a>(font: Font, language: Language) -> Tooltip<'a, Message, StyleType> {
    let content = button(
        Icon::Bin
            .to_text()
            .size(20)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center),
    )
    .padding(10)
    .height(50)
    .width(75)
    .on_press(Message::ShowModal(MyModal::ClearAll));

    Tooltip::new(
        content,
        Text::new(clear_all_translation(language)).font(font),
        Position::Top,
    )
    .gap(5)
    .class(ContainerType::Tooltip)
}

fn logged_notifications<'a>(sniffer: &Sniffer) -> Column<'a, Message, StyleType> {
    let ConfigSettings {
        style, language, ..
    } = sniffer.configs.settings;
    let chart_type = sniffer.traffic_chart.chart_type;
    let font = style.get_extension().font;
    let mut ret_val = Column::new()
        .padding(Padding::ZERO.right(15))
        .spacing(10)
        .align_x(Alignment::Center);

    let first_entry_data_info = sniffer
        .logged_notifications
        .0
        .iter()
        .map(LoggedNotification::data_info)
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, chart_type))
        .unwrap_or_default();

    for logged_notification in &sniffer.logged_notifications.0 {
        ret_val = ret_val.push(match logged_notification {
            LoggedNotification::DataThresholdExceeded(data_threshold_exceeded) => {
                data_notification_log(
                    data_threshold_exceeded.clone(),
                    first_entry_data_info,
                    language,
                    font,
                )
            }
            LoggedNotification::FavoriteTransmitted(favorite_transmitted) => {
                favorite_notification_log(
                    favorite_transmitted.clone(),
                    first_entry_data_info,
                    chart_type,
                    language,
                    font,
                )
            }
        });
    }
    ret_val
}

fn threshold_bar<'a>(
    logged_notification: &DataThresholdExceeded,
    chart_type: ChartType,
    first_entry_data_info: DataInfo,
    font: Font,
) -> Row<'a, Message, StyleType> {
    let data_info = logged_notification.data_info;
    let id = logged_notification.id;
    let is_expanded = logged_notification.is_expanded;

    let (incoming_bar_len, outgoing_bar_len) =
        get_bars_length(chart_type, &first_entry_data_info, &data_info);

    Row::new()
        .align_y(Alignment::Center)
        .spacing(5)
        .push(button_expand(id, is_expanded))
        .push(
            Column::new()
                .spacing(1)
                .push(
                    Row::new().push(horizontal_space()).push(
                        Text::new(if chart_type.eq(&ChartType::Packets) {
                            data_info.tot_packets().to_string()
                        } else {
                            ByteMultiple::formatted_string(data_info.tot_bytes())
                        })
                        .font(font),
                    ),
                )
                .push(get_bars(incoming_bar_len, outgoing_bar_len)),
        )
}

fn button_expand<'a>(
    notification_id: usize,
    is_expanded: bool,
) -> Container<'a, Message, StyleType> {
    let button = button(
        if is_expanded {
            Icon::Collapse
        } else {
            Icon::Expand
        }
        .to_text()
        .size(25)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    )
    .width(FLAGS_WIDTH_BIG)
    .padding(Padding::ZERO)
    .class(ButtonType::SortArrows)
    .on_press(Message::ExpandNotification(notification_id, !is_expanded));

    Container::new(button)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}
