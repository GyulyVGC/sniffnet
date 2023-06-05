use iced::widget::{Column, Radio, Row, Text};
use iced::{Alignment, Font, Length};
use iced_native::widget::horizontal_space;

use crate::gui::styles::style_constants::FONT_SIZE_SUBTITLE;
use crate::gui::styles::types::element_type::ElementType;
use crate::gui::styles::types::style_tuple::StyleTuple;
use crate::gui::types::message::Message;
use crate::notifications::types::notifications::{
    BytesNotification, FavoriteNotification, Notification, PacketsNotification,
};
use crate::notifications::types::sound::Sound;
use crate::translations::translations::{
    ip_version_translation, sound_translation, transport_protocol_translation,
};
use crate::{ChartType, IpVersion, Language, StyleType, TransProtocol};

pub fn ip_version_radios(
    active: IpVersion,
    font: Font,
    style: StyleType,
    language: Language,
) -> Column<'static, Message> {
    let mut ret_val = Column::new().spacing(10).padding(0).push(
        ip_version_translation(language)
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    );
    for option in IpVersion::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option.get_radio_label(language),
                option,
                Some(active),
                Message::IpVersionSelection,
            )
            .spacing(7)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val
}

pub fn transport_protocol_radios(
    active: TransProtocol,
    font: Font,
    style: StyleType,
    language: Language,
) -> Column<'static, Message> {
    let mut ret_val = Column::new().spacing(10).push(
        Text::new(transport_protocol_translation(language))
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    );
    for option in TransProtocol::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option.get_radio_label(language),
                option,
                Some(active),
                Message::TransportProtocolSelection,
            )
            .spacing(7)
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val
}

pub fn language_radios(
    active: Language,
    collection: &[Language],
    font: Font,
    style: StyleType,
) -> Row<'static, Message> {
    let mut ret_val = Row::new().spacing(10);
    for option in collection {
        ret_val = ret_val.push(
            Row::new().align_items(Alignment::Center).push(
                Row::new()
                    .width(Length::Fixed(180.0))
                    .push(
                        Radio::new(
                            format!("{} ({:?})", option.get_radio_label(), option),
                            *option,
                            Some(active),
                            Message::LanguageSelection,
                        )
                        .spacing(7)
                        .font(font)
                        .size(15)
                        .style(
                            <StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                                style,
                                ElementType::Standard,
                            )),
                        ),
                    )
                    .push(horizontal_space(Length::Fixed(8.0)))
                    .push(option.get_flag()),
            ),
        );
    }
    ret_val
}

pub fn sound_packets_threshold_radios(
    packets_notification: PacketsNotification,
    font: Font,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
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
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val
}

pub fn sound_bytes_threshold_radios(
    bytes_notification: BytesNotification,
    font: Font,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
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
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val
}

pub fn sound_favorite_radios(
    favorite_notification: FavoriteNotification,
    font: Font,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
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
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val
}

pub fn chart_radios(
    active: ChartType,
    font: Font,
    style: StyleType,
    language: Language,
) -> Column<'static, Message> {
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
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val
}

// pub fn report_radios(
//     active: ReportType,
//     font: Font,
//     style: StyleType,
//     language: Language,
// ) -> Row<'static, Message> {
//     let mut ret_val = Row::new()
//         .padding([10, 0, 15, 5])
//         .spacing(20)
//         .align_items(Alignment::Center)
//         .push(
//             relevant_connections_translation(language)
//                 .font(font)
//                 .size(FONT_SIZE_TITLE),
//         );
//     for option in ReportType::ALL {
//         ret_val = ret_val.push(
//             Radio::new(
//                 option.get_radio_label(language),
//                 option,
//                 Some(active),
//                 Message::ReportSelection,
//             )
//             .spacing(7)
//             .font(font)
//             .size(15)
//             .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
//                 style,
//                 ElementType::Standard,
//             ))),
//         );
//     }
//     ret_val = ret_val.push(horizontal_space(Length::Fixed(120.0)));
//     ret_val
// }
