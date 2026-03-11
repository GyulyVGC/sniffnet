use crate::networking::types::data_info::DataInfo;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct DataInfoFav {
    pub data_info: DataInfo,
    /// Determine if this item is one of the favorites
    pub is_favorite: bool,
}

impl From<DataInfo> for DataInfoFav {
    fn from(data_info: DataInfo) -> Self {
        Self {
            data_info,
            is_favorite: false,
        }
    }
}
