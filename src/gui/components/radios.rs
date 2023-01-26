use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::countries::get_flag;
use crate::utility::style_constants::FONT_SIZE_SUBTITLE;
use crate::{IpVersion, Language, StyleType, TransProtocol};
use iced::widget::{Column, Radio, Row, Text};
use iced::{Alignment, Font, Length};

pub fn ip_version_radios(
    active: IpVersion,
    font: Font,
    style: StyleType,
) -> Column<'static, Message> {
    let mut ret_val = Column::new()
        .spacing(10)
        .push(Text::new("IP version").font(font).size(FONT_SIZE_SUBTITLE));
    for option in IpVersion::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option,
                option.to_string(),
                Some(active),
                Message::IpVersionSelection,
            )
            .width(Length::Units(80))
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        )
    }
    ret_val
}

pub fn transport_protocol_radios(
    active: TransProtocol,
    font: Font,
    style: StyleType,
) -> Column<'static, Message> {
    let mut ret_val = Column::new().spacing(10).push(
        Text::new("Transport protocol")
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    );
    for option in TransProtocol::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option,
                option.to_string(),
                Some(active),
                Message::TransportProtocolSelection,
            )
            .width(Length::Units(80))
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        )
    }
    ret_val
}

pub fn language_radios(active: Language, font: Font, style: StyleType) -> Column<'static, Message> {
    let mut ret_val = Column::new().spacing(10);
    for option in Language::ALL {
        ret_val = ret_val.push(
            Row::new()
                .align_items(Alignment::Center)
                .push(
                    Radio::new(
                        option,
                        option.to_string(),
                        Some(active),
                        Message::LanguageSelection,
                    )
                    .width(Length::Units(120))
                    .font(font)
                    .size(15)
                    .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                        style,
                        ElementType::Standard,
                    ))),
                )
                .push(get_flag(&format!("{:?}", option))),
        )
    }
    ret_val
}
