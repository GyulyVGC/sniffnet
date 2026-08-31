#![doc = include_str!("../README.md")]

#[cfg(feature = "full")]
mod arp_type;
#[cfg(feature = "full")]
mod headers;
#[cfg(feature = "full")]
mod icmp_type;
#[cfg(feature = "full")]
mod link_type;
#[cfg(feature = "full")]
mod packet;
mod protocol;

#[cfg(feature = "full")]
pub use arp_type::ArpType;
#[cfg(feature = "full")]
pub use icmp_type::IcmpType;
#[cfg(feature = "full")]
pub use link_type::LinkType;
#[cfg(feature = "full")]
pub use packet::ParsedPacket;
pub use protocol::Protocol;
