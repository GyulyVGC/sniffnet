use crate::enums::element_type::ElementType;
use crate::enums::logged_notification::{
    BytesThresholdExceeded, FavoriteTransmitted, LoggedNotification, PacketsThresholdExceeded,
};
use crate::enums::message::Message;
use crate::enums::my_overlay::MyOverlay;
use crate::enums::traffic_type::TrafficType;
use crate::gui::components::header::get_button_settings;
use crate::gui::components::tab::get_pages_tabs;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::countries::get_flag;
use crate::utility::get_formatted_strings::get_formatted_bytes_string;
use crate::utility::style_constants::{get_font, FONT_SIZE_FOOTER, HEIGHT_BODY, ICONS};
use crate::utility::translations::{
    application_protocol_translation, bytes_exceeded_translation, bytes_exceeded_value_translation,
    clear_all_translation, favorite_transmitted_translation, incoming_translation,
    no_notifications_received_translation, no_notifications_set_translation,
    only_last_30_translation, outgoing_translation, packets_exceeded_translation,
    packets_exceeded_value_translation, per_second_translation, threshold_translation,
};
use crate::{Language, RunningPage, Sniffer, StyleType};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Column, Container, Row, Scrollable, Text, Tooltip};
use iced::Length::FillPortion;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;
use iced_native::widget::{button, vertical_space};

