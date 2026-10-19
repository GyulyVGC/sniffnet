use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/// The IGMP message type.
pub enum IgmpType {
    MembershipQuery,
    MembershipReportV1,
    MembershipReportV2,
    MembershipReportV3,
    LeaveGroup,
    Dvmrp,
    Pim,
    CiscoTrace,
    MulticastTracerouteResponse,
    MulticastTraceroute,
    MulticastRouterAdvertisement,
    MulticastRouterSolicitation,
    MulticastRouterTermination,
    Unknown,
}

impl IgmpType {
    #[must_use]
    pub(crate) fn from_etherparse(igmp_type: &etherparse::IgmpType) -> IgmpType {
        match igmp_type {
            etherparse::IgmpType::MembershipQuery(_)
            | etherparse::IgmpType::MembershipQueryWithSources(_) => IgmpType::MembershipQuery,
            etherparse::IgmpType::MembershipReportV1(_) => IgmpType::MembershipReportV1,
            etherparse::IgmpType::MembershipReportV2(_) => IgmpType::MembershipReportV2,
            etherparse::IgmpType::MembershipReportV3(_) => IgmpType::MembershipReportV3,
            etherparse::IgmpType::LeaveGroup(_) => IgmpType::LeaveGroup,
            etherparse::IgmpType::Unknown(h) => match h.igmp_type {
                0x13 => IgmpType::Dvmrp,
                0x14 => IgmpType::Pim,
                0x15 => IgmpType::CiscoTrace,
                0x1e => IgmpType::MulticastTracerouteResponse,
                0x1f => IgmpType::MulticastTraceroute,
                0x30 => IgmpType::MulticastRouterAdvertisement,
                0x31 => IgmpType::MulticastRouterSolicitation,
                0x32 => IgmpType::MulticastRouterTermination,
                _ => IgmpType::Unknown,
            },
        }
    }
}

impl Display for IgmpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IgmpType::MembershipQuery => "Membership Query",
                IgmpType::MembershipReportV1 => "Membership Report v1",
                IgmpType::MembershipReportV2 => "Membership Report v2",
                IgmpType::MembershipReportV3 => "Membership Report v3",
                IgmpType::LeaveGroup => "Leave Group",
                IgmpType::Dvmrp => "DVMRP",
                IgmpType::Pim => "PIM v1",
                IgmpType::CiscoTrace => "Cisco Trace",
                IgmpType::MulticastTracerouteResponse => "Multicast Traceroute Response",
                IgmpType::MulticastTraceroute => "Multicast Traceroute",
                IgmpType::MulticastRouterAdvertisement => "Multicast Router Advertisement",
                IgmpType::MulticastRouterSolicitation => "Multicast Router Solicitation",
                IgmpType::MulticastRouterTermination => "Multicast Router Termination",
                IgmpType::Unknown => "?",
            }
        )
    }
}
