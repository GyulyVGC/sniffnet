use std::fmt::Debug;

use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::report::types::sort_type::SortType;
use iced::widget::Text;
use serde::{Deserialize, Serialize};

/// Struct representing the possible kinds of sort for displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ReportSortType {
    pub data_sort: SortType,
}

impl ReportSortType {
    pub fn next_sort(self) -> Self {
        Self {
            data_sort: self.data_sort.next_sort(),
        }
    }

    pub fn icon<'a>(self) -> Text<'a, StyleType> {
        self.data_sort.icon()
    }

    pub fn button_type(self) -> ButtonType {
        self.data_sort.button_type()
    }
}

#[cfg(test)]
mod tests {
    use crate::report::types::report_sort_type::ReportSortType;
    use crate::report::types::sort_type::SortType;

    #[test]
    fn test_next_report_sort() {
        let mut sort = ReportSortType::default();
        assert_eq!(
            sort,
            ReportSortType {
                data_sort: SortType::Neutral,
            }
        );

        sort = sort.next_sort();
        assert_eq!(
            sort,
            ReportSortType {
                data_sort: SortType::Descending,
            }
        );

        sort = sort.next_sort();
        assert_eq!(
            sort,
            ReportSortType {
                data_sort: SortType::Ascending,
            }
        );

        sort = sort.next_sort();
        assert_eq!(
            sort,
            ReportSortType {
                data_sort: SortType::Neutral,
            }
        );

        sort = sort.next_sort();
        assert_eq!(
            sort,
            ReportSortType {
                data_sort: SortType::Descending,
            }
        );
    }
}
