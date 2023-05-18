use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, horizontal_space, vertical_space, Checkbox, Column, Container, Row, Scrollable, Text,
    TextInput, Tooltip,
};
use iced::Length::Fixed;
use iced::{Alignment, Length};
use iced_native::widget::tooltip::Position;
use iced_native::widget::Slider;

use crate::gui::components::radio::{
    sound_bytes_threshold_radios, sound_favorite_radios, sound_packets_threshold_radios,
};
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::style_constants::{
    get_font, get_font_headers, FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE, ICONS,
};
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::notifications::types::notifications::{
    BytesNotification, FavoriteNotification, Notification, PacketsNotification,
};
use crate::translations::translations::{
    bytes_threshold_translation, favorite_notification_translation, hide_translation,
    notifications_title_translation, packets_threshold_translation, per_second_translation,
    settings_translation, specify_multiples_translation, threshold_translation, volume_translation,
};
use crate::{Language, Sniffer, StyleType};

pub fn settings_notifications_page(sniffer: &Sniffer) -> Container<Message> {
    let font = get_font(sniffer.style);
    let mut content = Column::new()
        .width(Length::Fill)
        .push(settings_header(sniffer.style, sniffer.language))
        .push(get_settings_tabs(
            [
                SettingsPage::Notifications,
                SettingsPage::Appearance,
                SettingsPage::Language,
            ],
            &["7 ", "K ", "c "],
            &[
                Message::TickInit,
                Message::OpenSettings(SettingsPage::Appearance),
                Message::OpenSettings(SettingsPage::Language),
            ],
            SettingsPage::Notifications,
            sniffer.style,
            sniffer.language,
        ))
        .push(vertical_space(Fixed(15.0)))
        .push(
            notifications_title_translation(sniffer.language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .push(vertical_space(Fixed(5.0)));

    let volume_notification_col = Column::new()
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(volume_slider(
            sniffer.language,
            sniffer.style,
            sniffer.notifications.volume,
        ))
        .push(
            Scrollable::new(
                Column::new()
                    .width(Fixed(720.0))
                    .push(get_packets_notify(
                        sniffer.notifications.packets_notification,
                        sniffer.language,
                        sniffer.style,
                    ))
                    .push(get_bytes_notify(
                        sniffer.notifications.bytes_notification,
                        sniffer.language,
                        sniffer.style,
                    ))
                    .push(get_favorite_notify(
                        sniffer.notifications.favorite_notification,
                        sniffer.language,
                        sniffer.style,
                    )),
            )
            .style(<StyleTuple as Into<iced::theme::Scrollable>>::into(
                StyleTuple(sniffer.style, ElementType::Standard),
            )),
        );

    content = content.push(volume_notification_col);

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(<StyleTuple as Into<iced::theme::Container>>::into(
            StyleTuple(sniffer.style, ElementType::Standard),
        ))
}

fn get_packets_notify(
    packets_notification: PacketsNotification,
    language: Language,
    style: StyleType,
) -> Column<'static, Message> {
    let font = get_font(style);
    let checkbox = Checkbox::new(
        packets_threshold_translation(language),
        packets_notification.threshold.is_some(),
        move |toggled| {
            if toggled {
                Message::UpdateNotificationSettings(
                    Notification::Packets(PacketsNotification {
                        threshold: Some(packets_notification.previous_threshold),
                        ..packets_notification
                    }),
                    false,
                )
            } else {
                Message::UpdateNotificationSettings(
                    Notification::Packets(PacketsNotification {
                        threshold: None,
                        ..packets_notification
                    }),
                    false,
                )
            }
        },
    )
    .size(18)
    .font(font)
    .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
        StyleTuple(style, ElementType::Standard),
    ));

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if packets_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::BorderedRound),
                )),
        )
    } else {
        let input_row = Row::new()
            .push(horizontal_space(Fixed(50.0)))
            .push(Text::new(format!("{}: ", threshold_translation(language))).font(font))
            .push(input_group_packets(packets_notification, style, language));
        let sound_row =
            Row::new()
                .push(horizontal_space(Fixed(50.0)))
                .push(sound_packets_threshold_radios(
                    packets_notification,
                    font,
                    style,
                    language,
                ));
        ret_val = ret_val
            .push(vertical_space(Fixed(5.0)))
            .push(input_row)
            .push(sound_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::BorderedRound),
                )),
        )
    }
}

