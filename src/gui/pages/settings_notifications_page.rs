use iced::widget::scrollable::Direction;
use iced::widget::{Button, Slider, row};
use iced::widget::{Checkbox, Column, Container, Row, Scrollable, Space, Text, TextInput};
use iced::{Alignment, Length, Padding};

use crate::gui::components::button::button_hide;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::rule::RuleType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::data_representation::DataRepr;
use crate::notifications::types::notifications::{
    DataNotification, Notification, RemoteNotifications, SimpleNotification,
};
use crate::notifications::types::sound::Sound;
use crate::translations::translations::{
    favorite_transmitted_translation, notifications_title_translation, per_second_translation,
    settings_translation, sound_translation, threshold_translation, volume_translation,
};
use crate::translations::translations_2::data_representation_translation;
use crate::translations::translations_4::data_exceeded_translation;
use crate::translations::translations_5::{
    blacklisted_transmitted_translation, remote_notifications_translation,
};
use crate::utils::types::icon::Icon;
use crate::{Language, Sniffer, StyleType};

const CONTAINERS_WIDTH: f32 = 715.0;

pub fn settings_notifications_page<'a>(sniffer: &Sniffer) -> Container<'a, Message, StyleType> {
    let Settings {
        language,
        color_gradient,
        mut notifications,
        ..
    } = sniffer.conf.settings.clone();

    // Use threshold that has not yet been applied, if available
    if let Some(temp_data_notification) = sniffer.timing_events.temp_threshold() {
        notifications.data_notification.threshold = temp_data_notification.threshold;
        notifications.data_notification.byte_multiple = temp_data_notification.byte_multiple;
        notifications.data_notification.previous_threshold =
            temp_data_notification.previous_threshold;
    }

    let mut content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(color_gradient, language))
        .push(get_settings_tabs(SettingsPage::Notifications, language))
        .push(Space::new().height(15))
        .push(
            notifications_title_translation(language)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .push(Space::new().height(5));

    let volume_notification_col = Column::new()
        .spacing(10)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(volume_slider(language, notifications.volume))
        .push(Scrollable::with_direction(
            Column::new()
                .padding(Padding::ZERO.bottom(10))
                .spacing(10)
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .push(get_data_notify(notifications.data_notification, language))
                .push(get_favorite_notify(
                    notifications.favorite_notification,
                    language,
                ))
                .push(get_ip_blacklist_notify(
                    notifications.ip_blacklist_notification,
                    language,
                ))
                .push(
                    Container::new(RuleType::Standard.horizontal(10))
                        .padding(Padding::ZERO.left(40).right(40)),
                )
                .push(get_remote_notifications(
                    &notifications.remote_notifications,
                    language,
                )),
            Direction::Vertical(ScrollbarType::properties().margin(15)),
        ));

    content = content.push(volume_notification_col);

    Container::new(content)
        .height(400)
        .width(800)
        .class(ContainerType::Modal)
}

fn get_data_notify<'a>(
    data_notification: DataNotification,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let checkbox = Checkbox::new(data_notification.threshold.is_some())
        .label(data_exceeded_translation(language))
        .on_toggle(move |toggled| {
            if toggled {
                Message::UpdateNotificationSettings(
                    Notification::Data(DataNotification {
                        threshold: Some(data_notification.previous_threshold),
                        ..data_notification
                    }),
                    false,
                )
            } else {
                Message::UpdateNotificationSettings(
                    Notification::Data(DataNotification {
                        threshold: None,
                        ..data_notification
                    }),
                    false,
                )
            }
        })
        .size(18);

    let mut ret_val = Column::new().spacing(15).push(checkbox);

    if data_notification.threshold.is_none() {
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    } else {
        let data_representation_row =
            row_data_representation(data_notification, language, data_notification.data_repr);
        let input_row = input_group_bytes(data_notification, language);
        let sound_row = sound_buttons(Notification::Data(data_notification), language);
        ret_val = ret_val
            .push(sound_row)
            .push(data_representation_row)
            .push(input_row);

        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    }
}

