//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::{AppProtocol, IpVersion, TransProtocol};

/// Possible filters applicable to network traffic
#[derive(Clone)]
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
