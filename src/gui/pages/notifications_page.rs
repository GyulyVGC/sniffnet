use crate::countries::country_utils::get_computer_tooltip;
use crate::countries::flags_pictures::FLAGS_HEIGHT_BIG;
use crate::gui::components::header::get_button_settings;
use crate::gui::components::tab::get_pages_tabs;
use crate::gui::components::types::my_modal::MyModal;
use crate::gui::pages::overview_page::{get_bars, get_bars_length, host_bar, service_bar};
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::FONT_SIZE_FOOTER;
use crate::gui::styles::text::TextType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use crate::networking::types::traffic_type::TrafficType;
use crate::notifications::types::logged_notification::{
    DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
};
use crate::report::types::sort_type::SortType;
use crate::translations::translations::{
    clear_all_translation, favorite_transmitted_translation, no_notifications_received_translation,
    no_notifications_set_translation, only_last_30_translation, per_second_translation,
    threshold_translation,
};
use crate::utils::types::icon::Icon;
use crate::{Language, RunningPage, Sniffer, StyleType};
use iced::Length::FillPortion;
use iced::widget::scrollable::Direction;
use iced::widget::text::LineHeight;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Row, Rule, Scrollable, Text, Tooltip, horizontal_space};
use iced::widget::{Space, button, vertical_space};
use iced::{Alignment, Font, Length, Padding};
use std::cmp::max;

/// Computes the body of gui notifications page
pub fn notifications_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings {
        style,
        language,
        notifications,
        ..
    } = sniffer.conf.settings;
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

    if notifications.data_notification.threshold.is_none()
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
) -> Column<'_, Message, StyleType> {
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
    logged_notification: &DataThresholdExceeded,
    first_entry_data_info: DataInfo,
    language: Language,
    font: Font,
) -> Container<'a, Message, StyleType> {
    let data_repr = logged_notification.data_repr;
    let data_string = data_repr.formatted_string(logged_notification.threshold.into());
    let icon = if data_repr == DataRepr::Packets {
        Icon::PacketsThreshold
    } else {
        Icon::BytesThreshold
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
                    Text::new(data_repr.data_exceeded_translation(language).to_string())
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
            logged_notification,
            first_entry_data_info,
            language,
            font,
        ));
    let content_and_extra = Column::new()
        .spacing(10)
        .push(content)
        .push(button_expand(
            logged_notification.id,
            logged_notification.is_expanded,
        ))
        .push_maybe(data_notification_extra(logged_notification, font, language));
    Container::new(content_and_extra)
        .width(Length::Fill)
        .padding(15)
        .class(ContainerType::BorderedRound)
}

fn favorite_notification_log<'a>(
    logged_notification: &FavoriteTransmitted,
    first_entry_data_info: DataInfo,
    data_repr: DataRepr,
    language: Language,
    font: Font,
) -> Container<'a, Message, StyleType> {
    let host_bar = host_bar(
        &logged_notification.host,
        &logged_notification.data_info_host,
        data_repr,
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
                        .push(Text::new(logged_notification.timestamp.clone()).font(font)),
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
    let Settings {
        style, language, ..
    } = sniffer.conf.settings;
    let data_repr = sniffer.traffic_chart.data_repr;
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
        .max_by(|d1, d2| d1.compare(d2, SortType::Ascending, data_repr))
        .unwrap_or_default();

    for logged_notification in &sniffer.logged_notifications.0 {
        ret_val = ret_val.push(match logged_notification {
            LoggedNotification::DataThresholdExceeded(data_threshold_exceeded) => {
                data_notification_log(
                    data_threshold_exceeded,
                    first_entry_data_info,
                    language,
                    font,
                )
            }
            LoggedNotification::FavoriteTransmitted(favorite_transmitted) => {
                favorite_notification_log(
                    favorite_transmitted,
                    first_entry_data_info,
                    data_repr,
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
    first_entry_data_info: DataInfo,
    language: Language,
    font: Font,
) -> Row<'a, Message, StyleType> {
    let data_repr = logged_notification.data_repr;
    let data_info = logged_notification.data_info;
    let (incoming_bar_len, outgoing_bar_len) =
        get_bars_length(data_repr, &first_entry_data_info, &data_info);

    Row::new()
        .align_y(Alignment::Center)
        .spacing(5)
        .push(get_computer_tooltip(
            true,
            true,
            None,
            TrafficType::Unicast,
            language,
            font,
        ))
        .push(
            Column::new()
                .spacing(1)
                .push(Row::new().push(horizontal_space()).push(
                    Text::new(data_repr.formatted_string(data_info.tot_data(data_repr))).font(font),
                ))
                .push(get_bars(incoming_bar_len, outgoing_bar_len)),
        )
}

fn button_expand<'a>(
    notification_id: usize,
    is_expanded: bool,
) -> Container<'a, Message, StyleType> {
    let button = button(
        if is_expanded {
            Icon::SortAscending
        } else {
            Icon::SortDescending
        }
        .to_text()
        .size(11)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    )
    .padding(Padding::ZERO.top(if is_expanded { 0 } else { 2 }))
    .width(25)
    .height(25)
    .on_press(Message::ExpandNotification(notification_id, !is_expanded));

    Container::new(button)
        .padding(Padding::ZERO.left(395))
        .align_y(Alignment::Center)
}

fn data_notification_extra<'a>(
    logged_notification: &DataThresholdExceeded,
    font: Font,
    language: Language,
) -> Option<Row<'a, Message, StyleType>> {
    let max_entries = max(
        logged_notification.hosts.len(),
        logged_notification.services.len(),
    );
    if !logged_notification.is_expanded || max_entries == 0 {
        return None;
    }
    let spacing = 10.0;
    #[allow(clippy::cast_precision_loss)]
    let height = (FLAGS_HEIGHT_BIG + spacing) * max_entries as f32;

    let mut hosts_col = Column::new().spacing(spacing).width(Length::FillPortion(5));
    let first_data_info_host = logged_notification
        .hosts
        .first()
        .unwrap_or(&(Host::default(), DataInfoHost::default()))
        .1
        .data_info;
    for (host, data_info_host) in &logged_notification.hosts {
        let host_bar = host_bar(
            host,
            data_info_host,
            logged_notification.data_repr,
            first_data_info_host,
            font,
            language,
        );
        hosts_col = hosts_col.push(host_bar);
    }

    let mut services_col = Column::new().spacing(spacing).width(Length::FillPortion(2));
    let first_data_info_service = logged_notification
        .services
        .first()
        .unwrap_or(&(Service::default(), DataInfo::default()))
        .1;
    for (service, data_info) in &logged_notification.services {
        let service_bar = service_bar(
            service,
            data_info,
            logged_notification.data_repr,
            first_data_info_service,
            font,
        );
        services_col = services_col.push(service_bar);
    }

    Some(
        Row::new()
            .push(hosts_col)
            .push(Container::new(Rule::vertical(30)).height(height))
            .push(services_col),
    )
}
