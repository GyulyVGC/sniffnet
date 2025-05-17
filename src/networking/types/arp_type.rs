use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use etherparse::ArpOperation;
use std::fmt::Write;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub enum ArpType {
    Request,
    Reply,
    #[default]
    Unknown,
}

impl ArpType {
    pub fn from_etherparse(arp_operation: ArpOperation) -> ArpType {
        match arp_operation {
            ArpOperation(1) => Self::Request,
            ArpOperation(2) => Self::Reply,
            ArpOperation(_) => Self::Unknown,
        }
    }

    pub fn pretty_print_types(map: &HashMap<ArpType, usize>) -> String {
        let mut ret_val = String::new();

        let mut vec: Vec<(&ArpType, &usize)> = map.iter().collect();
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        for (arp_type, n) in vec {
            let _ = writeln!(ret_val, "   {arp_type} ({n})");
        }
        ret_val
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
