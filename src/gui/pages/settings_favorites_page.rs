use crate::gui::components::tab::get_settings_tabs;
use crate::gui::pages::settings_notifications_page::settings_header;
use crate::gui::pages::types::settings_page::SettingsPage;
use crate::gui::styles::container::ContainerType;
use crate::gui::types::message::Message;
use crate::gui::types::settings::Settings;
use crate::{Sniffer, StyleType};
use iced::widget::{Column, Container, Space};
use iced::{Alignment, Length};

pub fn settings_favorites_page(sniffer: &Sniffer) -> Container<'_, Message, StyleType> {
    let Settings {
        language,
        color_gradient,
        ..
    } = sniffer.conf.settings;

    let content = Column::new()
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .push(settings_header(color_gradient, language))
        .push(get_settings_tabs(SettingsPage::Favorites, language))
        .push(Space::new().height(10));

    Container::new(content)
        .height(400)
        .width(800)
        .class(ContainerType::Modal)
}
