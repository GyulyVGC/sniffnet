use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::{
    horizontal_space, vertical_space, Checkbox, Column, Container, Row, Scrollable, Text, TextInput,
};
use iced::widget::{Button, Slider};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

use crate::gui::components::button::button_hide;
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::notifications::types::notifications::{
    BytesNotification, FavoriteNotification, Notification, PacketsNotification,
};
use crate::notifications::types::sound::Sound;
use crate::translations::translations::{
    bytes_threshold_translation, favorite_notification_translation,
    notifications_title_translation, packets_threshold_translation, per_second_translation,
    settings_translation, sound_translation, specify_multiples_translation, threshold_translation,
    volume_translation,
};
use crate::utils::types::icon::Icon;
use crate::{ConfigSettings, Language, Sniffer, StyleType};

pub fn settings_notifications_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let ConfigSettings {
        style,
        language,
        color_gradient,
        notifications,
        ..
    } = sniffer.configs.lock().unwrap().settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    let mut content = Column::new()
        .width(Length::Fill)
        .push(settings_header(
            font,
            font_headers,
            color_gradient,
            language,
        ))
        .push(get_settings_tabs(
            SettingsPage::Notifications,
            font,
            language,
        ))
        .push(vertical_space(Fixed(15.0)))
        .push(
            notifications_title_translation(language)
                .font(font)
                .style(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .push(vertical_space(Fixed(5.0)));

    let volume_notification_col = Column::new()
        .padding([0, 0, 5, 0])
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .push(volume_slider(language, font, notifications.volume))
        .push(
            Scrollable::new(
                Column::new()
                    .width(Fixed(720.0))
                    .push(get_packets_notify(
                        notifications.packets_notification,
                        language,
                        font,
                    ))
                    .push(get_bytes_notify(
                        notifications.bytes_notification,
                        language,
                        font,
                    ))
                    .push(get_favorite_notify(
                        notifications.favorite_notification,
                        language,
                        font,
                    )),
            )
            .direction(Direction::Vertical(ScrollbarType::properties())),
        );

    content = content.push(volume_notification_col);

    Container::new(content)
        .height(Fixed(400.0))
        .width(Fixed(800.0))
        .style(ContainerType::Modal)
}

fn get_packets_notify(
    packets_notification: PacketsNotification,
    language: Language,
    font: Font,
) -> Column<'static, Message, Renderer<StyleType>> {
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
    .font(font);

    let mut ret_val = Column::new().spacing(10).push(checkbox);

    if packets_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    } else {
        let input_row = input_group_packets(packets_notification, font, language);
        let sound_row = sound_buttons(Notification::Packets(packets_notification), font, language);
        ret_val = ret_val.push(input_row).push(sound_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    }
}

fn get_bytes_notify(
    bytes_notification: BytesNotification,
    language: Language,
    font: Font,
) -> Column<'static, Message, Renderer<StyleType>> {
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
    .font(font);

    let mut ret_val = Column::new().spacing(10).push(checkbox);

    if bytes_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    } else {
        let input_row = input_group_bytes(bytes_notification, font, language);
        let sound_row = sound_buttons(Notification::Bytes(bytes_notification), font, language);
        ret_val = ret_val.push(input_row).push(sound_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    }
}

fn get_favorite_notify(
    favorite_notification: FavoriteNotification,
    language: Language,
    font: Font,
) -> Column<'static, Message, Renderer<StyleType>> {
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
    .font(font);

    let mut ret_val = Column::new().spacing(10).push(checkbox);

    if favorite_notification.notify_on_favorite {
        let sound_row = sound_buttons(
            Notification::Favorite(favorite_notification),
            font,
            language,
        );
        ret_val = ret_val.push(sound_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    } else {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    }
}

fn input_group_packets(
    packets_notification: PacketsNotification,
    font: Font,
    language: Language,
) -> Container<'static, Message, Renderer<StyleType>> {
    let curr_threshold_str = &packets_notification.threshold.unwrap().to_string();
    let input_row = Row::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(horizontal_space(Fixed(45.0)))
        .push(Text::new(format!("{}:", threshold_translation(language))).font(font))
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
            .padding([3, 5])
            .font(font)
            .width(Length::Fixed(100.0)),
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
    font: Font,
    language: Language,
) -> Container<'static, Message, Renderer<StyleType>> {
    let info_str = format!(
        "{}; {}",
        per_second_translation(language),
        specify_multiples_translation(language)
    );
    let mut curr_threshold_str = (bytes_notification.threshold.unwrap()
        / bytes_notification.byte_multiple.get_multiplier())
    .to_string();
    curr_threshold_str.push_str(bytes_notification.byte_multiple.get_char());
    let input_row = Row::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .push(horizontal_space(Fixed(45.0)))
        .push(Text::new(format!("{}:", threshold_translation(language))).font(font))
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
            .padding([3, 5])
            .font(font)
            .width(Length::Fixed(100.0)),
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

fn volume_slider(
    language: Language,
    font: Font,
    volume: u8,
) -> Container<'static, Message, Renderer<StyleType>> {
    Container::new(
        Column::new()
            .spacing(5)
            .align_items(Alignment::Center)
            .push(Text::new(format!("{}: {volume:^3}%", volume_translation(language))).font(font))
            .push(
                Row::new()
                    .push(
                        Icon::AudioMute
                            .to_text()
                            .width(Fixed(30.0))
                            .vertical_alignment(Vertical::Center)
                            .size(20),
                    )
                    .push(
                        Slider::new(0..=100, volume, Message::ChangeVolume)
                            .step(5)
                            .width(Fixed(200.0)),
                    )
                    .push(horizontal_space(Length::Fixed(15.0)))
                    .push(
                        Icon::AudioHigh
                            .to_text()
                            .vertical_alignment(Vertical::Center)
                            .size(20),
                    ),
            ),
    )
    .padding(5)
    .width(Length::Fill)
    .height(Length::Fixed(60.0))
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

fn sound_buttons(
    notification: Notification,
    font: Font,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    let current_sound = match notification {
        Notification::Packets(n) => n.sound,
        Notification::Bytes(n) => n.sound,
        Notification::Favorite(n) => n.sound,
    };

    let mut ret_val = Row::new()
        .align_items(Alignment::Center)
        .spacing(5)
        .push(horizontal_space(Fixed(45.0)))
        .push(Text::new(format!("{}:", sound_translation(language))).font(font));

    for option in Sound::ALL {
        let is_active = current_sound.eq(&option);
        let message_value = match notification {
            Notification::Packets(n) => {
                Notification::Packets(PacketsNotification { sound: option, ..n })
            }
            Notification::Bytes(n) => Notification::Bytes(BytesNotification { sound: option, ..n }),
            Notification::Favorite(n) => {
                Notification::Favorite(FavoriteNotification { sound: option, ..n })
            }
        };
        ret_val = ret_val.push(
            Button::new(option.get_text(font))
                .padding(0)
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(25.0))
                .style(if is_active {
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
    ret_val
}

pub fn settings_header(
    font: Font,
    font_headers: Font,
    color_gradient: GradientType,
    language: Language,
) -> Container<'static, Message, Renderer<StyleType>> {
    Container::new(
        Row::new()
            .push(horizontal_space(Length::FillPortion(1)))
            .push(
                Text::new(settings_translation(language))
                    .font(font_headers)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Container::new(button_hide(Message::CloseSettings, language, font))
                    .width(Length::FillPortion(1))
                    .align_x(Horizontal::Center),
            ),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .height(Fixed(40.0))
    .width(Length::Fill)
    .style(ContainerType::Gradient(color_gradient))
}
