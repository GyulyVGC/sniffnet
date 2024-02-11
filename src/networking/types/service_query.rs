// WARNING: this file is imported in build.rs

use std::hash::Hash;

/// Used to query the phf services map (key of the map).
#[derive(Hash, Eq, PartialEq)]
pub struct ServiceQuery(pub u16, pub crate::Protocol);

impl phf_shared::PhfHash for ServiceQuery {
    fn phf_hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let ServiceQuery(port, protocol) = self;
        port.hash(state);
        protocol.hash(state);
    }
}

impl phf_shared::PhfBorrow<ServiceQuery> for ServiceQuery {
    fn borrow(&self) -> &ServiceQuery {
        self
    }
}

impl phf_shared::FmtConst for ServiceQuery {
    fn fmt_const(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ServiceQuery(port, protocol) = self;
        write!(f, "ServiceQuery({port}, Protocol::{protocol})",)
    }
}
