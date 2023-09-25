//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::{AppProtocol, IpVersion, TransProtocol};

/// Possible filters applicable to network traffic
#[derive(Clone, Copy)]
pub struct Filters {
    /// Internet Protocol version
    pub ip: IpVersion,
    /// Transport layer protocol
    pub transport: TransProtocol,
    /// Application layer protocol
    pub application: AppProtocol,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            ip: IpVersion::Other,
            transport: TransProtocol::Other,
            application: AppProtocol::Other,
        }
    }
}

impl Filters {
    /// Checks whether the filters match the current packet's protocols
    pub fn matches(self, rhs: Self) -> bool {
        (self.ip.eq(&IpVersion::Other) || self.ip.eq(&rhs.ip))
            && (self.transport.eq(&TransProtocol::Other) || self.transport.eq(&rhs.transport))
            && (self.application.eq(&AppProtocol::Other) || self.application.eq(&rhs.application))
    }
}
