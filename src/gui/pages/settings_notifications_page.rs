use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::Direction;
use iced::widget::tooltip::Position;
use iced::widget::Slider;
use iced::widget::{
    button, horizontal_space, vertical_space, Checkbox, Column, Container, Row, Scrollable, Text,
    TextInput, Tooltip,
};
use iced::Length::Fixed;
use iced::{Alignment, Font, Length, Renderer};

use crate::gui::components::radio::{
    sound_bytes_threshold_radios, sound_favorite_radios, sound_packets_threshold_radios,
};
use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::scrollbar::ScrollbarType;
use crate::gui::styles::style_constants::{
    get_font, get_font_headers, FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE, FONT_SIZE_TITLE,
};
use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::types::message::Message;
use crate::notifications::types::notifications::{
    BytesNotification, FavoriteNotification, Notification, PacketsNotification,
};
use crate::translations::translations::{
    bytes_threshold_translation, favorite_notification_translation, hide_translation,
    notifications_title_translation, packets_threshold_translation, per_second_translation,
    settings_translation, specify_multiples_translation, threshold_translation, volume_translation,
};
use crate::utils::types::icon::Icon;
use crate::{Language, Sniffer, StyleType};

pub fn settings_notifications_page(sniffer: &Sniffer) -> Container<Message, Renderer<StyleType>> {
    let style = sniffer.settings.style;
    let language = sniffer.settings.language;
    let color_gradient = sniffer.settings.color_gradient;
    let notifications = sniffer.settings.notifications;
    let font = get_font(style);
    let font_headers = get_font_headers(style);

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

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if packets_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    } else {
        let input_row = Row::new()
            .push(horizontal_space(Fixed(50.0)))
            .push(Text::new(format!("{}: ", threshold_translation(language))).font(font))
            .push(input_group_packets(packets_notification, font, language));
        let sound_row =
            Row::new()
                .push(horizontal_space(Fixed(50.0)))
                .push(sound_packets_threshold_radios(
                    packets_notification,
                    font,
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

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if bytes_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(Fixed(700.0))
                .style(ContainerType::BorderedRound),
        )
    } else {
        let input_row = Row::new()
            .push(horizontal_space(Fixed(50.0)))
            .push(Text::new(format!("{}: ", threshold_translation(language))).font(font))
            .push(input_group_bytes(bytes_notification, font, language));
        let sound_row =
            Row::new()
                .push(horizontal_space(Fixed(50.0)))
                .push(sound_bytes_threshold_radios(
                    bytes_notification,
                    font,
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

    let mut ret_val = Column::new().spacing(5).push(checkbox);

    if favorite_notification.notify_on_favorite {
        let sound_row = Row::new()
            .push(horizontal_space(Fixed(50.0)))
            .push(sound_favorite_radios(favorite_notification, font, language));
        ret_val = ret_val.push(vertical_space(Fixed(5.0))).push(sound_row);
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

pub fn settings_header(
    font: Font,
    font_headers: Font,
    color_gradient: GradientType,
    language: Language,
) -> Container<'static, Message, Renderer<StyleType>> {
    let tooltip = hide_translation(language).to_string();
    //tooltip.push_str(" [esc]");
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
                Container::new(
                    Tooltip::new(
                        button(
                            Text::new("×")
                                .font(font)
                                .vertical_alignment(Vertical::Center)
                                .horizontal_alignment(Horizontal::Center)
                                .size(15),
                        )
                        .padding(2)
                        .height(Fixed(20.0))
                        .width(Fixed(20.0))
                        .on_press(Message::CloseSettings),
                        tooltip,
                        Position::Right,
                    )
                    .font(font)
                    .style(ContainerType::Tooltip),
                )
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
