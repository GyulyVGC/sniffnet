use iced::widget::scrollable::Direction;
use iced::widget::{Button, Slider, horizontal_space};
use iced::widget::{Checkbox, Column, Container, Row, Scrollable, Space, Text, TextInput};
use iced::{Alignment, Font, Length, Padding};

use crate::chart::types::chart_type::ChartType;
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
    DataNotification, FavoriteNotification, Notification,
};
use crate::notifications::types::sound::Sound;
use crate::translations::translations::{
    favorite_transmitted_translation, notifications_title_translation, per_second_translation,
    settings_translation, sound_translation, threshold_translation, volume_translation,
};
use crate::translations::translations_2::data_representation_translation;
use crate::translations::translations_4::data_exceeded_translation;
use crate::utils::types::icon::Icon;
use crate::{ConfigSettings, Language, Sniffer, StyleType};

pub fn settings_notifications_page<'a>(sniffer: &Sniffer) -> Container<'a, Message, StyleType> {
    let ConfigSettings {
        style,
        language,
        color_gradient,
        mut notifications,
        ..
    } = sniffer.configs.settings;
    let font = style.get_extension().font;
    let font_headers = style.get_extension().font_headers;

    // Use threshold that has not yet been applied, if available
    if let Some(temp_data_notification) = sniffer.timing_events.temp_threshold() {
        notifications.data_notification.threshold = temp_data_notification.threshold;
        notifications.data_notification.byte_multiple = temp_data_notification.byte_multiple;
        notifications.data_notification.previous_threshold =
            temp_data_notification.previous_threshold;
    }

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
        .push(Space::with_height(15))
        .push(
            notifications_title_translation(language)
                .font(font)
                .class(TextType::Subtitle)
                .size(FONT_SIZE_SUBTITLE)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .push(Space::with_height(5));

    let volume_notification_col = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(volume_slider(language, font, notifications.volume))
        .push(Scrollable::with_direction(
            Column::new()
                .align_x(Alignment::Center)
                .width(Length::Fill)
                .push(get_data_notify(
                    notifications.data_notification,
                    language,
                    font,
                ))
                .push(get_favorite_notify(
                    notifications.favorite_notification,
                    language,
                    font,
                )),
            Direction::Vertical(ScrollbarType::properties().margin(10)),
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
    font: Font,
) -> Column<'a, Message, StyleType> {
    let checkbox = Checkbox::new(
        data_exceeded_translation(language),
        data_notification.threshold.is_some(),
    )
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
    .size(18)
    .font(font);

    let mut ret_val = Column::new().spacing(15).push(checkbox);

    if data_notification.threshold.is_none() {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(700)
                .class(ContainerType::BorderedRound),
        )
    } else {
        let data_representation_row = row_data_representation(
            data_notification,
            language,
            font,
            data_notification.chart_type,
        );
        let input_row = input_group_bytes(data_notification, font, language);
        let sound_row = sound_buttons(Notification::Data(data_notification), font, language);
        ret_val = ret_val
            .push(sound_row)
            .push(data_representation_row)
            .push(input_row);
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(700)
                .class(ContainerType::BorderedRound),
        )
    }
}

fn get_favorite_notify<'a>(
    favorite_notification: FavoriteNotification,
    language: Language,
    font: Font,
) -> Column<'a, Message, StyleType> {
    let checkbox = Checkbox::new(
        favorite_transmitted_translation(language),
        favorite_notification.notify_on_favorite,
    )
    .on_toggle(move |toggled| {
        Message::UpdateNotificationSettings(
            if toggled {
                Notification::Favorite(FavoriteNotification::on(favorite_notification.sound))
            } else {
                Notification::Favorite(FavoriteNotification::off(favorite_notification.sound))
            },
            false,
        )
    })
    .size(18)
    .font(font);

    let mut ret_val = Column::new().spacing(15).push(checkbox);

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
                .width(700)
                .class(ContainerType::BorderedRound),
        )
    } else {
        Column::new().padding(5).push(
            Container::new(ret_val)
                .padding(10)
                .width(700)
                .class(ContainerType::BorderedRound),
        )
    }
}

fn input_group_bytes<'a>(
    bytes_notification: DataNotification,
    font: Font,
    language: Language,
) -> Container<'a, Message, StyleType> {
    let mut curr_threshold_str = (bytes_notification.threshold.unwrap_or_default()
        / bytes_notification.byte_multiple.multiplier())
    .to_string();
    curr_threshold_str.push_str(&bytes_notification.byte_multiple.get_char());
    let input_row = Row::new()
        .spacing(5)
        .align_y(Alignment::Center)
        .push(Space::with_width(45))
        .push(Text::new(format!("{}:", threshold_translation(language))).font(font))
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
            .font(font)
            .width(100),
        )
        .push(
            Text::new(per_second_translation(language))
                .font(font)
                .align_y(Alignment::Center)
                .size(FONT_SIZE_FOOTER),
        );
    Container::new(input_row)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
}

fn volume_slider<'a>(
    language: Language,
    font: Font,
    volume: u8,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Column::new()
            .spacing(5)
            .align_x(Alignment::Center)
            .push(Text::new(format!("{}: {volume:^3}%", volume_translation(language))).font(font))
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
                    .push(Space::with_width(15))
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
    font: Font,
    language: Language,
) -> Row<'a, Message, StyleType> {
    let current_sound = match notification {
        Notification::Data(n) => n.sound,
        Notification::Favorite(n) => n.sound,
    };

    let mut ret_val = Row::new()
        .width(Length::Shrink)
        .align_y(Alignment::Center)
        .spacing(5)
        .push(Space::with_width(45))
        .push(Text::new(format!("{}:", sound_translation(language))).font(font));

    for option in Sound::ALL {
        let is_active = current_sound.eq(&option);
        let message_value = match notification {
            Notification::Data(n) => Notification::Data(DataNotification { sound: option, ..n }),
            Notification::Favorite(n) => {
                Notification::Favorite(FavoriteNotification { sound: option, ..n })
            }
        };
        ret_val = ret_val.push(
            Button::new(
                option
                    .get_text(font)
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
    ret_val
}

pub fn settings_header<'a>(
    font: Font,
    font_headers: Font,
    color_gradient: GradientType,
    language: Language,
) -> Container<'a, Message, StyleType> {
    Container::new(
        Row::new()
            .push(horizontal_space())
            .push(
                Text::new(settings_translation(language))
                    .font(font_headers)
                    .size(FONT_SIZE_TITLE)
                    .width(Length::FillPortion(6))
                    .align_x(Alignment::Center),
            )
            .push(
                Container::new(button_hide(Message::CloseSettings, language, font))
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
    font: Font,
    chart_type: ChartType,
) -> Row<'a, Message, StyleType> {
    let mut ret_val = Row::new()
        .width(Length::Shrink)
        .align_y(Alignment::Center)
        .spacing(5)
        .push(Space::with_width(45))
        .push(Text::new(format!("{}:", data_representation_translation(language))).font(font));

    for option in ChartType::ALL {
        let is_active = chart_type.eq(&option);
        ret_val = ret_val.push(
            Button::new(
                Text::new(option.get_label(language).to_owned())
                    .size(FONT_SIZE_FOOTER)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .font(font),
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
                    chart_type: option,
                    ..data_notification
                }),
                false,
            )),
        );
    }
    ret_val
}
