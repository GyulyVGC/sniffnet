use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use etherparse::{Icmpv4Type, Icmpv6Type};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum IcmpType {
    V4(IcmpTypeV4),
    V6(IcmpTypeV6),
}

impl IcmpType {
    pub fn pretty_print_types(map: &HashMap<IcmpType, usize>) -> String {
        let mut ret_val = String::new();

        let mut vec: Vec<(&IcmpType, &usize)> = map.iter().collect();
        vec.sort_by(|(_, a), (_, b)| b.cmp(a));

        for (icmp_type, n) in vec {
            ret_val.push_str(&format!("   {icmp_type} ({n})\n"));
        }
        ret_val
    }
}

impl Default for IcmpType {
    fn default() -> Self {
        Self::V4(IcmpTypeV4::default())
    }
}

impl Display for IcmpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IcmpType::V4(icmpv4_type) => {
                    icmpv4_type.to_string()
                }
                IcmpType::V6(icmpv6_type) => {
                    icmpv6_type.to_string()
                }
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[allow(clippy::module_name_repetitions)]
pub enum IcmpTypeV4 {
    EchoReply,
    DestinationUnreachable,
    SourceQuench,
    Redirect,
    AlternateHostAddress,
    Echo,
    RouterAdvertisement,
    RouterSolicitation,
    TimeExceeded,
    ParameterProblem,
    Timestamp,
    TimestampReply,
    InformationRequest,
    InformationReply,
    AddressMaskRequest,
    AddressMaskReply,
    Traceroute,
    DatagramConversionError,
    MobileHostRedirect,
    IPv6WhereAreYou,
    IPv6IAmHere,
    MobileRegistrationRequest,
    MobileRegistrationReply,
    DomainNameRequest,
    DomainNameReply,
    Skip,
    Photuris,
    ExtendedEchoRequest,
    ExtendedEchoReply,
    #[default]
    Unknown,
}

impl IcmpTypeV4 {
    pub fn from_etherparse(icmpv4type: &Icmpv4Type) -> IcmpType {
        IcmpType::V4(match icmpv4type {
            Icmpv4Type::EchoReply(_) => Self::EchoReply,
            Icmpv4Type::DestinationUnreachable(_) => Self::DestinationUnreachable,
            Icmpv4Type::Redirect(_) => Self::Redirect,
            Icmpv4Type::EchoRequest(_) => Self::Echo,
            Icmpv4Type::TimeExceeded(_) => Self::TimeExceeded,
            Icmpv4Type::ParameterProblem(_) => Self::ParameterProblem,
            Icmpv4Type::TimestampRequest(_) => Self::Timestamp,
            Icmpv4Type::TimestampReply(_) => Self::TimestampReply,
            Icmpv4Type::Unknown { type_u8: id, .. } => match id {
                4 => Self::SourceQuench,
                6 => Self::AlternateHostAddress,
                9 => Self::RouterAdvertisement,
                10 => Self::RouterSolicitation,
                15 => Self::InformationRequest,
                16 => Self::InformationReply,
                17 => Self::AddressMaskRequest,
                18 => Self::AddressMaskReply,
                30 => Self::Traceroute,
                31 => Self::DatagramConversionError,
                32 => Self::MobileHostRedirect,
                33 => Self::IPv6WhereAreYou,
                34 => Self::IPv6IAmHere,
                35 => Self::MobileRegistrationRequest,
                36 => Self::MobileRegistrationReply,
                37 => Self::DomainNameRequest,
                38 => Self::DomainNameReply,
                39 => Self::Skip,
                40 => Self::Photuris,
                42 => Self::ExtendedEchoRequest,
                43 => Self::ExtendedEchoReply,
                _ => Self::Unknown,
            },
        })
    }
}

