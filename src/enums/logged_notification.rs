use crate::structs::address_port_pair::AddressPortPair;
use crate::structs::notifications::{BytesNotification, FavoriteNotification, PacketsNotification};

/// Enum representing the possible observed values of IP protocol version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoggedNotification {
    /// Packets threshold exceeded
    PacketsThresholdExceeded {
        notification: PacketsNotification,
        incoming: u32,
        outgoing: u32,
        timestamp: String,
    },
    /// Byte threshold exceeded
    BytesThresholdExceeded {
        notification: BytesNotification,
        incoming: u32,
        outgoing: u32,
        timestamp: String,
    },
    /// Favorite connection exchanged data
    FavoriteTransmitted {
        notification: FavoriteNotification,
        connection: AddressPortPair,
        timestamp: String,
    },
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
