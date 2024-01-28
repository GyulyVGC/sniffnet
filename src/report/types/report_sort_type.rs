use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::report::types::report_col::ReportCol;
use crate::report::types::sort_type::SortType;
use crate::utils::types::icon::Icon;
use iced::widget::Text;
use iced::Renderer;
use std::fmt::Debug;

/// Struct representing the possible kinds of sort for displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReportSortType {
    pub byte_sort: SortType,
    pub packet_sort: SortType,
}

impl ReportSortType {
    pub fn next_sort(self, report_col: &ReportCol) -> Self {
        match report_col {
            ReportCol::Bytes => Self {
                byte_sort: match self.byte_sort {
                    SortType::Ascending => SortType::Neutral,
                    SortType::Descending => SortType::Ascending,
                    SortType::Neutral => SortType::Descending,
                },
                packet_sort: SortType::Neutral,
            },
            ReportCol::Packets => Self {
                byte_sort: SortType::Neutral,
                packet_sort: match self.packet_sort {
                    SortType::Ascending => SortType::Neutral,
                    SortType::Descending => SortType::Ascending,
                    SortType::Neutral => SortType::Descending,
                },
            },
            _ => Self::default(),
        }
    }

    pub fn icon(self, report_col: &ReportCol) -> Text<'static, Renderer<StyleType>> {
        let mut size = 14;
        match report_col {
            ReportCol::Bytes => match self.byte_sort {
                SortType::Ascending => Icon::SortAscending,
                SortType::Descending => Icon::SortDescending,
                SortType::Neutral => {
                    size = 18;
                    Icon::SortNeutral
                }
            },
            ReportCol::Packets => match self.packet_sort {
                SortType::Ascending => Icon::SortAscending,
                SortType::Descending => Icon::SortDescending,
                SortType::Neutral => {
                    size = 18;
                    Icon::SortNeutral
                }
            },
            _ => Icon::SortNeutral,
        }
        .to_text()
        .size(size)
    }

    pub fn button_type(self, report_col: &ReportCol) -> ButtonType {
        match report_col {
            ReportCol::Bytes => match self.byte_sort {
                SortType::Ascending | SortType::Descending => ButtonType::SortArrowActive,
                SortType::Neutral => ButtonType::SortArrows,
            },
            ReportCol::Packets => match self.packet_sort {
                SortType::Ascending | SortType::Descending => ButtonType::SortArrowActive,
                SortType::Neutral => ButtonType::SortArrows,
            },
            _ => ButtonType::SortArrows,
        }
    }
}

impl Default for ReportSortType {
    fn default() -> Self {
        Self {
            byte_sort: SortType::Neutral,
            packet_sort: SortType::Neutral,
        }
    }
}