fn get_bytes_notify(
    bytes_notification: BytesNotification,
    language: Language,
    style: StyleType,
) -> Column<'static, Message> {
    let font = get_font(style);
    let checkbox = Checkbox::new(
        bytes_threshold_translation(language),
        bytes_notification.threshold.is_some(),
        move |toggled| {
            if toggled {
                Message::UpdateNotificationSettings(
                    Notification::Bytes(BytesNotification {
                        threshold: Some(bytes_notification.previous_threshold),
                        ..bytes_notification
                    }),
                    false,
                )
            } else {
                Message::UpdateNotificationSettings(
                    Notification::Bytes(BytesNotification {
                        threshold: None,
                        ..bytes_notification
                    }),
                    false,
                )
            }
        },
    )
    .size(18)
    .font(font)
    .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
        StyleTuple(style, ElementType::Standard),
    ));

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if bytes_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::BorderedRound),
                )),
        )
    } else {
        let input_row = Row::new()
            .push(horizontal_space(Fixed(50.0)))
            .push(Text::new(format!("{}: ", threshold_translation(language))).font(font))
            .push(input_group_bytes(bytes_notification, style, language));
        let sound_row =
            Row::new()
                .push(horizontal_space(Fixed(50.0)))
                .push(sound_bytes_threshold_radios(
                    bytes_notification,
                    font,
                    style,
                    language,
                ));
        ret_val = ret_val
            .push(vertical_space(Fixed(5.0)))
            .push(input_row)
            .push(sound_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::BorderedRound),
                )),
        )
    }
}

fn get_favorite_notify(
    favorite_notification: FavoriteNotification,
    language: Language,
    style: StyleType,
) -> Column<'static, Message> {
    let font = get_font(style);
    let checkbox = Checkbox::new(
        favorite_notification_translation(language),
        favorite_notification.notify_on_favorite,
        move |toggled| {
            Message::UpdateNotificationSettings(
                if toggled {
                    Notification::Favorite(FavoriteNotification::on(favorite_notification.sound))
                } else {
                    Notification::Favorite(FavoriteNotification::off(favorite_notification.sound))
                },
                false,
            )
        },
    )
    .size(18)
    .font(font)
    .style(<StyleTuple as Into<iced::theme::Checkbox>>::into(
        StyleTuple(style, ElementType::Standard),
    ));

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if favorite_notification.notify_on_favorite {
        let sound_row = Row::new()
            .push(horizontal_space(Fixed(50.0)))
            .push(sound_favorite_radios(
                favorite_notification,
                font,
                style,
                language,
            ));
        ret_val = ret_val.push(vertical_space(Fixed(5.0))).push(sound_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::BorderedRound),
                )),
        )
    } else {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(<StyleTuple as Into<iced::theme::Container>>::into(
                    StyleTuple(style, ElementType::BorderedRound),
                )),
        )
    }
}

