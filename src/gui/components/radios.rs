use crate::enums::element_type::ElementType;
use crate::enums::message::Message;
use crate::structs::style_tuple::StyleTuple;
use crate::utility::countries::get_flag;
use crate::utility::style_constants::FONT_SIZE_SUBTITLE;
use crate::utility::translations::{
    ip_version_translation, relevant_connections_translation, traffic_rate_translation,
    transport_protocol_translation,
};
use crate::{ChartType, IpVersion, Language, ReportType, StyleType, TransProtocol};
use iced::widget::{Column, Radio, Row};
use iced::{Alignment, Font, Length};
use iced_native::widget::horizontal_space;

pub fn ip_version_radios(
    active: IpVersion,
    font: Font,
    style: StyleType,
    language: Language,
) -> Column<'static, Message> {
    let mut ret_val = Column::new().spacing(10).push(
        ip_version_translation(language)
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    );
    for option in IpVersion::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option,
                option.get_radio_label(),
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
        transport_protocol_translation(language)
            .font(font)
            .size(FONT_SIZE_SUBTITLE),
    );
    for option in TransProtocol::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option,
                option.get_radio_label(),
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
        );
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
                        option.get_radio_label(),
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
                .push(get_flag(&format!("{option:?}"))),
        );
    }
    ret_val
}

pub fn chart_radios(
    active: ChartType,
    font: Font,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
    let mut ret_val = Row::new()
        .padding(15)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(
            traffic_rate_translation(language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(horizontal_space(Length::Units(10)));
    for option in ChartType::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option,
                option.get_radio_label(),
                Some(active),
                Message::ChartSelection,
            )
            .width(Length::Units(220))
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

pub fn report_radios(
    active: ReportType,
    font: Font,
    style: StyleType,
    language: Language,
) -> Row<'static, Message> {
    let mut ret_val = Row::new()
        .padding(15)
        .spacing(10)
        .align_items(Alignment::Center)
        .push(
            relevant_connections_translation(language)
                .font(font)
                .size(FONT_SIZE_SUBTITLE),
        )
        .push(horizontal_space(Length::Units(10)));
    for option in ReportType::ALL {
        ret_val = ret_val.push(
            Radio::new(
                option,
                option.get_radio_label(),
                Some(active),
                Message::ReportSelection,
            )
            .width(Length::Units(160))
            .font(font)
            .size(15)
            .style(<StyleTuple as Into<iced::theme::Radio>>::into(StyleTuple(
                style,
                ElementType::Standard,
            ))),
        );
    }
    ret_val = ret_val.push(horizontal_space(Length::Units(120)));
    ret_val
}