impl Display for IcmpTypeV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IcmpTypeV4::EchoReply => "Echo Reply",
                IcmpTypeV4::DestinationUnreachable => "Destination Unreachable",
                IcmpTypeV4::SourceQuench => "Source Quench",
                IcmpTypeV4::Redirect => "Redirect",
                IcmpTypeV4::AlternateHostAddress => "Alternate Host Address",
                IcmpTypeV4::Echo => "Echo",
                IcmpTypeV4::RouterAdvertisement => "Router Advertisement",
                IcmpTypeV4::RouterSolicitation => "Router Solicitation",
                IcmpTypeV4::TimeExceeded => "Time Exceeded",
                IcmpTypeV4::ParameterProblem => "Parameter Problem",
                IcmpTypeV4::Timestamp => "Timestamp",
                IcmpTypeV4::TimestampReply => "Timestamp Reply",
                IcmpTypeV4::InformationRequest => "Information Request",
                IcmpTypeV4::InformationReply => "Information Reply",
                IcmpTypeV4::AddressMaskRequest => "Address Mask Request",
                IcmpTypeV4::AddressMaskReply => "Address Mask Reply",
                IcmpTypeV4::Traceroute => "Traceroute",
                IcmpTypeV4::DatagramConversionError => "Datagram Conversion Error",
                IcmpTypeV4::MobileHostRedirect => "Mobile Host Redirect",
                IcmpTypeV4::IPv6WhereAreYou => "IPv6 Where-Are-You",
                IcmpTypeV4::IPv6IAmHere => "IPv6 I-Am-Here",
                IcmpTypeV4::MobileRegistrationRequest => "Mobile Registration Request",
                IcmpTypeV4::MobileRegistrationReply => "Mobile Registration Reply",
                IcmpTypeV4::DomainNameRequest => "Domain Name Request",
                IcmpTypeV4::DomainNameReply => "Domain Name Reply",
                IcmpTypeV4::Skip => "SKIP",
                IcmpTypeV4::Photuris => "Photuris",
                IcmpTypeV4::ExtendedEchoRequest => "Extended Echo Request",
                IcmpTypeV4::ExtendedEchoReply => "Extended Echo Reply",
                IcmpTypeV4::Unknown => "?",
            }
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[allow(clippy::module_name_repetitions)]
pub enum IcmpTypeV6 {
    DestinationUnreachable,
    PacketTooBig,
    TimeExceeded,
    ParameterProblem,
    EchoRequest,
    EchoReply,
    MulticastListenerQuery,
    MulticastListenerReport,
    MulticastListenerDone,
    RouterSolicitation,
    RouterAdvertisement,
    NeighborSolicitation,
    NeighborAdvertisement,
    RedirectMessage,
    RouterRenumbering,
    ICMPNodeInformationQuery,
    ICMPNodeInformationResponse,
    InverseNeighborDiscoverySolicitationMessage,
    InverseNeighborDiscoveryAdvertisementMessage,
    Version2MulticastListenerReport,
    HomeAgentAddressDiscoveryRequestMessage,
    HomeAgentAddressDiscoveryReplyMessage,
    MobilePrefixSolicitation,
    MobilePrefixAdvertisement,
    CertificationPathSolicitationMessage,
    CertificationPathAdvertisementMessage,
    MulticastRouterAdvertisement,
    MulticastRouterSolicitation,
    MulticastRouterTermination,
    FMIPv6Messages,
    RPLControlMessage,
    ILNPv6LocatorUpdateMessage,
    DuplicateAddressRequest,
    DuplicateAddressConfirmation,
    MPLControlMessage,
    ExtendedEchoRequest,
    ExtendedEchoReply,
    #[default]
    Unknown,
}

impl IcmpTypeV6 {
    pub fn from_etherparse(icmpv6type: &Icmpv6Type) -> IcmpType {
        IcmpType::V6(match icmpv6type {
            Icmpv6Type::DestinationUnreachable(_) => Self::DestinationUnreachable,
            Icmpv6Type::PacketTooBig { .. } => Self::PacketTooBig,
            Icmpv6Type::TimeExceeded(_) => Self::TimeExceeded,
            Icmpv6Type::ParameterProblem(_) => Self::ParameterProblem,
            Icmpv6Type::EchoRequest(_) => Self::EchoRequest,
            Icmpv6Type::EchoReply(_) => Self::EchoReply,
            Icmpv6Type::Unknown { type_u8: id, .. } => match id {
                130 => Self::MulticastListenerQuery,
                131 => Self::MulticastListenerReport,
                132 => Self::MulticastListenerDone,
                133 => Self::RouterSolicitation,
                134 => Self::RouterAdvertisement,
                135 => Self::NeighborSolicitation,
                136 => Self::NeighborAdvertisement,
                137 => Self::RedirectMessage,
                138 => Self::RouterRenumbering,
                139 => Self::ICMPNodeInformationQuery,
                140 => Self::ICMPNodeInformationResponse,
                141 => Self::InverseNeighborDiscoverySolicitationMessage,
                142 => Self::InverseNeighborDiscoveryAdvertisementMessage,
                143 => Self::Version2MulticastListenerReport,
                144 => Self::HomeAgentAddressDiscoveryRequestMessage,
                145 => Self::HomeAgentAddressDiscoveryReplyMessage,
                146 => Self::MobilePrefixSolicitation,
                147 => Self::MobilePrefixAdvertisement,
                148 => Self::CertificationPathSolicitationMessage,
                149 => Self::CertificationPathAdvertisementMessage,
                151 => Self::MulticastRouterAdvertisement,
                152 => Self::MulticastRouterSolicitation,
                153 => Self::MulticastRouterTermination,
                154 => Self::FMIPv6Messages,
                155 => Self::RPLControlMessage,
                156 => Self::ILNPv6LocatorUpdateMessage,
                157 => Self::DuplicateAddressRequest,
                158 => Self::DuplicateAddressConfirmation,
                159 => Self::MPLControlMessage,
                160 => Self::ExtendedEchoRequest,
                161 => Self::ExtendedEchoReply,
                _ => Self::Unknown,
            },
        })
    }
}

