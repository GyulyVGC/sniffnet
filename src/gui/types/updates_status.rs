use crate::gui::styles::types::style_type::StyleType;
use crate::utils::types::icon::Icon;
use iced::widget::Text;

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
}