/// Computes the body of gui notifications page
pub fn notifications_page(sniffer: &Sniffer) -> Container<Message> {
    let notifications = sniffer.notifications;
    let font = get_font(sniffer.style);

    let mut body = Column::new()
        .width(Length::Fixed(830.0))
        .padding(5)
        .spacing(10)
        .align_items(Alignment::Center);

    let mut tab_and_body = Column::new()
        .align_items(Alignment::Center)
        .height(FillPortion(HEIGHT_BODY));

    let tabs = get_pages_tabs(
        [
            RunningPage::Overview,
            //RunningPage::Inspect,
            RunningPage::Notifications,
        ],
        &["d ", "7 "],
        &[
            Message::ChangeRunningPage(RunningPage::Overview),
            // Message::ChangeRunningPage(RunningPage::Inspect),
            Message::TickInit,
        ],
        RunningPage::Notifications,
        sniffer.style,
        sniffer.language,
    );

    tab_and_body = tab_and_body
        .push(tabs)
        .push(vertical_space(Length::Fixed(15.0)));

    if notifications.packets_notification.threshold.is_none()
        && notifications.bytes_notification.threshold.is_none()
        && !notifications.favorite_notification.notify_on_favorite
        && sniffer
            .runtime_data
            .borrow()
            .logged_notifications
            .is_empty()
    {
        body = body
            .width(Length::Fill)
            .padding(5)
            .spacing(5)
            .align_items(Alignment::Center)
            .push(vertical_space(FillPortion(1)))
            .push(vertical_space(Length::Fixed(15.0)))
            .push(
                no_notifications_set_translation(sniffer.language)
                    .horizontal_alignment(Horizontal::Center)
                    .font(font),
            )
            .push(get_button_settings(
                sniffer.style,
                sniffer.language,
                MyOverlay::SettingsNotifications,
            ))
            .push(vertical_space(FillPortion(2)));
        tab_and_body = tab_and_body.push(body);
    } else if sniffer
        .runtime_data
        .borrow()
        .logged_notifications
        .is_empty()
    {
        body = body
            .width(Length::Fill)
            .padding(5)
            .spacing(5)
            .align_items(Alignment::Center)
            .push(vertical_space(FillPortion(1)))
            .push(vertical_space(Length::Fixed(15.0)))
            .push(
                no_notifications_received_translation(sniffer.language)
                    .horizontal_alignment(Horizontal::Center)
                    .font(font),
            )
            .push(Text::new(sniffer.waiting.clone()).font(font).size(50))
            .push(vertical_space(FillPortion(2)));
        tab_and_body = tab_and_body.push(body);
    } else {
        for logged_notification in &sniffer.runtime_data.borrow().logged_notifications {
            body = body.push(match logged_notification {
                LoggedNotification::PacketsThresholdExceeded(packet_threshold_exceeded) => {
                    packets_notification_log(
                        packet_threshold_exceeded.clone(),
                        sniffer.language,
                        sniffer.style,
                    )
                }
                LoggedNotification::BytesThresholdExceeded(byte_threshold_exceeded) => {
                    bytes_notification_log(
                        byte_threshold_exceeded.clone(),
                        sniffer.language,
                        sniffer.style,
                    )
                }
                LoggedNotification::FavoriteTransmitted(favorite_transmitted) => {
                    favorite_notification_log(
                        favorite_transmitted.clone(),
                        sniffer.language,
                        sniffer.style,
                    )
                }
            });
        }
        let body_row = Row::new()
            .width(Length::Fill)
            .push(
                Container::new(
                    if sniffer.runtime_data.borrow().logged_notifications.len() < 30 {
                        Text::new("")
                    } else {
                        Text::new(only_last_30_translation(sniffer.language)).font(font)
                    },
                )
                .padding(10)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            )
            .push(
                Scrollable::new(body).style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                    StyleTuple(sniffer.style, ElementType::Standard),
                )),
            )
            .push(
                Container::new(get_button_clear_all(sniffer.style, sniffer.language))
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            );
        tab_and_body = tab_and_body.push(body_row);
    }

    Container::new(Column::new().push(tab_and_body))
        .height(FillPortion(HEIGHT_BODY))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn packets_notification_log(
    logged_notification: PacketsThresholdExceeded,
    language: Language,
    style: StyleType,
) -> Container<'static, Message> {
    let font = get_font(style);
    let mut threshold_str = threshold_translation(language);
    threshold_str.push_str(&logged_notification.threshold.to_string());
    threshold_str.push_str(&format!(" {}", per_second_translation(language)));
    let mut incoming_str = " - ".to_string();
    incoming_str.push_str(incoming_translation(language));
    incoming_str.push_str(": ");
    incoming_str.push_str(&logged_notification.incoming.to_string());
    let mut outgoing_str = " - ".to_string();
    outgoing_str.push_str(outgoing_translation(language));
    outgoing_str.push_str(": ");
    outgoing_str.push_str(&logged_notification.outgoing.to_string());
    let content = Row::new()
        .spacing(30)
        .push(
            Tooltip::new(
                Text::new("e").font(ICONS).size(80),
                packets_exceeded_translation(language),
                Position::Left,
            )
            .gap(5)
            .font(font)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(style, ElementType::Tooltip),
            )),
        )
        .push(
            Column::new()
                .spacing(7)
                .width(Length::Fixed(250.0))
                .push(
                    Row::new()
                        .spacing(5)
                        .push(Text::new("9").font(ICONS))
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(Text::new(packets_exceeded_translation(language)).font(font))
                .push(Text::new(threshold_str).size(FONT_SIZE_FOOTER).font(font)),
        )
        .push(
            Column::new()
                .spacing(7)
                .push(
                    Text::new(packets_exceeded_value_translation(
                        language,
                        logged_notification.incoming + logged_notification.outgoing,
                    ))
                    .font(font),
                )
                .push(Text::new(incoming_str).font(font))
                .push(Text::new(outgoing_str).font(font)),
        );
    Container::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(800.0))
        .padding(15)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::BorderedRound),
        ))
}

