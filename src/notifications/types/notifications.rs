use serde::{Deserialize, Serialize};

use crate::ByteMultiple;
use crate::gui::types::conf::deserialize_or_default;
use crate::networking::types::data_representation::DataRepr;
use crate::notifications::types::sound::Sound;

/// Used to contain the notifications configuration set by the user
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default)]
pub struct Notifications {
    #[serde(deserialize_with = "deserialize_or_default")]
    pub volume: u8,
    // ---------------------------------------------------------------------------------------------
    #[serde(deserialize_with = "deserialize_or_default")]
    pub data_notification: DataNotification,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub favorite_notification: SimpleNotification,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub ip_blacklist_notification: SimpleNotification,
    #[allow(clippy::struct_field_names)]
    #[serde(deserialize_with = "deserialize_or_default")]
    pub remote_notifications: RemoteNotifications,
}

impl Default for Notifications {
    fn default() -> Self {
        Notifications {
            volume: 50,
            data_notification: DataNotification::default(),
            favorite_notification: SimpleNotification {
                is_active: false,
                sound: Sound::Pop,
            },
            ip_blacklist_notification: SimpleNotification {
                is_active: false,
                sound: Sound::Swhoosh,
            },
            remote_notifications: RemoteNotifications::default(),
        }
    }
}

/// Enum representing the possible notifications.
#[derive(Debug, Clone, Copy)]
pub enum Notification {
    /// Data notification
    Data(DataNotification),
    /// Favorites notification
    Favorite(SimpleNotification),
    /// IP Blacklist notification
    IpBlacklist(SimpleNotification),
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
#[serde(default)]
pub struct DataNotification {
    /// The sound to emit
    #[serde(deserialize_with = "deserialize_or_default")]
    pub sound: Sound,
    /// Data representation
    #[serde(deserialize_with = "deserialize_or_default")]
    pub data_repr: DataRepr,
    /// Threshold of received + sent bytes; if exceeded a notification is emitted
    #[serde(deserialize_with = "deserialize_or_default")]
    pub threshold: Option<u64>,
    /// B, KB, MB or GB
    #[serde(deserialize_with = "deserialize_or_default")]
    pub byte_multiple: ByteMultiple,
    /// The last used Some value for the threshold field
    #[serde(deserialize_with = "deserialize_or_default")]
    pub previous_threshold: u64,
}

impl Default for DataNotification {
    fn default() -> Self {
        DataNotification {
            data_repr: DataRepr::default(),
            threshold: None,
            byte_multiple: ByteMultiple::KB,
            sound: Sound::Gulp,
            previous_threshold: 800_000,
        }
    }
}

impl DataNotification {
    /// Arbitrary string constructor. Will fallback values to existing notification if set, or default otherwise
    pub fn from(value: &str, existing: Option<Self>) -> Self {
        let default = existing.unwrap_or_default();

        let mut byte_multiple_inserted = ByteMultiple::B;
        let chars: Vec<char> = value.trim().chars().collect();
        let new_threshold = if chars.is_empty() {
            0
        } else if !chars.iter().map(|c| char::is_numeric(*c)).any(|x| !x) {
            // no multiple
            value.parse::<u64>().unwrap_or(default.previous_threshold)
        } else {
            // multiple
            let last_char = chars.last().unwrap_or(&' ');
            byte_multiple_inserted = ByteMultiple::from_char(*last_char);
            let without_multiple: String = chars[0..chars.len() - 1].iter().collect();
            if without_multiple.parse::<u64>().is_ok()
                && TryInto::<u64>::try_into(
                    without_multiple.parse::<u128>().unwrap_or_default()
                        * u128::from(byte_multiple_inserted.multiplier()),
                )
                .is_ok()
            {
                without_multiple.parse::<u64>().unwrap_or_default()
                    * byte_multiple_inserted.multiplier()
            } else if without_multiple.is_empty() {
                byte_multiple_inserted = ByteMultiple::B;
                0
            } else {
                byte_multiple_inserted = default.byte_multiple;
                default.previous_threshold
            }
        };
        Self {
            threshold: Some(new_threshold),
            previous_threshold: new_threshold,
            byte_multiple: byte_multiple_inserted,
            ..default
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy, Default)]
#[serde(default)]
pub struct SimpleNotification {
    /// Flag to determine if this notification is enabled
    #[serde(deserialize_with = "deserialize_or_default")]
    pub is_active: bool,
    /// The sound to emit
    #[serde(deserialize_with = "deserialize_or_default")]
    pub sound: Sound,
}

impl SimpleNotification {
    /// Constructor when the notification is in use
    pub fn on(sound: Sound) -> Self {
        Self {
            is_active: true,
            sound,
        }
    }

