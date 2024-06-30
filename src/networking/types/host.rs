use crate::countries::types::country::Country;
use crate::networking::types::asn::Asn;

/// Struct to represent a network host
#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Host {
    /// Hostname (domain). Obtained from the reverse DNS.
    pub domain: String,
    /// Autonomous System which operates the host
    pub asn: Asn,
    /// Country
    pub country: Country,
}

/// Struct to represent a network host for representation in the thumbnail
///
/// This is necessary to remove possible duplicates in the thumbnail host list
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq)]
pub struct ThumbnailHost {
    /// Country
    pub country: Country,
    /// Text describing the host in the thumbnail
    pub text: String,
}
