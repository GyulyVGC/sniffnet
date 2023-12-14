use crate::{AppProtocol, IpVersion, TransProtocol};

/// Fields extracted from a packet to determine if this packet matches the defined filters
#[derive(Clone)]
pub struct PacketFiltersFields {
    /// Internet Protocol version
    pub ip: IpVersion,
    /// Transport layer protocol
    pub transport: TransProtocol,
    /// Application layer protocol
    pub application: AppProtocol,
}

impl Default for PacketFiltersFields {
    fn default() -> Self {
        Self {
            ip: IpVersion::IPv4,
            transport: TransProtocol::TCP,
            application: AppProtocol::Other,
        }
    }
}