    /// Constructor when the notification is not in use. Note that sound is used here for caching, although it won't actively be used.
    pub fn off(sound: Sound) -> Self {
        Self {
            is_active: false,
            sound,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RemoteNotifications {
    /// Flag to determine if remote notifications are enabled
    #[serde(deserialize_with = "deserialize_or_default")]
    is_active: bool,
    /// The URL to send notifications to
    #[serde(deserialize_with = "deserialize_or_default")]
    url: String,
}

impl RemoteNotifications {
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.trim().to_string();
    }

    pub fn is_active_and_set(&self) -> bool {
        self.is_active && !self.url.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("123",
        DataNotification{
        previous_threshold: 123, threshold: Some(123), byte_multiple: ByteMultiple::B, ..DataNotification::default() }
    )]
    #[case("500k",
        DataNotification{
        previous_threshold: 500_000, threshold: Some(500_000),byte_multiple: ByteMultiple::KB, ..DataNotification::default() }
    )]
    #[case("420m",
        DataNotification{
        previous_threshold: 420_000_000, threshold: Some(420_000_000),byte_multiple: ByteMultiple::MB, ..DataNotification::default() }
    )]
    #[case("744ь",
        DataNotification{
    previous_threshold: 744, threshold: Some(744),byte_multiple: ByteMultiple::B, ..DataNotification::default() }
    )]
    #[case("888g",
        DataNotification{
        previous_threshold: 888_000_000_000, threshold: Some(888_000_000_000),byte_multiple: ByteMultiple::GB, ..DataNotification::default() }
    )]
    fn test_can_instantiate_bytes_notification_from_string(
        #[case] input: &str,
        #[case] expected: DataNotification,
    ) {
        assert_eq!(expected, DataNotification::from(input, None));
    }

    #[rstest]
    #[case("foob@r")]
    #[case("2O6")]
    fn test_will_reuse_previous_value_if_cannot_parse(#[case] input: &str) {
        let existing_notification = DataNotification {
            previous_threshold: 420_000_000_000,
            byte_multiple: ByteMultiple::GB,
            ..Default::default()
        };
        let expected = DataNotification {
            previous_threshold: 420_000_000_000,
            threshold: Some(420_000_000_000),
            byte_multiple: ByteMultiple::GB,
            ..Default::default()
        };
        assert_eq!(
            expected,
            DataNotification::from(input, Some(existing_notification))
        );
    }

    #[test]
    fn test_can_instantiate_simple_notification() {
        assert_eq!(
            SimpleNotification::on(Sound::Gulp),
            SimpleNotification {
                is_active: true,
                sound: Sound::Gulp
            }
        );
        assert_eq!(
            SimpleNotification::on(Sound::Swhoosh),
            SimpleNotification {
                is_active: true,
                sound: Sound::Swhoosh
            }
        );
        assert_eq!(
            SimpleNotification::off(Sound::Pop),
            SimpleNotification {
                is_active: false,
                sound: Sound::Pop
            }
        );
        assert_eq!(
            SimpleNotification::off(Sound::None),
            SimpleNotification {
                is_active: false,
                sound: Sound::None
            }
        );
    }
}
