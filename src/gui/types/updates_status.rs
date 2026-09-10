use crate::gui::styles::types::style_type::StyleType;
use crate::utils::types::icon::Icon;
use iced::widget::Text;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpdatesStatus {
    #[default]
    Unknown,
    InProgress,
    UpToDate,
    UpdateAvailable,
}

impl UpdatesStatus {
    pub fn icon<'a>(self) -> Text<'a, StyleType> {
        match self {
            UpdatesStatus::Unknown => Icon::FunnelStar.to_text().size(23),
            UpdatesStatus::InProgress => Icon::get_hourglass(1).size(23),
            UpdatesStatus::UpToDate => Icon::File.to_text().size(23),
            UpdatesStatus::UpdateAvailable => Icon::NewerVersion.to_text().size(23),
        }
    }
}