fn bytes_notification_log(
    logged_notification: BytesThresholdExceeded,
    language: Language,
    style: StyleType,
) -> Container<'static, Message> {
    let font = get_font(style);
    let mut threshold_str = threshold_translation(language);
    threshold_str.push_str(
        &(logged_notification.threshold / logged_notification.byte_multiple.get_multiplier())
            .to_string(),
    );
    threshold_str.push_str(&format!(
        " {}",
        logged_notification.byte_multiple.get_char()
    ));
    threshold_str.push_str(&format!(" {}", per_second_translation(language)));
    let mut incoming_str = " - ".to_string();
    incoming_str.push_str(incoming_translation(language));
    incoming_str.push_str(": ");
    incoming_str.push_str(&get_formatted_bytes_string(u128::from(
        logged_notification.incoming,
    )));
    let mut outgoing_str = " - ".to_string();
    outgoing_str.push_str(outgoing_translation(language));
    outgoing_str.push_str(": ");
    outgoing_str.push_str(&get_formatted_bytes_string(u128::from(
        logged_notification.outgoing,
    )));
    let content = Row::new()
        .spacing(30)
        .push(
            Tooltip::new(
                Text::new("f").font(ICONS).size(80),
                bytes_exceeded_translation(language),
                Position::Left,
            )
            .gap(5)
            .font(font)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(style, ElementType::Tooltip),
            )),
        )
        .push(
            Column::new()
                .spacing(7)
                .width(Length::Fixed(250.0))
                .push(
                    Row::new()
                        .spacing(5)
                        .push(Text::new("9").font(ICONS))
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(Text::new(bytes_exceeded_translation(language)).font(font))
                .push(Text::new(threshold_str).size(FONT_SIZE_FOOTER).font(font)),
        )
        .push(
            Column::new()
                .spacing(7)
                .push(
                    Text::new(bytes_exceeded_value_translation(
                        language,
                        &get_formatted_bytes_string(u128::from(
                            logged_notification.incoming + logged_notification.outgoing,
                        )),
                    ))
                    .font(font),
                )
                .push(Text::new(incoming_str).font(font))
                .push(Text::new(outgoing_str).font(font)),
        );
    Container::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(800.0))
        .padding(15)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::BorderedRound),
        ))
}

fn favorite_notification_log(
    logged_notification: FavoriteTransmitted,
    language: Language,
    style: StyleType,
) -> Container<'static, Message> {
    let font = get_font(style);
    let traffic_type = logged_notification.connection.1.traffic_type;
    let country = logged_notification.connection.1.country;
    let src_str = format!("Src: {}", logged_notification.connection.0.address1);
    let dst_str = format!("Dst: {}", logged_notification.connection.0.address2);
    let mut app_str = application_protocol_translation(language).to_string();
    app_str.push_str(&format!(
        ": {:?}",
        logged_notification.connection.1.app_protocol
    ));
    let mut row_src_flag = Row::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(src_str).font(font));
    let mut row_dst_flag = Row::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(Text::new(dst_str).font(font));
    if !country.is_empty() {
        if traffic_type.eq(&TrafficType::Outgoing) {
            row_dst_flag = row_dst_flag.push(get_flag(&country));
        } else {
            row_src_flag = row_src_flag.push(get_flag(&country));
        }
    }
    let content = Row::new()
        .spacing(30)
        .push(
            Tooltip::new(
                Text::new("g").font(ICONS).size(80),
                favorite_transmitted_translation(language),
                Position::Left,
            )
            .gap(5)
            .font(font)
            .style(<StyleTuple as Into<iced::theme::Container>>::into(
                StyleTuple(style, ElementType::Tooltip),
            )),
        )
        .push(
            Column::new()
                .width(Length::Fixed(250.0))
                .spacing(7)
                .push(
                    Row::new()
                        .spacing(5)
                        .push(Text::new("9").font(ICONS))
                        .push(Text::new(logged_notification.timestamp).font(font)),
                )
                .push(Text::new(favorite_transmitted_translation(language)).font(font)), // .push(Text::new(threshold_str).font(font)),
        )
        .push(
            Column::new()
                .spacing(7)
                .width(Length::Fill)
                .push(row_src_flag)
                .push(row_dst_flag)
                .push(Text::new(app_str).font(font)),
        );
    Container::new(content)
        .height(Length::Fixed(120.0))
        .width(Length::Fixed(800.0))
        .padding(15)
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::BorderedRound),
        ))
}

pub fn get_button_clear_all(style: StyleType, language: Language) -> Tooltip<'static, Message> {
    let content = button(
        Text::new('h'.to_string())
            .font(ICONS)
            .size(20)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .padding(10)
    .height(Length::Fixed(50.0))
    .width(Length::Fixed(75.0))
    .style(StyleTuple(style, ElementType::Standard).into())
    .on_press(Message::ShowModal(MyOverlay::ClearAll));

    Tooltip::new(content, clear_all_translation(language), Position::Top)
        .gap(5)
        .font(get_font(style))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(style, ElementType::Tooltip),
        ))
}
