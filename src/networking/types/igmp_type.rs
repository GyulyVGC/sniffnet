use std::collections::HashMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};

use etherparse::IgmpType as EtherparseIgmpType;

/// Type of an IGMP (Internet Group Management Protocol) message.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum IgmpType {
    /// Membership Query (IGMP v1 and v2).
    MembershipQuery,
    /// Membership Query with sources (IGMP v3).
    MembershipQueryV3,
    /// Membership Report (introduced in IGMP v1).
    MembershipReportV1,
    /// Membership Report (introduced in IGMP v2).
    MembershipReportV2,
    /// Membership Report (introduced in IGMP v3).
    MembershipReportV3,
    /// Leave Group (introduced in IGMP v2).
    LeaveGroup,
    /// Unknown IGMP message type.
    #[default]
    Unknown,
}

impl IgmpType {
    pub fn from_etherparse(igmp_type: &EtherparseIgmpType) -> IgmpType {
        match igmp_type {
            EtherparseIgmpType::MembershipQuery(_) => Self::MembershipQuery,
            EtherparseIgmpType::MembershipQueryWithSources(_) => Self::MembershipQueryV3,
            EtherparseIgmpType::MembershipReportV1(_) => Self::MembershipReportV1,
            EtherparseIgmpType::MembershipReportV2(_) => Self::MembershipReportV2,
            EtherparseIgmpType::MembershipReportV3(_) => Self::MembershipReportV3,
            EtherparseIgmpType::LeaveGroup(_) => Self::LeaveGroup,
            EtherparseIgmpType::Unknown(_) => Self::Unknown,
        }
    }

    pub fn pretty_print_types(map: &HashMap<IgmpType, usize>) -> String {
        let mut ret_val = String::new();

        let mut vec: Vec<(&IgmpType, &usize)> = map.iter().collect();
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        for (igmp_type, n) in vec {
            let _ = writeln!(ret_val, "   {igmp_type} ({n})");
        }
        ret_val
    }
}

impl Display for IgmpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IgmpType::MembershipQuery => "Membership Query",
                IgmpType::MembershipQueryV3 => "Membership Query (v3)",
                IgmpType::MembershipReportV1 => "Membership Report (v1)",
                IgmpType::MembershipReportV2 => "Membership Report (v2)",
                IgmpType::MembershipReportV3 => "Membership Report (v3)",
                IgmpType::LeaveGroup => "Leave Group",
                IgmpType::Unknown => "?",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::networking::types::igmp_type::IgmpType;

    #[test]
    fn test_igmp_type_default_is_unknown() {
        assert_eq!(IgmpType::default(), IgmpType::Unknown);
    }

    #[test]
    fn test_igmp_type_display() {
        assert_eq!(IgmpType::MembershipQuery.to_string(), "Membership Query");
        assert_eq!(
            IgmpType::MembershipQueryV3.to_string(),
            "Membership Query (v3)"
        );
        assert_eq!(
            IgmpType::MembershipReportV1.to_string(),
            "Membership Report (v1)"
        );
        assert_eq!(
            IgmpType::MembershipReportV2.to_string(),
            "Membership Report (v2)"
        );
        assert_eq!(
            IgmpType::MembershipReportV3.to_string(),
            "Membership Report (v3)"
        );
        assert_eq!(IgmpType::LeaveGroup.to_string(), "Leave Group");
        assert_eq!(IgmpType::Unknown.to_string(), "?");
    }

    #[test]
    fn test_igmp_pretty_print_types_sorted_by_count_desc() {
        let map = HashMap::from([
            (IgmpType::MembershipReportV2, 2),
            (IgmpType::LeaveGroup, 5),
            (IgmpType::MembershipQuery, 1),
        ]);
        let pretty = IgmpType::pretty_print_types(&map);
        assert_eq!(
            pretty,
            "   Leave Group (5)\n   Membership Report (v2) (2)\n   Membership Query (1)\n"
        );
    }
}