fn input_group_packets(
    packets_notification: PacketsNotification,
    style: StyleType,
    language: Language,
) -> Container<'static, Message> {
    let font = get_font(style);
    let curr_threshold_str = &packets_notification.threshold.unwrap().to_string();
    let input_row = Row::new()
        .spacing(10)
        .push(
            TextInput::new(
                "0",
                if curr_threshold_str == "0" {
                    ""
                } else {
                    curr_threshold_str
                },
            )
            .on_input(move |value| {
                let packets_notification =
                    PacketsNotification::from(&value, Some(packets_notification));
                Message::UpdateNotificationSettings(
                    Notification::Packets(packets_notification),
                    false,
                )
            })
            .padding([0, 0, 0, 10])
            .font(font)
            .width(Length::Fixed(100.0))
            .style(<StyleTuple as Into<iced::theme::TextInput>>::into(
                StyleTuple(style, ElementType::Standard),
            )),
        )
        .push(
            Text::new(per_second_translation(language))
                .font(font)
                .vertical_alignment(Vertical::Center)
                .size(FONT_SIZE_FOOTER),
        );
    Container::new(input_row)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn input_group_bytes(
    bytes_notification: BytesNotification,
    style: StyleType,
    language: Language,
) -> Container<'static, Message> {
    let font = get_font(style);
    let mut info_str = per_second_translation(language).to_string();
    info_str.push_str(specify_multiples_translation(language));
    let mut curr_threshold_str = (bytes_notification.threshold.unwrap()
        / bytes_notification.byte_multiple.get_multiplier())
    .to_string();
    curr_threshold_str.push_str(bytes_notification.byte_multiple.get_char());
    let input_row = Row::new()
        .spacing(10)
        .push(
            TextInput::new(
                "0",
                if curr_threshold_str == "0" {
                    ""
                } else {
                    &curr_threshold_str
                },
            )
            .on_input(move |value| {
                let bytes_notification = BytesNotification::from(&value, Some(bytes_notification));
                Message::UpdateNotificationSettings(Notification::Bytes(bytes_notification), false)
            })
            .padding([0, 0, 0, 10])
            .font(font)
            .width(Length::Fixed(100.0))
            .style(<StyleTuple as Into<iced::theme::TextInput>>::into(
                StyleTuple(style, ElementType::Standard),
            )),
        )
        .push(
            Text::new(info_str)
                .font(font)
                .vertical_alignment(Vertical::Center)
                .size(FONT_SIZE_FOOTER),
        );
    Container::new(input_row)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

fn volume_slider(language: Language, style: StyleType, volume: u8) -> Container<'static, Message> {
    let font = get_font(style);
    Container::new(
        Column::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(Text::new(format!("{}: {volume:^3}%", volume_translation(language))).font(font))
            .push(
                Row::new()
                    .push(
                        Text::new('Y'.to_string())
                            .width(Fixed(30.0))
                            .vertical_alignment(Vertical::Center)
                            .size(20)
                            .font(ICONS),
                    )
                    .push(
                        Slider::new(0..=100, volume, Message::ChangeVolume)
                            .step(5)
                            .width(Fixed(200.0))
                            .style(<StyleTuple as Into<iced::theme::Slider>>::into(StyleTuple(
                                style,
                                ElementType::Standard,
                            ))),
                    )
                    .push(horizontal_space(Length::Fixed(15.0)))
                    .push(
                        Text::new("Z")
                            .vertical_alignment(Vertical::Center)
                            .size(20)
                            .font(ICONS),
                    ),
            ),
    )
    .padding(5)
    .width(Length::Fill)
    .height(Length::Fixed(60.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

pub fn settings_header(style: StyleType, language: Language) -> Container<'static, Message> {
    let font = get_font(style);
    let tooltip = hide_translation(language).to_string();
    //tooltip.push_str(" [esc]");
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new(settings_translation(language))
                    .font(get_font_headers(style))
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(
                    Tooltip::new(
                        button(
                            Text::new("x")
                                .font(font)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Fixed(20.0))
                        .width(Fixed(20.0))
                        .style(StyleTuple(style, ElementType::Standard).into())
                        .on_press(Message::CloseSettings),
                        tooltip,
                        Position::Right,
                    )
                    .font(font)
                    .style(<StyleTuple as Into<iced::theme::Container>>::into(
                        StyleTuple(style, ElementType::Tooltip),
                    )),
                )
                .width(Length::FillPortion(1))
                .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Fixed(40.0))
    .width(Length::Fill)
    .style(<StyleTuple as Into<iced::theme::Container>>::into(
        StyleTuple(style, ElementType::Headers),
    ))
}
