//! Module defining the `Filters` struct, which represents the possible filters applicable on network traffic.

use crate::{AppProtocol, TransProtocol};


/// Possible filters applicable to network traffic
pub struct Filters {
    pub ip: String,
    pub transport: TransProtocol,
    pub application: AppProtocol,
}