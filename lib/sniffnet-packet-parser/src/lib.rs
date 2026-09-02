#![doc = include_str!("../README.md")]

#[cfg(feature = "full")]
mod arp_type;
#[cfg(feature = "full")]
mod headers;
#[cfg(feature = "full")]
mod icmp_type;
#[cfg(feature = "full")]
mod igmp_type;
#[cfg(feature = "full")]
mod link_type;
#[cfg(feature = "full")]
mod packet;
mod protocol;

#[cfg(feature = "full")]
pub use arp_type::ArpType;
#[cfg(feature = "full")]
pub use headers::{LinkInfo, NetInfo, TransportInfo};
#[cfg(feature = "full")]
pub use icmp_type::{IcmpType, IcmpTypeV4, IcmpTypeV6};
#[cfg(feature = "full")]
pub use igmp_type::IgmpType;
#[cfg(feature = "full")]
pub use link_type::LinkType;
#[cfg(feature = "full")]
pub use packet::ParsedPacket;
pub use protocol::Protocol;
