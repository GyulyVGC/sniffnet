use crate::gui::pages::overview_page::col_device;
use crate::gui::sniffer::Sniffer;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_representation::DataRepr;
use crate::translations::translations::{
    error_translation, no_addresses_translation, waiting_translation,
};
use crate::translations::translations_3::unsupported_link_type_translation;
use crate::translations::translations_4::reading_from_pcap_translation;
use crate::utils::types::icon::Icon;
use iced::widget::{Column, Container, Space, Text};
use iced::{Alignment, Length};

pub fn waiting_page(sniffer: &Sniffer) -> Option<Container<'_, Message, StyleType>> {
    let Settings { language, .. } = sniffer.conf.settings;

    let dots = &sniffer.dots_pulse.0;
    let cs = &sniffer.capture_source;

    let pcap_error = sniffer.pcap_error.as_ref();
    let tot_packets = sniffer
        .info_traffic
        .tot_data_info
        .tot_data(DataRepr::Packets);

    if pcap_error.is_none() && tot_packets > 0 {
        return None;
    }

    let link_type = cs.get_link_type();
    let (icon_text, nothing_to_see_text) = if let Some(error) = pcap_error {
        (
            Icon::Error.to_text().size(60),
            format!("{}\n\n{error}", error_translation(language)),
        )
    } else if !link_type.is_supported() {
        (
            Icon::Forbidden.to_text().size(60),
            unsupported_link_type_translation(language).to_string(),
        )
    } else if matches!(cs, CaptureSource::File(_)) {
        (
            Icon::File.to_text().size(60),
            reading_from_pcap_translation(language).to_string(),
        )
    } else if cs.get_addresses().is_empty() {
        (
            Icon::Warning.to_text().size(60),
            no_addresses_translation(language).to_string(),
        )
    } else {
        (
            Icon::get_hourglass(dots.len()).size(60),
            waiting_translation(language).to_string(),
        )
    };

    let nothing_to_see_col = Column::new()
        .align_x(Alignment::Center)
        .push(icon_text)
        .push(Space::new().height(25))
        .push(Text::new(nothing_to_see_text).align_x(Alignment::Center))
        .push(Text::new(dots.to_owned()).size(50));

    Some(Container::new(
        Column::new()
            .width(Length::Fill)
            .padding(10)
            .align_x(Alignment::Center)
            .push(Space::new().height(Length::Fill))
            .push(
                Container::new(
                    col_device(language, cs, &sniffer.conf.filters).height(Length::Shrink),
                )
                .padding([15, 30])
                .class(ContainerType::BorderedRound),
            )
            .push(Space::new().height(20))
            .push(
                Container::new(nothing_to_see_col)
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .padding(15)
                    .class(ContainerType::BorderedRound),
            )
            .push(Space::new().height(Length::FillPortion(2))),
    ))
}
