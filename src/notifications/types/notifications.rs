use serde::{Deserialize, Serialize};

use crate::ByteMultiple;
use crate::chart::types::chart_type::ChartType;
use crate::notifications::types::sound::Sound;

/// Used to contain the notifications configuration set by the user
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Debug)]
pub struct Notifications {
    pub volume: u8,
    pub data_notification: DataNotification,
    pub favorite_notification: FavoriteNotification,
}

impl Default for Notifications {
    fn default() -> Self {
        Notifications {
            volume: 60,
            data_notification: DataNotification::default(),
            favorite_notification: FavoriteNotification::default(),
        }
    }
}

/// Enum representing the possible notifications.
#[derive(Debug, Clone, Copy)]
pub enum Notification {
    /// Data notification
    Data(DataNotification),
    /// Favorites notification
    Favorite(FavoriteNotification),
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Copy)]
pub struct DataNotification {
    /// Data representation
    pub chart_type: ChartType,
    /// Threshold of received + sent bytes; if exceeded a notification is emitted
    pub threshold: Option<u64>,
    /// B, KB, MB or GB
    pub byte_multiple: ByteMultiple,
    /// The sound to emit
    pub sound: Sound,
    /// The last used Some value for the threshold field
    pub previous_threshold: u64,
}

impl Default for DataNotification {
    fn default() -> Self {
        DataNotification {
            chart_type: ChartType::Bytes,
            threshold: None,
            byte_multiple: ByteMultiple::KB,
            sound: Sound::Pop,
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
}