impl Display for IcmpTypeV6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IcmpTypeV6::DestinationUnreachable => "Destination Unreachable",
                IcmpTypeV6::PacketTooBig => "Packet Too Big",
                IcmpTypeV6::TimeExceeded => "Time Exceeded",
                IcmpTypeV6::ParameterProblem => "Parameter Problem",
                IcmpTypeV6::EchoRequest => "Echo Request",
                IcmpTypeV6::EchoReply => "Echo Reply",
                IcmpTypeV6::MulticastListenerQuery => "Multicast Listener Query",
                IcmpTypeV6::MulticastListenerReport => "Multicast Listener Report",
                IcmpTypeV6::MulticastListenerDone => "Multicast Listener Done",
                IcmpTypeV6::RouterSolicitation => "Router Solicitation",
                IcmpTypeV6::RouterAdvertisement => "Router Advertisement",
                IcmpTypeV6::NeighborSolicitation => "Neighbor Solicitation",
                IcmpTypeV6::NeighborAdvertisement => "Neighbor Advertisement",
                IcmpTypeV6::RedirectMessage => "Redirect Message",
                IcmpTypeV6::RouterRenumbering => "Router Renumbering",
                IcmpTypeV6::ICMPNodeInformationQuery => "ICMP Node Information Query",
                IcmpTypeV6::ICMPNodeInformationResponse => "ICMP Node Information Response",
                IcmpTypeV6::InverseNeighborDiscoverySolicitationMessage =>
                    "Inverse Neighbor Discovery Solicitation Message",
                IcmpTypeV6::InverseNeighborDiscoveryAdvertisementMessage =>
                    "Inverse Neighbor Discovery Advertisement Message",
                IcmpTypeV6::Version2MulticastListenerReport =>
                    "Version 2 Multicast Listener Report",
                IcmpTypeV6::HomeAgentAddressDiscoveryRequestMessage =>
                    "Home Agent Address Discovery Request Message",
                IcmpTypeV6::HomeAgentAddressDiscoveryReplyMessage =>
                    "Home Agent Address Discovery Reply Message",
                IcmpTypeV6::MobilePrefixSolicitation => "Mobile Prefix Solicitation",
                IcmpTypeV6::MobilePrefixAdvertisement => "Mobile Prefix Advertisement",
                IcmpTypeV6::CertificationPathSolicitationMessage =>
                    "Certification Path Solicitation Message",
                IcmpTypeV6::CertificationPathAdvertisementMessage =>
                    "Certification Path Advertisement Message",
                IcmpTypeV6::MulticastRouterAdvertisement => "Multicast Router Advertisement",
                IcmpTypeV6::MulticastRouterSolicitation => "Multicast Router Solicitation",
                IcmpTypeV6::MulticastRouterTermination => "Multicast Router Termination",
                IcmpTypeV6::FMIPv6Messages => "FMIPv6 Messages",
                IcmpTypeV6::RPLControlMessage => "RPL Control Message",
                IcmpTypeV6::ILNPv6LocatorUpdateMessage => "ILNPv6 Locator Update Message",
                IcmpTypeV6::DuplicateAddressRequest => "Duplicate Address Request",
                IcmpTypeV6::DuplicateAddressConfirmation => "Duplicate Address Confirmation",
                IcmpTypeV6::MPLControlMessage => "MPL Control Message",
                IcmpTypeV6::ExtendedEchoRequest => "Extended Echo Request",
                IcmpTypeV6::ExtendedEchoReply => "Extended Echo Reply",
                IcmpTypeV6::Unknown => "?",
            }
        )
    }
}
