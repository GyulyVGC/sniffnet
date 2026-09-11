use crate::SNIFFNET_TITLECASE;
use crate::gui::components::button::row_open_link_tooltip;
use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::TOOLTIP_DELAY;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use crate::translations::translations_2::new_version_available_translation;
use crate::translations::translations_6::up_to_date_translation;
use crate::translations::types::language::Language;
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::types::icon::Icon;
use crate::utils::types::web_page::WebPage;
use iced::alignment::Horizontal;
use iced::widget::tooltip::Position;
use iced::widget::{Column, Container, Text, Tooltip, button, center};
use iced::{Length, alignment};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UpdatesStatus {
    #[default]
    Unknown,
    InProgress,
    UpToDate,
    UpdateAvailable(String),
}

impl UpdatesStatus {
    pub fn icon<'a>(&self, dots: usize) -> Text<'a, StyleType> {
        match self {
            UpdatesStatus::Unknown => Text::new("?"),
            UpdatesStatus::InProgress => Icon::get_hourglass(dots),
            UpdatesStatus::UpToDate => Text::new("✔"),
            UpdatesStatus::UpdateAvailable(_) => Icon::NewerVersion.to_text().size(22),
        }
    }

    pub fn desc<'a>(&self, language: Language, dots: usize) -> Container<'a, Message, StyleType> {
        let mut content = Column::new().spacing(10).align_x(Horizontal::Center);

        match self {
            UpdatesStatus::Unknown => {
                content = content.push(Text::new("?").size(50));
            }
            UpdatesStatus::InProgress => {
                content = content.push(Icon::get_hourglass(dots).size(50));
            }
            UpdatesStatus::UpToDate => {
                content = content
                    .push(Text::new(format!("{SNIFFNET_TITLECASE} {APP_VERSION} ✔")))
                    .push(up_to_date_translation(language));
            }
            UpdatesStatus::UpdateAvailable(version) => {
                let button = button(
                    Text::new(format!("{SNIFFNET_TITLECASE} {version}"))
                        .width(Length::Fill)
                        .align_x(alignment::Alignment::Center)
                        .align_y(alignment::Alignment::Center),
                )
                .height(35)
                .width(170)
                .class(ButtonType::Alert)
                .on_press(Message::OpenWebPage(WebPage::WebsiteDownload));
                let tooltip = Tooltip::new(
                    button,
                    row_open_link_tooltip("sniffnet.app/download"),
                    Position::Right,
                )
                .gap(5)
                .class(ContainerType::Tooltip)
                .delay(TOOLTIP_DELAY);
                content = content
                    .push(new_version_available_translation(language))
                    .push(tooltip);
            }
        }

        center(content)
    }
}
