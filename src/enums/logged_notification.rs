use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::info_address_port_pair::InfoAddressPortPair;
use crate::structs::notifications::{BytesNotification, FavoriteNotification, PacketsNotification};

/// Enum representing the possible observed values of IP protocol version.
pub enum LoggedNotification {
    /// Packets threshold exceeded
    PacketsThresholdExceeded(PacketsThresholdExceeded),
    /// Byte threshold exceeded
    BytesThresholdExceeded(BytesThresholdExceeded),
    /// Favorite connection exchanged data
    FavoriteTransmitted(FavoriteTransmitted),
}

#[derive(Clone)]
pub struct PacketsThresholdExceeded {
    pub(crate) notification: PacketsNotification,
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct BytesThresholdExceeded {
    pub(crate) notification: BytesNotification,
    pub(crate) incoming: u32,
    pub(crate) outgoing: u32,
    pub(crate) timestamp: String,
}

#[derive(Clone)]
pub struct FavoriteTransmitted {
    pub(crate) notification: FavoriteNotification,
    pub(crate) connection: (AddressPortPair, InfoAddressPortPair),
    pub(crate) timestamp: String,
}

// impl fmt::Display for LoggedNotification {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{self:?}")
//     }
// }

// impl LoggedNotification {
//     pub(crate) const ALL: [IpVersion; 3] = [IpVersion::IPv4, IpVersion::IPv6, IpVersion::Other];
//
//     pub fn get_radio_label(&self, language: Language) -> &str {
//         match self {
//             IpVersion::IPv4 => "IPv4",
//             IpVersion::IPv6 => "IPv6",
//             IpVersion::Other => both_translation(language),
//         }
//     }
// }
