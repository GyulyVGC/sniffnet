use std::fmt::Debug;

use iced::widget::Text;

use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::report::types::report_col::ReportCol;
use crate::report::types::sort_type::SortType;
use crate::utils::types::icon::Icon;

/// Struct representing the possible kinds of sort for displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ReportSortType {
    pub byte_sort: SortType,
    pub packet_sort: SortType,
}

impl ReportSortType {
    pub fn next_sort(self, report_col: &ReportCol) -> Self {
        match report_col {
            ReportCol::Bytes => Self {
                byte_sort: self.byte_sort.next_sort(),
                packet_sort: SortType::Neutral,
            },
            ReportCol::Packets => Self {
                byte_sort: SortType::Neutral,
                packet_sort: self.packet_sort.next_sort(),
            },
            _ => Self::default(),
        }
    }

    pub fn icon(self, report_col: &ReportCol) -> Text<'static, StyleType> {
        match report_col {
            ReportCol::Bytes => self.byte_sort.icon(),
            ReportCol::Packets => self.packet_sort.icon(),
            _ => Icon::SortNeutral.to_text(),
        }
    }

    pub fn button_type(self, report_col: &ReportCol) -> ButtonType {
        match report_col {
            ReportCol::Bytes => self.byte_sort.button_type(),
            ReportCol::Packets => self.packet_sort.button_type(),
            _ => ButtonType::SortArrows,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::report::types::report_col::ReportCol;
    use crate::report::types::report_sort_type::ReportSortType;
    use crate::report::types::sort_type::SortType;

    #[test]
    fn test_next_report_sort() {
        let mut sort = ReportSortType::default();
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Packets);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Descending
            }
        );

        sort = sort.next_sort(&ReportCol::Packets);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Ascending
            }
        );

        sort = sort.next_sort(&ReportCol::Packets);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Packets);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Descending
            }
        );

        sort = sort.next_sort(&ReportCol::Bytes);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Descending,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Packets);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Descending
            }
        );

        sort = sort.next_sort(&ReportCol::Bytes);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Descending,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Bytes);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Ascending,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Packets);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Descending
            }
        );

        sort = sort.next_sort(&ReportCol::Bytes);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Descending,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Bytes);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Ascending,
                packet_sort: SortType::Neutral
            }
        );

        sort = sort.next_sort(&ReportCol::Bytes);
        assert_eq!(
            sort,
            ReportSortType {
                byte_sort: SortType::Neutral,
                packet_sort: SortType::Neutral
            }
        );
    }
}
