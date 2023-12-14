use iced::widget::{Column, Radio, Row, Text};
use iced::{Alignment, Font, Renderer};

use crate::gui::types::message::Message;
use crate::notifications::types::notifications::{
    BytesNotification, FavoriteNotification, Notification, PacketsNotification,
};
use crate::notifications::types::sound::Sound;
use crate::translations::translations::sound_translation;
use crate::{ChartType, Language, StyleType};

pub fn sound_packets_threshold_radios(
    packets_notification: PacketsNotification,
    font: Font,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Row::new()
        .spacing(20)
        .push(Text::new(format!("{}:", sound_translation(language))).font(font));
    for option in Sound::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option.get_radio_label(language),
                option,
                Some(packets_notification.sound),
                |value| {
                    Message::UpdateNotificationSettings(
                        Notification::Packets(PacketsNotification {
                            sound: value,
                            ..packets_notification
                        }),
                        value.ne(&Sound::None),
                    )
                },
            )
            .spacing(7)
            .font(font)
            .size(15),
        );
    }
    ret_val
}

pub fn sound_bytes_threshold_radios(
    bytes_notification: BytesNotification,
    font: Font,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Row::new()
        .spacing(20)
        .push(Text::new(format!("{}:", sound_translation(language))).font(font));
    for option in Sound::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option.get_radio_label(language),
                option,
                Some(bytes_notification.sound),
                |value| {
                    Message::UpdateNotificationSettings(
                        Notification::Bytes(BytesNotification {
                            sound: value,
                            ..bytes_notification
                        }),
                        value.ne(&Sound::None),
                    )
                },
            )
            .spacing(7)
            .font(font)
            .size(15),
        );
    }
    ret_val
}

pub fn sound_favorite_radios(
    favorite_notification: FavoriteNotification,
    font: Font,
    language: Language,
) -> Row<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Row::new()
        .spacing(20)
        .push(Text::new(format!("{}:", sound_translation(language))).font(font));
    for option in Sound::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option.get_radio_label(language),
                option,
                Some(favorite_notification.sound),
                |value| {
                    Message::UpdateNotificationSettings(
                        Notification::Favorite(FavoriteNotification {
                            sound: value,
                            ..favorite_notification
                        }),
                        value.ne(&Sound::None),
                    )
                },
            )
            .spacing(7)
            .font(font)
            .size(15),
        );
    }
    ret_val
}

pub fn chart_radios(
    active: ChartType,
    font: Font,
    language: Language,
) -> Column<'static, Message, Renderer<StyleType>> {
    let mut ret_val = Column::new()
        .padding([0, 0, 0, 25])
        .spacing(5)
        .align_items(Alignment::Start);
    for option in ChartType::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option.get_radio_label(language),
                option,
                Some(active),
                Message::ChartSelection,
            )
            .spacing(7)
            .font(font)
            .size(15),
        );
    }
    ret_val
}
