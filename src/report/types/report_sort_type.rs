use crate::gui::styles::button::ButtonType;
use crate::gui::styles::types::style_type::StyleType;
use crate::report::types::report_col::ReportCol;
use crate::utils::types::icon::Icon;
use iced::advanced::widget::Text;
use iced::Renderer;
use std::fmt::Debug;

/// Struct representing the possible kinds of sort for displayed relevant connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReportSortType {
    pub byte_sort: ByteSort,
    pub packet_sort: PacketSort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteSort {
    Ascending,
    Descending,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketSort {
    Ascending,
    Descending,
    Neutral,
}

impl ReportSortType {
    pub fn next_sort(self, report_col: &ReportCol) -> Self {
        match report_col {
            ReportCol::Bytes => Self {
                byte_sort: match self.byte_sort {
                    ByteSort::Ascending => ByteSort::Neutral,
                    ByteSort::Descending => ByteSort::Ascending,
                    ByteSort::Neutral => ByteSort::Descending,
                },
                packet_sort: PacketSort::Neutral,
            },
            ReportCol::Packets => Self {
                byte_sort: ByteSort::Neutral,
                packet_sort: match self.packet_sort {
                    PacketSort::Ascending => PacketSort::Neutral,
                    PacketSort::Descending => PacketSort::Ascending,
                    PacketSort::Neutral => PacketSort::Descending,
                },
            },
            _ => panic!(),
        }
    }

    pub fn icon(self, report_col: &ReportCol) -> Text<'static, Renderer<StyleType>> {
        let mut size = 15;
        match report_col {
            ReportCol::Bytes => match self.byte_sort {
                ByteSort::Ascending => Icon::SortAscending,
                ByteSort::Descending => Icon::SortDescending,
                ByteSort::Neutral => {
                    size = 20;
                    Icon::SortNeutral
                }
            },
            ReportCol::Packets => match self.packet_sort {
                PacketSort::Ascending => Icon::SortAscending,
                PacketSort::Descending => Icon::SortDescending,
                PacketSort::Neutral => {
                    size = 20;
                    Icon::SortNeutral
                }
            },
            _ => panic!(),
        }
        .to_text()
        .size(size)
    }

    pub fn button_type(self, report_col: &ReportCol) -> ButtonType {
        match report_col {
            ReportCol::Bytes => match self.byte_sort {
                ByteSort::Ascending | ByteSort::Descending => ButtonType::SortArrowActive,
                ByteSort::Neutral => ButtonType::SortArrows,
            },
            ReportCol::Packets => match self.packet_sort {
                PacketSort::Ascending | PacketSort::Descending => ButtonType::SortArrowActive,
                PacketSort::Neutral => ButtonType::SortArrows,
            },
            _ => panic!(),
        }
    }
}

impl Default for ReportSortType {
    fn default() -> Self {
        Self {
            byte_sort: ByteSort::Neutral,
            packet_sort: PacketSort::Neutral,
        }
    }
}
