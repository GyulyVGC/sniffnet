use std::fmt::{Display, Formatter};

use etherparse::ArpOperation;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/// The ARP message type.
pub enum ArpType {
    Request,
    Reply,
    Unknown,
}

impl ArpType {
    #[must_use]
    pub(crate) fn from_etherparse(arp_operation: ArpOperation) -> ArpType {
        match arp_operation {
            ArpOperation(1) => Self::Request,
            ArpOperation(2) => Self::Reply,
            ArpOperation(_) => Self::Unknown,
        }
    }
}

impl Display for ArpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArpType::Request => "Request",
                ArpType::Reply => "Reply",
                ArpType::Unknown => "?",
            }
        )
    }
}