fn get_favorite_notify<'a>(
    favorite_notification: SimpleNotification,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let checkbox = Checkbox::new(favorite_notification.is_active)
        .label(favorite_transmitted_translation(language))
        .on_toggle(move |toggled| {
            Message::UpdateNotificationSettings(
                if toggled {
                    Notification::Favorite(SimpleNotification::on(favorite_notification.sound))
                } else {
                    Notification::Favorite(SimpleNotification::off(favorite_notification.sound))
                },
                false,
            )
        })
        .size(18);

    let mut ret_val = Column::new().spacing(15).push(checkbox);

    if favorite_notification.is_active {
        let sound_row = sound_buttons(Notification::Favorite(favorite_notification), language);
        ret_val = ret_val.push(sound_row);
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    } else {
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    }
}

fn get_ip_blacklist_notify<'a>(
    ip_blacklist_notification: SimpleNotification,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let checkbox = Checkbox::new(ip_blacklist_notification.is_active)
        .label(blacklisted_transmitted_translation(language))
        .on_toggle(move |toggled| {
            Message::UpdateNotificationSettings(
                if toggled {
                    Notification::IpBlacklist(SimpleNotification::on(
                        ip_blacklist_notification.sound,
                    ))
                } else {
                    Notification::IpBlacklist(SimpleNotification::off(
                        ip_blacklist_notification.sound,
                    ))
                },
                false,
            )
        })
        .size(18);

    let mut ret_val = Column::new().spacing(15).push(checkbox);

    if ip_blacklist_notification.is_active {
        let sound_row = sound_buttons(
            Notification::IpBlacklist(ip_blacklist_notification),
            language,
        );
        ret_val = ret_val.push(sound_row);
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    } else {
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    }
}

fn get_remote_notifications<'a>(
    remote_notifications: &RemoteNotifications,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let checkbox = Checkbox::new(remote_notifications.is_active())
        .label(remote_notifications_translation(language))
        .on_toggle(move |_| Message::ToggleRemoteNotifications)
        .size(18);

    let mut ret_val = Column::new().spacing(15).push(checkbox);

    if remote_notifications.is_active() {
        let input_row = Row::new()
            .spacing(5)
            .align_y(Alignment::Center)
            .padding(Padding::ZERO.left(26))
            .push(Text::new("URL:".to_string()))
            .push(
                TextInput::new("https://example.com/notify", remote_notifications.url())
                    .on_input(Message::RemoteNotificationsUrl)
                    .padding([2, 5]),
            );
        ret_val = ret_val.push(input_row);
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    } else {
        Container::new(ret_val)
            .padding(15)
            .width(CONTAINERS_WIDTH)
            .class(ContainerType::BorderedRound)
    }
}

fn input_group_bytes<'a>(
    bytes_notification: DataNotification,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let mut curr_threshold_str = (bytes_notification.threshold.unwrap_or_default()
        / bytes_notification.byte_multiple.multiplier())
    .to_string();
    curr_threshold_str.push_str(&bytes_notification.byte_multiple.get_char());
    let input_row = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .padding(Padding::ZERO.left(26))
        .push(Text::new(format!("{}:", threshold_translation(language))))
        .push(
            TextInput::new(
                "0",
                if curr_threshold_str.starts_with('0') {
                    ""
                } else {
                    &curr_threshold_str
                },
            )
            .on_input(move |value| {
                let bytes_notification = DataNotification::from(&value, Some(bytes_notification));
                Message::UpdateNotificationSettings(Notification::Data(bytes_notification), false)
            })
            .padding([2, 5])
            .width(100),
        )
        .push(
            Text::new(per_second_translation(language))
                .align_y(Alignment::Center)
                .size(FONT_SIZE_FOOTER),
        );
    Container::new(input_row)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}

