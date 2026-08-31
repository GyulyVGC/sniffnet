//! Link-layer and packet header parsing shared by Sniffnet and Sniffnet Agent.
//!
//! [`parse`] turns a raw `pcap` packet into a [`ParsedPacket`]: addresses,
//! ports, protocol, MACs, and byte count. Everything above that — flow
//! aggregation, direction, services, IPFIX export — belongs to the caller.

pub mod arp_type;
pub mod headers;
pub mod icmp_type;
pub mod link_type;
pub mod packet;
pub mod protocol;

pub use arp_type::ArpType;
pub use headers::LinkInfo;
pub use icmp_type::{IcmpType, IcmpTypeV4, IcmpTypeV6};
pub use link_type::LinkType;
pub use packet::ParsedPacket;
pub use protocol::Protocol;
