use serde::{Deserialize, Serialize};

use crate::notifications::types::sound::Sound;
use crate::ByteMultiple;

/// Used to contain the notifications configuration set by the user
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Debug)]
pub struct Notifications {
    pub volume: u8,
    pub packets_notification: PacketsNotification,
    pub bytes_notification: BytesNotification,
    pub favorite_notification: FavoriteNotification,
}

impl Default for Notifications {
    fn default() -> Self {
        Notifications {
            volume: 60,
            packets_notification: PacketsNotification::default(),
            bytes_notification: BytesNotification::default(),
            favorite_notification: FavoriteNotification::default(),
        }
    }
}

/// Enum representing the possible notifications.
#[derive(Debug, Clone, Copy)]
pub enum Notification {
    /// Packets notification
    Packets(PacketsNotification),
    /// Bytes notification
    Bytes(BytesNotification),
    /// Favorites notification
    Favorite(FavoriteNotification),
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct PacketsNotification {
    /// Threshold of received + sent bytes; if exceeded a notification is emitted
    pub threshold: Option<u32>,
    /// The sound to emit
    pub sound: Sound,
    /// The last used Some value for the threshold field
    pub previous_threshold: u32,
}

impl Default for PacketsNotification {
    fn default() -> Self {
        PacketsNotification {
            threshold: None,
            sound: Sound::Gulp,
            previous_threshold: 750,
        }
    }
}

impl PacketsNotification {
    /// Arbitrary string constructor. Will fallback values to existing notification if set, or default otherwise
    pub fn from(value: &str, existing: Option<Self>) -> Self {
        let default = existing.unwrap_or_default();

        let new_threshold = if value.is_empty() {
            0
        } else {
            value.parse().unwrap_or(default.previous_threshold)
        };
        Self {
            threshold: Some(new_threshold),
            previous_threshold: new_threshold,
            ..default
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct BytesNotification {
    /// Threshold of received + sent bytes; if exceeded a notification is emitted
    pub threshold: Option<u64>,
    /// B, KB, MB or GB
    pub byte_multiple: ByteMultiple,
    /// The sound to emit
    pub sound: Sound,
    /// The last used Some value for the threshold field
    pub previous_threshold: u64,
}

impl Default for BytesNotification {
    fn default() -> Self {
        BytesNotification {
            threshold: None,
            byte_multiple: ByteMultiple::KB,
            sound: Sound::Pop,
            previous_threshold: 800_000,
        }
    }
}

impl BytesNotification {
    /// Arbitrary string constructor. Will fallback values to existing notification if set, or default otherwise
    pub fn from(value: &str, existing: Option<Self>) -> Self {
        let default = existing.unwrap_or_default();

        let mut byte_multiple_inserted = ByteMultiple::B;
        let new_threshold = if value.is_empty() {
            0
        } else if !value.trim().chars().map(char::is_numeric).any(|x| !x) {
            // no multiple
            value.parse::<u64>().unwrap_or(default.previous_threshold)
        } else {
            // multiple
            let last_char = value.chars().last().unwrap();
            byte_multiple_inserted = ByteMultiple::from_char(last_char);
            let without_multiple = value[0..value.len() - 1].trim().to_string();
            if without_multiple.parse::<u64>().is_ok()
                && TryInto::<u64>::try_into(
                    without_multiple.parse::<u128>().unwrap()
                        * u128::from(byte_multiple_inserted.multiplier()),
                )
                .is_ok()
            {
                without_multiple.parse::<u64>().unwrap() * byte_multiple_inserted.multiplier()
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

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct FavoriteNotification {
    /// Flag to determine if this notification is enabled
    pub notify_on_favorite: bool,
    /// The sound to emit
    pub sound: Sound,
}

impl Default for FavoriteNotification {
    fn default() -> Self {
        FavoriteNotification {
            notify_on_favorite: false,
            sound: Sound::Swhoosh,
        }
    }
}

impl FavoriteNotification {
    /// Constructor when the notification is in use
    pub fn on(sound: Sound) -> Self {
        FavoriteNotification {
            notify_on_favorite: true,
            sound,
        }
    }

    /// Constructor when the notification is not in use. Note that sound is used here for caching, although it won't actively be used.
    pub fn off(sound: Sound) -> Self {
        FavoriteNotification {
            notify_on_favorite: false,
            sound,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("123", BytesNotification {
        previous_threshold: 123, threshold: Some(123), byte_multiple: ByteMultiple::B, ..BytesNotification::default() })]
    #[case("500k", BytesNotification {
        previous_threshold: 500_000, threshold: Some(500_000),byte_multiple: ByteMultiple::KB, ..BytesNotification::default() })]
    #[case("420 m", BytesNotification {
        previous_threshold: 420_000_000, threshold: Some(420_000_000),byte_multiple: ByteMultiple::MB, ..BytesNotification::default() })]
    #[case("foob@r", BytesNotification{
        threshold: Some(800000),
        ..Default::default()
    })]
    #[case(" 888 g", BytesNotification {
        previous_threshold: 888_000_000_000, threshold: Some(888_000_000_000),byte_multiple: ByteMultiple::GB, ..BytesNotification::default() })]
    fn test_can_instantiate_bytes_notification_from_string(
        #[case] input: &str,
        #[case] expected: BytesNotification,
    ) {
        assert_eq!(expected, BytesNotification::from(input, None));
    }

    #[rstest]
    #[case("foob@r")]
    #[case("2O6")]
    fn test_will_reuse_previous_value_if_cannot_parse(#[case] input: &str) {
        let existing_notification = BytesNotification {
            previous_threshold: 420_000_000_000,
            byte_multiple: ByteMultiple::GB,
            ..Default::default()
        };
        let expected = BytesNotification {
            previous_threshold: 420_000_000_000,
            threshold: Some(420_000_000_000),
            byte_multiple: ByteMultiple::GB,
            ..Default::default()
        };
        assert_eq!(
            expected,
            BytesNotification::from(input, Some(existing_notification))
        );
    }

    #[test]
    fn test_can_instantiate_favourite_notification() {
        assert_eq!(
            FavoriteNotification::on(Sound::Gulp),
            FavoriteNotification {
                notify_on_favorite: true,
                sound: Sound::Gulp
            }
        );
        assert_eq!(
            FavoriteNotification::on(Sound::Swhoosh),
            FavoriteNotification {
                notify_on_favorite: true,
                sound: Sound::Swhoosh
            }
        );
        assert_eq!(
            FavoriteNotification::off(Sound::Pop),
            FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::Pop
            }
        );
        assert_eq!(
            FavoriteNotification::off(Sound::None),
            FavoriteNotification {
                notify_on_favorite: false,
                sound: Sound::None
            }
        );
    }

    #[rstest]
    #[case("123", PacketsNotification {
        previous_threshold: 123,
        threshold: Some(123),
        ..PacketsNotification::default() })]
    #[case("8888", PacketsNotification {
        previous_threshold: 8888,
        threshold: Some(8888),
        ..PacketsNotification::default() })]
    #[case("420 m", PacketsNotification {
        threshold: Some(750),
        ..PacketsNotification::default() })]
    #[case("foob@r", PacketsNotification {
        threshold: Some(750),
        ..PacketsNotification::default() })]
    fn test_can_instantiate_packet_notification_from_string(
        #[case] input: &str,
        #[case] expected: PacketsNotification,
    ) {
        assert_eq!(expected, PacketsNotification::from(input, None));
    }
}