fn volume_slider<'a>(language: Language, volume: u8) -> Container<'a, Message, StyleType> {
    Container::new(
        Column::new()
            .spacing(5)
            .align_x(Alignment::Center)
            .push(Text::new(format!(
                "{}: {volume:^3}%",
                volume_translation(language)
            )))
            .push(
                Row::new()
                    .align_y(Alignment::Center)
                    .push(
                        Icon::AudioMute
                            .to_text()
                            .width(30)
                            .align_y(Alignment::Center)
                            .size(20),
                    )
                    .push(
                        Slider::new(0..=100, volume, Message::ChangeVolume)
                            .step(5)
                            .width(200),
                    )
                    .push(Space::new().width(15))
                    .push(
                        Icon::AudioHigh
                            .to_text()
                            .align_y(Alignment::Center)
                            .size(20),
                    ),
            ),
    )
    .padding(5)
    .width(Length::Fill)
    .height(60)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
}

fn sound_buttons<'a>(
    notification: Notification,
    language: Language,
) -> row::Wrapping<'a, Message, StyleType> {
    let current_sound = match notification {
        Notification::Data(n) => n.sound,
        Notification::Favorite(n) => n.sound,
        Notification::IpBlacklist(n) => n.sound,
    };

    let mut ret_val = Row::new()
        .width(Length::Shrink)
        .align_y(Alignment::Center)
        .spacing(5)
        .padding(Padding::ZERO.left(26))
        .push(Text::new(format!("{}:", sound_translation(language))));

    for option in Sound::ALL {
        let is_active = current_sound.eq(&option);
        let message_value = match notification {
            Notification::Data(n) => Notification::Data(DataNotification { sound: option, ..n }),
            Notification::Favorite(n) => {
                Notification::Favorite(SimpleNotification { sound: option, ..n })
            }
            Notification::IpBlacklist(n) => {
                Notification::IpBlacklist(SimpleNotification { sound: option, ..n })
            }
        };
        ret_val = ret_val.push(
            Button::new(
                option
                    .get_text()
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .padding(Padding::ZERO.left(15).right(15))
            .height(25)
            .class(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::UpdateNotificationSettings(
                message_value,
                option.ne(&Sound::None),
            )),
        );
    }
    ret_val.wrap()
}

pub fn settings_header<'a>(
    color_gradient: GradientType,
    language: Language,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Row::new()
            .push(Space::new().width(Length::Fill))
            .push(
                Text::new(settings_translation(language))
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .align_x(Alignment::Center),
            )
            .push(
                Container::new(button_hide(Message::CloseSettings, language))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            ),
    )
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .height(40)
    .width(Length::Fill)
    .class(ContainerType::Gradient(color_gradient))
}

fn row_data_representation<'a>(
    data_notification: DataNotification,
    language: Language,
    data_repr: DataRepr,
) -> row::Wrapping<'a, Message, StyleType> {
    let mut ret_val = Row::new()
        .width(Length::Shrink)
        .align_y(Alignment::Center)
        .spacing(5)
        .padding(Padding::ZERO.left(26))
        .push(Text::new(format!(
            "{}:",
            data_representation_translation(language)
        )));

    for option in DataRepr::ALL {
        let is_active = data_repr.eq(&option);
        ret_val = ret_val.push(
            Button::new(
                Text::new(option.get_label(language).to_owned())
                    .size(FONT_SIZE_FOOTER)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .padding(Padding::ZERO.left(15).right(15))
            .height(25)
            .class(if is_active {
                ButtonType::BorderedRoundSelected
            } else {
                ButtonType::BorderedRound
            })
            .on_press(Message::UpdateNotificationSettings(
                Notification::Data(DataNotification {
                    data_repr: option,
                    ..data_notification
                }),
                false,
            )),
        );
    }
    ret_val.wrap()
}
