use iced::widget::Text;

use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::utils::types::icon::Icon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SortType {
    Ascending,
    Descending,
    #[default]
    Neutral,
}

impl SortType {
    pub fn next_sort(self) -> Self {
        match self {
            SortType::Ascending => SortType::Neutral,
            SortType::Descending => SortType::Ascending,
            SortType::Neutral => SortType::Descending,
        }
    }

    pub fn icon(self) -> Text<'static, StyleType> {
        let mut size = 14;
        match self {
            SortType::Ascending => Icon::SortAscending,
            SortType::Descending => Icon::SortDescending,
            SortType::Neutral => {
                size = 18;
                Icon::SortNeutral
            }
        }
        .to_text()
        .size(size)
    }

    pub fn button_type(self) -> ButtonType {
        match self {
            SortType::Ascending | SortType::Descending => ButtonType::SortArrowActive,
            SortType::Neutral => ButtonType::SortArrows,
        }
    }
}
